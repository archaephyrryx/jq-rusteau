use chumsky::prelude::*;
use jqlang::FilterOp;

pub fn parser<'a>() -> impl Parser<char, Vec<FilterOp>, Error = Simple<char>> {
    let filtop = just::<char, _, Simple<char>>('.').to(FilterOp::Identity).padded();
    filtop.separated_by(just('|').padded()).collect()
}
