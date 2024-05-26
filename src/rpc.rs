use crate::keyword;
use syn::{
  braced, parenthesized,
  parse::{Parse, ParseBuffer, ParseStream},
  Ident, Result, Type,
};

pub struct Rpc {
  pub name: Ident,
  pub request: Type,
  pub response: Type,
  pub request_streaming: bool,
  pub response_streaming: bool,
}

impl Parse for Rpc {
  fn parse(input: ParseStream) -> Result<Self> {
    input.parse::<keyword::rpc>()?;

    let name = input.parse::<Ident>()?;

    let content: ParseBuffer;
    parenthesized!(content in input);

    let request_streaming = if content.peek(keyword::stream) {
      content.parse::<keyword::stream>()?;
      true
    } else {
      false
    };

    let request = content.parse::<Type>()?;

    input.parse::<keyword::returns>()?;

    let content: ParseBuffer;
    parenthesized!(content in input);

    let response_streaming = if content.peek(keyword::stream) {
      content.parse::<keyword::stream>()?;
      true
    } else {
      false
    };

    let response = content.parse::<Type>()?;

    let _content: ParseBuffer;
    braced!(_content in input);

    Ok(Self {
      name,
      request,
      response,
      request_streaming,
      response_streaming,
    })
  }
}
