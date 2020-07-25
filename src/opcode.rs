use OpFMT::*;
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[repr(u8)]
enum OpFMT {
    OpFMT_none = 0,
    OpFMT_none_int,
    OpFMT_none_loc,
    OpFMT_none_arg,
    OpFMT_none_var_ref,
    OpFMT_u8,
    OpFMT_i8,
    OpFMT_loc8,
    OpFMT_const8,
    OpFMT_label8,
    OpFMT_u16,
    OpFMT_i16,
    OpFMT_label16,
    OpFMT_npop,
    OpFMT_npopx,
    OpFMT_npop_u16,
    OpFMT_loc,
    OpFMT_arg,
    OpFMT_var_ref,
    OpFMT_u32,
    OpFMT_i32,
    OpFMT_const,
    OpFMT_label,
    OpFMT_atom,
    OpFMT_atom_u8,
    OpFMT_atom_u16,
    OpFMT_atom_label_u8,
    OpFMT_atom_label_u16,
    OpFMT_label_u16,
}
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[repr(u8)]
enum OpCodeEnum {
    OpCode_push_i32,
    OpCode_push_const,
    OpCode_fclosure,
    OpCode_push_atom_value,
    OpCode_private_symbol,
    OpCode_undefined,
    OpCode_null,
    OpCode_push_this,
    OpCode_push_false,
    OpCode_push_true,
    OpCode_object,
    OpCode_special_object,
    OpCode_rest,

    OpCode_drop,
    OpCode_nip,
    OpCode_nip1,
    OpCode_dup,
    OpCode_dup1,
    OpCode_dup2,
    OpCode_dup3,
    OpCode_insert2,
    OpCode_insert3,
    OpCode_insert4,
    OpCode_perm3,
    OpCode_perm4,
    OpCode_perm5,
    OpCode_swap,
    OpCode_swap2,
    OpCode_rot3l,
    OpCode_rot3r,
    OpCode_rot4l,
    OpCode_rot5l,

    OpCode_call_constructor,
    OpCode_call,
    OpCode_tail_call,
    OpCode_call_method,
    OpCode_tail_call_method,
    OpCode_array_from,
    OpCode_apply,
    OpCode_return,
    OpCode_return_undef,
    OpCode_check_ctor_return,
    OpCode_check_ctor,
    OpCode_check_brand,
    OpCode_add_brand,
    OpCode_return_async,
    OpCode_throw,
    OpCode_throw_var,
    OpCode_eval,
    OpCode_apply_eval,
    OpCode_regexp,

    OpCode_get_super,
    OpCode_import,

    OpCode_check_var,
    OpCode_get_var_undef,
    OpCode_get_var,
    OpCode_put_var,
    OpCode_put_var_init,
    OpCode_put_var_strict,

    OpCode_get_ref_value,
    OpCode_put_ref_value,

    OpCode_define_var,
    OpCode_check_define_var,
    OpCode_define_func,
    OpCode_get_field,
    OpCode_get_field2,
    OpCode_put_field,
    OpCode_get_private_field,
    OpCode_put_private_field,
    OpCode_define_private_field,
    OpCode_get_array_el,
    OpCode_get_array_el2,
    OpCode_put_array_el,
    OpCode_get_super_value,
    OpCode_put_super_value,
    OpCode_define_field,
    OpCode_set_name,
    OpCode_set_name_computed,
    OpCode_set_proto,
    OpCode_set_home_object,
    OpCode_define_array_el,
    OpCode_append,
    OpCode_copy_data_properties,
    OpCode_define_method,
    OpCode_define_method_computed,
    OpCode_define_class,
    OpCode_define_class_computed,

    OpCode_get_loc,
    OpCode_put_loc,
    OpCode_set_loc,
    OpCode_get_arg,
    OpCode_put_arg,
    OpCode_set_arg,
    OpCode_get_var_ref,
    OpCode_put_var_ref,
    OpCode_set_var_ref,
    OpCode_set_loc_uninitialized,
    OpCode_get_loc_check,
    OpCode_put_loc_check,
    OpCode_put_loc_check_init,
    OpCode_get_var_ref_check,
    OpCode_put_var_ref_check,
    OpCode_put_var_ref_check_init,
    OpCode_close_loc,
    OpCode_if_false,
    OpCode_if_true,
    OpCode_goto,
    OpCode_catch,
    OpCode_gosub,
    OpCode_ret,

