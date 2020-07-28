pub enum JSGCPhase {
    NONE,
    DECREF,
    REMOVE_CYCLES,
}

pub enum JSAtomType {
    STRING = 1,
    GLOBAL_SYMBOL,
    SYMBOL,
    PRIVATE,
}

enum JSAsyncGeneratorState {
    SUSPENDED_START,
    SUSPENDED_YIELD,
    SUSPENDED_YIELD_STAR,
    EXECUTING,
    AWAITING_RETURN,
    COMPLETED,
}

// #[derive(Copy, Clone)]
#[repr(C)]
pub struct JSObject {
    pub shape: *mut JSShape,
    // pub prop: *mut JSProperty,
    // pub first_weak_ref: *mut JSMapRecord,
}

// #[derive(Copy, Clone)]
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

// #[derive(Copy, Clone)]
#[repr(C)]
pub struct JSString {
    pub header: JSRefCountHeader,
    // TODO: as method
    pub len: usize,
    // TODO: fuck wide char
    pub is_wide_char: bool,
    // TODO: as method
    pub hash: u32,
    pub atom_type: JSAtomType,
    pub hash_next: u32,
    // TODO: *mut u8 use rust String
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

pub enum JSGCObjectType {
    JS_OBJECT,
    FUNCTION_BYTECODE,
    SHAPE,
    VAR_REF,
    ASYNC_FUNCTION,
    JS_CONTEXT,
}

enum JSVarRefStatus{
    OnStack(u16),
    Detached,
}

// GC object could be used in FFI
#[repr(C)]
// #[derive(Copy, Clone)]
pub struct JSGCObjectHeader {
    pub ref_count: Option<usize>,
    pub gc_obj_type: JSGCObjectType,
    pub mark: u8,
    is_arg: bool,
    status: JSVarRefStatus,
    pub link: ListHead,
}

//TODO: std::collections::LinkedList
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ListHead {
    pub prev: *mut ListHead,
    pub next: *mut ListHead,
}

pub enum JSValue{
    Int(i32),
    Float(f64),
    Ptr(*mut std::ffi::c_void)
}

//TODO: enum prob is better
struct JSVarRef{
    header : JSGCObjectHeader,
    pvalue: *const JSValue,
    value: JSValue,
}

enum JSAutoInitID{
    PROTOTYPE,
    MODULE_NS,
    PROP,
}

#[repr(C)]
pub struct JSContext {
    pub header: JSGCObjectHeader,
    pub rt: *mut JSRuntime,
    pub link: ListHead,
    pub binary_object_count: u16,
    pub binary_object_size: u32,
    pub array_shape: *mut JSShape,
    pub class_proto: *mut JSValue,
    pub function_proto: JSValue,
    pub function_ctor: JSValue,
    pub array_ctor: JSValue,
    pub regexp_ctor: JSValue,
    pub promise_ctor: JSValue,
    pub native_error_proto: [JSValue; 8],
    pub iterator_proto: JSValue,
    pub async_iterator_proto: JSValue,
    pub array_proto_values: JSValue,
    pub throw_type_error: JSValue,
    pub eval_obj: JSValue,
    pub global_obj: JSValue,
    pub global_var_obj: JSValue,
    pub random_state: u64,
    pub interrupt_counter: u32,
    pub is_error_property_enabled: u32,
    pub loaded_modules: ListHead,
    // TODO: 写impl里
    // pub compile_regexp: Option<unsafe extern "C" fn(_: *mut JSContext,
    //                                                 _: JSValue, _: JSValue)
    //                                -> JSValue>,
    // pub eval_internal: Option<unsafe extern "C" fn(_: *mut JSContext,
    //                                                _: JSValue,
    //                                                _: *const libc::c_char,
    //                                                _: size_t,
    //                                                _: *const libc::c_char,
    //                                                _: u32,
    //                                                _: u32)
    //                               -> JSValue>,
    pub user_opaque: *mut std::ffi::c_void,
}

// #[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct JSRuntime {
    // TODO: 单写
    // pub mf: JSMallocFunctions,
    // pub malloc_state: JSMallocState,
    pub runtime_info: String,
    pub atom_hash_size: u32,
    pub atom_count: u32,
    pub atom_size: u32,
    pub atom_count_resize: u32,

    // TODO: vec
    // pub atom_hash: *mut u32,
    // pub atom_array: *mut *mut JSAtomStruct,

    pub atom_free_index: u32,
    
    // TODO: vec
    // pub class_count: u32,
    // pub class_array: *mut JSClass,
    
