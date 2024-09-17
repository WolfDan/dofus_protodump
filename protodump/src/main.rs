mod proto_writer;

use std::{fs::File, io::Read};

use proto_writer::ProtoWriter;

fn main() {
    let mut f = File::open("proto.bin").unwrap();
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).unwrap();

    let mut proto = ProtoWriter::new(&buffer);

    proto.generate_proto_file();
}
