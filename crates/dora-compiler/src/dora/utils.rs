use crate::conversion::rewriter::Rewriter;
use crate::errors::Result;
use melior::{
    dialect::arith::{self},
    ir::{attribute::IntegerAttribute, r#type::IntegerType, *},
    Context,
};

pub(crate) fn round_up_32<'c>(
    size: Value<'c, 'c>,
    context: &'c Context,
    rewriter: &'c Rewriter,
    location: Location<'c>,
) -> Result<Value<'c, 'c>> {
    let uint32 = IntegerType::new(context, 32).into();

    let constant_31 = rewriter
        .create(arith::constant(
            context,
            IntegerAttribute::new(uint32, 31).into(),
            location,
        ))
        .result(0)?
        .into();

    let constant_32 = rewriter
        .create(arith::constant(
            context,
            IntegerAttribute::new(uint32, 32).into(),
            location,
        ))
        .result(0)?
        .into();

    let size_plus_31 = rewriter
        .create(arith::addi(size, constant_31, location))
        .result(0)?
        .into();

    let memory_size_word = rewriter
        .create(arith::divui(size_plus_31, constant_32, location))
        .result(0)?
        .into();

    let memory_size_bytes = rewriter
        .create(arith::muli(memory_size_word, constant_32, location))
        .result(0)?
        .into();

    Ok(memory_size_bytes)
}
