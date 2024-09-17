use protobuf::descriptor::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
};

use crate::proto_writer::ProtoWriter;

pub struct ProtoResult {
    pub value: String,
}

impl ProtoResult {
    pub fn new(proto: FileDescriptorProto) -> Self {
        Self {
            value: Self::write_file_descriptor(proto),
        }
    }

    fn write_file_descriptor(proto: FileDescriptorProto) -> String {
        let mut result = ProtoWriter::new();

        result.push_str("syntax = \"");
        result.push_str(proto.syntax());
        result.push_str("\";\n\n");

        result.push_str("package ");
        result.push_str(proto.package());
        result.push_str(";\n\n");

        // TODO file options
        // kind of irrelevant if using swift/rust
        // might add is later since it's annoying to do so

        Self::write_dependencies(&mut result, proto.clone());

        // TODO services
        // afaik the Dofus protocol does not implement any services or rpc related
        // code in its proto definition
        // if it ever does then we have to implement this
        if !proto.service.is_empty() {
            panic!("service")
        }

        for message in proto.message_type {
            Self::write_message(&mut result, &message);
        }

        for enum_proto in proto.enum_type {
            Self::write_enum(&mut result, &enum_proto);
        }

        result.result()
    }

    fn write_dependencies(result: &mut ProtoWriter, proto: FileDescriptorProto) {
        for (i, dependency) in proto.dependency.iter().enumerate() {
            result.push_str("import ");
            if proto.public_dependency.contains(&(i as i32)) {
                result.push_str("public ");
            }
            result.push('"');
            result.push_str(dependency);
            result.push_str("\";\n");
        }

        if !proto.dependency.is_empty() {
            result.push('\n');
        }
    }

    fn write_message(result: &mut ProtoWriter, msg: &DescriptorProto) {
        result.push_str_indented("message ");
        result.push_str(msg.name());
        result.push_str(" {\n");
        result.indent();

        // dofus proto does not use it afaik
        if !msg.reserved_name.is_empty() {
            panic!("reserverd name")
        }

        // dofus proto does not use it afaik
        if !msg.reserved_range.is_empty() {
            panic!("reserverd range")
        }

        for message in &msg.nested_type {
            Self::write_message(result, message);
        }

        for enum_proto in &msg.enum_type {
            Self::write_enum(result, enum_proto);
        }

        for field in &msg.field {
            if field.proto3_optional() || !field.has_oneof_index() {
                Self::write_field(result, field);
            }
        }

        Self::write_one_of(result, msg);

        result.deindent();
        result.push_str_indented("}\n\n");
    }

    fn write_enum(result: &mut ProtoWriter, enum_proto: &EnumDescriptorProto) {
        result.push_str_indented("enum ");
        result.push_str(enum_proto.name());
        result.push_str(" {\n");
        result.indent();

        for value in enum_proto.value.clone() {
            result.push_str_indented(value.name());
            result.push_str(" = ");
            result.push_str(&format!("{}", value.number()));
            result.push_str(";\n");
        }

        result.deindent();
        result.push_str_indented("}\n\n");
    }

    fn write_field(result: &mut ProtoWriter, field: &FieldDescriptorProto) {
        result.push_str_indented("");

        if field.has_label() {
            match field.label() {
                protobuf::descriptor::field_descriptor_proto::Label::LABEL_OPTIONAL => {
                    // this is important, since the binary will mark all of them
                    // as optional label for proto2 compability
                    if field.proto3_optional() {
                        result.push_str("optional ")
                    }
                }
                protobuf::descriptor::field_descriptor_proto::Label::LABEL_REQUIRED => {
                    result.push_str("required ")
                }
                protobuf::descriptor::field_descriptor_proto::Label::LABEL_REPEATED => {
                    result.push_str("repeated ")
                }
            }
        }

        Self::write_type(result, field);
        result.push_str(" ");
        result.push_str(field.name());
        result.push_str(" = ");
        result.push_str(&format!("{}", field.number()));

        if field.has_default_value() {
            result.push_str(" [default =");

            println!("default: {}", field.default_value());

            result.push_str(field.default_value());
            result.push_str("]")
        }

        result.push_str(";\n");
    }

    fn write_one_of(result: &mut ProtoWriter, proto: &DescriptorProto) {
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
                result.push_str_indented("");
                result.push_str("oneof ");
                result.push_str(one_of.name());
                result.push_str(" {\n");
                result.indent();

                for field in fields {
                    Self::write_field(result, field);
                }

                result.deindent();
                result.push_str_indented("}\n");
            }
        }
    }

    fn write_type(result: &mut ProtoWriter, field: &FieldDescriptorProto) {
        if !field.has_type() {
            panic!("no type")
        }

        match field.type_() {
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_DOUBLE => {
                result.push_str("double")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_FLOAT => {
                result.push_str("float")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_INT64 => {
                result.push_str("int64")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_UINT64 => {
                result.push_str("uint64")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_INT32 => {
                result.push_str("int32")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_FIXED64 => {
                result.push_str("fixed64")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_FIXED32 => {
                result.push_str("fixed32")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL => {
                result.push_str("bool")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING => {
                result.push_str("string")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_GROUP => {
                // afaik dofus proto does not use this
                println!("group: {:?}", field);
                panic!("group (MAP)")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_MESSAGE
            | protobuf::descriptor::field_descriptor_proto::Type::TYPE_ENUM => {
                result.push_str(field.type_name());
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_BYTES => {
                result.push_str("bytes")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_UINT32 => {
                result.push_str("uint32")
            }

            protobuf::descriptor::field_descriptor_proto::Type::TYPE_SFIXED32 => {
                result.push_str("sfixed32")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_SFIXED64 => {
                result.push_str("sfixed64")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_SINT32 => {
                result.push_str("sint32")
            }
            protobuf::descriptor::field_descriptor_proto::Type::TYPE_SINT64 => {
                result.push_str("sint64")
            }
        }
    }
}