    pub context_list: ListHead,
    pub gc_obj_list: ListHead,
    pub gc_zero_ref_count_list: ListHead,
    pub tmp_obj_list: ListHead,
    pub gc_phase: JSGCPhase,
    pub malloc_gc_threshold: u64,
    pub stack_top: *const u8,
    pub stack_size: u64,
    pub current_exception: JSValue,
    pub in_out_of_memory: bool,
    pub current_stack_frame: *mut JSStackFrame,
    // TODO: impl
    // pub interrupt_handler: Option<JSInterruptHandler>,
    pub interrupt_opaque: *mut std::ffi::c_void,
    // pub host_promise_rejection_tracker: Option<JSHostPromiseRejectionTracker>,
    pub host_promise_rejection_tracker_opaque: *mut std::ffi::c_void,
    pub job_list: ListHead,
    // TODO: 单写
    // pub module_normalize_func: Option<JSModuleNormalizeFunc>,
    // pub module_loader_func: Option<JSModuleLoaderFunc>,
    pub module_loader_opaque: *mut std::ffi::c_void,
    pub can_block: bool,
    // TODO: impl
    // pub sab_funcs: JSSharedArrayBufferFunctions,
    pub shape_hash_bits: u32,
    pub shape_hash_size: u32,
    pub shape_hash_count: u32,
    pub shape_hash: *mut *mut JSShape,
    pub user_opaque: *mut std::ffi::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSClass {
    //TODO: JS_CLASS_XXX
    pub class_type: u32,
    pub class_name: JSAtom,
    //TODO: impl
    // pub finalizer: Option<JSClassFinalizer>,
    // pub gc_mark: Option<JSClassGCMark>,
    // pub call: Option<JSClassCall>,
    // pub exotic: *const JSClassExoticMethods,
}

pub enum JSMode{
    // C functions also use Default
    Default = 0,
    Strict,
    // TODO: currently not supported
    // Strip,
    // TODO: currently not supported
    // Math
}

#[repr(C)]
pub struct JSStackFrame {
    // TODO: use LinkedList instead of using pointer
    pub prev_frame: *mut JSStackFrame,
    pub cur_func: JSValue,
    // TODO: use vec instead of raw pointer
    pub arg_buf: *mut JSValue,
    pub arg_count: u32,
    pub var_buf: *mut JSValue,
    // TODO: use LinkedList instead of using pointer
    pub var_ref_list: ListHead,
    pub cur_pc: *const u8,
    pub js_mode: JSMode,
    pub cur_sp: *mut JSValue,
}

struct JSClosureVar{
    is_local:bool,
    is_arg:bool,
    is_const:bool,
    is_lexial:bool,
    var_kind:JSVarKind,
    // TODO: Closure type, this only used if is local
    var_idx: u16,
    var_name: JSAtom,
}

struct JSVarScope{
    parent: u32,
    first: u32
}

enum JSVarKind{
    /* XXX: add more variable kinds here instead of using bit fields */
    NORMAL,
    FUNCTION_DECL, /* lexical var with function declaration */
    NEW_FUNCTION_DECL, /* lexical var with async/generator
                                 function declaration */
    CATCH,
    PRIVATE_FIELD,
    PRIVATE_METHOD,
    PRIVATE_GETTER,
    PRIVATE_SETTER, /* must come after PRIVATE_GETTER */
    PRIVATE_GETTER_SETTER, /* must come after PRIVATE_SETTER */
}

struct JSVarDef{
    name: JSAtom,
    scope_level: u32,
    scope_next: u32,

    is_func_var: bool,
    is_const: bool,
    is_lexial: bool,
    is_captured:bool,
    kind: JSVarKind,
    // only use in compilation
    func_pool_or_scope_idx:u32,
}

enum JSFuncKind{
    Normal,
    Generator,
    Async,
    AsyncGenerator
}

struct JSFunctionBytecodeDebugInfo{
    filename:String,
    line_number:u64,
    source_len: u64,
    //TODO: use vec
    pc2line_len:u64,
    pc2line_buf:*mut u8,
    source:String
}

struct JSFunctionBytecode{
header:JSGCObjectHeader,
mode:JSMode,
has_proto:bool,
has_simple_param_list:bool,
is_derived_class_constructor:bool,
need_home_object:bool,
func_kind:JSFuncKind,
new_target_allowed:bool,
super_call_allowed:bool,
arguments_allowed:bool,
has_debug:bool,
backtrace_barrier:bool,
read_only_bytecode:bool,
// TODO: vec
byte_code_buf:*mut u8,
byte_code_len:u32,

func_name:JSAtom,
// TODO: all use vec!
var_defs:*mut JSVarDef,
closure_vars:*mut JSClosureVar,
closure_vars_count:u32,
arg_count:u16,
var_count:u16,
defined_arg_count:u16,
stack_size:u16,
realm : JSContext,
constant_pool:*mut JSValue,
cpool_count:u32,

debug_info: JSFunctionBytecodeDebugInfo
}