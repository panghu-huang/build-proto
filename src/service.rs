use crate::{keyword, rpc::Rpc};
use syn::{
  braced,
  parse::{Parse, ParseBuffer, ParseStream},
  Ident, Result,
};

pub struct Service {
  pub name: Ident,
  pub rpcs: Vec<Rpc>,
}

impl Parse for Service {
  fn parse(input: ParseStream) -> Result<Self> {
    input.parse::<keyword::service>()?;

    let name = input.parse::<Ident>()?;

    let content: ParseBuffer;
    braced!(content in input);

    let mut rpcs = Vec::new();

    while !content.is_empty() {
      let rpc = content.parse::<Rpc>()?;
      rpcs.push(rpc);
    }

    Ok(Self { name, rpcs })
  }
}
