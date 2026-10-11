//! Tests for the `BuildIdCallbacks` parser callbacks.

use bindgen::callbacks::{ParseCallbacks as _, Token, TokenKind};
use rsphp_build_tools::BuildIdCallbacks;

macro_rules! make_tokens {
    ( correct_tokens_for_build_id ) => {
        make_tokens!([
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
        ])
    };
    ( [ $( ( $kind:ident, $str:literal ) ),* ] ) => {
        vec![
            $(
                Token { kind: TokenKind::$kind, raw: $str.to_string().into_bytes().into_boxed_slice(), },
            )*
        ]
    };
}

macro_rules! test_failure {
    (
        $name:ident,
        #[$expectation:meta]
        [ $( $macro_name:literal => $tokens:tt, )* ]
    ) => {
        #[test]
        #[$expectation]
        fn $name() {
            let callbacks = BuildIdCallbacks::new();
            $( callbacks.modify_macro( $macro_name, &mut make_tokens!( $tokens ) ); )*
        }
    }
}

macro_rules! test_success {
    (
        $name:ident,
        [ $( $macro_name:literal => $tokens:tt, )* ],
        $expected_tokens:expr
    ) => {
        #[test]
        fn $name() {
            // Make a slice so that we can easily keep a reference to the last
            // set of tokens, which are going to be mutated
            let mut original = [
                $(
                    ($macro_name, make_tokens!( $tokens ) ),
                )*
            ];
            let callbacks = BuildIdCallbacks::new();
            for pair in original.iter_mut() {
                callbacks.modify_macro(pair.0, &mut pair.1);
            }
            assert_eq!(Some($expected_tokens), original.last().map(|pair| pair.1.clone()));
        }
    }
}

test_failure!(
    api_no_wrong_token_count,
    #[should_panic = "ZEND_MODULE_API_NO should have 2 tokens, the name and the number"]
    [
        "ZEND_MODULE_API_NO" => [(Identifier, "foo")],
    ]
);

test_failure!(
    api_no_wrong_token_ident,
    #[should_panic(expected = "ZEND_MODULE_API_NO is being declared")]
    [
        "ZEND_MODULE_API_NO" => [(Identifier, "foo"), (Identifier, "bar")],
    ]
);

test_failure!(
    api_no_non_literal,
    #[should_panic(expected = "ZEND_MODULE_API_NO should be a literal")]
    [
        "ZEND_MODULE_API_NO" => [(Identifier, "ZEND_MODULE_API_NO"), (Identifier, "20250925")],
    ]
);

test_failure!(
    api_no_twice,
    #[should_panic(expected = "ZEND_MODULE_API_NO should not have been read yet")]
    [
        "ZEND_MODULE_API_NO" => [(Identifier, "ZEND_MODULE_API_NO"), (Literal, "20250925")],
        "ZEND_MODULE_API_NO" => [(Identifier, "ZEND_MODULE_API_NO"), (Literal, "20250925")],
    ]
);

test_failure!(
    build_ts_wrong_token_count,
    #[should_panic = "ZEND_BUILD_TS should have 2 tokens, the name and the timestamp"]
    [
        "ZEND_BUILD_TS" => [(Identifier, "foo")],
    ]
);

test_failure!(
    build_ts_wrong_token_ident,
    #[should_panic(expected = "ZEND_BUILD_TS is being declared")]
    [
        "ZEND_BUILD_TS" => [(Identifier, "foo"), (Identifier, "foo")],
    ]
);

test_failure!(
    build_ts_wrong_token_value,
    #[should_panic(expected = "ZEND_BUILD_TS is either TS or NTS")]
    [
        "ZEND_BUILD_TS" => [(Identifier, "ZEND_BUILD_TS"), (Identifier, "20250925")],
    ]
);

test_failure!(
    build_ts_twice,
    #[should_panic(expected = "ZEND_BUILD_TS should not have been read yet")]
    [
        "ZEND_BUILD_TS" => [(Identifier, "ZEND_BUILD_TS"), (Literal, "\",TS\"")],
        "ZEND_BUILD_TS" => [(Identifier, "ZEND_BUILD_TS"), (Literal, "\",NTS\"")],
    ]
);

