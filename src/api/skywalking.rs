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
//! Generated file from `envoy/extensions/tracers/skywalking/v4alpha/skywalking.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct SkyWalkingConfig {
    // message fields
    pub grpc_service: ::protobuf::SingularPtrField<super::grpc_service::GrpcService>,
    pub client_config: ::protobuf::SingularPtrField<ClientConfig>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SkyWalkingConfig {
    fn default() -> &'a SkyWalkingConfig {
        <SkyWalkingConfig as ::protobuf::Message>::default_instance()
    }
}

impl SkyWalkingConfig {
    pub fn new() -> SkyWalkingConfig {
        ::std::default::Default::default()
    }

    // .envoy.config.core.v4alpha.GrpcService grpc_service = 1;


    pub fn get_grpc_service(&self) -> &super::grpc_service::GrpcService {
        self.grpc_service.as_ref().unwrap_or_else(|| <super::grpc_service::GrpcService as ::protobuf::Message>::default_instance())
    }
    pub fn clear_grpc_service(&mut self) {
        self.grpc_service.clear();
    }

    pub fn has_grpc_service(&self) -> bool {
        self.grpc_service.is_some()
    }

    // Param is passed by value, moved
    pub fn set_grpc_service(&mut self, v: super::grpc_service::GrpcService) {
        self.grpc_service = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_grpc_service(&mut self) -> &mut super::grpc_service::GrpcService {
        if self.grpc_service.is_none() {
            self.grpc_service.set_default();
        }
        self.grpc_service.as_mut().unwrap()
    }

    // Take field
    pub fn take_grpc_service(&mut self) -> super::grpc_service::GrpcService {
        self.grpc_service.take().unwrap_or_else(|| super::grpc_service::GrpcService::new())
    }

    // .envoy.extensions.tracers.skywalking.v4alpha.ClientConfig client_config = 2;


    pub fn get_client_config(&self) -> &ClientConfig {
        self.client_config.as_ref().unwrap_or_else(|| <ClientConfig as ::protobuf::Message>::default_instance())
    }
    pub fn clear_client_config(&mut self) {
        self.client_config.clear();
    }

    pub fn has_client_config(&self) -> bool {
        self.client_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_config(&mut self, v: ClientConfig) {
        self.client_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_config(&mut self) -> &mut ClientConfig {
        if self.client_config.is_none() {
            self.client_config.set_default();
        }
        self.client_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_config(&mut self) -> ClientConfig {
        self.client_config.take().unwrap_or_else(|| ClientConfig::new())
    }
}

impl ::protobuf::Message for SkyWalkingConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.grpc_service {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.client_config {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.grpc_service)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.client_config)?;
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
        if let Some(ref v) = self.grpc_service.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.client_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.grpc_service.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.client_config.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn new() -> SkyWalkingConfig {
        SkyWalkingConfig::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::grpc_service::GrpcService>>(
                "grpc_service",
                |m: &SkyWalkingConfig| { &m.grpc_service },
                |m: &mut SkyWalkingConfig| { &mut m.grpc_service },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClientConfig>>(
                "client_config",
                |m: &SkyWalkingConfig| { &m.client_config },
                |m: &mut SkyWalkingConfig| { &mut m.client_config },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SkyWalkingConfig>(
                "SkyWalkingConfig",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SkyWalkingConfig {
        static instance: ::protobuf::rt::LazyV2<SkyWalkingConfig> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SkyWalkingConfig::new)
    }
}

impl ::protobuf::Clear for SkyWalkingConfig {
    fn clear(&mut self) {
        self.grpc_service.clear();
        self.client_config.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SkyWalkingConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SkyWalkingConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClientConfig {
    // message fields
    pub service_name: ::std::string::String,
    pub instance_name: ::std::string::String,
    pub max_cache_size: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    // message oneof groups
    pub backend_token_specifier: ::std::option::Option<ClientConfig_oneof_backend_token_specifier>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ClientConfig {
    fn default() -> &'a ClientConfig {
        <ClientConfig as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum ClientConfig_oneof_backend_token_specifier {
    backend_token(::std::string::String),
}

impl ClientConfig {
    pub fn new() -> ClientConfig {
        ::std::default::Default::default()
    }

    // string service_name = 1;


    pub fn get_service_name(&self) -> &str {
        &self.service_name
    }
    pub fn clear_service_name(&mut self) {
        self.service_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_service_name(&mut self, v: ::std::string::String) {
        self.service_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service_name(&mut self) -> &mut ::std::string::String {
        &mut self.service_name
    }

    // Take field
    pub fn take_service_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.service_name, ::std::string::String::new())
    }

    // string instance_name = 2;


    pub fn get_instance_name(&self) -> &str {
        &self.instance_name
    }
    pub fn clear_instance_name(&mut self) {
        self.instance_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_instance_name(&mut self, v: ::std::string::String) {
        self.instance_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_instance_name(&mut self) -> &mut ::std::string::String {
        &mut self.instance_name
    }

    // Take field
    pub fn take_instance_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.instance_name, ::std::string::String::new())
    }

    // string backend_token = 3;


    pub fn get_backend_token(&self) -> &str {
        match self.backend_token_specifier {
            ::std::option::Option::Some(ClientConfig_oneof_backend_token_specifier::backend_token(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_backend_token(&mut self) {
        self.backend_token_specifier = ::std::option::Option::None;
    }

    pub fn has_backend_token(&self) -> bool {
        match self.backend_token_specifier {
            ::std::option::Option::Some(ClientConfig_oneof_backend_token_specifier::backend_token(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_backend_token(&mut self, v: ::std::string::String) {
        self.backend_token_specifier = ::std::option::Option::Some(ClientConfig_oneof_backend_token_specifier::backend_token(v))
    }

    // Mutable pointer to the field.
    pub fn mut_backend_token(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(ClientConfig_oneof_backend_token_specifier::backend_token(_)) = self.backend_token_specifier {
        } else {
            self.backend_token_specifier = ::std::option::Option::Some(ClientConfig_oneof_backend_token_specifier::backend_token(::std::string::String::new()));
        }
        match self.backend_token_specifier {
            ::std::option::Option::Some(ClientConfig_oneof_backend_token_specifier::backend_token(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_backend_token(&mut self) -> ::std::string::String {
        if self.has_backend_token() {
            match self.backend_token_specifier.take() {
                ::std::option::Option::Some(ClientConfig_oneof_backend_token_specifier::backend_token(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // .google.protobuf.UInt32Value max_cache_size = 4;


    pub fn get_max_cache_size(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.max_cache_size.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::UInt32Value as ::protobuf::Message>::default_instance())
    }
    pub fn clear_max_cache_size(&mut self) {
        self.max_cache_size.clear();
    }

    pub fn has_max_cache_size(&self) -> bool {
        self.max_cache_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_cache_size(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.max_cache_size = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max_cache_size(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.max_cache_size.is_none() {
            self.max_cache_size.set_default();
        }
        self.max_cache_size.as_mut().unwrap()
    }

    // Take field
    pub fn take_max_cache_size(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.max_cache_size.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }
}

impl ::protobuf::Message for ClientConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.max_cache_size {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.service_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.instance_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.backend_token_specifier = ::std::option::Option::Some(ClientConfig_oneof_backend_token_specifier::backend_token(is.read_string()?));
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.max_cache_size)?;
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
        if !self.service_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.service_name);
        }
        if !self.instance_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.instance_name);
        }
        if let Some(ref v) = self.max_cache_size.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.backend_token_specifier {
            match v {
                &ClientConfig_oneof_backend_token_specifier::backend_token(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.service_name.is_empty() {
            os.write_string(1, &self.service_name)?;
        }
        if !self.instance_name.is_empty() {
            os.write_string(2, &self.instance_name)?;
        }
        if let Some(ref v) = self.max_cache_size.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.backend_token_specifier {
            match v {
                &ClientConfig_oneof_backend_token_specifier::backend_token(ref v) => {
                    os.write_string(3, v)?;
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

    fn new() -> ClientConfig {
        ClientConfig::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "service_name",
                |m: &ClientConfig| { &m.service_name },
                |m: &mut ClientConfig| { &mut m.service_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "instance_name",
                |m: &ClientConfig| { &m.instance_name },
                |m: &mut ClientConfig| { &mut m.instance_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "backend_token",
                ClientConfig::has_backend_token,
                ClientConfig::get_backend_token,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                "max_cache_size",
                |m: &ClientConfig| { &m.max_cache_size },
                |m: &mut ClientConfig| { &mut m.max_cache_size },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ClientConfig>(
                "ClientConfig",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ClientConfig {
        static instance: ::protobuf::rt::LazyV2<ClientConfig> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ClientConfig::new)
    }
}

impl ::protobuf::Clear for ClientConfig {
    fn clear(&mut self) {
        self.service_name.clear();
        self.instance_name.clear();
        self.backend_token_specifier = ::std::option::Option::None;
        self.max_cache_size.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n<envoy/extensions/tracers/skywalking/v4alpha/skywalking.proto\x12+envo\
    y.extensions.tracers.skywalking.v4alpha\x1a,envoy/config/core/v4alpha/gr\
    pc_service.proto\x1a\x1egoogle/protobuf/wrappers.proto\x1a\x20udpa/annot\
    ations/sensitive.proto\x1a\x1dudpa/annotations/status.proto\x1a!udpa/ann\
    otations/versioning.proto\x1a\x17validate/validate.proto\"\xf6\x01\n\x10\
    SkyWalkingConfig\x12S\n\x0cgrpc_service\x18\x01\x20\x01(\x0b2&.envoy.con\
    fig.core.v4alpha.GrpcServiceR\x0bgrpcServiceB\x08\xfaB\x05\x8a\x01\x02\
    \x10\x01\x12^\n\rclient_config\x18\x02\x20\x01(\x0b29.envoy.extensions.t\
    racers.skywalking.v4alpha.ClientConfigR\x0cclientConfig:-\x9a\xc5\x88\
    \x1e(\n&envoy.config.trace.v3.SkyWalkingConfig\"\x8f\x02\n\x0cClientConf\
    ig\x12!\n\x0cservice_name\x18\x01\x20\x01(\tR\x0bserviceName\x12#\n\rins\
    tance_name\x18\x02\x20\x01(\tR\x0cinstanceName\x12-\n\rbackend_token\x18\
    \x03\x20\x01(\tH\0R\x0cbackendTokenB\x06\xb8\xb7\x8b\xa4\x02\x01\x12B\n\
    \x0emax_cache_size\x18\x04\x20\x01(\x0b2\x1c.google.protobuf.UInt32Value\
    R\x0cmaxCacheSizeB\x19\n\x17backend_token_specifier:)\x9a\xc5\x88\x1e$\n\
    \"envoy.config.trace.v3.ClientConfigBV\n9io.envoyproxy.envoy.extensions.\
    tracers.skywalking.v4alphaB\x0fSkywalkingProtoP\x01\xba\x80\xc8\xd1\x06\
    \x02\x10\x03b\x06proto3\
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
