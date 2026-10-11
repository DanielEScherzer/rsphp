use bindgen::callbacks::{ParseCallbacks, Token, TokenKind};
use std::cell::RefCell;

/// Actual implementation of the `BuildIdCallbacks` callbacks.
#[derive(Debug)]
struct BuildIdCallbacksImpl {
    /// The API number to use, from the `ZEND_MODULE_API_NO` macro.
    ///
    /// `None` until the macro is processed.
    api_no: Option<String>,
    /// Either `ZTS` or `NTS`, from the `ZEND_BUILD_TS` macro.
    ///
    /// `None` until the macro is processed.
    build_ts: Option<&'static str>,
    /// Either an empty string or `,debug`, from the `ZEND_BUILD_DEBUG` macro.
    ///
    /// `None` until the macro is processed.
    build_debug: Option<&'static str>,
    /// Whether the build ID was actually updated, if this is still
    /// false when the object is `drop`ed it will panic.
    updated_build_id: bool,
}

/// Helper to create a token for the given kind and content for comparison.
macro_rules! make_token {
    ( $kind:ident, $str:literal ) => {
        Token {
            kind: TokenKind::$kind,
            raw: $str.to_string().into_bytes().into_boxed_slice(),
        }
    };
}

impl BuildIdCallbacksImpl {
    /// Create a new instance that has not processed anything yet.
    fn new() -> Self {
        BuildIdCallbacksImpl {
            api_no: None,
            build_ts: None,
            build_debug: None,
            updated_build_id: false,
        }
    }

    /// Process the declaration of the `ZEND_MODULE_API_NO` macro.
    fn read_api_no(&mut self, tokens: &Vec<Token>) {
        assert_eq!(
            None, self.api_no,
            "ZEND_MODULE_API_NO should not have been read yet"
        );
        assert_eq!(
            2,
            tokens.len(),
            "ZEND_MODULE_API_NO should have 2 tokens, the name and the number, got: {tokens:?}"
        );
        assert_eq!(
            make_token!(Identifier, "ZEND_MODULE_API_NO"),
            tokens[0],
            "ZEND_MODULE_API_NO is being declared"
        );
        assert_eq!(
            TokenKind::Literal,
            tokens[1].kind,
            "ZEND_MODULE_API_NO should be a literal"
        );
        self.api_no = Some(
            std::str::from_utf8(&tokens[1].raw)
                .expect("Should be valid UTF8")
                .to_owned(),
        );
    }

    /// Process the declaration of the `ZEND_BUILD_TS` macro.
    fn read_build_ts(&mut self, tokens: &Vec<Token>) {
        assert_eq!(
            None, self.build_ts,
            "ZEND_BUILD_TS should not have been read yet"
        );
        assert_eq!(
            2,
            tokens.len(),
            "ZEND_BUILD_TS should have 2 tokens, the name and the timestamp, got: {tokens:?}"
        );
        assert_eq!(
            make_token!(Identifier, "ZEND_BUILD_TS"),
            tokens[0],
            "ZEND_BUILD_TS is being declared"
        );
        if tokens[1] == make_token!(Literal, "\",TS\"") {
            self.build_ts = Some("TS");
        } else {
            assert_eq!(
                make_token!(Literal, "\",NTS\""),
                tokens[1],
                "ZEND_BUILD_TS is either TS or NTS"
            );
            self.build_ts = Some("NTS");
        }
    }

    /// Process the declaration of the `ZEND_BUILD_DEBUG` macro.
    fn read_build_debug(&mut self, tokens: &Vec<Token>) {
        assert_eq!(
            None, self.build_debug,
            "ZEND_BUILD_DEBUG should not have been read yet"
        );
        assert!(
            !tokens.is_empty(),
            "ZEND_BUILD_DEBUG should have 1 or 2 tokens, the name either '1' or no second token, got: {tokens:?}"
        );
        assert_eq!(
            make_token!(Identifier, "ZEND_BUILD_DEBUG"),
            tokens[0],
            "ZEND_BUILD_DEBUG is being declared"
        );
        if tokens.len() == 1 {
            self.build_debug = Some("");
        } else {
            assert_eq!(
                2,
                tokens.len(),
                "ZEND_BUILD_DEBUG cannot have more than 2 tokens"
            );
            assert_eq!(
                make_token!(Literal, "\",debug\""),
                tokens[1],
                "ZEND_BUILD_DEBUG can only be \",debug\" if set"
            );
            self.build_debug = Some(",debug");
        }
    }

