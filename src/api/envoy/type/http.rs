// This file is generated by rust-protobuf 2.18.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `envoy/type/http.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CodecClientType {
    HTTP1 = 0,
    HTTP2 = 1,
    HTTP3 = 2,
}

impl ::protobuf::ProtobufEnum for CodecClientType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CodecClientType> {
        match value {
            0 => ::std::option::Option::Some(CodecClientType::HTTP1),
            1 => ::std::option::Option::Some(CodecClientType::HTTP2),
            2 => ::std::option::Option::Some(CodecClientType::HTTP3),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CodecClientType] = &[
            CodecClientType::HTTP1,
            CodecClientType::HTTP2,
            CodecClientType::HTTP3,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<CodecClientType>("CodecClientType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for CodecClientType {
}

impl ::std::default::Default for CodecClientType {
    fn default() -> Self {
        CodecClientType::HTTP1
    }
}

impl ::protobuf::reflect::ProtobufValue for CodecClientType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15envoy/type/http.proto\x12\nenvoy.type\x1a\x1dudpa/annotations/stat\
    us.proto*2\n\x0fCodecClientType\x12\t\n\x05HTTP1\x10\0\x12\t\n\x05HTTP2\
    \x10\x01\x12\t\n\x05HTTP3\x10\x02B/\n\x18io.envoyproxy.envoy.typeB\tHttp\
    ProtoP\x01\xba\x80\xc8\xd1\x06\x02\x10\x01b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
