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
//! Generated file from `envoy/extensions/access_loggers/grpc/v4alpha/als.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct HttpGrpcAccessLogConfig {
    // message fields
    pub common_config: ::protobuf::SingularPtrField<CommonGrpcAccessLogConfig>,
    pub additional_request_headers_to_log: ::protobuf::RepeatedField<::std::string::String>,
    pub additional_response_headers_to_log: ::protobuf::RepeatedField<::std::string::String>,
    pub additional_response_trailers_to_log: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a HttpGrpcAccessLogConfig {
    fn default() -> &'a HttpGrpcAccessLogConfig {
        <HttpGrpcAccessLogConfig as ::protobuf::Message>::default_instance()
    }
}

impl HttpGrpcAccessLogConfig {
    pub fn new() -> HttpGrpcAccessLogConfig {
        ::std::default::Default::default()
    }

    // .envoy.extensions.access_loggers.grpc.v4alpha.CommonGrpcAccessLogConfig common_config = 1;


    pub fn get_common_config(&self) -> &CommonGrpcAccessLogConfig {
        self.common_config.as_ref().unwrap_or_else(|| <CommonGrpcAccessLogConfig as ::protobuf::Message>::default_instance())
    }
    pub fn clear_common_config(&mut self) {
        self.common_config.clear();
    }

