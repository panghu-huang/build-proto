use syn::{
  braced, parenthesized,
  parse::{Parse, ParseBuffer, ParseStream},
  Ident, Result, Token, Type,
};

mod keyword {
  syn::custom_keyword!(package);
  syn::custom_keyword!(service);
  syn::custom_keyword!(rpc);
  syn::custom_keyword!(stream);
  syn::custom_keyword!(returns);
}

#[derive(Debug)]
pub struct Rpc {
  pub name: Ident,
  pub request: Type,
  pub response: Type,
  pub request_streaming: bool,
  pub response_streaming: bool,
}

#[derive(Debug)]
pub struct Service {
  pub name: Ident,
  pub rpcs: Vec<Rpc>,
}

#[derive(Debug)]
pub struct ProtoContext {
  pub package: Option<Ident>,
  pub services: Vec<Service>,
}

#[derive(Default)]
struct ProtoContextBuilder {
  package: Option<Ident>,
  services: Vec<Service>,
}

/// Parse the `Rpc` struct
///
/// ```
/// rpc SayHello (crate::HelloRequest) returns (stream crate::HelloResponse) {}
/// ```
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

impl Parse for ProtoContext {
  fn parse(input: ParseStream) -> Result<Self> {
    let mut builder = ProtoContextBuilder::new();

    // let content: ParseBuffer;
    // braced!(content in input);
    let content = input;

    while !input.is_empty() {
      // let key = content.parse::<Ident>()?;

      // match key.to_string().as_str() {
      //   "package" => {
      //     let package = content.parse::<Ident>()?;

      //     println!("package: {}", package);
      //     builder = builder.package(package);

      //     content.parse::<Token![;]>()?;
      //   }
      //   "service" => {
      //     let service = content.parse::<Service>()?;

      //     println!("service: {:#?}", service);
      //     builder = builder.service(service);
      //   }
      //   _ => {
      //     return Err(syn::Error::new(key.span(), "Unknown key"));
      //   }
      // }
      let is_package = content.peek(keyword::package);

      if is_package {
        content.parse::<keyword::package>()?;
        let package = content.parse::<Ident>()?;

        builder = builder.package(package);

        content.parse::<Token![;]>()?;
      } else {
        let service = content.parse::<Service>()?;

        builder = builder.service(service);
      }
    }

    Ok(builder.build())
  }
}

impl ProtoContextBuilder {
  fn new() -> Self {
    Self::default()
  }

  fn package(mut self, package: Ident) -> Self {
    self.package = Some(package);
    self
  }

  fn service(mut self, service: Service) -> Self {
    self.services.push(service);
    self
  }

  fn build(self) -> ProtoContext {
    ProtoContext {
      package: self.package,
      services: self.services,
    }
  }
}
