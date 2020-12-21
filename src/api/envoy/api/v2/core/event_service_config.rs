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
//! Generated file from `envoy/api/v2/core/event_service_config.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct EventServiceConfig {
    // message oneof groups
    pub config_source_specifier: ::std::option::Option<EventServiceConfig_oneof_config_source_specifier>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EventServiceConfig {
    fn default() -> &'a EventServiceConfig {
        <EventServiceConfig as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum EventServiceConfig_oneof_config_source_specifier {
    grpc_service(super::grpc_service::GrpcService),
}

impl EventServiceConfig {
    pub fn new() -> EventServiceConfig {
        ::std::default::Default::default()
    }

    // .envoy.api.v2.core.GrpcService grpc_service = 1;


    pub fn get_grpc_service(&self) -> &super::grpc_service::GrpcService {
        match self.config_source_specifier {
            ::std::option::Option::Some(EventServiceConfig_oneof_config_source_specifier::grpc_service(ref v)) => v,
            _ => <super::grpc_service::GrpcService as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_grpc_service(&mut self) {
        self.config_source_specifier = ::std::option::Option::None;
    }

    pub fn has_grpc_service(&self) -> bool {
        match self.config_source_specifier {
            ::std::option::Option::Some(EventServiceConfig_oneof_config_source_specifier::grpc_service(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_grpc_service(&mut self, v: super::grpc_service::GrpcService) {
        self.config_source_specifier = ::std::option::Option::Some(EventServiceConfig_oneof_config_source_specifier::grpc_service(v))
    }

    // Mutable pointer to the field.
    pub fn mut_grpc_service(&mut self) -> &mut super::grpc_service::GrpcService {
        if let ::std::option::Option::Some(EventServiceConfig_oneof_config_source_specifier::grpc_service(_)) = self.config_source_specifier {
        } else {
            self.config_source_specifier = ::std::option::Option::Some(EventServiceConfig_oneof_config_source_specifier::grpc_service(super::grpc_service::GrpcService::new()));
        }
        match self.config_source_specifier {
            ::std::option::Option::Some(EventServiceConfig_oneof_config_source_specifier::grpc_service(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_grpc_service(&mut self) -> super::grpc_service::GrpcService {
        if self.has_grpc_service() {
            match self.config_source_specifier.take() {
                ::std::option::Option::Some(EventServiceConfig_oneof_config_source_specifier::grpc_service(v)) => v,
                _ => panic!(),
            }
        } else {
            super::grpc_service::GrpcService::new()
        }
    }
}

impl ::protobuf::Message for EventServiceConfig {
    fn is_initialized(&self) -> bool {
        if let Some(EventServiceConfig_oneof_config_source_specifier::grpc_service(ref v)) = self.config_source_specifier {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.config_source_specifier = ::std::option::Option::Some(EventServiceConfig_oneof_config_source_specifier::grpc_service(is.read_message()?));
                },
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
        if let ::std::option::Option::Some(ref v) = self.config_source_specifier {
            match v {
                &EventServiceConfig_oneof_config_source_specifier::grpc_service(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.config_source_specifier {
            match v {
                &EventServiceConfig_oneof_config_source_specifier::grpc_service(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
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

    fn new() -> EventServiceConfig {
        EventServiceConfig::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::grpc_service::GrpcService>(
                "grpc_service",
                EventServiceConfig::has_grpc_service,
                EventServiceConfig::get_grpc_service,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<EventServiceConfig>(
                "EventServiceConfig",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static EventServiceConfig {
        static instance: ::protobuf::rt::LazyV2<EventServiceConfig> = ::protobuf::rt::LazyV2::INIT;
        instance.get(EventServiceConfig::new)
    }
}

impl ::protobuf::Clear for EventServiceConfig {
    fn clear(&mut self) {
        self.config_source_specifier = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EventServiceConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EventServiceConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n,envoy/api/v2/core/event_service_config.proto\x12\x11envoy.api.v2.core\
    \x1a$envoy/api/v2/core/grpc_service.proto\x1a\x1eudpa/annotations/migrat\
    e.proto\x1a\x1dudpa/annotations/status.proto\x1a\x17validate/validate.pr\
    oto\"y\n\x12EventServiceConfig\x12C\n\x0cgrpc_service\x18\x01\x20\x01(\
    \x0b2\x1e.envoy.api.v2.core.GrpcServiceH\0R\x0bgrpcServiceB\x1e\n\x17con\
    fig_source_specifier\x12\x03\xf8B\x01B`\n\x1fio.envoyproxy.envoy.api.v2.\
    coreB\x17EventServiceConfigProtoP\x01\xf2\x98\xfe\x8f\x05\x16\x12\x14env\
    oy.config.core.v3\xba\x80\xc8\xd1\x06\x02\x10\x01b\x06proto3\
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
