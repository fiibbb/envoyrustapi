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
//! Generated file from `envoy/service/listener/v3/lds.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct LdsDummy {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a LdsDummy {
    fn default() -> &'a LdsDummy {
        <LdsDummy as ::protobuf::Message>::default_instance()
    }
}

impl LdsDummy {
    pub fn new() -> LdsDummy {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for LdsDummy {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> LdsDummy {
        LdsDummy::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let fields = ::std::vec::Vec::new();
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<LdsDummy>(
                "LdsDummy",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static LdsDummy {
        static instance: ::protobuf::rt::LazyV2<LdsDummy> = ::protobuf::rt::LazyV2::INIT;
        instance.get(LdsDummy::new)
    }
}

impl ::protobuf::Clear for LdsDummy {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LdsDummy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LdsDummy {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#envoy/service/listener/v3/lds.proto\x12\x19envoy.service.listener.v3\
    \x1a*envoy/service/discovery/v3/discovery.proto\x1a\x1cgoogle/api/annota\
    tions.proto\x1a\x1egoogle/protobuf/duration.proto\x1a\x1egoogle/protobuf\
    /wrappers.proto\x1a\x20envoy/annotations/resource.proto\x1a\x1dudpa/anno\
    tations/status.proto\x1a!udpa/annotations/versioning.proto\x1a\x17valida\
    te/validate.proto\"(\n\x08LdsDummy:\x1c\x9a\xc5\x88\x1e\x17\n\x15envoy.a\
    pi.v2.LdsDummy2\xd4\x03\n\x18ListenerDiscoveryService\x12}\n\x0eDeltaLis\
    teners\x121.envoy.service.discovery.v3.DeltaDiscoveryRequest\x1a2.envoy.\
    service.discovery.v3.DeltaDiscoveryResponse\"\0(\x010\x01\x12t\n\x0fStre\
    amListeners\x12,.envoy.service.discovery.v3.DiscoveryRequest\x1a-.envoy.\
    service.discovery.v3.DiscoveryResponse\"\0(\x010\x01\x12\x97\x01\n\x0eFe\
    tchListeners\x12,.envoy.service.discovery.v3.DiscoveryRequest\x1a-.envoy\
    .service.discovery.v3.DiscoveryResponse\"(\x82\xd3\xe4\x93\x02\x19\"\x17\
    /v3/discovery:listeners\x82\xd3\xe4\x93\x02\x03:\x01*\x1a)\x8a\xa4\x96\
    \xf3\x07#\n!envoy.config.listener.v3.ListenerB@\n'io.envoyproxy.envoy.se\
    rvice.listener.v3B\x08LdsProtoP\x01\x88\x01\x01\xba\x80\xc8\xd1\x06\x02\
    \x10\x02b\x06proto3\
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