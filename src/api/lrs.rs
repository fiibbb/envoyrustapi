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
//! Generated file from `envoy/service/load_stats/v4alpha/lrs.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct LoadStatsRequest {
    // message fields
    pub node: ::protobuf::SingularPtrField<super::base::Node>,
    pub cluster_stats: ::protobuf::RepeatedField<super::load_report::ClusterStats>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a LoadStatsRequest {
    fn default() -> &'a LoadStatsRequest {
        <LoadStatsRequest as ::protobuf::Message>::default_instance()
    }
}

impl LoadStatsRequest {
    pub fn new() -> LoadStatsRequest {
        ::std::default::Default::default()
    }

    // .envoy.config.core.v4alpha.Node node = 1;


    pub fn get_node(&self) -> &super::base::Node {
        self.node.as_ref().unwrap_or_else(|| <super::base::Node as ::protobuf::Message>::default_instance())
    }
    pub fn clear_node(&mut self) {
        self.node.clear();
    }

    pub fn has_node(&self) -> bool {
        self.node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node(&mut self, v: super::base::Node) {
        self.node = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node(&mut self) -> &mut super::base::Node {
        if self.node.is_none() {
            self.node.set_default();
        }
        self.node.as_mut().unwrap()
    }

    // Take field
    pub fn take_node(&mut self) -> super::base::Node {
        self.node.take().unwrap_or_else(|| super::base::Node::new())
    }

    // repeated .envoy.config.endpoint.v3.ClusterStats cluster_stats = 2;


    pub fn get_cluster_stats(&self) -> &[super::load_report::ClusterStats] {
        &self.cluster_stats
    }
    pub fn clear_cluster_stats(&mut self) {
        self.cluster_stats.clear();
    }

    // Param is passed by value, moved
    pub fn set_cluster_stats(&mut self, v: ::protobuf::RepeatedField<super::load_report::ClusterStats>) {
        self.cluster_stats = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cluster_stats(&mut self) -> &mut ::protobuf::RepeatedField<super::load_report::ClusterStats> {
        &mut self.cluster_stats
    }

    // Take field
    pub fn take_cluster_stats(&mut self) -> ::protobuf::RepeatedField<super::load_report::ClusterStats> {
        ::std::mem::replace(&mut self.cluster_stats, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for LoadStatsRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.node {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cluster_stats {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.node)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cluster_stats)?;
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
        if let Some(ref v) = self.node.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.cluster_stats {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.node.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.cluster_stats {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn new() -> LoadStatsRequest {
        LoadStatsRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Node>>(
                "node",
                |m: &LoadStatsRequest| { &m.node },
                |m: &mut LoadStatsRequest| { &mut m.node },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::load_report::ClusterStats>>(
                "cluster_stats",
                |m: &LoadStatsRequest| { &m.cluster_stats },
                |m: &mut LoadStatsRequest| { &mut m.cluster_stats },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<LoadStatsRequest>(
                "LoadStatsRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static LoadStatsRequest {
        static instance: ::protobuf::rt::LazyV2<LoadStatsRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(LoadStatsRequest::new)
    }
}

impl ::protobuf::Clear for LoadStatsRequest {
    fn clear(&mut self) {
        self.node.clear();
        self.cluster_stats.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoadStatsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoadStatsRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoadStatsResponse {
    // message fields
    pub clusters: ::protobuf::RepeatedField<::std::string::String>,
    pub send_all_clusters: bool,
    pub load_reporting_interval: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    pub report_endpoint_granularity: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a LoadStatsResponse {
    fn default() -> &'a LoadStatsResponse {
        <LoadStatsResponse as ::protobuf::Message>::default_instance()
    }
}

impl LoadStatsResponse {
    pub fn new() -> LoadStatsResponse {
        ::std::default::Default::default()
    }

    // repeated string clusters = 1;


    pub fn get_clusters(&self) -> &[::std::string::String] {
        &self.clusters
    }
    pub fn clear_clusters(&mut self) {
        self.clusters.clear();
    }

    // Param is passed by value, moved
    pub fn set_clusters(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.clusters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_clusters(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.clusters
    }

    // Take field
    pub fn take_clusters(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.clusters, ::protobuf::RepeatedField::new())
    }

    // bool send_all_clusters = 4;


    pub fn get_send_all_clusters(&self) -> bool {
        self.send_all_clusters
    }
    pub fn clear_send_all_clusters(&mut self) {
        self.send_all_clusters = false;
    }

    // Param is passed by value, moved
    pub fn set_send_all_clusters(&mut self, v: bool) {
        self.send_all_clusters = v;
    }

    // .google.protobuf.Duration load_reporting_interval = 2;


    pub fn get_load_reporting_interval(&self) -> &::protobuf::well_known_types::Duration {
        self.load_reporting_interval.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Duration as ::protobuf::Message>::default_instance())
    }
    pub fn clear_load_reporting_interval(&mut self) {
        self.load_reporting_interval.clear();
    }

    pub fn has_load_reporting_interval(&self) -> bool {
        self.load_reporting_interval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_load_reporting_interval(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.load_reporting_interval = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_load_reporting_interval(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.load_reporting_interval.is_none() {
            self.load_reporting_interval.set_default();
        }
        self.load_reporting_interval.as_mut().unwrap()
    }

    // Take field
    pub fn take_load_reporting_interval(&mut self) -> ::protobuf::well_known_types::Duration {
        self.load_reporting_interval.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }

    // bool report_endpoint_granularity = 3;


    pub fn get_report_endpoint_granularity(&self) -> bool {
        self.report_endpoint_granularity
    }
    pub fn clear_report_endpoint_granularity(&mut self) {
        self.report_endpoint_granularity = false;
    }

    // Param is passed by value, moved
    pub fn set_report_endpoint_granularity(&mut self, v: bool) {
        self.report_endpoint_granularity = v;
    }
}

impl ::protobuf::Message for LoadStatsResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.load_reporting_interval {
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
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.clusters)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.send_all_clusters = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.load_reporting_interval)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.report_endpoint_granularity = tmp;
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
        for value in &self.clusters {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if self.send_all_clusters != false {
            my_size += 2;
        }
        if let Some(ref v) = self.load_reporting_interval.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.report_endpoint_granularity != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.clusters {
            os.write_string(1, &v)?;
        };
        if self.send_all_clusters != false {
            os.write_bool(4, self.send_all_clusters)?;
        }
        if let Some(ref v) = self.load_reporting_interval.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.report_endpoint_granularity != false {
            os.write_bool(3, self.report_endpoint_granularity)?;
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

    fn new() -> LoadStatsResponse {
        LoadStatsResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "clusters",
                |m: &LoadStatsResponse| { &m.clusters },
                |m: &mut LoadStatsResponse| { &mut m.clusters },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "send_all_clusters",
                |m: &LoadStatsResponse| { &m.send_all_clusters },
                |m: &mut LoadStatsResponse| { &mut m.send_all_clusters },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                "load_reporting_interval",
                |m: &LoadStatsResponse| { &m.load_reporting_interval },
                |m: &mut LoadStatsResponse| { &mut m.load_reporting_interval },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "report_endpoint_granularity",
                |m: &LoadStatsResponse| { &m.report_endpoint_granularity },
                |m: &mut LoadStatsResponse| { &mut m.report_endpoint_granularity },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<LoadStatsResponse>(
                "LoadStatsResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static LoadStatsResponse {
        static instance: ::protobuf::rt::LazyV2<LoadStatsResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(LoadStatsResponse::new)
    }
}

impl ::protobuf::Clear for LoadStatsResponse {
    fn clear(&mut self) {
        self.clusters.clear();
        self.send_all_clusters = false;
        self.load_reporting_interval.clear();
        self.report_endpoint_granularity = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoadStatsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoadStatsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n*envoy/service/load_stats/v4alpha/lrs.proto\x12\x20envoy.service.load_\
    stats.v4alpha\x1a$envoy/config/core/v4alpha/base.proto\x1a*envoy/config/\
    endpoint/v3/load_report.proto\x1a\x1egoogle/protobuf/duration.proto\x1a\
    \x1dudpa/annotations/status.proto\x1a!udpa/annotations/versioning.proto\
    \x1a\x17validate/validate.proto\"\xc9\x01\n\x10LoadStatsRequest\x123\n\
    \x04node\x18\x01\x20\x01(\x0b2\x1f.envoy.config.core.v4alpha.NodeR\x04no\
    de\x12K\n\rcluster_stats\x18\x02\x20\x03(\x0b2&.envoy.config.endpoint.v3\
    .ClusterStatsR\x0cclusterStats:3\x9a\xc5\x88\x1e.\n,envoy.service.load_s\
    tats.v3.LoadStatsRequest\"\xa4\x02\n\x11LoadStatsResponse\x12\x1a\n\x08c\
    lusters\x18\x01\x20\x03(\tR\x08clusters\x12*\n\x11send_all_clusters\x18\
    \x04\x20\x01(\x08R\x0fsendAllClusters\x12Q\n\x17load_reporting_interval\
    \x18\x02\x20\x01(\x0b2\x19.google.protobuf.DurationR\x15loadReportingInt\
    erval\x12>\n\x1breport_endpoint_granularity\x18\x03\x20\x01(\x08R\x19rep\
    ortEndpointGranularity:4\x9a\xc5\x88\x1e/\n-envoy.service.load_stats.v3.\
    LoadStatsResponse2\x99\x01\n\x14LoadReportingService\x12\x80\x01\n\x0fSt\
    reamLoadStats\x122.envoy.service.load_stats.v4alpha.LoadStatsRequest\x1a\
    3.envoy.service.load_stats.v4alpha.LoadStatsResponse\"\0(\x010\x01BG\n.i\
    o.envoyproxy.envoy.service.load_stats.v4alphaB\x08LrsProtoP\x01\x88\x01\
    \x01\xba\x80\xc8\xd1\x06\x02\x10\x03b\x06proto3\
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