    OpCode_to_object,
    //OpCode_to_string,
    OpCode_to_propkey,
    OpCode_to_propkey2,

    OpCode_with_get_var,
    OpCode_with_put_var,
    OpCode_with_delete_var,
    OpCode_with_make_ref,
    OpCode_with_get_ref,
    OpCode_with_get_ref_undef,

    OpCode_make_loc_ref,
    OpCode_make_arg_ref,
    OpCode_make_var_ref_ref,
    OpCode_make_var_ref,

    OpCode_for_in_start,
    OpCode_for_of_start,
    OpCode_for_await_of_start,
    OpCode_for_in_next,
    OpCode_for_of_next,
    OpCode_for_await_of_next,
    OpCode_iterator_get_value_done,
    OpCode_iterator_close,
    OpCode_iterator_close_return,
    OpCode_async_iterator_close,
    OpCode_async_iterator_next,
    OpCode_async_iterator_get,
    OpCode_initial_yield,
    OpCode_yield,
    OpCode_yield_star,
    OpCode_async_yield_star,
    OpCode_await,

    /* arithmetic/logic operations */
    OpCode_neg,
    OpCode_plus,
    OpCode_dec,
    OpCode_inc,
    OpCode_post_dec,
    OpCode_post_inc,
    OpCode_dec_loc,
    OpCode_inc_loc,
    OpCode_add_loc,
    OpCode_not,
    OpCode_lnot,
    OpCode_typeof,
    OpCode_delete,
    OpCode_delete_var,

    OpCode_mul,
    OpCode_div,
    OpCode_mod,
    OpCode_add,
    OpCode_sub,
    OpCode_pow,
    OpCode_shl,
    OpCode_sar,
    OpCode_shr,
    OpCode_lt,
    OpCode_lte,
    OpCode_gt,
    OpCode_gte,
    OpCode_instanceof,
    OpCode_in,
    OpCode_eq,
    OpCode_neq,
    OpCode_strict_eq,
    OpCode_strict_neq,
    OpCode_and,
    OpCode_xor,
    OpCode_or,
    OpCode_is_undefined_or_null,
    OpCode_nop,
}

#[allow(dead_code)]
struct OpCode {
    //TODO: 能这么写？
    #[cfg(debug_opcodes)]
    id: u8,
    size: u8,
    n_pop: u8,
    n_push: u8,
    fmt: OpFMT,
}

