use bindgen::callbacks::{IntKind, ParseCallbacks};

/// Callbacks to specify what type of integer should be used for various
/// C `#define`d constants.
#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct ExplicitIntTypesCallbacks;

impl ExplicitIntTypesCallbacks {
    /// Create a new callbacks instance to int macro types.
    #[must_use]
    pub fn new() -> Self {
        ExplicitIntTypesCallbacks
    }
}

impl ParseCallbacks for ExplicitIntTypesCallbacks {
    #[expect(clippy::too_many_lines, reason = "Lots of cases to match")]
    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        // Force some int kinds
        #[expect(clippy::match_same_arms, reason = "Group related constants")]
        match name {
            // Build metadata
            "ZEND_DEBUG" | "USING_ZTS" => Some(IntKind::U8),
            // Type information
            "IS_UNDEF" | "IS_NULL" | "IS_FALSE" | "IS_TRUE" | "IS_LONG" | "IS_DOUBLE"
            | "IS_STRING" | "IS_ARRAY" | "IS_OBJECT" | "IS_RESOURCE" | "IS_REFERENCE"
            | "IS_CONSTANT_AST" | "IS_CALLABLE" | "IS_ITERABLE" | "IS_VOID" | "IS_STATIC"
            | "IS_MIXED" | "IS_NEVER" | "IS_INDIRECT" | "IS_PTR" | "IS_ALIAS_PTR" | "_IS_ERROR"
            | "_IS_BOOL" | "_IS_NUMBER" => Some(IntKind::U8),
            // INI flags
            "ZEND_INI_USER" | "ZEND_INI_PERDIR" | "ZEND_INI_SYSTEM" | "ZEND_INI_ALL"
            | "PHP_INI_USER" | "PHP_INI_PERDIR" | "PHP_INI_SYSTEM" | "PHP_INI_ALL" => {
                Some(IntKind::U8)
            }
            // Module loading types
            "MODULE_PERSISTENT" | "MODULE_TEMPORARY" => Some(IntKind::U8),
            // Class types
            // In PHP 8.6 this becomes a dedicated enum type with u8 backing,
            // until then these are stored in a char field
            "ZEND_INTERNAL_CLASS" | "ZEND_USER_CLASS" => Some(IntKind::I8),
            // Function types
            // In PHP 8.6 this becomes a dedicated enum type with u8 backing,
            // until then these are stored in a u8 field
            "ZEND_INTERNAL_FUNCTION" | "ZEND_USER_FUNCTION" => Some(IntKind::U8),
            // Module dependency types are stored in an unsigned char field
            "MODULE_DEP_REQUIRED" | "MODULE_DEP_CONFLICTS" | "MODULE_DEP_OPTIONAL" => {
                Some(IntKind::UChar)
            }
            // Kinds passed to object handlers for has_set_exists
            "ZEND_PROPERTY_ISSET" | "ZEND_PROPERTY_NOT_EMPTY" | "ZEND_PROPERTY_EXISTS" => {
                Some(IntKind::Int)
            }
            // Parameter parsing api
            "ZEND_PARSE_PARAMS_QUIET" => Some(IntKind::Int),
            // Opcodes that are stored in u8 fields
            "ZEND_NOP"
            | "ZEND_ADD"
            | "ZEND_SUB"
            | "ZEND_MUL"
            | "ZEND_DIV"
            | "ZEND_MOD"
            | "ZEND_SL"
            | "ZEND_SR"
            | "ZEND_CONCAT"
            | "ZEND_BW_OR"
            | "ZEND_BW_AND"
            | "ZEND_BW_XOR"
            | "ZEND_POW"
            | "ZEND_BW_NOT"
            | "ZEND_BOOL_NOT"
            | "ZEND_BOOL_XOR"
            | "ZEND_IS_IDENTICAL"
            | "ZEND_IS_NOT_IDENTICAL"
            | "ZEND_IS_EQUAL"
            | "ZEND_IS_NOT_EQUAL"
            | "ZEND_IS_SMALLER"
            | "ZEND_IS_SMALLER_OR_EQUAL"
            | "ZEND_ASSIGN"
            | "ZEND_ASSIGN_DIM"
            | "ZEND_ASSIGN_OBJ"
            | "ZEND_ASSIGN_STATIC_PROP"
            | "ZEND_ASSIGN_OP"
            | "ZEND_ASSIGN_DIM_OP"
            | "ZEND_ASSIGN_OBJ_OP"
            | "ZEND_ASSIGN_STATIC_PROP_OP"
            | "ZEND_ASSIGN_REF"
            | "ZEND_QM_ASSIGN"
            | "ZEND_ASSIGN_OBJ_REF"
            | "ZEND_ASSIGN_STATIC_PROP_REF"
            | "ZEND_PRE_INC"
            | "ZEND_PRE_DEC"
            | "ZEND_POST_INC"
            | "ZEND_POST_DEC"
            | "ZEND_PRE_INC_STATIC_PROP"
            | "ZEND_PRE_DEC_STATIC_PROP"
            | "ZEND_POST_INC_STATIC_PROP"
            | "ZEND_POST_DEC_STATIC_PROP"
            | "ZEND_JMP"
            | "ZEND_JMPZ"
            | "ZEND_JMPNZ"
            | "ZEND_JMPZ_EX"
            | "ZEND_JMPNZ_EX"
            | "ZEND_CASE"
            | "ZEND_CHECK_VAR"
            | "ZEND_SEND_VAR_NO_REF_EX"
            | "ZEND_CAST"
            | "ZEND_BOOL"
            | "ZEND_FAST_CONCAT"
            | "ZEND_ROPE_INIT"
            | "ZEND_ROPE_ADD"
            | "ZEND_ROPE_END"
            | "ZEND_BEGIN_SILENCE"
            | "ZEND_END_SILENCE"
            | "ZEND_INIT_FCALL_BY_NAME"
            | "ZEND_DO_FCALL"
            | "ZEND_INIT_FCALL"
            | "ZEND_RETURN"
            | "ZEND_RECV"
            | "ZEND_RECV_INIT"
            | "ZEND_SEND_VAL"
            | "ZEND_SEND_VAR_EX"
            | "ZEND_SEND_REF"
            | "ZEND_NEW"
            | "ZEND_INIT_NS_FCALL_BY_NAME"
            | "ZEND_FREE"
            | "ZEND_INIT_ARRAY"
            | "ZEND_ADD_ARRAY_ELEMENT"
            | "ZEND_INCLUDE_OR_EVAL"
            | "ZEND_UNSET_VAR"
            | "ZEND_UNSET_DIM"
            | "ZEND_UNSET_OBJ"
            | "ZEND_FE_RESET_R"
            | "ZEND_FE_FETCH_R"
            | "ZEND_FETCH_R"
            | "ZEND_FETCH_DIM_R"
            | "ZEND_FETCH_OBJ_R"
            | "ZEND_FETCH_W"
            | "ZEND_FETCH_DIM_W"
            | "ZEND_FETCH_OBJ_W"
            | "ZEND_FETCH_RW"
            | "ZEND_FETCH_DIM_RW"
            | "ZEND_FETCH_OBJ_RW"
            | "ZEND_FETCH_IS"
            | "ZEND_FETCH_DIM_IS"
            | "ZEND_FETCH_OBJ_IS"
            | "ZEND_FETCH_FUNC_ARG"
            | "ZEND_FETCH_DIM_FUNC_ARG"
            | "ZEND_FETCH_OBJ_FUNC_ARG"
            | "ZEND_FETCH_UNSET"
            | "ZEND_FETCH_DIM_UNSET"
            | "ZEND_FETCH_OBJ_UNSET"
            | "ZEND_FETCH_LIST_R"
            | "ZEND_FETCH_CONSTANT"
            | "ZEND_CHECK_FUNC_ARG"
            | "ZEND_EXT_STMT"
            | "ZEND_EXT_FCALL_BEGIN"
            | "ZEND_EXT_FCALL_END"
            | "ZEND_EXT_NOP"
            | "ZEND_TICKS"
            | "ZEND_SEND_VAR_NO_REF"
            | "ZEND_CATCH"
            | "ZEND_THROW"
            | "ZEND_FETCH_CLASS"
            | "ZEND_CLONE"
            | "ZEND_RETURN_BY_REF"
            | "ZEND_INIT_METHOD_CALL"
            | "ZEND_INIT_STATIC_METHOD_CALL"
            | "ZEND_ISSET_ISEMPTY_VAR"
            | "ZEND_ISSET_ISEMPTY_DIM_OBJ"
            | "ZEND_SEND_VAL_EX"
            | "ZEND_SEND_VAR"
            | "ZEND_INIT_USER_CALL"
            | "ZEND_SEND_ARRAY"
            | "ZEND_SEND_USER"
            | "ZEND_STRLEN"
            | "ZEND_DEFINED"
            | "ZEND_TYPE_CHECK"
            | "ZEND_VERIFY_RETURN_TYPE"
            | "ZEND_FE_RESET_RW"
            | "ZEND_FE_FETCH_RW"
            | "ZEND_FE_FREE"
            | "ZEND_INIT_DYNAMIC_CALL"
            | "ZEND_DO_ICALL"
            | "ZEND_DO_UCALL"
            | "ZEND_DO_FCALL_BY_NAME"
            | "ZEND_PRE_INC_OBJ"
            | "ZEND_PRE_DEC_OBJ"
            | "ZEND_POST_INC_OBJ"
            | "ZEND_POST_DEC_OBJ"
            | "ZEND_ECHO"
            | "ZEND_OP_DATA"
            | "ZEND_INSTANCEOF"
            | "ZEND_GENERATOR_CREATE"
            | "ZEND_MAKE_REF"
            | "ZEND_DECLARE_FUNCTION"
            | "ZEND_DECLARE_LAMBDA_FUNCTION"
            | "ZEND_DECLARE_CONST"
            | "ZEND_DECLARE_CLASS"
            | "ZEND_DECLARE_CLASS_DELAYED"
            | "ZEND_DECLARE_ANON_CLASS"
            | "ZEND_ADD_ARRAY_UNPACK"
            | "ZEND_ISSET_ISEMPTY_PROP_OBJ"
            | "ZEND_HANDLE_EXCEPTION"
            | "ZEND_USER_OPCODE"
            | "ZEND_ASSERT_CHECK"
            | "ZEND_JMP_SET"
            | "ZEND_UNSET_CV"
            | "ZEND_ISSET_ISEMPTY_CV"
            | "ZEND_FETCH_LIST_W"
            | "ZEND_SEPARATE"
            | "ZEND_FETCH_CLASS_NAME"
            | "ZEND_CALL_TRAMPOLINE"
            | "ZEND_DISCARD_EXCEPTION"
            | "ZEND_YIELD"
            | "ZEND_GENERATOR_RETURN"
            | "ZEND_FAST_CALL"
            | "ZEND_FAST_RET"
            | "ZEND_RECV_VARIADIC"
            | "ZEND_SEND_UNPACK"
            | "ZEND_YIELD_FROM"
            | "ZEND_COPY_TMP"
            | "ZEND_BIND_GLOBAL"
            | "ZEND_COALESCE"
            | "ZEND_SPACESHIP"
            | "ZEND_FUNC_NUM_ARGS"
            | "ZEND_FUNC_GET_ARGS"
            | "ZEND_FETCH_STATIC_PROP_R"
            | "ZEND_FETCH_STATIC_PROP_W"
            | "ZEND_FETCH_STATIC_PROP_RW"
            | "ZEND_FETCH_STATIC_PROP_IS"
            | "ZEND_FETCH_STATIC_PROP_FUNC_ARG"
            | "ZEND_FETCH_STATIC_PROP_UNSET"
            | "ZEND_UNSET_STATIC_PROP"
            | "ZEND_ISSET_ISEMPTY_STATIC_PROP"
            | "ZEND_FETCH_CLASS_CONSTANT"
            | "ZEND_BIND_LEXICAL"
            | "ZEND_BIND_STATIC"
            | "ZEND_FETCH_THIS"
            | "ZEND_SEND_FUNC_ARG"
            | "ZEND_ISSET_ISEMPTY_THIS"
            | "ZEND_SWITCH_LONG"
            | "ZEND_SWITCH_STRING"
            | "ZEND_IN_ARRAY"
            | "ZEND_COUNT"
            | "ZEND_GET_CLASS"
            | "ZEND_GET_CALLED_CLASS"
            | "ZEND_GET_TYPE"
            | "ZEND_ARRAY_KEY_EXISTS"
            | "ZEND_MATCH"
            | "ZEND_CASE_STRICT"
            | "ZEND_MATCH_ERROR"
            | "ZEND_JMP_NULL"
            | "ZEND_CHECK_UNDEF_ARGS"
            | "ZEND_FETCH_GLOBALS"
            | "ZEND_VERIFY_NEVER_TYPE"
            | "ZEND_CALLABLE_CONVERT"
            | "ZEND_BIND_INIT_STATIC_OR_JMP"
            | "ZEND_FRAMELESS_ICALL_0"
            | "ZEND_FRAMELESS_ICALL_1"
            | "ZEND_FRAMELESS_ICALL_2"
            | "ZEND_FRAMELESS_ICALL_3"
            | "ZEND_JMP_FRAMELESS"
            | "ZEND_INIT_PARENT_PROPERTY_HOOK_CALL"
            | "ZEND_DECLARE_ATTRIBUTED_CONST"
            | "ZEND_VM_LAST_OPCODE" => Some(IntKind::U8),
            _ => None,
        }
    }
}
