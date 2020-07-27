enum JSGCPhaseEnum {
    JS_GC_PHASE_NONE,
    JS_GC_PHASE_DECREF,
    JS_GC_PHASE_REMOVE_CYCLES,
}

enum JSAtomType {
    JS_ATOM_TYPE_STRING = 1,
    JS_ATOM_TYPE_GLOBAL_SYMBOL,
    JS_ATOM_TYPE_SYMBOL,
    JS_ATOM_TYPE_PRIVATE,
}

enum JSAsyncGeneratorStateEnum {
    JS_ASYNC_GENERATOR_STATE_SUSPENDED_START,
    JS_ASYNC_GENERATOR_STATE_SUSPENDED_YIELD,
    JS_ASYNC_GENERATOR_STATE_SUSPENDED_YIELD_STAR,
    JS_ASYNC_GENERATOR_STATE_EXECUTING,
    JS_ASYNC_GENERATOR_STATE_AWAITING_RETURN,
    JS_ASYNC_GENERATOR_STATE_COMPLETED,
}

enum JSErrorEnum {
    JS_EVAL_ERROR,
    JS_RANGE_ERROR,
    JS_REFERENCE_ERROR,
    JS_SYNTAX_ERROR,
    JS_TYPE_ERROR,
    JS_URI_ERROR,
    JS_INTERNAL_ERROR,
    JS_AGGREGATE_ERROR,
    JS_NATIVE_ERROR_COUNT,
}

enum JSGCObjectTypeEnum {
    JS_GC_OBJ_TYPE_JS_OBJECT,
    JS_GC_OBJ_TYPE_FUNCTION_BYTECODE,
    JS_GC_OBJ_TYPE_SHAPE,
    JS_GC_OBJ_TYPE_VAR_REF,
    JS_GC_OBJ_TYPE_ASYNC_FUNCTION,
    JS_GC_OBJ_TYPE_JS_CONTEXT,
}

enum JSIteratorKindEnum {
    JS_ITERATOR_KIND_KEY,
    JS_ITERATOR_KIND_VALUE,
    JS_ITERATOR_KIND_KEY_AND_VALUE,
}

/// XXX: add more variable kinds here instead of using bit fields
enum JSVarKindEnum {
    JS_VAR_NORMAL,
    /// lexical var with function declaration
    JS_VAR_FUNCTION_DECL,
    /// lexical var with async/generator function declaration
    JS_VAR_NEW_FUNCTION_DECL,
    JS_VAR_CATCH,
    JS_VAR_PRIVATE_FIELD,
    JS_VAR_PRIVATE_METHOD,
    JS_VAR_PRIVATE_GETTER,
    /// must come after JS_VAR_PRIVATE_GETTER
    JS_VAR_PRIVATE_SETTER,
    /// must come after JS_VAR_PRIVATE_SETTER
    JS_VAR_PRIVATE_GETTER_SETTER,
}

enum JSStrictEqModeEnum {
    JS_EQ_STRICT,
    JS_EQ_SAME_VALUE,
    JS_EQ_SAME_VALUE_ZERO,
}

#[repr(u8)]
enum JSFunctionKindEnum {
    JS_FUNC_NORMAL = 0,
    JS_FUNC_GENERATOR = 1 << 0,
    JS_FUNC_ASYNC = 1 << 1,
    JS_FUNC_ASYNC_GENERATOR =
        (JSFunctionKindEnum::JS_FUNC_GENERATOR | JSFunctionKindEnum::JS_FUNC_ASYNC),
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSClosureVar {
    pub is_local: bool,
    pub is_arg: bool,
    pub is_const: bool,
    pub is_lexical: bool,
    pub var_kind: JSVarKindEnum,
    pub var_idx: u16,
    pub var_name: JSAtom,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSObject {
    pub shape: *mut JSShape,
    pub prop: *mut JSProperty,
    pub first_weak_ref: *mut JSMapRecord,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSShape {
    pub prop_hash_end: [u32; 0],
    pub header: JSGCObjectHeader,
    pub is_hashed: u8,
    pub has_small_array_index: u8,
    pub hash: u32,
    pub prop_hash_mask: u32,
    pub prop_size: usize,
    pub prop_count: usize,
    pub deleted_prop_count: usize,
    pub shape_hash_next: *mut JSShape,
    pub proto: *mut JSObject,
    pub prop: [JSShapeProperty; 0],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSShapeProperty {
    pub hash_next: u32,
    pub flags: u8,
    pub atom: JSAtom,
}

type JSAtom = u32;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSString {
    pub header: JSRefCountHeader,
    pub len: usize,
    pub is_wide_char: bool,
    pub hash: u32,
    pub atom_type: JSAtomType,
    pub hash_next: u32,
    pub data: String,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSRefCountHeader {
    pub ref_count: Option<usize>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSGCObjectHeader {
    pub ref_count: Option<usize>,
    pub gc_obj_type: JSGCObjectTypeEnum,
    pub mark: u8,
    pub dummy1: u8,
    pub dummy2: u16,
    pub link: ListHead,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ListHead {
    pub prev: *mut ListHead,
    pub next: *mut ListHead,
}