test_failure!(
    build_debug_empty,
    #[should_panic = "ZEND_BUILD_DEBUG should have 1 or 2 tokens"]
    [
        "ZEND_BUILD_DEBUG" => [],
    ]
);

test_failure!(
    build_debug_wrong_token_ident,
    #[should_panic(expected = "ZEND_BUILD_DEBUG is being declared")]
    [
        "ZEND_BUILD_DEBUG" => [(Identifier, "foo"), (Identifier, "bar")],
    ]
);

test_failure!(
    build_debug_wrong_token_count,
    #[should_panic(expected = "ZEND_BUILD_DEBUG cannot have more than 2 tokens")]
    [
        "ZEND_BUILD_DEBUG" => [(Identifier, "ZEND_BUILD_DEBUG"), (Identifier, "a"), (Identifier, "b")],
    ]
);

test_failure!(
    build_debug_wrong_value,
    #[should_panic(expected = "ZEND_BUILD_DEBUG can only be \",debug\" if set")]
    [
        "ZEND_BUILD_DEBUG" => [(Identifier, "ZEND_BUILD_DEBUG"), (Identifier, "a")],
    ]
);

test_failure!(
    build_debug_twice,
    #[should_panic(expected = "ZEND_BUILD_DEBUG should not have been read yet")]
    [
        "ZEND_BUILD_DEBUG" => [(Identifier, "ZEND_BUILD_DEBUG"), (Literal, "\",debug\"")],
        "ZEND_BUILD_DEBUG" => [(Identifier, "ZEND_BUILD_DEBUG"), (Literal, "\",debug\"")],
    ]
);

test_failure!(
    build_id_wrong_tokens,
    #[should_panic(expected = "ZEND_MODULE_BUILD_ID is being declared")]
    [
        "ZEND_MODULE_BUILD_ID" => [(Identifier, "ZEND_MODULE_BUILD_ID")],
    ]
);

test_failure!(
    build_id_missing_api_no,
    #[should_panic(expected = "API number should be read already")]
    [
        "ZEND_MODULE_BUILD_ID" => correct_tokens_for_build_id,
    ]
);

test_failure!(
    build_id_missing_build_ts,
    #[should_panic(expected = "Build ts should be read already")]
    [
        "ZEND_MODULE_API_NO" => [(Identifier, "ZEND_MODULE_API_NO"), (Literal, "20250925")],
        "ZEND_MODULE_BUILD_ID" => correct_tokens_for_build_id,
    ]
);

test_failure!(
    build_id_missing_build_debug,
    #[should_panic(expected = "Build debug should be read already")]
    [
        "ZEND_MODULE_API_NO" => [(Identifier, "ZEND_MODULE_API_NO"), (Literal, "20250925")],
        "ZEND_BUILD_TS" => [(Identifier, "ZEND_BUILD_TS"), (Literal, "\",TS\"")],
        "ZEND_MODULE_BUILD_ID" => correct_tokens_for_build_id,
    ]
);

test_failure!(
    module_no_twice,
    #[should_panic(expected = "ZEND_MODULE_BUILD_ID should not have been updated yet")]
    [
        "ZEND_MODULE_API_NO" => [(Identifier, "ZEND_MODULE_API_NO"), (Literal, "20250925")],
        "ZEND_BUILD_TS" => [(Identifier, "ZEND_BUILD_TS"), (Literal, "\",TS\"")],
        "ZEND_BUILD_DEBUG" => [(Identifier, "ZEND_BUILD_DEBUG")],
        "ZEND_MODULE_BUILD_ID" => correct_tokens_for_build_id,
        "ZEND_MODULE_BUILD_ID" => [],
    ]
);

test_failure!(
    drop_no_api_no,
    #[should_panic(expected = "Drop: missing API number")]
    [
        "OTHER" => [],
    ]
);

