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
//! Generated file from `envoy/extensions/filters/network/ext_authz/v4alpha/ext_authz.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct ExtAuthz {
    // message fields
    pub stat_prefix: ::std::string::String,
    pub grpc_service: ::protobuf::SingularPtrField<super::grpc_service::GrpcService>,
    pub failure_mode_allow: bool,
    pub include_peer_certificate: bool,
    pub transport_api_version: super::config_source::ApiVersion,
    pub filter_enabled_metadata: ::protobuf::SingularPtrField<super::metadata::MetadataMatcher>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ExtAuthz {
    fn default() -> &'a ExtAuthz {
        <ExtAuthz as ::protobuf::Message>::default_instance()
    }
}

impl ExtAuthz {
    pub fn new() -> ExtAuthz {
        ::std::default::Default::default()
    }

    // string stat_prefix = 1;


    pub fn get_stat_prefix(&self) -> &str {
        &self.stat_prefix
    }
    pub fn clear_stat_prefix(&mut self) {
        self.stat_prefix.clear();
    }

    // Param is passed by value, moved
    pub fn set_stat_prefix(&mut self, v: ::std::string::String) {
        self.stat_prefix = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stat_prefix(&mut self) -> &mut ::std::string::String {
        &mut self.stat_prefix
    }

    // Take field
    pub fn take_stat_prefix(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.stat_prefix, ::std::string::String::new())
    }

    // .envoy.config.core.v4alpha.GrpcService grpc_service = 2;


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

    // bool failure_mode_allow = 3;


    pub fn get_failure_mode_allow(&self) -> bool {
        self.failure_mode_allow
    }
    pub fn clear_failure_mode_allow(&mut self) {
        self.failure_mode_allow = false;
    }

    // Param is passed by value, moved
    pub fn set_failure_mode_allow(&mut self, v: bool) {
        self.failure_mode_allow = v;
    }

    // bool include_peer_certificate = 4;


    pub fn get_include_peer_certificate(&self) -> bool {
        self.include_peer_certificate
    }
    pub fn clear_include_peer_certificate(&mut self) {
        self.include_peer_certificate = false;
    }

    // Param is passed by value, moved
    pub fn set_include_peer_certificate(&mut self, v: bool) {
        self.include_peer_certificate = v;
    }

    // .envoy.config.core.v4alpha.ApiVersion transport_api_version = 5;


    pub fn get_transport_api_version(&self) -> super::config_source::ApiVersion {
        self.transport_api_version
    }
    pub fn clear_transport_api_version(&mut self) {
        self.transport_api_version = super::config_source::ApiVersion::DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE;
    }

    // Param is passed by value, moved
    pub fn set_transport_api_version(&mut self, v: super::config_source::ApiVersion) {
        self.transport_api_version = v;
    }

    // .envoy.type.matcher.v4alpha.MetadataMatcher filter_enabled_metadata = 6;


    pub fn get_filter_enabled_metadata(&self) -> &super::metadata::MetadataMatcher {
        self.filter_enabled_metadata.as_ref().unwrap_or_else(|| <super::metadata::MetadataMatcher as ::protobuf::Message>::default_instance())
    }
    pub fn clear_filter_enabled_metadata(&mut self) {
        self.filter_enabled_metadata.clear();
    }

