mod proto_result;
mod proto_writer;

use std::{
    fs::{self, File},
    io::Read,
};

use proto_result::ProtoResult;
use protobuf::{descriptor::FileDescriptorProto, Message};

fn main() {
    let mut f = File::open("proto.bin").unwrap();
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).unwrap();

    let msg = FileDescriptorProto::parse_from_bytes(&buffer).unwrap();

    let proto = ProtoResult::new(msg);

    fs::write("dofus.proto", proto.value).expect("Unable to write file");
}
