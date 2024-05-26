use build_proto::build_proto;

#[test]
fn basic() {
  build_proto! {
    package example;

    service Greeter {
      rpc SayHello (crate::HelloRequest) returns (stream crate::HelloResponse) {}
    }
  }
}
