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
//! Generated file from `envoy/extensions/transport_sockets/tls/v3/cert.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

static file_descriptor_proto_data: &'static [u8] = b"\
    \n4envoy/extensions/transport_sockets/tls/v3/cert.proto\x12)envoy.extens\
    ions.transport_sockets.tls.v3\x1a\x1dudpa/annotations/status.proto\x1a6e\
    nvoy/extensions/transport_sockets/tls/v3/common.proto\x1a6envoy/extensio\
    ns/transport_sockets/tls/v3/secret.proto\x1a3envoy/extensions/transport_\
    sockets/tls/v3/tls.protoP\x01P\x02P\x03BF\n7io.envoyproxy.envoy.extensio\
    ns.transport_sockets.tls.v3B\tCertProtoP\x01b\x06proto3\
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