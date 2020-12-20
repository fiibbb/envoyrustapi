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
//! Generated file from `envoy/config/trace/v3/trace.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!envoy/config/trace/v3/trace.proto\x12\x15envoy.config.trace.v3\x1a\
    \x1dudpa/annotations/status.proto\x1a#envoy/config/trace/v3/datadog.prot\
    o\x1a&envoy/config/trace/v3/dynamic_ot.proto\x1a'envoy/config/trace/v3/h\
    ttp_tracer.proto\x1a%envoy/config/trace/v3/lightstep.proto\x1a&envoy/con\
    fig/trace/v3/opencensus.proto\x1a#envoy/config/trace/v3/service.proto\
    \x1a\"envoy/config/trace/v3/zipkin.protoP\x01P\x02P\x03P\x04P\x05P\x06P\
    \x07B3\n#io.envoyproxy.envoy.config.trace.v3B\nTraceProtoP\x01b\x06proto\
    3\
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
