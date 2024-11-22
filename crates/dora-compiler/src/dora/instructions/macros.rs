#[macro_export]
macro_rules! operands {
    ($op:expr, $o1:ident) => {
        let $o1 = $op.operand(0)?;
    };
    ($op:expr, $o1:ident, $o2:ident) => {
        let ($o1, $o2) = ($op.operand(0)?, $op.operand(1)?);
    };
    ($op:expr, $o1:ident, $o2:ident, $o3:ident) => {
        let ($o1, $o2, $o3) = ($op.operand(0)?, $op.operand(1)?, $op.operand(2)?);
    };
    ($op:expr, $o1:ident, $o2:ident, $o3:ident, $o4:ident) => {
        let ($o1, $o2, $o3, $o4) = (
            $op.operand(0)?,
            $op.operand(1)?,
            $op.operand(2)?,
            $op.operand(3)?,
        );
    };
}

#[macro_export]
macro_rules! rewrite_ctx {
    ($context:expr, $op:expr, $rewriter:ident, $loc:ident) => {
        let r = DeferredRewriter::new_with_op($context, *$op);
        let l = r.get_insert_location();

        let ($rewriter, $loc) = (r, l);

        scopeguard::defer! {
            $rewriter.remove();
        }
    };
}

#[macro_export]
macro_rules! syscall_ctx {
    ($op:expr, $syscall_ctx:ident) => {
        let func_op = $op.parent_operation().unwrap();
        let region = func_op.region(0).unwrap();
        let func_block = region.first_block().unwrap();
        let $syscall_ctx = func_block.argument(0).unwrap();
    };
}

#[macro_export]
macro_rules! arith_constant {
    ($rewriter:expr, $context:expr, $ty:expr, $value:expr, $location:expr) => {
        arith::constant(
            $context,
            IntegerAttribute::new($ty.into(), $value).into(),
            $location,
        )
    };
}

#[macro_export]
macro_rules! create_var {
    ($rewriter:expr, $context:expr, $location:expr) => {{
        let array_size = $rewriter.make(arith_constant!(
            $rewriter,
            $context,
            $rewriter.intrinsics.i256_ty,
            1_i64,
            $location
        ))?;
        $rewriter.make(llvm::alloca(
            $context,
            array_size,
            $rewriter.ptr_ty(),
            $location,
            AllocaOptions::new().elem_type(Some(TypeAttribute::new(
                $rewriter.intrinsics.i256_ty.into(),
            ))),
        ))?
    }};
    ($rewriter:expr, $context:expr, $elm_type:expr, $location:expr) => {{
        let array_size = $rewriter.make(arith_constant!(
            $rewriter, $context, $elm_type, 1_i64, $location
        ))?;
        $rewriter.make(llvm::alloca(
            $context,
            array_size,
            $rewriter.ptr_ty(),
            $location,
            AllocaOptions::new().elem_type(Some(TypeAttribute::new($elm_type))),
        ))?
    }};
}