#[allow(dead_code)]
impl OpCode {
    fn new(id: OpCodeEnum, size: u8, n_pop: u8, n_push: u8, fmt: OpFMT) -> Self {
        OpCode {
            #[cfg(debug_opcodes)]
            id,
            size,
            n_pop,
            n_push,
            fmt,
        }
    }
}
// use OpCodeEnum::*;
// static mut OP_CODE_INFO: Vec<OpCode> = vec![
//     OpCode::new(OpCode_push_i32, 5, 0, 1, OpFMT_i32),
//     OpCode::new(OpCode_push_const, 5, 0, 1, OpFMT_const),
//     /* must follow push_const */
//     OpCode::new(OpCode_fclosure, 5, 0, 1, OpFMT_const),
//     OpCode::new(OpCode_push_atom_value, 5, 0, 1, OpFMT_atom),
//     OpCode::new(OpCode_private_symbol, 5, 0, 1, OpFMT_atom),
//     OpCode::new(OpCode_undefined, 1, 0, 1, OpFMT_none),
//     OpCode::new(OpCode_null, 1, 0, 1, OpFMT_none),
//     /* only used at the start of a function */
//     OpCode::new(OpCode_push_this, 1, 0, 1, OpFMT_none),
//     OpCode::new(OpCode_push_false, 1, 0, 1, OpFMT_none),
//     OpCode::new(OpCode_push_true, 1, 0, 1, OpFMT_none),
//     OpCode::new(OpCode_object, 1, 0, 1, OpFMT_none),
//     /* only used at the start of a function */
//     OpCode::new(OpCode_special_object, 2, 0, 1, OpFMT_u8),
//     /* only used at the start of a function */
//     OpCode::new(OpCode_rest, 3, 0, 1, OpFMT_u16),
//     /* a -> */
//     OpCode::new(OpCode_drop, 1, 1, 0, OpFMT_none) ,
//      /* a b -> b */
//     OpCode::new(OpCode_nip, 1, 2, 1, OpFMT_none) ,
//     /* a b c -> b c */
//     OpCode::new(OpCode_nip1, 1, 3, 2, OpFMT_none),
//     /* a -> a a */
//     OpCode::new(OpCode_dup, 1, 1, 2, OpFMT_none),
//     /* a b -> a a b */
//     OpCode::new(OpCode_dup1, 1, 2, 3, OpFMT_none),
//     /* a b -> a b a b */
//     OpCode::new(OpCode_dup2, 1, 2, 4, OpFMT_none),
//     /* a b c -> a b c a b c */
//     OpCode::new(OpCode_dup3, 1, 3, 6, OpFMT_none),
//     /* obj a -> a obj a (dup_x1) */
//     OpCode::new(OpCode_insert2, 1, 2, 3, OpFMT_none),
//     /* obj prop a -> a obj prop a (dup_x2) */
//     OpCode::new(OpCode_insert3, 1, 3, 4, OpFMT_none),
//     /* this obj prop a -> a this obj prop a */
//     OpCode::new(OpCode_insert4, 1, 4, 5, OpFMT_none),
//     /* obj a b -> a obj b */
//     OpCode::new(OpCode_perm3, 1, 3, 3, OpFMT_none),
//     /* obj prop a b -> a obj prop b */
//     OpCode::new(OpCode_perm4, 1, 4, 4, OpFMT_none),
//     /* this obj prop a b -> a this obj prop b */
//     OpCode::new(OpCode_perm5, 1, 5, 5, OpFMT_none),
//     /* a b -> b a */
//     OpCode::new(OpCode_swap, 1, 2, 2, OpFMT_none),
//     /* a b c d -> c d a b */
//     OpCode::new(OpCode_swap2, 1, 4, 4, OpFMT_none),
//     /* x a b -> a b x */
//     OpCode::new(OpCode_rot3l, 1, 3, 3, OpFMT_none),
//     /* a b x -> x a b */
//     OpCode::new(OpCode_rot3r, 1, 3, 3, OpFMT_none),
//     /* x a b c -> a b c x */
//     OpCode::new(OpCode_rot4l, 1, 4, 4, OpFMT_none),
//     /* x a b c d -> a b c d x */
//     OpCode::new(OpCode_rot5l, 1, 5, 5, OpFMT_none),
//     /* func new.target args -> ret. arguments are not counted in n_pop */
//     OpCode::new(OpCode_call_constructor, 3, 2, 1,OpFMT_npop),

//     // arguments are not counted in n_pop
//     OpCode::new(OpCode_call, 3, 1, 1, OpFMT_npop),
//     OpCode::new(OpCode_tail_call, 3, 1, 0, OpFMT_npop),
//     OpCode::new(OpCode_call_method, 3, 2, 1, OpFMT_npop),
//     OpCode::new(OpCode_tail_call_method, 3, 2, 0, OpFMT_npop),
//     OpCode::new(OpCode_array_from, 3, 0, 1, OpFMT_npop),

//     OpCode::new(OpCode_apply, 3, 3, 1, OpFMT_u16),
//     OpCode::new(OpCode_return, 1, 1, 0, OpFMT_none),
//     OpCode::new(OpCode_return_undef, 1, 0, 0, OpFMT_none),
//     OpCode::new(OpCode_check_ctor_return, 1, 1, 2, OpFMT_none),
//     OpCode::new(OpCode_check_ctor, 1, 0, 0, OpFMT_none),
//     /* this_obj func -> this_obj func */
//     OpCode::new(OpCode_check_brand, 1, 2, 2, OpFMT_none),
//     /* this_obj home_obj -> */
//     OpCode::new(OpCode_add_brand, 1, 2, 0, OpFMT_none),
//     OpCode::new(OpCode_return_async, 1, 1, 0, OpFMT_none),
//     OpCode::new(OpCode_throw, 1, 1, 0, OpFMT_none),
//     OpCode::new(OpCode_throw_var, 6, 0, 0, OpFMT_atom_u8),
//     /* func args... -> ret_val */
//     OpCode::new(OpCode_eval, 5, 1, 1, OpFMT_npop_u16),
//     /* func array -> ret_eval */
//     OpCode::new(OpCode_apply_eval, 3, 2, 1, OpFMT_u16),
//     /* create a RegExp object from the pattern and a bytecode string */
//     OpCode::new(OpCode_regexp, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_get_super, 1, 1, 1, OpFMT_none),
//     /* dynamic module import */
//     OpCode::new(OpCode_import, 1, 1, 1, OpFMT_none),

