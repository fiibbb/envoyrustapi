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
//! Generated file from `envoy/extensions/filters/network/thrift_proxy/filters/ratelimit/v4alpha/rate_limit.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct RateLimit {
    // message fields
    pub domain: ::std::string::String,
    pub stage: u32,
    pub timeout: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub failure_mode_deny: bool,
    pub rate_limit_service: ::protobuf::SingularPtrField<super::rls::RateLimitServiceConfig>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RateLimit {
    fn default() -> &'a RateLimit {
        <RateLimit as ::protobuf::Message>::default_instance()
    }
}

impl RateLimit {
    pub fn new() -> RateLimit {
        ::std::default::Default::default()
    }

    // string domain = 1;


    pub fn get_domain(&self) -> &str {
        &self.domain
    }
    pub fn clear_domain(&mut self) {
        self.domain.clear();
    }

    // Param is passed by value, moved
    pub fn set_domain(&mut self, v: ::std::string::String) {
        self.domain = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_domain(&mut self) -> &mut ::std::string::String {
        &mut self.domain
    }

    // Take field
    pub fn take_domain(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.domain, ::std::string::String::new())
    }

    // uint32 stage = 2;


    pub fn get_stage(&self) -> u32 {
        self.stage
    }
    pub fn clear_stage(&mut self) {
        self.stage = 0;
    }

    // Param is passed by value, moved
    pub fn set_stage(&mut self, v: u32) {
        self.stage = v;
    }

    // .google.protobuf.Duration timeout = 3;


    pub fn get_timeout(&self) -> &::protobuf::well_known_types::Duration {
        self.timeout.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Duration as ::protobuf::Message>::default_instance())
    }
    pub fn clear_timeout(&mut self) {
        self.timeout.clear();
    }

    pub fn has_timeout(&self) -> bool {
        self.timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timeout(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.timeout = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_timeout(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.timeout.is_none() {
            self.timeout.set_default();
        }
        self.timeout.as_mut().unwrap()
    }

    // Take field
    pub fn take_timeout(&mut self) -> ::protobuf::well_known_types::Duration {
        self.timeout.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    // bool failure_mode_deny = 4;


    pub fn get_failure_mode_deny(&self) -> bool {
        self.failure_mode_deny
    }
    pub fn clear_failure_mode_deny(&mut self) {
        self.failure_mode_deny = false;
    }

    // Param is passed by value, moved
    pub fn set_failure_mode_deny(&mut self, v: bool) {
        self.failure_mode_deny = v;
    }

    // .envoy.config.ratelimit.v4alpha.RateLimitServiceConfig rate_limit_service = 5;


    pub fn get_rate_limit_service(&self) -> &super::rls::RateLimitServiceConfig {
        self.rate_limit_service.as_ref().unwrap_or_else(|| <super::rls::RateLimitServiceConfig as ::protobuf::Message>::default_instance())
    }
    pub fn clear_rate_limit_service(&mut self) {
        self.rate_limit_service.clear();
    }

    pub fn has_rate_limit_service(&self) -> bool {
        self.rate_limit_service.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rate_limit_service(&mut self, v: super::rls::RateLimitServiceConfig) {
        self.rate_limit_service = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rate_limit_service(&mut self) -> &mut super::rls::RateLimitServiceConfig {
        if self.rate_limit_service.is_none() {
            self.rate_limit_service.set_default();
        }
        self.rate_limit_service.as_mut().unwrap()
    }

    // Take field
    pub fn take_rate_limit_service(&mut self) -> super::rls::RateLimitServiceConfig {
        self.rate_limit_service.take().unwrap_or_else(|| super::rls::RateLimitServiceConfig::new())
    }
}

impl ::protobuf::Message for RateLimit {
    fn is_initialized(&self) -> bool {
        for v in &self.timeout {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.rate_limit_service {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.domain)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.stage = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.timeout)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.failure_mode_deny = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rate_limit_service)?;
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
        if !self.domain.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.domain);
        }
        if self.stage != 0 {
            my_size += ::protobuf::rt::value_size(2, self.stage, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.timeout.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.failure_mode_deny != false {
            my_size += 2;
        }
        if let Some(ref v) = self.rate_limit_service.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.domain.is_empty() {
            os.write_string(1, &self.domain)?;
        }
        if self.stage != 0 {
            os.write_uint32(2, self.stage)?;
        }
        if let Some(ref v) = self.timeout.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.failure_mode_deny != false {
            os.write_bool(4, self.failure_mode_deny)?;
        }
        if let Some(ref v) = self.rate_limit_service.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> RateLimit {
        RateLimit::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "domain",
                |m: &RateLimit| { &m.domain },
                |m: &mut RateLimit| { &mut m.domain },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "stage",
                |m: &RateLimit| { &m.stage },
                |m: &mut RateLimit| { &mut m.stage },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                "timeout",
                |m: &RateLimit| { &m.timeout },
                |m: &mut RateLimit| { &mut m.timeout },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "failure_mode_deny",
                |m: &RateLimit| { &m.failure_mode_deny },
                |m: &mut RateLimit| { &mut m.failure_mode_deny },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rls::RateLimitServiceConfig>>(
                "rate_limit_service",
                |m: &RateLimit| { &m.rate_limit_service },
                |m: &mut RateLimit| { &mut m.rate_limit_service },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RateLimit>(
                "RateLimit",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RateLimit {
        static instance: ::protobuf::rt::LazyV2<RateLimit> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RateLimit::new)
    }
}

impl ::protobuf::Clear for RateLimit {
    fn clear(&mut self) {
        self.domain.clear();
        self.stage = 0;
        self.timeout.clear();
        self.failure_mode_deny = false;
        self.rate_limit_service.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RateLimit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RateLimit {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \nXenvoy/extensions/filters/network/thrift_proxy/filters/ratelimit/v4alp\
    ha/rate_limit.proto\x12Genvoy.extensions.filters.network.thrift_proxy.fi\
    lters.ratelimit.v4alpha\x1a(envoy/config/ratelimit/v4alpha/rls.proto\x1a\
    \x1egoogle/protobuf/duration.proto\x1a\x1dudpa/annotations/status.proto\
    \x1a!udpa/annotations/versioning.proto\x1a\x17validate/validate.proto\"\
    \xf1\x02\n\tRateLimit\x12\x1f\n\x06domain\x18\x01\x20\x01(\tR\x06domainB\
    \x07\xfaB\x04r\x02\x10\x01\x12\x1d\n\x05stage\x18\x02\x20\x01(\rR\x05sta\
    geB\x07\xfaB\x04*\x02\x18\n\x123\n\x07timeout\x18\x03\x20\x01(\x0b2\x19.\
    google.protobuf.DurationR\x07timeout\x12*\n\x11failure_mode_deny\x18\x04\
    \x20\x01(\x08R\x0ffailureModeDeny\x12n\n\x12rate_limit_service\x18\x05\
    \x20\x01(\x0b26.envoy.config.ratelimit.v4alpha.RateLimitServiceConfigR\
    \x10rateLimitServiceB\x08\xfaB\x05\x8a\x01\x02\x10\x01:S\x9a\xc5\x88\x1e\
    N\nLenvoy.extensions.filters.network.thrift_proxy.filters.ratelimit.v3.R\
    ateLimitBq\nUio.envoyproxy.envoy.extensions.filters.network.thrift_proxy\
    .filters.ratelimit.v4alphaB\x0eRateLimitProtoP\x01\xba\x80\xc8\xd1\x06\
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
