extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/cast",
        input: &[
            "protobuf/authority_keys.proto",
            "protobuf/cast_channel.proto",
        ],
        includes: &["protobuf"],
        customize: Customize {
            ..Default::default()
        },
    }).expect("protoc");
}
