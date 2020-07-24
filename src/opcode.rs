use OpFMT::*;

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

struct OpCode {
    //TODO: 能这么写？
    #[cfg(debug_opcodes)]
    id: u8,
    size: u8,
    n_pop: u8,
    n_push: u8,
    fmt: OpFMT,
}

impl OpCode {
    fn new(
        #[cfg(debug_opcodes)]
        id: u8,
        size: u8,
        n_pop: u8,
        n_push: u8,
        fmt: OpFMT,
    ) -> Self {
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

static mut OP_CODE_INFO: Vec<OpCode> = vec![
    OpCode::new(       push_i32, 5, 0, 1, OpFMT_i32),
    OpCode::new(     push_const, 5, 0, 1, OpFMT_const),
    /* must follow push_const */
    OpCode::new(       fclosure, 5, 0, 1, OpFMT_const),
    OpCode::new(push_atom_value, 5, 0, 1, OpFMT_atom),
    OpCode::new( private_symbol, 5, 0, 1, OpFMT_atom),
    OpCode::new(      undefined, 1, 0, 1, OpFMT_none),
    OpCode::new(           op_null, 1, 0, 1, OpFMT_none),
    /* only used at the start of a function */
    OpCode::new(      push_this, 1, 0, 1, OpFMT_none),
    OpCode::new(     push_false, 1, 0, 1, OpFMT_none),
    OpCode::new(      push_true, 1, 0, 1, OpFMT_none),
    OpCode::new(         object, 1, 0, 1, OpFMT_none),
    /* only used at the start of a function */
    OpCode::new( special_object, 2, 0, 1, OpFMT_u8),
    /* only used at the start of a function */
    OpCode::new(           rest, 3, 0, 1, OpFMT_u16),
    /* a -> */
    OpCode::new(           op_drop, 1, 1, 0, OpFMT_none) ,
     /* a b -> b */
    OpCode::new(            nip, 1, 2, 1, OpFMT_none) ,
    /* a b c -> b c */
    OpCode::new(           nip1, 1, 3, 2, OpFMT_none),
    /* a -> a a */
    OpCode::new(            dup, 1, 1, 2, OpFMT_none),
    /* a b -> a a b */
    OpCode::new(           dup1, 1, 2, 3, OpFMT_none),
    /* a b -> a b a b */
    OpCode::new(           dup2, 1, 2, 4, OpFMT_none),
    /* a b c -> a b c a b c */
    OpCode::new(           dup3, 1, 3, 6, OpFMT_none),
    /* obj a -> a obj a (dup_x1) */
    OpCode::new(        insert2, 1, 2, 3, OpFMT_none),
    /* obj prop a -> a obj prop a (dup_x2) */
    OpCode::new(        insert3, 1, 3, 4, OpFMT_none),
    /* this obj prop a -> a this obj prop a */
    OpCode::new(        insert4, 1, 4, 5, OpFMT_none),
    /* obj a b -> a obj b */
    OpCode::new(          perm3, 1, 3, 3, OpFMT_none),
    /* obj prop a b -> a obj prop b */
    OpCode::new(          perm4, 1, 4, 4, OpFMT_none),
    /* this obj prop a b -> a this obj prop b */
    OpCode::new(          perm5, 1, 5, 5, OpFMT_none),
    /* a b -> b a */
    OpCode::new(           op_swap, 1, 2, 2, OpFMT_none),
    /* a b c d -> c d a b */
    OpCode::new(          swap2, 1, 4, 4, OpFMT_none),
    /* x a b -> a b x */
    OpCode::new(          rot3l, 1, 3, 3, OpFMT_none),
    /* a b x -> x a b */
    OpCode::new(          rot3r, 1, 3, 3, OpFMT_none),
    /* x a b c -> a b c x */
    OpCode::new(          rot4l, 1, 4, 4, OpFMT_none),
    /* x a b c d -> a b c d x */
    OpCode::new(          rot5l, 1, 5, 5, OpFMT_none),
    /* func new.target args -> ret. arguments are not counted in n_pop */
    OpCode::new(call_constructor, 3, 2, 1,OpFMT_npop),

    // arguments are not counted in n_pop
    OpCode::new(           call, 3, 1, 1, OpFMT_npop),
    OpCode::new(      tail_call, 3, 1, 0, OpFMT_npop),
    OpCode::new(    call_method, 3, 2, 1, OpFMT_npop),
    OpCode::new(tail_call_method, 3, 2, 0, OpFMT_npop),
    OpCode::new(     array_from, 3, 0, 1, OpFMT_npop),

    OpCode::new(          apply, 3, 3, 1, OpFMT_u16),
    OpCode::new(         return, 1, 1, 0, OpFMT_none),
    OpCode::new(   return_undef, 1, 0, 0, OpFMT_none),
    OpCode::new(check_ctor_return, 1, 1, 2, OpFMT_none),
    OpCode::new(     check_ctor, 1, 0, 0, OpFMT_none),
    /* this_obj func -> this_obj func */
    OpCode::new(    check_brand, 1, 2, 2, OpFMT_none),
    /* this_obj home_obj -> */
    OpCode::new(      add_brand, 1, 2, 0, OpFMT_none),
    OpCode::new(   return_async, 1, 1, 0, OpFMT_none),
    OpCode::new(          throw, 1, 1, 0, OpFMT_none),
    OpCode::new(      throw_var, 6, 0, 0, OpFMT_atom_u8),
    /* func args... -> ret_val */
    OpCode::new(           eval, 5, 1, 1, OpFMT_npop_u16),
    /* func array -> ret_eval */
    OpCode::new(     apply_eval, 3, 2, 1, OpFMT_u16),
    /* create a RegExp object from the pattern and a bytecode string */
    OpCode::new(         regexp, 1, 2, 1, OpFMT_none),
    OpCode::new(      get_super, 1, 1, 1, OpFMT_none),
    /* dynamic module import */
    OpCode::new(         import, 1, 1, 1, OpFMT_none),

    /* check if a variable exists */
    OpCode::new(      check_var, 5, 0, 1, OpFMT_atom),
    /* push undefined if the variable does not exist */
    OpCode::new(  get_var_undef, 5, 0, 1, OpFMT_atom),
    /* throw an exception if the variable does not exist */
    OpCode::new(        get_var, 5, 0, 1, OpFMT_atom),
    /* must come after get_var */
    OpCode::new(        put_var, 5, 1, 0, OpFMT_atom),
    /* must come after put_var. Used to initialize a global lexical variable */
    OpCode::new(   put_var_init, 5, 1, 0, OpFMT_atom),
    /* for strict mode variable write */
    OpCode::new( put_var_strict, 5, 2, 0, OpFMT_atom),

    OpCode::new(  get_ref_value, 1, 2, 3, OpFMT_none),
    OpCode::new(  put_ref_value, 1, 3, 0, OpFMT_none),

    OpCode::new(     define_var, 6, 0, 0, OpFMT_atom_u8),
    OpCode::new(check_define_var, 6, 0, 0, OpFMT_atom_u8),
    OpCode::new(    define_func, 6, 1, 0, OpFMT_atom_u8),
    OpCode::new(      get_field, 5, 1, 1, OpFMT_atom),
    OpCode::new(     get_field2, 5, 1, 2, OpFMT_atom),
    OpCode::new(      put_field, 5, 2, 0, OpFMT_atom),
    /* obj prop -> value */
    OpCode::new( get_private_field, 1, 2, 1, OpFMT_none),
    /* obj value prop -> */
    OpCode::new( put_private_field, 1, 3, 0, OpFMT_none),
    /* obj prop value -> obj */
    OpCode::new(define_private_field, 1, 3, 1, OpFMT_none),
    OpCode::new(   get_array_el, 1, 2, 1, OpFMT_none),
    /* obj prop -> obj value */
    OpCode::new(  get_array_el2, 1, 2, 2, OpFMT_none),
    OpCode::new(   put_array_el, 1, 3, 0, OpFMT_none),
    /* this obj prop -> value */
    OpCode::new(get_super_value, 1, 3, 1, OpFMT_none),
    /* this obj prop value -> */
    OpCode::new(put_super_value, 1, 4, 0, OpFMT_none),
    OpCode::new(   define_field, 5, 2, 1, OpFMT_atom),
    OpCode::new(       set_name, 5, 1, 1, OpFMT_atom),
    OpCode::new(set_name_computed, 1, 2, 2, OpFMT_none),
    OpCode::new(      set_proto, 1, 2, 1, OpFMT_none),
    OpCode::new(set_home_object, 1, 2, 2, OpFMT_none),
    OpCode::new(define_array_el, 1, 3, 2, OpFMT_none),
    /* append enumerated object, update length */
    OpCode::new(         append, 1, 3, 2, OpFMT_none),
    OpCode::new(copy_data_properties, 2, 3, 3, OpFMT_u8),
    OpCode::new(  define_method, 6, 2, 1, OpFMT_atom_u8),
    /* must come after define_method */
    OpCode::new(define_method_computed, 2, 3, 1, OpFMT_u8),
    /* parent ctor -> ctor proto */
    OpCode::new(   define_class, 6, 2, 2, OpFMT_atom_u8),
    /* field_name parent ctor -> field_name ctor proto (class with computed name) */
    OpCode::new(   define_class_computed, 6, 3, 3, OpFMT_atom_u8),

    OpCode::new(        get_loc, 3, 0, 1, OpFMT_loc),
    /* must come after get_loc */
    OpCode::new(        put_loc, 3, 1, 0, OpFMT_loc),
    /* must come after put_loc */
    OpCode::new(        set_loc, 3, 1, 1, OpFMT_loc),
    OpCode::new(        get_arg, 3, 0, 1, OpFMT_arg),
    /* must come after get_arg */
    OpCode::new(        put_arg, 3, 1, 0, OpFMT_arg),
    /* must come after put_arg */
    OpCode::new(        set_arg, 3, 1, 1, OpFMT_arg),
    OpCode::new(    get_var_ref, 3, 0, 1, OpFMT_var_ref),
    /* must come after get_var_ref */
    OpCode::new(    put_var_ref, 3, 1, 0, OpFMT_var_ref),
    /* must come after put_var_ref */
    OpCode::new(    set_var_ref, 3, 1, 1, OpFMT_var_ref),
    OpCode::new(set_loc_uninitialized, 3, 0, 0, OpFMT_loc),
    OpCode::new(  get_loc_check, 3, 0, 1, OpFMT_loc),
    /* must come after get_loc_check */
    OpCode::new(  put_loc_check, 3, 1, 0, OpFMT_loc),
    OpCode::new(  put_loc_check_init, 3, 1, 0, OpFMT_loc),
    OpCode::new(get_var_ref_check, 3, 0, 1, OpFMT_var_ref),
    /* must come after get_var_ref_check */
    OpCode::new(put_var_ref_check, 3, 1, 0, OpFMT_var_ref),
    OpCode::new(put_var_ref_check_init, 3, 1, 0, OpFMT_var_ref),
    OpCode::new(      close_loc, 3, 0, 0, OpFMT_loc),
    OpCode::new(       if_false, 5, 1, 0, OpFMT_label),
    /* must come after if_false */
    OpCode::new(        if_true, 5, 1, 0, OpFMT_label),
    /* must come after if_true */
    OpCode::new(           goto, 5, 0, 0, OpFMT_label),
    OpCode::new(          catch, 5, 0, 1, OpFMT_label),
    /* used to execute the finally block */
    OpCode::new(          gosub, 5, 0, 0, OpFMT_label),
    /* used to return from the finally block */
    OpCode::new(            ret, 1, 1, 0, OpFMT_none),

    OpCode::new(      to_object, 1, 1, 1, OpFMT_none),
    //OpCode::new(      to_string, 1, 1, 1, none),
    OpCode::new(     to_propkey, 1, 1, 1, OpFMT_none),
    OpCode::new(    to_propkey2, 1, 2, 2, OpFMT_none),

    // must be in the same order as scope_xxx
    OpCode::new(   with_get_var, 10, 1, 0, OpFMT_atom_label_u8),
    OpCode::new(   with_put_var, 10, 2, 1, OpFMT_atom_label_u8),
    OpCode::new(with_delete_var, 10, 1, 0, OpFMT_atom_label_u8),
    OpCode::new(  with_make_ref, 10, 1, 0, OpFMT_atom_label_u8),
    OpCode::new(   with_get_ref, 10, 1, 0, OpFMT_atom_label_u8),

    OpCode::new(with_get_ref_undef, 10, 1, 0, OpFMT_atom_label_u8),

    OpCode::new(   make_loc_ref, 7, 0, 2, OpFMT_atom_u16),
    OpCode::new(   make_arg_ref, 7, 0, 2, OpFMT_atom_u16),
    OpCode::new(make_var_ref_ref, 7, 0, 2, OpFMT_atom_u16),
    OpCode::new(   make_var_ref, 5, 0, 2, OpFMT_atom),

    OpCode::new(   for_in_start, 1, 1, 1, OpFMT_none),
    OpCode::new(   for_of_start, 1, 1, 3, OpFMT_none),
    OpCode::new(for_await_of_start, 1, 1, 3, OpFMT_none),
    OpCode::new(    for_in_next, 1, 1, 3, OpFMT_none),
    OpCode::new(    for_of_next, 2, 3, 5, OpFMT_u8),
    OpCode::new(for_await_of_next, 1, 3, 4, OpFMT_none),
    OpCode::new(iterator_get_value_done, 1, 1, 2, OpFMT_none),
    OpCode::new( iterator_close, 1, 3, 0, OpFMT_none),
    OpCode::new(iterator_close_return, 1, 4, 4, OpFMT_none),
    OpCode::new(async_iterator_close, 1, 3, 2, OpFMT_none),
    OpCode::new(async_iterator_next, 1, 4, 4, OpFMT_none),
    OpCode::new(async_iterator_get, 2, 4, 5, OpFMT_u8),
    OpCode::new(  initial_yield, 1, 0, 0, OpFMT_none),
    OpCode::new(          op_yield, 1, 1, 2, OpFMT_none),
    OpCode::new(     yield_star, 1, 2, 2, OpFMT_none),
    OpCode::new(async_yield_star, 1, 1, 2, OpFMT_none),
    OpCode::new(          op_await, 1, 1, 1, OpFMT_none),

    /// arithmetic/logic operations 
    OpCode::new(            neg, 1, 1, 1, OpFMT_none),
    OpCode::new(           plus, 1, 1, 1, OpFMT_none),
    OpCode::new(            dec, 1, 1, 1, OpFMT_none),
    OpCode::new(            inc, 1, 1, 1, OpFMT_none),
    OpCode::new(       post_dec, 1, 1, 2, OpFMT_none),
    OpCode::new(       post_inc, 1, 1, 2, OpFMT_none),
    OpCode::new(        dec_loc, 2, 0, 0, OpFMT_loc8),
    OpCode::new(        inc_loc, 2, 0, 0, OpFMT_loc8),
    OpCode::new(        add_loc, 2, 1, 0, OpFMT_loc8),
    OpCode::new(            not, 1, 1, 1, OpFMT_none),
    OpCode::new(           lnot, 1, 1, 1, OpFMT_none),
    OpCode::new(         typeof, 1, 1, 1, OpFMT_none),
    OpCode::new(         delete, 1, 2, 1, OpFMT_none),
    OpCode::new(     delete_var, 5, 0, 1, OpFMT_atom),

    OpCode::new(            mul, 1, 2, 1, OpFMT_none),
    OpCode::new(            div, 1, 2, 1, OpFMT_none),
    OpCode::new(            mod, 1, 2, 1, OpFMT_none),
    OpCode::new(            add, 1, 2, 1, OpFMT_none),
    OpCode::new(            sub, 1, 2, 1, OpFMT_none),
    OpCode::new(            pow, 1, 2, 1, OpFMT_none),
    OpCode::new(            shl, 1, 2, 1, OpFMT_none),
    OpCode::new(            sar, 1, 2, 1, OpFMT_none),
    OpCode::new(            shr, 1, 2, 1, OpFMT_none),
    OpCode::new(             lt, 1, 2, 1, OpFMT_none),
    OpCode::new(            lte, 1, 2, 1, OpFMT_none),
    OpCode::new(             gt, 1, 2, 1, OpFMT_none),
    OpCode::new(            gte, 1, 2, 1, OpFMT_none),
    OpCode::new(     instanceof, 1, 2, 1, OpFMT_none),
    OpCode::new(             in, 1, 2, 1, OpFMT_none),
    OpCode::new(             eq, 1, 2, 1, OpFMT_none),
    OpCode::new(            neq, 1, 2, 1, OpFMT_none),
    OpCode::new(      strict_eq, 1, 2, 1, OpFMT_none),
    OpCode::new(     strict_neq, 1, 2, 1, OpFMT_none),
    OpCode::new(            and, 1, 2, 1, OpFMT_none),
    OpCode::new(            xor, 1, 2, 1, OpFMT_none),
    OpCode::new(             or, 1, 2, 1, OpFMT_none),
    OpCode::new(is_undefined_or_null, 1, 1, 1, OpFMT_none),
    // #ifdef CONFIG_BIGNUM
    // OpCode::new(      mul_pow10, 1, 2, 1, none),
    // OpCode::new(       math_mod, 1, 2, 1, none),
    // #endif
    /* must be the last non short and non temporary opcode */
    OpCode::new(            nop, 1, 0, 0, OpFMT_none),
];