    pub fn has_filter_enabled_metadata(&self) -> bool {
        self.filter_enabled_metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filter_enabled_metadata(&mut self, v: super::metadata::MetadataMatcher) {
        self.filter_enabled_metadata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filter_enabled_metadata(&mut self) -> &mut super::metadata::MetadataMatcher {
        if self.filter_enabled_metadata.is_none() {
            self.filter_enabled_metadata.set_default();
        }
        self.filter_enabled_metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_filter_enabled_metadata(&mut self) -> super::metadata::MetadataMatcher {
        self.filter_enabled_metadata.take().unwrap_or_else(|| super::metadata::MetadataMatcher::new())
    }
}

impl ::protobuf::Message for ExtAuthz {
    fn is_initialized(&self) -> bool {
        for v in &self.grpc_service {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.filter_enabled_metadata {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.stat_prefix)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.grpc_service)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.failure_mode_allow = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.include_peer_certificate = tmp;
                },
                5 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.transport_api_version, 5, &mut self.unknown_fields)?
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.filter_enabled_metadata)?;
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
        if !self.stat_prefix.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.stat_prefix);
        }
        if let Some(ref v) = self.grpc_service.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.failure_mode_allow != false {
            my_size += 2;
        }
        if self.include_peer_certificate != false {
            my_size += 2;
        }
        if self.transport_api_version != super::config_source::ApiVersion::DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE {
            my_size += ::protobuf::rt::enum_size(5, self.transport_api_version);
        }
        if let Some(ref v) = self.filter_enabled_metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.stat_prefix.is_empty() {
            os.write_string(1, &self.stat_prefix)?;
        }
        if let Some(ref v) = self.grpc_service.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.failure_mode_allow != false {
            os.write_bool(3, self.failure_mode_allow)?;
        }
        if self.include_peer_certificate != false {
            os.write_bool(4, self.include_peer_certificate)?;
        }
        if self.transport_api_version != super::config_source::ApiVersion::DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE {
            os.write_enum(5, ::protobuf::ProtobufEnum::value(&self.transport_api_version))?;
        }
        if let Some(ref v) = self.filter_enabled_metadata.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> ExtAuthz {
        ExtAuthz::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "stat_prefix",
                |m: &ExtAuthz| { &m.stat_prefix },
                |m: &mut ExtAuthz| { &mut m.stat_prefix },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::grpc_service::GrpcService>>(
                "grpc_service",
                |m: &ExtAuthz| { &m.grpc_service },
                |m: &mut ExtAuthz| { &mut m.grpc_service },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "failure_mode_allow",
                |m: &ExtAuthz| { &m.failure_mode_allow },
                |m: &mut ExtAuthz| { &mut m.failure_mode_allow },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "include_peer_certificate",
                |m: &ExtAuthz| { &m.include_peer_certificate },
                |m: &mut ExtAuthz| { &mut m.include_peer_certificate },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::config_source::ApiVersion>>(
                "transport_api_version",
                |m: &ExtAuthz| { &m.transport_api_version },
                |m: &mut ExtAuthz| { &mut m.transport_api_version },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metadata::MetadataMatcher>>(
                "filter_enabled_metadata",
                |m: &ExtAuthz| { &m.filter_enabled_metadata },
                |m: &mut ExtAuthz| { &mut m.filter_enabled_metadata },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ExtAuthz>(
                "ExtAuthz",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ExtAuthz {
        static instance: ::protobuf::rt::LazyV2<ExtAuthz> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ExtAuthz::new)
    }
}

impl ::protobuf::Clear for ExtAuthz {
    fn clear(&mut self) {
        self.stat_prefix.clear();
        self.grpc_service.clear();
        self.failure_mode_allow = false;
        self.include_peer_certificate = false;
        self.transport_api_version = super::config_source::ApiVersion::DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE;
        self.filter_enabled_metadata.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExtAuthz {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExtAuthz {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \nBenvoy/extensions/filters/network/ext_authz/v4alpha/ext_authz.proto\
    \x122envoy.extensions.filters.network.ext_authz.v4alpha\x1a-envoy/config\
    /core/v4alpha/config_source.proto\x1a,envoy/config/core/v4alpha/grpc_ser\
    vice.proto\x1a)envoy/type/matcher/v4alpha/metadata.proto\x1a\x1dudpa/ann\
    otations/status.proto\x1a!udpa/annotations/versioning.proto\x1a\x17valid\
    ate/validate.proto\"\xf0\x03\n\x08ExtAuthz\x12(\n\x0bstat_prefix\x18\x01\
    \x20\x01(\tR\nstatPrefixB\x07\xfaB\x04r\x02\x10\x01\x12I\n\x0cgrpc_servi\
    ce\x18\x02\x20\x01(\x0b2&.envoy.config.core.v4alpha.GrpcServiceR\x0bgrpc\
    Service\x12,\n\x12failure_mode_allow\x18\x03\x20\x01(\x08R\x10failureMod\
    eAllow\x128\n\x18include_peer_certificate\x18\x04\x20\x01(\x08R\x16inclu\
    dePeerCertificate\x12c\n\x15transport_api_version\x18\x05\x20\x01(\x0e2%\
    .envoy.config.core.v4alpha.ApiVersionR\x13transportApiVersionB\x08\xfaB\
    \x05\x82\x01\x02\x10\x01\x12c\n\x17filter_enabled_metadata\x18\x06\x20\
    \x01(\x0b2+.envoy.type.matcher.v4alpha.MetadataMatcherR\x15filterEnabled\
    Metadata:=\x9a\xc5\x88\x1e8\n6envoy.extensions.filters.network.ext_authz\
    .v3.ExtAuthzB[\n@io.envoyproxy.envoy.extensions.filters.network.ext_auth\
    z.v4alphaB\rExtAuthzProtoP\x01\xba\x80\xc8\xd1\x06\x02\x10\x03b\x06proto\
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
