# proto

`proto` allows you to define Protocol Buffers using Rust code to reuse Rust's type system and ecosystem.

## Example

```rust
use proto::proto;

fn main() {
  // Define a Protocol Buffers service
  let service = proto! {
    package example;
    codec crate::common::JsonCodec;

    service Greeter {
      rpc SayHello (crate::HelloRequest) returns (stream crate::HelloResponse) {}
    }
  };

  tonic_build::manual::Builder::new()
    .out_dir("./pb")
    .compile(&[service]);
}
```