//     /* check if a variable exists */
//     OpCode::new(OpCode_check_var, 5, 0, 1, OpFMT_atom),
//     /* push undefined if the variable does not exist */
//     OpCode::new(OpCode_get_var_undef, 5, 0, 1, OpFMT_atom),
//     /* throw an exception if the variable does not exist */
//     OpCode::new(OpCode_get_var, 5, 0, 1, OpFMT_atom),
//     /* must come after get_var */
//     OpCode::new(OpCode_put_var, 5, 1, 0, OpFMT_atom),
//     /* must come after put_var. Used to initialize a global lexical variable */
//     OpCode::new(OpCode_put_var_init, 5, 1, 0, OpFMT_atom),
//     /* for strict mode variable write */
//     OpCode::new(OpCode_put_var_strict, 5, 2, 0, OpFMT_atom),

//     OpCode::new(OpCode_get_ref_value, 1, 2, 3, OpFMT_none),
//     OpCode::new(OpCode_put_ref_value, 1, 3, 0, OpFMT_none),

//     OpCode::new(OpCode_define_var, 6, 0, 0, OpFMT_atom_u8),
//     OpCode::new(OpCode_check_define_var, 6, 0, 0, OpFMT_atom_u8),
//     OpCode::new(OpCode_define_func, 6, 1, 0, OpFMT_atom_u8),
//     OpCode::new(OpCode_get_field, 5, 1, 1, OpFMT_atom),
//     OpCode::new(OpCode_get_field2, 5, 1, 2, OpFMT_atom),
//     OpCode::new(OpCode_put_field, 5, 2, 0, OpFMT_atom),
//     /* obj prop -> value */
//     OpCode::new(OpCode_get_private_field, 1, 2, 1, OpFMT_none),
//     /* obj value prop -> */
//     OpCode::new(OpCode_put_private_field, 1, 3, 0, OpFMT_none),
//     /* obj prop value -> obj */
//     OpCode::new(OpCode_define_private_field, 1, 3, 1, OpFMT_none),
//     OpCode::new(OpCode_get_array_el, 1, 2, 1, OpFMT_none),
//     /* obj prop -> obj value */
//     OpCode::new(OpCode_get_array_el2, 1, 2, 2, OpFMT_none),
//     OpCode::new(OpCode_put_array_el, 1, 3, 0, OpFMT_none),
//     /* this obj prop -> value */
//     OpCode::new(OpCode_get_super_value, 1, 3, 1, OpFMT_none),
//     /* this obj prop value -> */
//     OpCode::new(OpCode_put_super_value, 1, 4, 0, OpFMT_none),
//     OpCode::new(OpCode_define_field, 5, 2, 1, OpFMT_atom),
//     OpCode::new(OpCode_set_name, 5, 1, 1, OpFMT_atom),
//     OpCode::new(OpCode_set_name_computed, 1, 2, 2, OpFMT_none),
//     OpCode::new(OpCode_set_proto, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_set_home_object, 1, 2, 2, OpFMT_none),
//     OpCode::new(OpCode_define_array_el, 1, 3, 2, OpFMT_none),
//     /* append enumerated object, update length */
//     OpCode::new(OpCode_append, 1, 3, 2, OpFMT_none),
//     OpCode::new(OpCode_copy_data_properties, 2, 3, 3, OpFMT_u8),
//     OpCode::new(OpCode_define_method, 6, 2, 1, OpFMT_atom_u8),
//     /* must come after define_method */
//     OpCode::new(OpCode_define_method_computed, 2, 3, 1, OpFMT_u8),
//     /* parent ctor -> ctor proto */
//     OpCode::new(OpCode_define_class, 6, 2, 2, OpFMT_atom_u8),
//     /* field_name parent ctor -> field_name ctor proto (class with computed name) */
//     OpCode::new(OpCode_define_class_computed, 6, 3, 3, OpFMT_atom_u8),