test_failure!(
    drop_no_build_ts,
    #[should_panic(expected = "Drop: missing build ts")]
    [
        "ZEND_MODULE_API_NO" => [(Identifier, "ZEND_MODULE_API_NO"), (Literal, "20250925")],
        "OTHER" => [],
    ]
);

test_failure!(
    drop_no_build_debug,
    #[should_panic(expected = "Drop: missing build debug")]
    [
        "ZEND_MODULE_API_NO" => [(Identifier, "ZEND_MODULE_API_NO"), (Literal, "20250925")],
        "ZEND_BUILD_TS" => [(Identifier, "ZEND_BUILD_TS"), (Literal, "\",NTS\"")],
        "OTHER" => [],
    ]
);

test_failure!(
    drop_no_build_id,
    #[should_panic(expected = "Drop: build ID not updated")]
    [
        "ZEND_MODULE_API_NO" => [(Identifier, "ZEND_MODULE_API_NO"), (Literal, "20250925")],
        "ZEND_BUILD_TS" => [(Identifier, "ZEND_BUILD_TS"), (Literal, "\",NTS\"")],
        "ZEND_BUILD_DEBUG" => [(Identifier, "ZEND_BUILD_DEBUG")],
        "OTHER" => [],
    ]
);

test_success!(
    normal,
    [
        "ZEND_MODULE_API_NO" => [(Identifier, "ZEND_MODULE_API_NO"), (Literal, "20250925")],
        "ZEND_BUILD_TS" => [(Identifier, "ZEND_BUILD_TS"), (Literal, "\",NTS\"")],
        "ZEND_BUILD_DEBUG" => [(Identifier, "ZEND_BUILD_DEBUG")],
        "ZEND_MODULE_BUILD_ID" => correct_tokens_for_build_id,
    ],
    make_tokens!([
        (Identifier, "ZEND_MODULE_BUILD_ID"),
        (Literal, "\"API20250925,NTS\"")
    ])
);

test_success!(
    debug,
    [
        "ZEND_MODULE_API_NO" => [(Identifier, "ZEND_MODULE_API_NO"), (Literal, "20250925")],
        "ZEND_BUILD_TS" => [(Identifier, "ZEND_BUILD_TS"), (Literal, "\",NTS\"")],
        "ZEND_BUILD_DEBUG" => [(Identifier, "ZEND_BUILD_DEBUG"), (Literal, "\",debug\"")],
        "ZEND_MODULE_BUILD_ID" => correct_tokens_for_build_id,
    ],
    make_tokens!([
        (Identifier, "ZEND_MODULE_BUILD_ID"),
        (Literal, "\"API20250925,NTS,debug\"")
    ])
);

test_success!(
    zts,
    [
        "ZEND_MODULE_API_NO" => [(Identifier, "ZEND_MODULE_API_NO"), (Literal, "20250925")],
        "ZEND_BUILD_TS" => [(Identifier, "ZEND_BUILD_TS"), (Literal, "\",TS\"")],
        "ZEND_BUILD_DEBUG" => [(Identifier, "ZEND_BUILD_DEBUG")],
        "ZEND_MODULE_BUILD_ID" => correct_tokens_for_build_id,
    ],
    make_tokens!([
        (Identifier, "ZEND_MODULE_BUILD_ID"),
        (Literal, "\"API20250925,TS\"")
    ])
);

test_success!(
    zts_debug,
    [
        "ZEND_MODULE_API_NO" => [(Identifier, "ZEND_MODULE_API_NO"), (Literal, "20250925")],
        "ZEND_BUILD_TS" => [(Identifier, "ZEND_BUILD_TS"), (Literal, "\",TS\"")],
        "ZEND_BUILD_DEBUG" => [(Identifier, "ZEND_BUILD_DEBUG"), (Literal, "\",debug\"")],
        "ZEND_MODULE_BUILD_ID" => correct_tokens_for_build_id,
    ],
    make_tokens!([
        (Identifier, "ZEND_MODULE_BUILD_ID"),
        (Literal, "\"API20250925,TS,debug\"")
    ])
);
