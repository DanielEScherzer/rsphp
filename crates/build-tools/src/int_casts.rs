use bindgen::callbacks::{IntKind, ParseCallbacks, Token, TokenKind};
use std::cell::RefCell;
use std::collections::HashMap;
use std::str;

/// Actual implementation of the `IntTypeCastCallbacks` callbacks.
#[derive(Debug)]
struct IntTypeCastCallbacksImpl {
    /// Map of macros that have been redefined for a specific type, and still
    /// need to have their type overridden.
    pending: HashMap<String, IntKind>,
}

/// Nicer version of [`bindgen::callbacks::Token`] that stores the values as
/// readable strings.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedToken<'tok> {
    /// Token kind, from bindgen.
    kind: TokenKind,
    /// Token content, from parsing the bindgen value.
    value: &'tok str,
}

impl<'tok> ParsedToken<'tok> {
    /// Try to create a `ParsedToken` from a [`bindgen::callbacks::Token`].
    fn from_raw(token: &'tok Token) -> Result<Self, str::Utf8Error> {
        str::from_utf8(&token.raw).map(|parsed_value| ParsedToken {
            kind: token.kind,
            value: parsed_value,
        })
    }

    /// If the overall tokens start with an opening parenthesis and end with
    /// the closing parenthesis that matches that opening, remove both the
    /// opening and the closing.
    fn trim_wrapping_parens(tokens: &mut Vec<ParsedToken<'tok>>) {
        let Some(ParsedToken {
            kind: TokenKind::Punctuation,
            value: "(",
        }) = tokens.first()
        else {
            return;
        };
        let Some(ParsedToken {
            kind: TokenKind::Punctuation,
            value: ")",
        }) = tokens.last()
        else {
            return;
        };
        // Starts and ends with parentheses, see if they match
        let last = tokens.len() - 1;
        let mut nesting_level: usize = 0;
        for (index, tok) in tokens.iter().enumerate() {
            match tok {
                ParsedToken {
                    kind: TokenKind::Punctuation,
                    value: "(",
                } => nesting_level += 1,
                ParsedToken {
                    kind: TokenKind::Punctuation,
                    value: ")",
                } => nesting_level -= 1,
                _ => (),
            }
            if nesting_level == 0 {
                if index != last {
                    return;
                }
                break;
            }
        }
        // If we got here, the parens should be trimmed
        assert_eq!(
            Some(ParsedToken {
                kind: TokenKind::Punctuation,
                value: ")"
            }),
            tokens.pop(),
            "Tokens were checked to end with a parenthesis",
        );
        assert_eq!(
            ParsedToken {
                kind: TokenKind::Punctuation,
                value: "("
            },
            tokens.remove(0),
            "Tokens were checked to start with a parenthesis",
        );
    }
}

impl IntTypeCastCallbacksImpl {
    /// Create a new instance that has not processed anything yet.
    fn new() -> Self {
        IntTypeCastCallbacksImpl {
            pending: HashMap::new(),
        }
    }

    /// Implementation of parser callback to specify an integer macro type,
    /// applying any pending override from a modification.
    fn int_macro(&mut self, name: &str) -> Option<IntKind> {
        self.pending.remove(name)
    }

