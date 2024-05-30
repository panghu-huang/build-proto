use crate::{case::RenameRule, keyword, service::Service};
use proc_macro::TokenStream;
use syn::{
  parse::{Parse, ParseStream},
  Ident, Result, Token, Type,
};

pub struct Proto {
  pub package: Ident,
  pub service: Service,
  pub codec_path: Type,
}

#[derive(Default)]
struct ProtoBuilder {
  package: Option<Ident>,
  service: Option<Service>,
  codec_path: Option<Type>,
}

impl Parse for Proto {
  fn parse(input: ParseStream) -> Result<Self> {
    let mut builder = ProtoBuilder::new();

    while !input.is_empty() {
      if input.peek(keyword::package) {
        input.parse::<keyword::package>()?;

        let package = input.parse::<Ident>()?;

        builder.package(package);

        input.parse::<Token![;]>()?;

        continue;
      }

      if input.peek(keyword::service) {
        let service = input.parse::<Service>()?;

        builder.service(service);

        continue;
      }

      if input.peek(keyword::codec) {
        input.parse::<keyword::codec>()?;

        let codec_path = input.parse::<Type>()?;

        builder.codec_path(codec_path);

        input.parse::<Token![;]>()?;

        continue;
      }

      return Err(input.error("Unexpected token. Expected `package`, `service`, or `codec`"));
    }

    Ok(builder.build())
  }
}

impl Proto {
  pub fn codegen(&self) -> TokenStream {
    let rpcs = self.service.rpcs.iter().map(|rpc| {
      let name = RenameRule::SnakeCase.apply_to_variant(&rpc.name.to_string());
      let name = Ident::new(&name, rpc.name.span());

      let route_name = RenameRule::PascalCase.apply_to_field(&rpc.name.to_string());
      let route_name = Ident::new(&route_name, rpc.name.span());

      let request = &rpc.request;
      let response = &rpc.response;
      let codec_path = &self.codec_path;
      let request_streaming = rpc.request_streaming;
      let response_streaming = rpc.response_streaming;

      let client_streaming = if request_streaming {
        quote::quote! {
          .client_streaming()
        }
      } else {
        quote::quote! {}
      };

      let server_streaming = if response_streaming {
        quote::quote! {
          .server_streaming()
        }
      } else {
        quote::quote! {}
      };

      quote::quote! {
        tonic_build::manual::Method::builder()
          .name(stringify!(#name))
          .route_name(stringify!(#route_name))
          .input_type(stringify!(#request))
          .output_type(stringify!(#response))
          .codec_path(stringify!(#codec_path))
          #client_streaming
          #server_streaming
          .build()
      }
    });

    let package = &self.package;
    let service = &self.service.name;

    quote::quote! {
      {
        tonic_build::manual::Service::builder()
        .package(stringify!(#package))
        .name(stringify!(#service))
        #(.method(#rpcs))*
        .build()
      }
    }
    .into()
  }
}

impl ProtoBuilder {
  fn new() -> Self {
    Self::default()
  }

  fn package(&mut self, package: Ident) -> &mut Self {
    self.package = Some(package);

    self
  }

  fn codec_path(&mut self, codec_path: Type) -> &mut Self {
    self.codec_path = Some(codec_path);

    self
  }

  fn service(&mut self, service: Service) -> &mut Self {
    if self.service.is_some() {
      panic!("Currently only one service is supported per proto definition");
    }

    self.service = Some(service);

    self
  }

  fn build(self) -> Proto {
    Proto {
      package: self.package.expect("Package is required"),
      service: self.service.expect("Service is required"),
      codec_path: self.codec_path.expect("Codec path is required"),
    }
  }
}