    /// Given the tokens for the `ZEND_MODULE_BUILD_ID` macro, replace them with
    /// a generated version that bindgen can use.
    fn replace_build_id(&mut self, tokens: &mut Vec<Token>) {
        assert!(
            !self.updated_build_id,
            "ZEND_MODULE_BUILD_ID should not have been updated yet"
        );
        macro_rules! make_tokens {
            ( $( ( $kind:ident, $str:literal ) ),+ ) => { vec![ $( make_token!( $kind, $str ) ),+ ] }
        }
        assert_eq!(
            &make_tokens!(
                (Identifier, "ZEND_MODULE_BUILD_ID"),
                (Literal, "\"API\""),
                (Identifier, "ZEND_TOSTR"),
                (Punctuation, "("),
                (Identifier, "ZEND_MODULE_API_NO"),
                (Punctuation, ")"),
                (Identifier, "ZEND_BUILD_TS"),
                (Identifier, "ZEND_BUILD_DEBUG"),
                (Identifier, "ZEND_BUILD_SYSTEM"),
                (Identifier, "ZEND_BUILD_EXTRA")
            ),
            tokens,
            "ZEND_MODULE_BUILD_ID is being declared",
        );
        assert!(self.api_no.is_some(), "API number should be read already");
        assert!(self.build_ts.is_some(), "Build ts should be read already");
        assert!(
            self.build_debug.is_some(),
            "Build debug should be read already"
        );
        tokens.truncate(1);
        tokens.push(Token {
            kind: TokenKind::Literal,
            raw: format!(
                "\"API{},{}{}\"",
                self.api_no.as_ref().expect("Confirmed to not be none"),
                self.build_ts.as_ref().expect("Confirmed to not be none"),
                self.build_debug.as_ref().expect("Confirmed to not be none"),
            )
            .into_bytes()
            .into_boxed_slice(),
        });
        self.updated_build_id = true;
    }
}

impl Drop for BuildIdCallbacksImpl {
    #[expect(clippy::print_stderr, reason = "Better to print than to double panic")]
    fn drop(&mut self) {
        // Avoid double panics if we didn't get through reading all of the tokens
        if !self.updated_build_id
            && (self.api_no.is_none() || self.build_ts.is_none() || self.build_debug.is_none())
            && std::thread::panicking()
        {
            eprintln!(
                "BuildIdCallbacks: Build ID was not updated, but thread is already panicking"
            );
            return;
        }
        // Chck the preconditions so that failures are clearer
        assert!(self.api_no.is_some(), "Drop: missing API number");
        assert!(self.build_ts.is_some(), "Drop: missing build ts");
        assert!(self.build_debug.is_some(), "Drop: missing build debug");
        assert!(self.updated_build_id, "Drop: build ID not updated");
    }
}

/// Callbacks to modify the `ZEND_MODULE_BUILD_ID` C macro to make it usable
/// by bindgen.
#[derive(Debug)]
pub struct BuildIdCallbacks {
    /// The actual implementation, wrapped in a `RefCell` because it needs to
    /// allow mutability.
    inner: RefCell<BuildIdCallbacksImpl>,
}

#[expect(
    clippy::new_without_default,
    reason = "Default isn't really applicable"
)]
impl BuildIdCallbacks {
    #[must_use]
    /// Create a new callbacks instance to process macro definitions.
    pub fn new() -> Self {
        BuildIdCallbacks {
            inner: RefCell::new(BuildIdCallbacksImpl::new()),
        }
    }
}

impl ParseCallbacks for BuildIdCallbacks {
    fn modify_macro(&self, name: &str, tokens: &mut Vec<Token>) {
        let mut inner = self.inner.borrow_mut();
        match name {
            "ZEND_MODULE_API_NO" => inner.read_api_no(tokens),
            "ZEND_BUILD_TS" => inner.read_build_ts(tokens),
            "ZEND_BUILD_DEBUG" => inner.read_build_debug(tokens),
            "ZEND_MODULE_BUILD_ID" => inner.replace_build_id(tokens),
            _ => (),
        }
    }
}
