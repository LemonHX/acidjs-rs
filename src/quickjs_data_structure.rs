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
    pub u: Terminator,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSRefCountHeader {
    pub ref_count: Option<usize>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union Terminator {
    pub str8: [u8; 0],
    pub str16: [u16; 0],
}

enum JSGCObjectTypeEnum {
    JS_GC_OBJ_TYPE_JS_OBJECT,
    JS_GC_OBJ_TYPE_FUNCTION_BYTECODE,
    JS_GC_OBJ_TYPE_SHAPE,
    JS_GC_OBJ_TYPE_VAR_REF,
    JS_GC_OBJ_TYPE_ASYNC_FUNCTION,
    JS_GC_OBJ_TYPE_JS_CONTEXT,
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
