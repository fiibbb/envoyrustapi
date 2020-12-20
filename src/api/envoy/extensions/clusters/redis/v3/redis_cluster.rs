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
//! Generated file from `envoy/extensions/clusters/redis/v3/redis_cluster.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct RedisClusterConfig {
    // message fields
    pub cluster_refresh_rate: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub cluster_refresh_timeout: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub redirect_refresh_interval: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub redirect_refresh_threshold: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub failure_refresh_threshold: u32,
    pub host_degraded_refresh_threshold: u32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RedisClusterConfig {
    fn default() -> &'a RedisClusterConfig {
        <RedisClusterConfig as ::protobuf::Message>::default_instance()
    }
}

impl RedisClusterConfig {
    pub fn new() -> RedisClusterConfig {
        ::std::default::Default::default()
    }

    // .google.protobuf.Duration cluster_refresh_rate = 1;


    pub fn get_cluster_refresh_rate(&self) -> &::protobuf::well_known_types::Duration {
        self.cluster_refresh_rate.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Duration as ::protobuf::Message>::default_instance())
    }
    pub fn clear_cluster_refresh_rate(&mut self) {
        self.cluster_refresh_rate.clear();
    }

    pub fn has_cluster_refresh_rate(&self) -> bool {
        self.cluster_refresh_rate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cluster_refresh_rate(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.cluster_refresh_rate = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cluster_refresh_rate(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.cluster_refresh_rate.is_none() {
            self.cluster_refresh_rate.set_default();
        }
        self.cluster_refresh_rate.as_mut().unwrap()
    }

    // Take field
    pub fn take_cluster_refresh_rate(&mut self) -> ::protobuf::well_known_types::Duration {
        self.cluster_refresh_rate.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    // .google.protobuf.Duration cluster_refresh_timeout = 2;


    pub fn get_cluster_refresh_timeout(&self) -> &::protobuf::well_known_types::Duration {
        self.cluster_refresh_timeout.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Duration as ::protobuf::Message>::default_instance())
    }
    pub fn clear_cluster_refresh_timeout(&mut self) {
        self.cluster_refresh_timeout.clear();
    }

    pub fn has_cluster_refresh_timeout(&self) -> bool {
        self.cluster_refresh_timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cluster_refresh_timeout(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.cluster_refresh_timeout = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cluster_refresh_timeout(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.cluster_refresh_timeout.is_none() {
            self.cluster_refresh_timeout.set_default();
        }
        self.cluster_refresh_timeout.as_mut().unwrap()
    }

    // Take field
    pub fn take_cluster_refresh_timeout(&mut self) -> ::protobuf::well_known_types::Duration {
        self.cluster_refresh_timeout.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    // .google.protobuf.Duration redirect_refresh_interval = 3;


    pub fn get_redirect_refresh_interval(&self) -> &::protobuf::well_known_types::Duration {
        self.redirect_refresh_interval.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Duration as ::protobuf::Message>::default_instance())
    }
    pub fn clear_redirect_refresh_interval(&mut self) {
        self.redirect_refresh_interval.clear();
    }

    pub fn has_redirect_refresh_interval(&self) -> bool {
        self.redirect_refresh_interval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_redirect_refresh_interval(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.redirect_refresh_interval = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_redirect_refresh_interval(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.redirect_refresh_interval.is_none() {
            self.redirect_refresh_interval.set_default();
        }
        self.redirect_refresh_interval.as_mut().unwrap()
    }

    // Take field
    pub fn take_redirect_refresh_interval(&mut self) -> ::protobuf::well_known_types::Duration {
        self.redirect_refresh_interval.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    // .google.protobuf.UInt32Value redirect_refresh_threshold = 4;


    pub fn get_redirect_refresh_threshold(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.redirect_refresh_threshold.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::UInt32Value as ::protobuf::Message>::default_instance())
    }
    pub fn clear_redirect_refresh_threshold(&mut self) {
        self.redirect_refresh_threshold.clear();
    }

    pub fn has_redirect_refresh_threshold(&self) -> bool {
        self.redirect_refresh_threshold.is_some()
    }

    // Param is passed by value, moved
    pub fn set_redirect_refresh_threshold(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.redirect_refresh_threshold = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_redirect_refresh_threshold(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.redirect_refresh_threshold.is_none() {
            self.redirect_refresh_threshold.set_default();
        }
        self.redirect_refresh_threshold.as_mut().unwrap()
    }

    // Take field
    pub fn take_redirect_refresh_threshold(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.redirect_refresh_threshold.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    // uint32 failure_refresh_threshold = 5;


    pub fn get_failure_refresh_threshold(&self) -> u32 {
        self.failure_refresh_threshold
    }
    pub fn clear_failure_refresh_threshold(&mut self) {
        self.failure_refresh_threshold = 0;
    }

    // Param is passed by value, moved
    pub fn set_failure_refresh_threshold(&mut self, v: u32) {
        self.failure_refresh_threshold = v;
    }

    // uint32 host_degraded_refresh_threshold = 6;


    pub fn get_host_degraded_refresh_threshold(&self) -> u32 {
        self.host_degraded_refresh_threshold
    }
    pub fn clear_host_degraded_refresh_threshold(&mut self) {
        self.host_degraded_refresh_threshold = 0;
    }

    // Param is passed by value, moved
    pub fn set_host_degraded_refresh_threshold(&mut self, v: u32) {
        self.host_degraded_refresh_threshold = v;
    }
}

impl ::protobuf::Message for RedisClusterConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.cluster_refresh_rate {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cluster_refresh_timeout {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.redirect_refresh_interval {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.redirect_refresh_threshold {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cluster_refresh_rate)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cluster_refresh_timeout)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.redirect_refresh_interval)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.redirect_refresh_threshold)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.failure_refresh_threshold = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.host_degraded_refresh_threshold = tmp;
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
        if let Some(ref v) = self.cluster_refresh_rate.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.cluster_refresh_timeout.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.redirect_refresh_interval.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.redirect_refresh_threshold.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.failure_refresh_threshold != 0 {
            my_size += ::protobuf::rt::value_size(5, self.failure_refresh_threshold, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.host_degraded_refresh_threshold != 0 {
            my_size += ::protobuf::rt::value_size(6, self.host_degraded_refresh_threshold, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.cluster_refresh_rate.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.cluster_refresh_timeout.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.redirect_refresh_interval.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.redirect_refresh_threshold.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.failure_refresh_threshold != 0 {
            os.write_uint32(5, self.failure_refresh_threshold)?;
        }
        if self.host_degraded_refresh_threshold != 0 {
            os.write_uint32(6, self.host_degraded_refresh_threshold)?;
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

    fn new() -> RedisClusterConfig {
        RedisClusterConfig::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                "cluster_refresh_rate",
                |m: &RedisClusterConfig| { &m.cluster_refresh_rate },
                |m: &mut RedisClusterConfig| { &mut m.cluster_refresh_rate },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                "cluster_refresh_timeout",
                |m: &RedisClusterConfig| { &m.cluster_refresh_timeout },
                |m: &mut RedisClusterConfig| { &mut m.cluster_refresh_timeout },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                "redirect_refresh_interval",
                |m: &RedisClusterConfig| { &m.redirect_refresh_interval },
                |m: &mut RedisClusterConfig| { &mut m.redirect_refresh_interval },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                "redirect_refresh_threshold",
                |m: &RedisClusterConfig| { &m.redirect_refresh_threshold },
                |m: &mut RedisClusterConfig| { &mut m.redirect_refresh_threshold },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "failure_refresh_threshold",
                |m: &RedisClusterConfig| { &m.failure_refresh_threshold },
                |m: &mut RedisClusterConfig| { &mut m.failure_refresh_threshold },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "host_degraded_refresh_threshold",
                |m: &RedisClusterConfig| { &m.host_degraded_refresh_threshold },
                |m: &mut RedisClusterConfig| { &mut m.host_degraded_refresh_threshold },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RedisClusterConfig>(
                "RedisClusterConfig",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RedisClusterConfig {
        static instance: ::protobuf::rt::LazyV2<RedisClusterConfig> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RedisClusterConfig::new)
    }
}

impl ::protobuf::Clear for RedisClusterConfig {
    fn clear(&mut self) {
        self.cluster_refresh_rate.clear();
        self.cluster_refresh_timeout.clear();
        self.redirect_refresh_interval.clear();
        self.redirect_refresh_threshold.clear();
        self.failure_refresh_threshold = 0;
        self.host_degraded_refresh_threshold = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RedisClusterConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RedisClusterConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n6envoy/extensions/clusters/redis/v3/redis_cluster.proto\x12\"envoy.ext\
    ensions.clusters.redis.v3\x1a\x1egoogle/protobuf/duration.proto\x1a\x1eg\
    oogle/protobuf/wrappers.proto\x1a\x1dudpa/annotations/status.proto\x1a!u\
    dpa/annotations/versioning.proto\x1a\x17validate/validate.proto\"\xb4\
    \x04\n\x12RedisClusterConfig\x12U\n\x14cluster_refresh_rate\x18\x01\x20\
    \x01(\x0b2\x19.google.protobuf.DurationR\x12clusterRefreshRateB\x08\xfaB\
    \x05\xaa\x01\x02*\0\x12[\n\x17cluster_refresh_timeout\x18\x02\x20\x01(\
    \x0b2\x19.google.protobuf.DurationR\x15clusterRefreshTimeoutB\x08\xfaB\
    \x05\xaa\x01\x02*\0\x12U\n\x19redirect_refresh_interval\x18\x03\x20\x01(\
    \x0b2\x19.google.protobuf.DurationR\x17redirectRefreshInterval\x12Z\n\
    \x1aredirect_refresh_threshold\x18\x04\x20\x01(\x0b2\x1c.google.protobuf\
    .UInt32ValueR\x18redirectRefreshThreshold\x12:\n\x19failure_refresh_thre\
    shold\x18\x05\x20\x01(\rR\x17failureRefreshThreshold\x12E\n\x1fhost_degr\
    aded_refresh_threshold\x18\x06\x20\x01(\rR\x1chostDegradedRefreshThresho\
    ld:4\x9a\xc5\x88\x1e/\n-envoy.config.cluster.redis.RedisClusterConfigBO\
    \n0io.envoyproxy.envoy.extensions.clusters.redis.v3B\x11RedisClusterProt\
    oP\x01\xba\x80\xc8\xd1\x06\x02\x10\x02b\x06proto3\
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