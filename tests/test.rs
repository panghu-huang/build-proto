use proto::proto;

#[test]
fn basic() {
  let greeter_service = proto! {
    package example;
    codec crate::common::JsonCodec;

    service Greeter {
      rpc SayHello (crate::HelloRequest) returns (stream crate::HelloResponse) {}
    }
  };

  let helloworld_service = proto! {
    package example;
    codec crate::common::JsonCodec;

    service HelloWorld {
      rpc hello_world (crate::HelloRequest) returns (stream crate::HelloResponse) {}
    }
  };

  tonic_build::manual::Builder::new()
    .out_dir("./tests/pb")
    .compile(&[greeter_service, helloworld_service]);
}
