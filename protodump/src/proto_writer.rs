use std::fs;

use protobuf::{
    descriptor::{DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto},
    Message,
};

pub struct ProtoWriter {
    pub indent: usize,
    result: String,
    proto: FileDescriptorProto,
}

// TODO get rid of the `.clone()` but I'll do it later :p
impl ProtoWriter {
    pub fn new(bytes: &[u8]) -> Self {
        let msg = FileDescriptorProto::parse_from_bytes(bytes).unwrap();

        Self {
            indent: 0,
            result: String::new(),
            proto: msg,
        }
    }

    pub fn push(&mut self, ch: char) {
        self.result.push(ch);
    }

    pub fn push_str(&mut self, string: &str) {
        self.result.push_str(string);
    }

    pub fn push_str_indented(&mut self, string: &str) {
        self.result
            .push_str(&format!("{:indent$}{}", "", string, indent = self.indent));
    }

    pub fn indent(&mut self) {
        self.indent += 1;
    }

    pub fn deindent(&mut self) {
        self.indent -= 1;
    }

    pub fn generate_proto_file(&mut self) {
        self.push_str("syntax = \"");
        self.push_str(self.proto.clone().syntax());
        self.push_str("\";\n\n");

        self.push_str("package ");
        self.push_str(self.proto.clone().package());
        self.push_str(";\n\n");

        // TODO file options
        // kind of irrelevant if using swift/rust
        // might add is later since it's annoying to do so

        self.write_dependencies();

        // TODO services
        // afaik the Dofus protocol does not implement any services or rpc related
        // code in its proto definition
        // if it ever does then we have to implement this
        if !self.proto.service.is_empty() {
            panic!("service")
        }

        for message in &self.proto.clone().message_type {
            self.write_message(message);
        }

        for enum_proto in &self.proto.clone().enum_type {
            self.write_enum(enum_proto);
        }

        // write file
        fs::write(self.proto.name(), self.result.clone()).expect("Unable to write file");
    }

    fn write_dependencies(&mut self) {
        for (i, dependency) in self.proto.clone().dependency.iter().enumerate() {
            self.push_str("import ");
            if self.proto.public_dependency.contains(&(i as i32)) {
                self.push_str("public ");
            }
            self.push('"');
            self.push_str(dependency);
            self.push_str("\";\n");
        }

        if !self.proto.dependency.is_empty() {
            self.result.push('\n');
        }
    }

    fn write_message(&mut self, msg: &DescriptorProto) {
        self.push_str_indented("message ");
        self.push_str(msg.name());
        self.push_str(" {\n");
        self.indent();

        // dofus proto does not use it afaik
        if !msg.reserved_name.is_empty() {
            panic!("reserverd name")
        }

        // dofus proto does not use it afaik
        if !msg.reserved_range.is_empty() {
            panic!("reserverd range")
        }

        for message in &msg.nested_type {
            self.write_message(message);
        }

        for enum_proto in &msg.enum_type {
            self.write_enum(enum_proto);
        }

        for field in &msg.field {
            if field.proto3_optional() || !field.has_oneof_index() {
                self.write_field(field);
            }
        }

        self.write_one_of(msg);

        self.deindent();
        self.push_str_indented("}\n\n");
    }

    fn write_enum(&mut self, enum_proto: &EnumDescriptorProto) {
        self.push_str_indented("enum ");
        self.push_str(enum_proto.name());
        self.push_str(" {\n");
        self.indent();

        for value in enum_proto.value.clone() {
            self.push_str_indented(value.name());
            self.push_str(" = ");
            self.push_str(&format!("{}", value.number()));
            self.push_str(";\n");
        }

        self.deindent();
        self.push_str_indented("}\n\n");
    }

    fn write_field(&mut self, field: &FieldDescriptorProto) {
        self.push_str_indented("");

        if field.has_label() {
            match field.label() {
                protobuf::descriptor::field_descriptor_proto::Label::LABEL_OPTIONAL => {
                    // this is important, since the binary will mark all of them
                    // as optional label for proto2 compability
                    if field.proto3_optional() {
                        self.push_str("optional ")
                    }
                }
                protobuf::descriptor::field_descriptor_proto::Label::LABEL_REQUIRED => {
                    self.push_str("required ")
                }
                protobuf::descriptor::field_descriptor_proto::Label::LABEL_REPEATED => {
                    self.push_str("repeated ")
                }
            }
        }

        self.write_type(field);
        self.push_str(" ");
        self.push_str(field.name());
        self.push_str(" = ");
        self.push_str(&format!("{}", field.number()));

        if field.has_default_value() {
            self.push_str(" [default =");

            println!("default: {}", field.default_value());

            self.push_str(field.default_value());
            self.push_str("]")
        }

        self.push_str(";\n");
    }

    fn write_one_of(&mut self, proto: &DescriptorProto) {
        for (i, one_of) in proto.oneof_decl.iter().enumerate() {
            let fields: Vec<&FieldDescriptorProto> = proto
                .field
                .iter()
                .filter(|x| {
                    !x.proto3_optional()
                        && x.has_oneof_index()
                        && x.oneof_index() == i.try_into().unwrap()
                })
                .collect();

            if fields.len() > 1 {
                self.push_str_indented("");
                self.push_str("oneof ");
                self.push_str(one_of.name());
                self.push_str(" {\n");
                self.indent();

                for field in fields {
                    self.write_field(field);
                }

                self.deindent();
                self.push_str_indented("}\n");
            }
        }
    }

    fn write_type(&mut self, field: &FieldDescriptorProto) {
        if !field.has_type() {
            panic!("no type")
        }

        match field.type_() {
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_DOUBLE => {
                self.push_str("double")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_FLOAT => {
                self.push_str("float")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_INT64 => {
                self.push_str("int64")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_UINT64 => {
                self.push_str("uint64")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_INT32 => {
                self.push_str("int32")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_FIXED64 => {
                self.push_str("fixed64")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_FIXED32 => {
                self.push_str("fixed32")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL => self.push_str("bool"),
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING => {
                self.push_str("string")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_GROUP => {
                // afaik dofus proto does not use this
                println!("group: {:?}", field);
                panic!("group (MAP)")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_MESSAGE
            | protobuf::descriptor::field_descriptor_proto::Type::TYPE_ENUM => {
                self.push_str(field.type_name());
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_BYTES => {
                self.push_str("bytes")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_UINT32 => {
                self.push_str("uint32")
            }

            protobuf::descriptor::field_descriptor_proto::Type::TYPE_SFIXED32 => {
                self.push_str("sfixed32")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_SFIXED64 => {
                self.push_str("sfixed64")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_SINT32 => {
                self.push_str("sint32")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_SINT64 => {
                self.push_str("sint64")
            }
        }
    }
}
