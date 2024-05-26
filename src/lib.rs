mod proto_context;

use proc_macro::TokenStream;
use proto_context::ProtoContext;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
/// `build_proto` allows you to define Protocol Buffers using Rust code to reuse Rust's type system and ecosystem.
///
/// ### Example
/// ```rust
/// use build_proto::build_proto;
///
/// build_proto! {
///   package example;
///
///   service Greeter {
///     rpc SayHello (crate::HelloRequest) returns (crate::HelloResponse) {}
///   }
/// }
/// ```
///
pub fn build_proto(input: TokenStream) -> TokenStream {
  let ctx = parse_macro_input!(input as ProtoContext);

  println!("{:#?}", ctx);

  quote!().into()
}