    /// Implementation of parser callback to modify a macro, replacing C casts
    /// with the resulting value if applicable.
    fn modify_macro(&mut self, name: &str, tokens: &mut Vec<Token>) {
        let Ok(mut parsed) = tokens
            .iter()
            .map(ParsedToken::from_raw)
            .collect::<Result<Vec<_>, _>>()
        else {
            return;
        };
        // Double check that the macro starts with the name
        if parsed.is_empty() {
            return;
        }
        let first = parsed.remove(0);
        if first.kind != TokenKind::Identifier || first.value != name {
            return;
        }
        ParsedToken::trim_wrapping_parens(&mut parsed);

        let [
            ParsedToken {
                kind: TokenKind::Punctuation,
                value: "(",
            },
            ParsedToken {
                kind: TokenKind::Identifier,
                value: cast_target_type,
            },
            ParsedToken {
                kind: TokenKind::Punctuation,
                value: ")",
            },
            ParsedToken {
                kind: TokenKind::Punctuation,
                value: "-",
            },
            ParsedToken {
                kind: TokenKind::Literal,
                value: cast_target_value,
            },
        ] = parsed[..]
        else {
            return;
        };

        match cast_target_type {
            "uint32_t" => {
                let Ok(value) = cast_target_value.parse::<u32>() else {
                    return;
                };
                let new_value: u32 = u32::MIN.wrapping_sub(value);
                #[expect(clippy::panic, reason = "No nicer option in HashMap")]
                match self.pending.insert(name.to_owned(), IntKind::U32) {
                    None => (),
                    Some(old) => panic!("Redefining macro {name}, previously was {old:?}"),
                }
                tokens.truncate(1);
                tokens.push({
                    Token {
                        kind: TokenKind::Literal,
                        raw: new_value.to_string().into_bytes().into_boxed_slice(),
                    }
                });
            }
            "uint16_t" => {
                let Ok(value) = cast_target_value.parse::<u16>() else {
                    return;
                };
                let new_value: u16 = u16::MIN.wrapping_sub(value);
                #[expect(clippy::panic, reason = "No nicer option in HashMap")]
                match self.pending.insert(name.to_owned(), IntKind::U16) {
                    None => (),
                    Some(old) => panic!("Redefining macro {name}, previously was {old:?}"),
                }
                tokens.truncate(1);
                tokens.push({
                    Token {
                        kind: TokenKind::Literal,
                        raw: new_value.to_string().into_bytes().into_boxed_slice(),
                    }
                });
            }
            _ => (),
        }
    }
}

impl Drop for IntTypeCastCallbacksImpl {
    #[expect(clippy::print_stderr, reason = "Better to print than to double panic")]
    fn drop(&mut self) {
        // Avoid double panics if we didn't get through reading all of the tokens
        if !self.pending.is_empty() && std::thread::panicking() {
            eprintln!(
                "IntTypeCastCallbacks: Not all type overrides were used, but thread is already panicking"
            );
            return;
        }
        assert!(
            self.pending.is_empty(),
            "All type overrides should be used, remaining: {:?}",
            self.pending
        );
    }
}

/// Callbacks to fix handling of C macros that are defined as integer casts.
#[derive(Debug)]
pub struct IntTypeCastCallbacks {
    /// The actual implementation, wrapped in a `RefCell` because it needs to
    /// allow mutability.
    inner: RefCell<IntTypeCastCallbacksImpl>,
}

#[expect(
    clippy::new_without_default,
    reason = "Default isn't really applicable"
)]
impl IntTypeCastCallbacks {
    /// Create a new callbacks instance to handle int macro casts.
    #[must_use]
    pub fn new() -> Self {
        IntTypeCastCallbacks {
            inner: RefCell::new(IntTypeCastCallbacksImpl::new()),
        }
    }
}

impl ParseCallbacks for IntTypeCastCallbacks {
    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        self.inner.borrow_mut().int_macro(name)
    }

    fn modify_macro(&self, name: &str, tokens: &mut Vec<Token>) {
        self.inner.borrow_mut().modify_macro(name, tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_parens() {
        use bindgen::callbacks::TokenKind::*;
        let mut tokens = vec![
            ParsedToken {
                kind: Punctuation,
                value: "(",
            },
            ParsedToken {
                kind: Literal,
                value: "5",
            },
            ParsedToken {
                kind: Punctuation,
                value: ")",
            },
        ];
        ParsedToken::trim_wrapping_parens(&mut tokens);
        assert_eq!(
            vec![ParsedToken {
                kind: Literal,
                value: "5"
            }],
            tokens
        );
    }
}
