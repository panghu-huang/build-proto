use proto::proto;

#[test]
fn basic() {
  let service = proto! {
    package example;
    codec crate::common::JsonCodec;

    service Greeter {
      rpc SayHello (crate::HelloRequest) returns (stream crate::HelloResponse) {}
    }
  };

  tonic_build::manual::Builder::new()
    .out_dir("./tests/pb")
    .compile(&[service]);
}