    pub fn has_common_config(&self) -> bool {
        self.common_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_common_config(&mut self, v: CommonGrpcAccessLogConfig) {
        self.common_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_common_config(&mut self) -> &mut CommonGrpcAccessLogConfig {
        if self.common_config.is_none() {
            self.common_config.set_default();
        }
        self.common_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_common_config(&mut self) -> CommonGrpcAccessLogConfig {
        self.common_config.take().unwrap_or_else(|| CommonGrpcAccessLogConfig::new())
    }

    // repeated string additional_request_headers_to_log = 2;


    pub fn get_additional_request_headers_to_log(&self) -> &[::std::string::String] {
        &self.additional_request_headers_to_log
    }
    pub fn clear_additional_request_headers_to_log(&mut self) {
        self.additional_request_headers_to_log.clear();
    }

    // Param is passed by value, moved
    pub fn set_additional_request_headers_to_log(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.additional_request_headers_to_log = v;
    }

    // Mutable pointer to the field.
    pub fn mut_additional_request_headers_to_log(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.additional_request_headers_to_log
    }

    // Take field
    pub fn take_additional_request_headers_to_log(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.additional_request_headers_to_log, ::protobuf::RepeatedField::new())
    }

    // repeated string additional_response_headers_to_log = 3;


    pub fn get_additional_response_headers_to_log(&self) -> &[::std::string::String] {
        &self.additional_response_headers_to_log
    }
    pub fn clear_additional_response_headers_to_log(&mut self) {
        self.additional_response_headers_to_log.clear();
    }

    // Param is passed by value, moved
    pub fn set_additional_response_headers_to_log(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.additional_response_headers_to_log = v;
    }

    // Mutable pointer to the field.
    pub fn mut_additional_response_headers_to_log(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.additional_response_headers_to_log
    }

    // Take field
    pub fn take_additional_response_headers_to_log(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.additional_response_headers_to_log, ::protobuf::RepeatedField::new())
    }

    // repeated string additional_response_trailers_to_log = 4;


    pub fn get_additional_response_trailers_to_log(&self) -> &[::std::string::String] {
        &self.additional_response_trailers_to_log
    }
    pub fn clear_additional_response_trailers_to_log(&mut self) {
        self.additional_response_trailers_to_log.clear();
    }

    // Param is passed by value, moved
    pub fn set_additional_response_trailers_to_log(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.additional_response_trailers_to_log = v;
    }

    // Mutable pointer to the field.
    pub fn mut_additional_response_trailers_to_log(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.additional_response_trailers_to_log
    }

    // Take field
    pub fn take_additional_response_trailers_to_log(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.additional_response_trailers_to_log, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for HttpGrpcAccessLogConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.common_config {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.common_config)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.additional_request_headers_to_log)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.additional_response_headers_to_log)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.additional_response_trailers_to_log)?;
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
        if let Some(ref v) = self.common_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.additional_request_headers_to_log {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.additional_response_headers_to_log {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.additional_response_trailers_to_log {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.common_config.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.additional_request_headers_to_log {
            os.write_string(2, &v)?;
        };
        for v in &self.additional_response_headers_to_log {
            os.write_string(3, &v)?;
        };
        for v in &self.additional_response_trailers_to_log {
            os.write_string(4, &v)?;
        };
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

    fn new() -> HttpGrpcAccessLogConfig {
        HttpGrpcAccessLogConfig::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CommonGrpcAccessLogConfig>>(
                "common_config",
                |m: &HttpGrpcAccessLogConfig| { &m.common_config },
                |m: &mut HttpGrpcAccessLogConfig| { &mut m.common_config },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "additional_request_headers_to_log",
                |m: &HttpGrpcAccessLogConfig| { &m.additional_request_headers_to_log },
                |m: &mut HttpGrpcAccessLogConfig| { &mut m.additional_request_headers_to_log },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "additional_response_headers_to_log",
                |m: &HttpGrpcAccessLogConfig| { &m.additional_response_headers_to_log },
                |m: &mut HttpGrpcAccessLogConfig| { &mut m.additional_response_headers_to_log },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "additional_response_trailers_to_log",
                |m: &HttpGrpcAccessLogConfig| { &m.additional_response_trailers_to_log },
                |m: &mut HttpGrpcAccessLogConfig| { &mut m.additional_response_trailers_to_log },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<HttpGrpcAccessLogConfig>(
                "HttpGrpcAccessLogConfig",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static HttpGrpcAccessLogConfig {
        static instance: ::protobuf::rt::LazyV2<HttpGrpcAccessLogConfig> = ::protobuf::rt::LazyV2::INIT;
        instance.get(HttpGrpcAccessLogConfig::new)
    }
}

impl ::protobuf::Clear for HttpGrpcAccessLogConfig {
    fn clear(&mut self) {
        self.common_config.clear();
        self.additional_request_headers_to_log.clear();
        self.additional_response_headers_to_log.clear();
        self.additional_response_trailers_to_log.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HttpGrpcAccessLogConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HttpGrpcAccessLogConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TcpGrpcAccessLogConfig {
    // message fields
    pub common_config: ::protobuf::SingularPtrField<CommonGrpcAccessLogConfig>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TcpGrpcAccessLogConfig {
    fn default() -> &'a TcpGrpcAccessLogConfig {
        <TcpGrpcAccessLogConfig as ::protobuf::Message>::default_instance()
    }
}

impl TcpGrpcAccessLogConfig {
    pub fn new() -> TcpGrpcAccessLogConfig {
        ::std::default::Default::default()
    }

    // .envoy.extensions.access_loggers.grpc.v4alpha.CommonGrpcAccessLogConfig common_config = 1;


    pub fn get_common_config(&self) -> &CommonGrpcAccessLogConfig {
        self.common_config.as_ref().unwrap_or_else(|| <CommonGrpcAccessLogConfig as ::protobuf::Message>::default_instance())
    }
    pub fn clear_common_config(&mut self) {
        self.common_config.clear();
    }

    pub fn has_common_config(&self) -> bool {
        self.common_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_common_config(&mut self, v: CommonGrpcAccessLogConfig) {
        self.common_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_common_config(&mut self) -> &mut CommonGrpcAccessLogConfig {
        if self.common_config.is_none() {
            self.common_config.set_default();
        }
        self.common_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_common_config(&mut self) -> CommonGrpcAccessLogConfig {
        self.common_config.take().unwrap_or_else(|| CommonGrpcAccessLogConfig::new())
    }
}

impl ::protobuf::Message for TcpGrpcAccessLogConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.common_config {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.common_config)?;
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
        if let Some(ref v) = self.common_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.common_config.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> TcpGrpcAccessLogConfig {
        TcpGrpcAccessLogConfig::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CommonGrpcAccessLogConfig>>(
                "common_config",
                |m: &TcpGrpcAccessLogConfig| { &m.common_config },
                |m: &mut TcpGrpcAccessLogConfig| { &mut m.common_config },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TcpGrpcAccessLogConfig>(
                "TcpGrpcAccessLogConfig",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TcpGrpcAccessLogConfig {
        static instance: ::protobuf::rt::LazyV2<TcpGrpcAccessLogConfig> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TcpGrpcAccessLogConfig::new)
    }
}

impl ::protobuf::Clear for TcpGrpcAccessLogConfig {
    fn clear(&mut self) {
        self.common_config.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TcpGrpcAccessLogConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TcpGrpcAccessLogConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CommonGrpcAccessLogConfig {
    // message fields
    pub log_name: ::std::string::String,
    pub grpc_service: ::protobuf::SingularPtrField<super::grpc_service::GrpcService>,
    pub transport_api_version: super::config_source::ApiVersion,
    pub buffer_flush_interval: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub buffer_size_bytes: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub filter_state_objects_to_log: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CommonGrpcAccessLogConfig {
    fn default() -> &'a CommonGrpcAccessLogConfig {
        <CommonGrpcAccessLogConfig as ::protobuf::Message>::default_instance()
    }
}

impl CommonGrpcAccessLogConfig {
    pub fn new() -> CommonGrpcAccessLogConfig {
        ::std::default::Default::default()
    }

    // string log_name = 1;


    pub fn get_log_name(&self) -> &str {
        &self.log_name
    }
    pub fn clear_log_name(&mut self) {
        self.log_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_log_name(&mut self, v: ::std::string::String) {
        self.log_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log_name(&mut self) -> &mut ::std::string::String {
        &mut self.log_name
    }

    // Take field
    pub fn take_log_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.log_name, ::std::string::String::new())
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

    // .envoy.config.core.v4alpha.ApiVersion transport_api_version = 6;


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

    // .google.protobuf.Duration buffer_flush_interval = 3;


    pub fn get_buffer_flush_interval(&self) -> &::protobuf::well_known_types::Duration {
        self.buffer_flush_interval.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Duration as ::protobuf::Message>::default_instance())
    }
    pub fn clear_buffer_flush_interval(&mut self) {
        self.buffer_flush_interval.clear();
    }

    pub fn has_buffer_flush_interval(&self) -> bool {
        self.buffer_flush_interval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buffer_flush_interval(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.buffer_flush_interval = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_buffer_flush_interval(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.buffer_flush_interval.is_none() {
            self.buffer_flush_interval.set_default();
        }
        self.buffer_flush_interval.as_mut().unwrap()
    }

    // Take field
    pub fn take_buffer_flush_interval(&mut self) -> ::protobuf::well_known_types::Duration {
        self.buffer_flush_interval.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    // .google.protobuf.UInt32Value buffer_size_bytes = 4;


    pub fn get_buffer_size_bytes(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.buffer_size_bytes.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::UInt32Value as ::protobuf::Message>::default_instance())
    }
    pub fn clear_buffer_size_bytes(&mut self) {
        self.buffer_size_bytes.clear();
    }

    pub fn has_buffer_size_bytes(&self) -> bool {
        self.buffer_size_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buffer_size_bytes(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.buffer_size_bytes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_buffer_size_bytes(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.buffer_size_bytes.is_none() {
            self.buffer_size_bytes.set_default();
        }
        self.buffer_size_bytes.as_mut().unwrap()
    }

    // Take field
    pub fn take_buffer_size_bytes(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.buffer_size_bytes.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    // repeated string filter_state_objects_to_log = 5;


    pub fn get_filter_state_objects_to_log(&self) -> &[::std::string::String] {
        &self.filter_state_objects_to_log
    }
    pub fn clear_filter_state_objects_to_log(&mut self) {
        self.filter_state_objects_to_log.clear();
    }

    // Param is passed by value, moved
    pub fn set_filter_state_objects_to_log(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.filter_state_objects_to_log = v;
    }

    // Mutable pointer to the field.
    pub fn mut_filter_state_objects_to_log(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.filter_state_objects_to_log
    }

    // Take field
    pub fn take_filter_state_objects_to_log(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.filter_state_objects_to_log, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for CommonGrpcAccessLogConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.grpc_service {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.buffer_flush_interval {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.buffer_size_bytes {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.log_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.grpc_service)?;
                },
                6 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.transport_api_version, 6, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.buffer_flush_interval)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.buffer_size_bytes)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.filter_state_objects_to_log)?;
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
        if !self.log_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.log_name);
        }
        if let Some(ref v) = self.grpc_service.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.transport_api_version != super::config_source::ApiVersion::DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE {
            my_size += ::protobuf::rt::enum_size(6, self.transport_api_version);
        }
        if let Some(ref v) = self.buffer_flush_interval.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.buffer_size_bytes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.filter_state_objects_to_log {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.log_name.is_empty() {
            os.write_string(1, &self.log_name)?;
        }
        if let Some(ref v) = self.grpc_service.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.transport_api_version != super::config_source::ApiVersion::DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE {
            os.write_enum(6, ::protobuf::ProtobufEnum::value(&self.transport_api_version))?;
        }
        if let Some(ref v) = self.buffer_flush_interval.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.buffer_size_bytes.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.filter_state_objects_to_log {
            os.write_string(5, &v)?;
        };
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

    fn new() -> CommonGrpcAccessLogConfig {
        CommonGrpcAccessLogConfig::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "log_name",
                |m: &CommonGrpcAccessLogConfig| { &m.log_name },
                |m: &mut CommonGrpcAccessLogConfig| { &mut m.log_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::grpc_service::GrpcService>>(
                "grpc_service",
                |m: &CommonGrpcAccessLogConfig| { &m.grpc_service },
                |m: &mut CommonGrpcAccessLogConfig| { &mut m.grpc_service },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::config_source::ApiVersion>>(
                "transport_api_version",
                |m: &CommonGrpcAccessLogConfig| { &m.transport_api_version },
                |m: &mut CommonGrpcAccessLogConfig| { &mut m.transport_api_version },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                "buffer_flush_interval",
                |m: &CommonGrpcAccessLogConfig| { &m.buffer_flush_interval },
                |m: &mut CommonGrpcAccessLogConfig| { &mut m.buffer_flush_interval },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                "buffer_size_bytes",
                |m: &CommonGrpcAccessLogConfig| { &m.buffer_size_bytes },
                |m: &mut CommonGrpcAccessLogConfig| { &mut m.buffer_size_bytes },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "filter_state_objects_to_log",
                |m: &CommonGrpcAccessLogConfig| { &m.filter_state_objects_to_log },
                |m: &mut CommonGrpcAccessLogConfig| { &mut m.filter_state_objects_to_log },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CommonGrpcAccessLogConfig>(
                "CommonGrpcAccessLogConfig",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CommonGrpcAccessLogConfig {
        static instance: ::protobuf::rt::LazyV2<CommonGrpcAccessLogConfig> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CommonGrpcAccessLogConfig::new)
    }
}

impl ::protobuf::Clear for CommonGrpcAccessLogConfig {
    fn clear(&mut self) {
        self.log_name.clear();
        self.grpc_service.clear();
        self.transport_api_version = super::config_source::ApiVersion::DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE;
        self.buffer_flush_interval.clear();
        self.buffer_size_bytes.clear();
        self.filter_state_objects_to_log.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CommonGrpcAccessLogConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommonGrpcAccessLogConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n6envoy/extensions/access_loggers/grpc/v4alpha/als.proto\x12,envoy.exte\
    nsions.access_loggers.grpc.v4alpha\x1a-envoy/config/core/v4alpha/config_\
    source.proto\x1a,envoy/config/core/v4alpha/grpc_service.proto\x1a\x1egoo\
    gle/protobuf/duration.proto\x1a\x1egoogle/protobuf/wrappers.proto\x1a\
    \x1dudpa/annotations/status.proto\x1a!udpa/annotations/versioning.proto\
    \x1a\x17validate/validate.proto\"\xbd\x03\n\x17HttpGrpcAccessLogConfig\
    \x12v\n\rcommon_config\x18\x01\x20\x01(\x0b2G.envoy.extensions.access_lo\
    ggers.grpc.v4alpha.CommonGrpcAccessLogConfigR\x0ccommonConfigB\x08\xfaB\
    \x05\x8a\x01\x02\x10\x01\x12H\n!additional_request_headers_to_log\x18\
    \x02\x20\x03(\tR\x1dadditionalRequestHeadersToLog\x12J\n\"additional_res\
    ponse_headers_to_log\x18\x03\x20\x03(\tR\x1eadditionalResponseHeadersToL\
    og\x12L\n#additional_response_trailers_to_log\x18\x04\x20\x03(\tR\x1fadd\
    itionalResponseTrailersToLog:F\x9a\xc5\x88\x1eA\n?envoy.extensions.acces\
    s_loggers.grpc.v3.HttpGrpcAccessLogConfig\"\xd7\x01\n\x16TcpGrpcAccessLo\
    gConfig\x12v\n\rcommon_config\x18\x01\x20\x01(\x0b2G.envoy.extensions.ac\
    cess_loggers.grpc.v4alpha.CommonGrpcAccessLogConfigR\x0ccommonConfigB\
    \x08\xfaB\x05\x8a\x01\x02\x10\x01:E\x9a\xc5\x88\x1e@\n>envoy.extensions.\
    access_loggers.grpc.v3.TcpGrpcAccessLogConfig\"\xa4\x04\n\x19CommonGrpcA\
    ccessLogConfig\x12\"\n\x08log_name\x18\x01\x20\x01(\tR\x07logNameB\x07\
    \xfaB\x04r\x02\x10\x01\x12S\n\x0cgrpc_service\x18\x02\x20\x01(\x0b2&.env\
    oy.config.core.v4alpha.GrpcServiceR\x0bgrpcServiceB\x08\xfaB\x05\x8a\x01\
    \x02\x10\x01\x12c\n\x15transport_api_version\x18\x06\x20\x01(\x0e2%.envo\
    y.config.core.v4alpha.ApiVersionR\x13transportApiVersionB\x08\xfaB\x05\
    \x82\x01\x02\x10\x01\x12W\n\x15buffer_flush_interval\x18\x03\x20\x01(\
    \x0b2\x19.google.protobuf.DurationR\x13bufferFlushIntervalB\x08\xfaB\x05\
    \xaa\x01\x02*\0\x12H\n\x11buffer_size_bytes\x18\x04\x20\x01(\x0b2\x1c.go\
    ogle.protobuf.UInt32ValueR\x0fbufferSizeBytes\x12<\n\x1bfilter_state_obj\
    ects_to_log\x18\x05\x20\x03(\tR\x17filterStateObjectsToLog:H\x9a\xc5\x88\
    \x1eC\nAenvoy.extensions.access_loggers.grpc.v3.CommonGrpcAccessLogConfi\
    gBP\n:io.envoyproxy.envoy.extensions.access_loggers.grpc.v4alphaB\x08Als\
    ProtoP\x01\xba\x80\xc8\xd1\x06\x02\x10\x03b\x06proto3\
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
