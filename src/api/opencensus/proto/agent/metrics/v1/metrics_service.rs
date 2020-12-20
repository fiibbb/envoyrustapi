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
//! Generated file from `opencensus/proto/agent/metrics/v1/metrics_service.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct ExportMetricsServiceRequest {
    // message fields
    pub node: ::protobuf::SingularPtrField<super::common::Node>,
    pub metrics: ::protobuf::RepeatedField<super::metrics::Metric>,
    pub resource: ::protobuf::SingularPtrField<super::resource::Resource>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ExportMetricsServiceRequest {
    fn default() -> &'a ExportMetricsServiceRequest {
        <ExportMetricsServiceRequest as ::protobuf::Message>::default_instance()
    }
}

impl ExportMetricsServiceRequest {
    pub fn new() -> ExportMetricsServiceRequest {
        ::std::default::Default::default()
    }

    // .opencensus.proto.agent.common.v1.Node node = 1;


    pub fn get_node(&self) -> &super::common::Node {
        self.node.as_ref().unwrap_or_else(|| <super::common::Node as ::protobuf::Message>::default_instance())
    }
    pub fn clear_node(&mut self) {
        self.node.clear();
    }

    pub fn has_node(&self) -> bool {
        self.node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node(&mut self, v: super::common::Node) {
        self.node = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node(&mut self) -> &mut super::common::Node {
        if self.node.is_none() {
            self.node.set_default();
        }
        self.node.as_mut().unwrap()
    }

    // Take field
    pub fn take_node(&mut self) -> super::common::Node {
        self.node.take().unwrap_or_else(|| super::common::Node::new())
    }

    // repeated .opencensus.proto.metrics.v1.Metric metrics = 2;


    pub fn get_metrics(&self) -> &[super::metrics::Metric] {
        &self.metrics
    }
    pub fn clear_metrics(&mut self) {
        self.metrics.clear();
    }

    // Param is passed by value, moved
    pub fn set_metrics(&mut self, v: ::protobuf::RepeatedField<super::metrics::Metric>) {
        self.metrics = v;
    }

    // Mutable pointer to the field.
    pub fn mut_metrics(&mut self) -> &mut ::protobuf::RepeatedField<super::metrics::Metric> {
        &mut self.metrics
    }

    // Take field
    pub fn take_metrics(&mut self) -> ::protobuf::RepeatedField<super::metrics::Metric> {
        ::std::mem::replace(&mut self.metrics, ::protobuf::RepeatedField::new())
    }

    // .opencensus.proto.resource.v1.Resource resource = 3;


    pub fn get_resource(&self) -> &super::resource::Resource {
        self.resource.as_ref().unwrap_or_else(|| <super::resource::Resource as ::protobuf::Message>::default_instance())
    }
    pub fn clear_resource(&mut self) {
        self.resource.clear();
    }

    pub fn has_resource(&self) -> bool {
        self.resource.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resource(&mut self, v: super::resource::Resource) {
        self.resource = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resource(&mut self) -> &mut super::resource::Resource {
        if self.resource.is_none() {
            self.resource.set_default();
        }
        self.resource.as_mut().unwrap()
    }

    // Take field
    pub fn take_resource(&mut self) -> super::resource::Resource {
        self.resource.take().unwrap_or_else(|| super::resource::Resource::new())
    }
}

impl ::protobuf::Message for ExportMetricsServiceRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.node {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.metrics {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.resource {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.metrics)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.resource)?;
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
        for value in &self.metrics {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.resource.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
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
        for v in &self.metrics {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.resource.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> ExportMetricsServiceRequest {
        ExportMetricsServiceRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Node>>(
                "node",
                |m: &ExportMetricsServiceRequest| { &m.node },
                |m: &mut ExportMetricsServiceRequest| { &mut m.node },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metrics::Metric>>(
                "metrics",
                |m: &ExportMetricsServiceRequest| { &m.metrics },
                |m: &mut ExportMetricsServiceRequest| { &mut m.metrics },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::resource::Resource>>(
                "resource",
                |m: &ExportMetricsServiceRequest| { &m.resource },
                |m: &mut ExportMetricsServiceRequest| { &mut m.resource },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ExportMetricsServiceRequest>(
                "ExportMetricsServiceRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ExportMetricsServiceRequest {
        static instance: ::protobuf::rt::LazyV2<ExportMetricsServiceRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ExportMetricsServiceRequest::new)
    }
}

impl ::protobuf::Clear for ExportMetricsServiceRequest {
    fn clear(&mut self) {
        self.node.clear();
        self.metrics.clear();
        self.resource.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExportMetricsServiceRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExportMetricsServiceRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ExportMetricsServiceResponse {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ExportMetricsServiceResponse {
    fn default() -> &'a ExportMetricsServiceResponse {
        <ExportMetricsServiceResponse as ::protobuf::Message>::default_instance()
    }
}

impl ExportMetricsServiceResponse {
    pub fn new() -> ExportMetricsServiceResponse {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for ExportMetricsServiceResponse {
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

    fn new() -> ExportMetricsServiceResponse {
        ExportMetricsServiceResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let fields = ::std::vec::Vec::new();
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ExportMetricsServiceResponse>(
                "ExportMetricsServiceResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ExportMetricsServiceResponse {
        static instance: ::protobuf::rt::LazyV2<ExportMetricsServiceResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ExportMetricsServiceResponse::new)
    }
}

impl ::protobuf::Clear for ExportMetricsServiceResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExportMetricsServiceResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExportMetricsServiceResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n7opencensus/proto/agent/metrics/v1/metrics_service.proto\x12!opencensu\
    s.proto.agent.metrics.v1\x1a-opencensus/proto/agent/common/v1/common.pro\
    to\x1a)opencensus/proto/metrics/v1/metrics.proto\x1a+opencensus/proto/re\
    source/v1/resource.proto\"\xdc\x01\n\x1bExportMetricsServiceRequest\x12:\
    \n\x04node\x18\x01\x20\x01(\x0b2&.opencensus.proto.agent.common.v1.NodeR\
    \x04node\x12=\n\x07metrics\x18\x02\x20\x03(\x0b2#.opencensus.proto.metri\
    cs.v1.MetricR\x07metrics\x12B\n\x08resource\x18\x03\x20\x01(\x0b2&.openc\
    ensus.proto.resource.v1.ResourceR\x08resource\"\x1e\n\x1cExportMetricsSe\
    rviceResponse2\xa2\x01\n\x0eMetricsService\x12\x8f\x01\n\x06Export\x12>.\
    opencensus.proto.agent.metrics.v1.ExportMetricsServiceRequest\x1a?.openc\
    ensus.proto.agent.metrics.v1.ExportMetricsServiceResponse\"\0(\x010\x01B\
    \xad\x01\n$io.opencensus.proto.agent.metrics.v1B\x13MetricsServiceProtoP\
    \x01ZJgithub.com/census-instrumentation/opencensus-proto/gen-go/agent/me\
    trics/v1\xea\x02!OpenCensus.Proto.Agent.Metrics.V1b\x06proto3\
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