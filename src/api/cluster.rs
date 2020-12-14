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
//! Generated file from `envoy/extensions/clusters/dynamic_forward_proxy/v3/cluster.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct ClusterConfig {
    // message fields
    pub dns_cache_config: ::protobuf::SingularPtrField<super::dns_cache::DnsCacheConfig>,
    pub allow_insecure_cluster_options: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ClusterConfig {
    fn default() -> &'a ClusterConfig {
        <ClusterConfig as ::protobuf::Message>::default_instance()
    }
}

impl ClusterConfig {
    pub fn new() -> ClusterConfig {
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

    // bool allow_insecure_cluster_options = 2;


    pub fn get_allow_insecure_cluster_options(&self) -> bool {
        self.allow_insecure_cluster_options
    }
    pub fn clear_allow_insecure_cluster_options(&mut self) {
        self.allow_insecure_cluster_options = false;
    }

    // Param is passed by value, moved
    pub fn set_allow_insecure_cluster_options(&mut self, v: bool) {
        self.allow_insecure_cluster_options = v;
    }
}

impl ::protobuf::Message for ClusterConfig {
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
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.allow_insecure_cluster_options = tmp;
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
        if self.allow_insecure_cluster_options != false {
            my_size += 2;
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
        if self.allow_insecure_cluster_options != false {
            os.write_bool(2, self.allow_insecure_cluster_options)?;
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

    fn new() -> ClusterConfig {
        ClusterConfig::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::dns_cache::DnsCacheConfig>>(
                "dns_cache_config",
                |m: &ClusterConfig| { &m.dns_cache_config },
                |m: &mut ClusterConfig| { &mut m.dns_cache_config },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "allow_insecure_cluster_options",
                |m: &ClusterConfig| { &m.allow_insecure_cluster_options },
                |m: &mut ClusterConfig| { &mut m.allow_insecure_cluster_options },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ClusterConfig>(
                "ClusterConfig",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ClusterConfig {
        static instance: ::protobuf::rt::LazyV2<ClusterConfig> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ClusterConfig::new)
    }
}

impl ::protobuf::Clear for ClusterConfig {
    fn clear(&mut self) {
        self.dns_cache_config.clear();
        self.allow_insecure_cluster_options = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClusterConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClusterConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n@envoy/extensions/clusters/dynamic_forward_proxy/v3/cluster.proto\x122\
    envoy.extensions.clusters.dynamic_forward_proxy.v3\x1a@envoy/extensions/\
    common/dynamic_forward_proxy/v3/dns_cache.proto\x1a\x1dudpa/annotations/\
    status.proto\x1a!udpa/annotations/versioning.proto\x1a\x17validate/valid\
    ate.proto\"\x93\x02\n\rClusterConfig\x12t\n\x10dns_cache_config\x18\x01\
    \x20\x01(\x0b2@.envoy.extensions.common.dynamic_forward_proxy.v3.DnsCach\
    eConfigR\x0ednsCacheConfigB\x08\xfaB\x05\x8a\x01\x02\x10\x01\x12C\n\x1ea\
    llow_insecure_cluster_options\x18\x02\x20\x01(\x08R\x1ballowInsecureClus\
    terOptions:G\x9a\xc5\x88\x1eB\n@envoy.config.cluster.dynamic_forward_pro\
    xy.v2alpha.ClusterConfigBZ\n@io.envoyproxy.envoy.extensions.clusters.dyn\
    amic_forward_proxy.v3B\x0cClusterProtoP\x01\xba\x80\xc8\xd1\x06\x02\x10\
    \x02b\x06proto3\
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
