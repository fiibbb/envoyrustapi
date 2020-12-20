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
//! Generated file from `envoy/api/v2/auth/cert.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cenvoy/api/v2/auth/cert.proto\x12\x11envoy.api.v2.auth\x1a\x1eudpa/\
    annotations/migrate.proto\x1a\x1dudpa/annotations/status.proto\x1a\x1een\
    voy/api/v2/auth/common.proto\x1a\x1eenvoy/api/v2/auth/secret.proto\x1a\
    \x1benvoy/api/v2/auth/tls.protoP\x02P\x03P\x04B_\n\x1fio.envoyproxy.envo\
    y.api.v2.authB\tCertProtoP\x01\xf2\x98\xfe\x8f\x05+\x12)envoy.extensions\
    .transport_sockets.tls.v3b\x06proto3\
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