#[macro_export]
macro_rules! load_var {
    ($rewriter:expr, $context:expr, $arg:expr, $ltype:expr, $location:expr) => {{
        $rewriter.make(llvm::load(
            $context,
            $arg,
            $ltype,
            $location,
            LoadStoreOptions::default(),
        ))?
    }};
    ($rewriter:expr, $context:expr, $syscall_ctx:expr, $symbol:expr, $args:expr, $ltype:expr, $location:expr) => {{
        let args = [&[$syscall_ctx.into()][..], &$args[..]].concat();
        $rewriter.create(func::call(
            $context,
            FlatSymbolRefAttribute::new($context, $symbol),
            &args,
            &[],
            $location,
        ));
        $rewriter.make(llvm::load(
            $context,
            $args[0],
            $ltype,
            $location,
            LoadStoreOptions::default(),
        ))?
    }};
    ($rewriter:expr, $context:expr, $syscall_ctx:expr, $symbol:expr, $args:expr, $laddr:expr, $ltype:expr, $location:expr) => {{
        let args = [&[$syscall_ctx.into()][..], &$args[..]].concat();
        $rewriter.create(func::call(
            $context,
            FlatSymbolRefAttribute::new($context, $symbol),
            &args,
            &[],
            $location,
        ));
        $rewriter.make(llvm::load(
            $context,
            $laddr,
            $ltype,
            $location,
            LoadStoreOptions::default(),
        ))?
    }};
    ($rewriter:expr, $context:expr, $syscall_ctx:expr, $symbol:expr, $args:expr, $rtn_types:expr, $laddr:expr, $ltype:expr, $location:expr) => {{
        let args = [&[$syscall_ctx.into()][..], &$args[..]].concat();
        $rewriter.create(func::call(
            $context,
            FlatSymbolRefAttribute::new($context, $symbol),
            &args,
            &$rtn_types,
            $location,
        ));
        $rewriter.make(llvm::load(
            $context,
            $laddr,
            $ltype,
            $location,
            LoadStoreOptions::default(),
        ))?
    }};
    ($rewriter:expr, $context:expr, $syscall_ctx:expr, $symbol:expr, $args:expr, $rtn_types:expr, $laddr:expr, $ltype:expr, $location:expr, $extra_options:expr) => {{
        let args = [&[$syscall_ctx.into()][..], &$args[..]].concat();
        $rewriter.create(func::call(
            $context,
            FlatSymbolRefAttribute::new($context, $symbol),
            &args,
            &rtn_types,
            $location,
        ));
        $rewriter.make(llvm::load(
            $context,
            $laddr,
            $ltype,
            $location,
            $extra_options,
        ))?
    }};
}

#[macro_export]
macro_rules! load_by_addr {
    ($rewriter:expr, $name:expr, $rtn_type:expr) => {
        $rewriter.make($rewriter.load(
            $rewriter.make($rewriter.addressof($name, $rewriter.ptr_ty()))?,
            $rtn_type,
        ))?
    };
}

#[macro_export]
macro_rules! store_var {
    ($rewriter:expr, $context:expr, $value:expr, $addr:expr, $location:expr) => {{
        llvm::store(
            $context,
            $value,
            $addr,
            $location,
            LoadStoreOptions::default(),
        )
    }};
    ($rewriter:expr, $context:expr, $value:expr, $addr:expr, $location:expr, $extra_options:expr) => {{
        llvm::store($context, $value, $addr, $location, $extra_options)
    }};
}

#[macro_export]
macro_rules! maybe_revert_here {
    ($op:expr, $rewriter:expr, $cond:expr) => {
        if let Some(block) = $op.block() {
            if let Some(region) = block.parent_region() {
                if let Some(setup_block) = region.first_block() {
                    if let Some(revert_block) = setup_block.next_in_region() {
                        if let Some(insert_point) = $rewriter.get_insert_point() {
                            let next_block = $rewriter.split_block(block, Some(insert_point))?;
                            block.append_operation(cf::cond_br(
                                $rewriter.context(),
                                $cond,
                                &revert_block,
                                &next_block,
                                &[],
                                &[],
                                $rewriter.get_insert_location(),
                            ));
                        }
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! check_resize_memory {
    ($op:expr, $rewriter:expr, $required_memory_size:expr) => {
        // Check the memory offset halt error
        let zero = $rewriter.make($rewriter.iconst_64(0))?;
        let overflow =
            $rewriter.make($rewriter.icmp(IntCC::SignedLessThan, $required_memory_size, zero))?;
        maybe_revert_here!($op, $rewriter, overflow);
    };
}

#[macro_export]
macro_rules! check_u256_to_u64_overflow {
    ($op:expr, $rewriter:expr, $required_memory_size:expr) => {
        // Check the memory offset halt error
        let max_u64 =
            $rewriter.make($rewriter.iconst_256(BigUint::from(18446744073709551615_u128))?)?;
        let overflow = $rewriter.make($rewriter.icmp(
            IntCC::UnsignedGreaterThan,
            $required_memory_size,
            max_u64,
        ))?;
        maybe_revert_here!($op, $rewriter, overflow);
    };
}