//     OpCode::new(OpCode_get_loc, 3, 0, 1, OpFMT_loc),
//     /* must come after get_loc */
//     OpCode::new(OpCode_put_loc, 3, 1, 0, OpFMT_loc),
//     /* must come after put_loc */
//     OpCode::new(OpCode_set_loc, 3, 1, 1, OpFMT_loc),
//     OpCode::new(OpCode_get_arg, 3, 0, 1, OpFMT_arg),
//     /* must come after get_arg */
//     OpCode::new(OpCode_put_arg, 3, 1, 0, OpFMT_arg),
//     /* must come after put_arg */
//     OpCode::new(OpCode_set_arg, 3, 1, 1, OpFMT_arg),
//     OpCode::new(OpCode_get_var_ref, 3, 0, 1, OpFMT_var_ref),
//     /* must come after get_var_ref */
//     OpCode::new(OpCode_put_var_ref, 3, 1, 0, OpFMT_var_ref),
//     /* must come after put_var_ref */
//     OpCode::new(OpCode_set_var_ref, 3, 1, 1, OpFMT_var_ref),
//     OpCode::new(OpCode_set_loc_uninitialized, 3, 0, 0, OpFMT_loc),
//     OpCode::new(OpCode_get_loc_check, 3, 0, 1, OpFMT_loc),
//     /* must come after get_loc_check */
//     OpCode::new(OpCode_put_loc_check, 3, 1, 0, OpFMT_loc),
//     OpCode::new(OpCode_put_loc_check_init, 3, 1, 0, OpFMT_loc),
//     OpCode::new(OpCode_get_var_ref_check, 3, 0, 1, OpFMT_var_ref),
//     /* must come after get_var_ref_check */
//     OpCode::new(OpCode_put_var_ref_check, 3, 1, 0, OpFMT_var_ref),
//     OpCode::new(OpCode_put_var_ref_check_init, 3, 1, 0, OpFMT_var_ref),
//     OpCode::new(OpCode_close_loc, 3, 0, 0, OpFMT_loc),
//     OpCode::new(OpCode_if_false, 5, 1, 0, OpFMT_label),
//     /* must come after if_false */
//     OpCode::new(OpCode_if_true, 5, 1, 0, OpFMT_label),
//     /* must come after if_true */
//     OpCode::new(OpCode_goto, 5, 0, 0, OpFMT_label),
//     OpCode::new(OpCode_catch, 5, 0, 1, OpFMT_label),
//     /* used to execute the finally block */
//     OpCode::new(OpCode_gosub, 5, 0, 0, OpFMT_label),
//     /* used to return from the finally block */
//     OpCode::new(OpCode_ret, 1, 1, 0, OpFMT_none),

//     OpCode::new(OpCode_to_object, 1, 1, 1, OpFMT_none),
//     //OpCode::new(OpCode_to_string, 1, 1, 1, none),
//     OpCode::new(OpCode_to_propkey, 1, 1, 1, OpFMT_none),
//     OpCode::new(OpCode_to_propkey2, 1, 2, 2, OpFMT_none),

//     // must be in the same order as scope_xxx
//     OpCode::new(OpCode_with_get_var, 10, 1, 0, OpFMT_atom_label_u8),
//     OpCode::new(OpCode_with_put_var, 10, 2, 1, OpFMT_atom_label_u8),
//     OpCode::new(OpCode_with_delete_var, 10, 1, 0, OpFMT_atom_label_u8),
//     OpCode::new(OpCode_with_make_ref, 10, 1, 0, OpFMT_atom_label_u8),
//     OpCode::new(OpCode_with_get_ref, 10, 1, 0, OpFMT_atom_label_u8),

//     OpCode::new(OpCode_with_get_ref_undef, 10, 1, 0, OpFMT_atom_label_u8),

