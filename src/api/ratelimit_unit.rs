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
//! Generated file from `envoy/type/v3/ratelimit_unit.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RateLimitUnit {
    UNKNOWN = 0,
    SECOND = 1,
    MINUTE = 2,
    HOUR = 3,
    DAY = 4,
}

impl ::protobuf::ProtobufEnum for RateLimitUnit {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RateLimitUnit> {
        match value {
            0 => ::std::option::Option::Some(RateLimitUnit::UNKNOWN),
            1 => ::std::option::Option::Some(RateLimitUnit::SECOND),
            2 => ::std::option::Option::Some(RateLimitUnit::MINUTE),
            3 => ::std::option::Option::Some(RateLimitUnit::HOUR),
            4 => ::std::option::Option::Some(RateLimitUnit::DAY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RateLimitUnit] = &[
            RateLimitUnit::UNKNOWN,
            RateLimitUnit::SECOND,
            RateLimitUnit::MINUTE,
            RateLimitUnit::HOUR,
            RateLimitUnit::DAY,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<RateLimitUnit>("RateLimitUnit", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for RateLimitUnit {
}

impl ::std::default::Default for RateLimitUnit {
    fn default() -> Self {
        RateLimitUnit::UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimitUnit {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"envoy/type/v3/ratelimit_unit.proto\x12\renvoy.type.v3\x1a\x1dudpa/an\
    notations/status.proto*G\n\rRateLimitUnit\x12\x0b\n\x07UNKNOWN\x10\0\x12\
    \n\n\x06SECOND\x10\x01\x12\n\n\x06MINUTE\x10\x02\x12\x08\n\x04HOUR\x10\
    \x03\x12\x07\n\x03DAY\x10\x04B;\n\x1bio.envoyproxy.envoy.type.v3B\x12Rat\
    elimitUnitProtoP\x01\xba\x80\xc8\xd1\x06\x02\x10\x02b\x06proto3\
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
