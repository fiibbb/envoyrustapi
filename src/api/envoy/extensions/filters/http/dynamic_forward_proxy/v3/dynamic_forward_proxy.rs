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
//! Generated file from `envoy/extensions/filters/http/dynamic_forward_proxy/v3/dynamic_forward_proxy.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct FilterConfig {
    // message fields
    pub dns_cache_config: ::protobuf::SingularPtrField<super::dns_cache::DnsCacheConfig>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a FilterConfig {
    fn default() -> &'a FilterConfig {
        <FilterConfig as ::protobuf::Message>::default_instance()
    }
}

impl FilterConfig {
    pub fn new() -> FilterConfig {
        ::std::default::Default::default()
    }

    // .envoy.extensions.common.dynamic_forward_proxy.v3.DnsCacheConfig dns_cache_config = 1;


    pub fn get_dns_cache_config(&self) -> &super::dns_cache::DnsCacheConfig {
        self.dns_cache_config.as_ref().unwrap_or_else(|| <super::dns_cache::DnsCacheConfig as ::protobuf::Message>::default_instance())
    }
    pub fn clear_dns_cache_config(&mut self) {
        self.dns_cache_config.clear();
    }

    pub fn has_dns_cache_config(&self) -> bool {
        self.dns_cache_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dns_cache_config(&mut self, v: super::dns_cache::DnsCacheConfig) {
        self.dns_cache_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dns_cache_config(&mut self) -> &mut super::dns_cache::DnsCacheConfig {
        if self.dns_cache_config.is_none() {
            self.dns_cache_config.set_default();
        }
        self.dns_cache_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_dns_cache_config(&mut self) -> super::dns_cache::DnsCacheConfig {
        self.dns_cache_config.take().unwrap_or_else(|| super::dns_cache::DnsCacheConfig::new())
    }
}

impl ::protobuf::Message for FilterConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.dns_cache_config {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dns_cache_config)?;
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
        if let Some(ref v) = self.dns_cache_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.dns_cache_config.as_ref() {
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

    fn new() -> FilterConfig {
        FilterConfig::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::dns_cache::DnsCacheConfig>>(
                "dns_cache_config",
                |m: &FilterConfig| { &m.dns_cache_config },
                |m: &mut FilterConfig| { &mut m.dns_cache_config },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<FilterConfig>(
                "FilterConfig",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static FilterConfig {
        static instance: ::protobuf::rt::LazyV2<FilterConfig> = ::protobuf::rt::LazyV2::INIT;
        instance.get(FilterConfig::new)
    }
}

impl ::protobuf::Clear for FilterConfig {
    fn clear(&mut self) {
        self.dns_cache_config.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FilterConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FilterConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PerRouteConfig {
    // message oneof groups
    pub host_rewrite_specifier: ::std::option::Option<PerRouteConfig_oneof_host_rewrite_specifier>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PerRouteConfig {
    fn default() -> &'a PerRouteConfig {
        <PerRouteConfig as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum PerRouteConfig_oneof_host_rewrite_specifier {
    host_rewrite_literal(::std::string::String),
    host_rewrite_header(::std::string::String),
}

impl PerRouteConfig {
    pub fn new() -> PerRouteConfig {
        ::std::default::Default::default()
    }

    // string host_rewrite_literal = 1;


    pub fn get_host_rewrite_literal(&self) -> &str {
        match self.host_rewrite_specifier {
            ::std::option::Option::Some(PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_literal(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_host_rewrite_literal(&mut self) {
        self.host_rewrite_specifier = ::std::option::Option::None;
    }

    pub fn has_host_rewrite_literal(&self) -> bool {
        match self.host_rewrite_specifier {
            ::std::option::Option::Some(PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_literal(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_host_rewrite_literal(&mut self, v: ::std::string::String) {
        self.host_rewrite_specifier = ::std::option::Option::Some(PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_literal(v))
    }

    // Mutable pointer to the field.
    pub fn mut_host_rewrite_literal(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_literal(_)) = self.host_rewrite_specifier {
        } else {
            self.host_rewrite_specifier = ::std::option::Option::Some(PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_literal(::std::string::String::new()));
        }
        match self.host_rewrite_specifier {
            ::std::option::Option::Some(PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_literal(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_host_rewrite_literal(&mut self) -> ::std::string::String {
        if self.has_host_rewrite_literal() {
            match self.host_rewrite_specifier.take() {
                ::std::option::Option::Some(PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_literal(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // string host_rewrite_header = 2;


    pub fn get_host_rewrite_header(&self) -> &str {
        match self.host_rewrite_specifier {
            ::std::option::Option::Some(PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_header(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_host_rewrite_header(&mut self) {
        self.host_rewrite_specifier = ::std::option::Option::None;
    }

    pub fn has_host_rewrite_header(&self) -> bool {
        match self.host_rewrite_specifier {
            ::std::option::Option::Some(PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_header(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_host_rewrite_header(&mut self, v: ::std::string::String) {
        self.host_rewrite_specifier = ::std::option::Option::Some(PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_header(v))
    }

    // Mutable pointer to the field.
    pub fn mut_host_rewrite_header(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_header(_)) = self.host_rewrite_specifier {
        } else {
            self.host_rewrite_specifier = ::std::option::Option::Some(PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_header(::std::string::String::new()));
        }
        match self.host_rewrite_specifier {
            ::std::option::Option::Some(PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_header(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_host_rewrite_header(&mut self) -> ::std::string::String {
        if self.has_host_rewrite_header() {
            match self.host_rewrite_specifier.take() {
                ::std::option::Option::Some(PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_header(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }
}

impl ::protobuf::Message for PerRouteConfig {
    fn is_initialized(&self) -> bool {
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
                    self.host_rewrite_specifier = ::std::option::Option::Some(PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_literal(is.read_string()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.host_rewrite_specifier = ::std::option::Option::Some(PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_header(is.read_string()?));
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
        if let ::std::option::Option::Some(ref v) = self.host_rewrite_specifier {
            match v {
                &PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_literal(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_header(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.host_rewrite_specifier {
            match v {
                &PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_literal(ref v) => {
                    os.write_string(1, v)?;
                },
                &PerRouteConfig_oneof_host_rewrite_specifier::host_rewrite_header(ref v) => {
                    os.write_string(2, v)?;
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

    fn new() -> PerRouteConfig {
        PerRouteConfig::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "host_rewrite_literal",
                PerRouteConfig::has_host_rewrite_literal,
                PerRouteConfig::get_host_rewrite_literal,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "host_rewrite_header",
                PerRouteConfig::has_host_rewrite_header,
                PerRouteConfig::get_host_rewrite_header,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<PerRouteConfig>(
                "PerRouteConfig",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static PerRouteConfig {
        static instance: ::protobuf::rt::LazyV2<PerRouteConfig> = ::protobuf::rt::LazyV2::INIT;
        instance.get(PerRouteConfig::new)
    }
}

impl ::protobuf::Clear for PerRouteConfig {
    fn clear(&mut self) {
        self.host_rewrite_specifier = ::std::option::Option::None;
        self.host_rewrite_specifier = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PerRouteConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PerRouteConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \nRenvoy/extensions/filters/http/dynamic_forward_proxy/v3/dynamic_forwar\
    d_proxy.proto\x126envoy.extensions.filters.http.dynamic_forward_proxy.v3\
    \x1a@envoy/extensions/common/dynamic_forward_proxy/v3/dns_cache.proto\
    \x1a\x1dudpa/annotations/status.proto\x1a!udpa/annotations/versioning.pr\
    oto\x1a\x17validate/validate.proto\"\xd0\x01\n\x0cFilterConfig\x12t\n\
    \x10dns_cache_config\x18\x01\x20\x01(\x0b2@.envoy.extensions.common.dyna\
    mic_forward_proxy.v3.DnsCacheConfigR\x0ednsCacheConfigB\x08\xfaB\x05\x8a\
    \x01\x02\x10\x01:J\x9a\xc5\x88\x1eE\nCenvoy.config.filter.http.dynamic_f\
    orward_proxy.v2alpha.FilterConfig\"\xde\x01\n\x0ePerRouteConfig\x122\n\
    \x14host_rewrite_literal\x18\x01\x20\x01(\tH\0R\x12hostRewriteLiteral\
    \x120\n\x13host_rewrite_header\x18\x02\x20\x01(\tH\0R\x11hostRewriteHead\
    erB\x18\n\x16host_rewrite_specifier:L\x9a\xc5\x88\x1eG\nEenvoy.config.fi\
    lter.http.dynamic_forward_proxy.v2alpha.PerRouteConfigBj\nDio.envoyproxy\
    .envoy.extensions.filters.http.dynamic_forward_proxy.v3B\x18DynamicForwa\
    rdProxyProtoP\x01\xba\x80\xc8\xd1\x06\x02\x10\x02b\x06proto3\
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