//     OpCode::new(OpCode_make_loc_ref, 7, 0, 2, OpFMT_atom_u16),
//     OpCode::new(OpCode_make_arg_ref, 7, 0, 2, OpFMT_atom_u16),
//     OpCode::new(OpCode_make_var_ref_ref, 7, 0, 2, OpFMT_atom_u16),
//     OpCode::new(OpCode_make_var_ref, 5, 0, 2, OpFMT_atom),

//     OpCode::new(OpCode_for_in_start, 1, 1, 1, OpFMT_none),
//     OpCode::new(OpCode_for_of_start, 1, 1, 3, OpFMT_none),
//     OpCode::new(OpCode_for_await_of_start, 1, 1, 3, OpFMT_none),
//     OpCode::new(OpCode_for_in_next, 1, 1, 3, OpFMT_none),
//     OpCode::new(OpCode_for_of_next, 2, 3, 5, OpFMT_u8),
//     OpCode::new(OpCode_for_await_of_next, 1, 3, 4, OpFMT_none),
//     OpCode::new(OpCode_iterator_get_value_done, 1, 1, 2, OpFMT_none),
//     OpCode::new(OpCode_iterator_close, 1, 3, 0, OpFMT_none),
//     OpCode::new(OpCode_iterator_close_return, 1, 4, 4, OpFMT_none),
//     OpCode::new(OpCode_async_iterator_close, 1, 3, 2, OpFMT_none),
//     OpCode::new(OpCode_async_iterator_next, 1, 4, 4, OpFMT_none),
//     OpCode::new(OpCode_async_iterator_get, 2, 4, 5, OpFMT_u8),
//     OpCode::new(OpCode_initial_yield, 1, 0, 0, OpFMT_none),
//     OpCode::new(OpCode_yield, 1, 1, 2, OpFMT_none),
//     OpCode::new(OpCode_yield_star, 1, 2, 2, OpFMT_none),
//     OpCode::new(OpCode_async_yield_star, 1, 1, 2, OpFMT_none),
//     OpCode::new(OpCode_await, 1, 1, 1, OpFMT_none),

//     /// arithmetic/logic operations
//     OpCode::new(OpCode_neg, 1, 1, 1, OpFMT_none),
//     OpCode::new(OpCode_plus, 1, 1, 1, OpFMT_none),
//     OpCode::new(OpCode_dec, 1, 1, 1, OpFMT_none),
//     OpCode::new(OpCode_inc, 1, 1, 1, OpFMT_none),
//     OpCode::new(OpCode_post_dec, 1, 1, 2, OpFMT_none),
//     OpCode::new(OpCode_post_inc, 1, 1, 2, OpFMT_none),
//     OpCode::new(OpCode_dec_loc, 2, 0, 0, OpFMT_loc8),
//     OpCode::new(OpCode_inc_loc, 2, 0, 0, OpFMT_loc8),
//     OpCode::new(OpCode_add_loc, 2, 1, 0, OpFMT_loc8),
//     OpCode::new(OpCode_not, 1, 1, 1, OpFMT_none),
//     OpCode::new(OpCode_lnot, 1, 1, 1, OpFMT_none),
//     OpCode::new(OpCode_typeof, 1, 1, 1, OpFMT_none),
//     OpCode::new(OpCode_delete, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_delete_var, 5, 0, 1, OpFMT_atom),

//     OpCode::new(OpCode_mul, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_div, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_mod, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_add, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_sub, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_pow, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_shl, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_sar, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_shr, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_lt, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_lte, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_gt, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_gte, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_instanceof, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_in, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_eq, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_neq, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_strict_eq, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_strict_neq, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_and, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_xor, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_or, 1, 2, 1, OpFMT_none),
//     OpCode::new(OpCode_is_undefined_or_null, 1, 1, 1, OpFMT_none),
//     // #ifdef CONFIG_BIGNUM
//     // OpCode::new(OpCode_mul_pow10, 1, 2, 1, none),
//     // OpCode::new(OpCode_math_mod, 1, 2, 1, none),
//     // #endif
//     /* must be the last non short and non temporary opcode */
//     OpCode::new(OpCode_nop, 1, 0, 0, OpFMT_none),
// ];
