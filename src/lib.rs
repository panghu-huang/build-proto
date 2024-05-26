pub(crate) mod rpc;
pub(crate) mod service;

mod protobuf;

use proc_macro::TokenStream;
use protobuf::Proto;
use syn::parse_macro_input;

/// Custom keywords for Protocol Buffers
pub(crate) mod keyword {
  syn::custom_keyword!(package);
  syn::custom_keyword!(service);
  syn::custom_keyword!(rpc);
  syn::custom_keyword!(stream);
  syn::custom_keyword!(returns);
  syn::custom_keyword!(codec);
}

#[proc_macro]
/// `proto` allows you to define Protocol Buffers using Rust code to reuse Rust's type system and ecosystem.
///
/// ### Example
///
/// ```rust,ignore
/// use proto::proto;
///
/// proto! {
///   package example;
///   codec crate::common::YourCodec;
///
///   service Greeter {
///     rpc SayHello (crate::HelloRequest) returns (crate::HelloResponse) {}
///   }
/// }
/// ```
///
pub fn proto(input: TokenStream) -> TokenStream {
  let proto = parse_macro_input!(input as Proto);

  proto.codegen().into()
}
