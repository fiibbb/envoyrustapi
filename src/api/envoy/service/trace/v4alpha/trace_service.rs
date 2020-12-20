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
//! Generated file from `envoy/service/trace/v4alpha/trace_service.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct StreamTracesResponse {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a StreamTracesResponse {
    fn default() -> &'a StreamTracesResponse {
        <StreamTracesResponse as ::protobuf::Message>::default_instance()
    }
}

impl StreamTracesResponse {
    pub fn new() -> StreamTracesResponse {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for StreamTracesResponse {
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

    fn new() -> StreamTracesResponse {
        StreamTracesResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let fields = ::std::vec::Vec::new();
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<StreamTracesResponse>(
                "StreamTracesResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static StreamTracesResponse {
        static instance: ::protobuf::rt::LazyV2<StreamTracesResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(StreamTracesResponse::new)
    }
}

impl ::protobuf::Clear for StreamTracesResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StreamTracesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StreamTracesResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StreamTracesMessage {
    // message fields
    pub identifier: ::protobuf::SingularPtrField<StreamTracesMessage_Identifier>,
    pub spans: ::protobuf::RepeatedField<super::trace::Span>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a StreamTracesMessage {
    fn default() -> &'a StreamTracesMessage {
        <StreamTracesMessage as ::protobuf::Message>::default_instance()
    }
}

impl StreamTracesMessage {
    pub fn new() -> StreamTracesMessage {
        ::std::default::Default::default()
    }

    // .envoy.service.trace.v4alpha.StreamTracesMessage.Identifier identifier = 1;


    pub fn get_identifier(&self) -> &StreamTracesMessage_Identifier {
        self.identifier.as_ref().unwrap_or_else(|| <StreamTracesMessage_Identifier as ::protobuf::Message>::default_instance())
    }
    pub fn clear_identifier(&mut self) {
        self.identifier.clear();
    }

    pub fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_identifier(&mut self, v: StreamTracesMessage_Identifier) {
        self.identifier = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_identifier(&mut self) -> &mut StreamTracesMessage_Identifier {
        if self.identifier.is_none() {
            self.identifier.set_default();
        }
        self.identifier.as_mut().unwrap()
    }

    // Take field
    pub fn take_identifier(&mut self) -> StreamTracesMessage_Identifier {
        self.identifier.take().unwrap_or_else(|| StreamTracesMessage_Identifier::new())
    }

    // repeated .opencensus.proto.trace.v1.Span spans = 2;


    pub fn get_spans(&self) -> &[super::trace::Span] {
        &self.spans
    }
    pub fn clear_spans(&mut self) {
        self.spans.clear();
    }

    // Param is passed by value, moved
    pub fn set_spans(&mut self, v: ::protobuf::RepeatedField<super::trace::Span>) {
        self.spans = v;
    }

    // Mutable pointer to the field.
    pub fn mut_spans(&mut self) -> &mut ::protobuf::RepeatedField<super::trace::Span> {
        &mut self.spans
    }

    // Take field
    pub fn take_spans(&mut self) -> ::protobuf::RepeatedField<super::trace::Span> {
        ::std::mem::replace(&mut self.spans, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for StreamTracesMessage {
    fn is_initialized(&self) -> bool {
        for v in &self.identifier {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.spans {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.identifier)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.spans)?;
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
        if let Some(ref v) = self.identifier.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.spans {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.identifier.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.spans {
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

    fn new() -> StreamTracesMessage {
        StreamTracesMessage::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StreamTracesMessage_Identifier>>(
                "identifier",
                |m: &StreamTracesMessage| { &m.identifier },
                |m: &mut StreamTracesMessage| { &mut m.identifier },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::trace::Span>>(
                "spans",
                |m: &StreamTracesMessage| { &m.spans },
                |m: &mut StreamTracesMessage| { &mut m.spans },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<StreamTracesMessage>(
                "StreamTracesMessage",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static StreamTracesMessage {
        static instance: ::protobuf::rt::LazyV2<StreamTracesMessage> = ::protobuf::rt::LazyV2::INIT;
        instance.get(StreamTracesMessage::new)
    }
}

impl ::protobuf::Clear for StreamTracesMessage {
    fn clear(&mut self) {
        self.identifier.clear();
        self.spans.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StreamTracesMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StreamTracesMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StreamTracesMessage_Identifier {
    // message fields
    pub node: ::protobuf::SingularPtrField<super::base::Node>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a StreamTracesMessage_Identifier {
    fn default() -> &'a StreamTracesMessage_Identifier {
        <StreamTracesMessage_Identifier as ::protobuf::Message>::default_instance()
    }
}

impl StreamTracesMessage_Identifier {
    pub fn new() -> StreamTracesMessage_Identifier {
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
}

impl ::protobuf::Message for StreamTracesMessage_Identifier {
    fn is_initialized(&self) -> bool {
        for v in &self.node {
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

    fn new() -> StreamTracesMessage_Identifier {
        StreamTracesMessage_Identifier::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Node>>(
                "node",
                |m: &StreamTracesMessage_Identifier| { &m.node },
                |m: &mut StreamTracesMessage_Identifier| { &mut m.node },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<StreamTracesMessage_Identifier>(
                "StreamTracesMessage.Identifier",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static StreamTracesMessage_Identifier {
        static instance: ::protobuf::rt::LazyV2<StreamTracesMessage_Identifier> = ::protobuf::rt::LazyV2::INIT;
        instance.get(StreamTracesMessage_Identifier::new)
    }
}

impl ::protobuf::Clear for StreamTracesMessage_Identifier {
    fn clear(&mut self) {
        self.node.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StreamTracesMessage_Identifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StreamTracesMessage_Identifier {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n/envoy/service/trace/v4alpha/trace_service.proto\x12\x1benvoy.service.\
    trace.v4alpha\x1a$envoy/config/core/v4alpha/base.proto\x1a\x1cgoogle/api\
    /annotations.proto\x1a%opencensus/proto/trace/v1/trace.proto\x1a\x1dudpa\
    /annotations/status.proto\x1a!udpa/annotations/versioning.proto\x1a\x17v\
    alidate/validate.proto\"J\n\x14StreamTracesResponse:2\x9a\xc5\x88\x1e-\n\
    +envoy.service.trace.v3.StreamTracesResponse\"\xe8\x02\n\x13StreamTraces\
    Message\x12[\n\nidentifier\x18\x01\x20\x01(\x0b2;.envoy.service.trace.v4\
    alpha.StreamTracesMessage.IdentifierR\nidentifier\x125\n\x05spans\x18\
    \x02\x20\x03(\x0b2\x1f.opencensus.proto.trace.v1.SpanR\x05spans\x1a\x89\
    \x01\n\nIdentifier\x12=\n\x04node\x18\x01\x20\x01(\x0b2\x1f.envoy.config\
    .core.v4alpha.NodeR\x04nodeB\x08\xfaB\x05\x8a\x01\x02\x10\x01:<\x9a\xc5\
    \x88\x1e7\n5envoy.service.trace.v3.StreamTracesMessage.Identifier:1\x9a\
    \xc5\x88\x1e,\n*envoy.service.trace.v3.StreamTracesMessage2\x87\x01\n\
    \x0cTraceService\x12w\n\x0cStreamTraces\x120.envoy.service.trace.v4alpha\
    .StreamTracesMessage\x1a1.envoy.service.trace.v4alpha.StreamTracesRespon\
    se\"\0(\x01BK\n)io.envoyproxy.envoy.service.trace.v4alphaB\x11TraceServi\
    ceProtoP\x01\x88\x01\x01\xba\x80\xc8\xd1\x06\x02\x10\x03b\x06proto3\
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
