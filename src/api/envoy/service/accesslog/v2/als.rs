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
//! Generated file from `envoy/service/accesslog/v2/als.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct StreamAccessLogsResponse {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a StreamAccessLogsResponse {
    fn default() -> &'a StreamAccessLogsResponse {
        <StreamAccessLogsResponse as ::protobuf::Message>::default_instance()
    }
}

impl StreamAccessLogsResponse {
    pub fn new() -> StreamAccessLogsResponse {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for StreamAccessLogsResponse {
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

    fn new() -> StreamAccessLogsResponse {
        StreamAccessLogsResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let fields = ::std::vec::Vec::new();
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<StreamAccessLogsResponse>(
                "StreamAccessLogsResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static StreamAccessLogsResponse {
        static instance: ::protobuf::rt::LazyV2<StreamAccessLogsResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(StreamAccessLogsResponse::new)
    }
}

impl ::protobuf::Clear for StreamAccessLogsResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StreamAccessLogsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StreamAccessLogsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StreamAccessLogsMessage {
    // message fields
    pub identifier: ::protobuf::SingularPtrField<StreamAccessLogsMessage_Identifier>,
    // message oneof groups
    pub log_entries: ::std::option::Option<StreamAccessLogsMessage_oneof_log_entries>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a StreamAccessLogsMessage {
    fn default() -> &'a StreamAccessLogsMessage {
        <StreamAccessLogsMessage as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum StreamAccessLogsMessage_oneof_log_entries {
    http_logs(StreamAccessLogsMessage_HTTPAccessLogEntries),
    tcp_logs(StreamAccessLogsMessage_TCPAccessLogEntries),
}

impl StreamAccessLogsMessage {
    pub fn new() -> StreamAccessLogsMessage {
        ::std::default::Default::default()
    }

    // .envoy.service.accesslog.v2.StreamAccessLogsMessage.Identifier identifier = 1;


    pub fn get_identifier(&self) -> &StreamAccessLogsMessage_Identifier {
        self.identifier.as_ref().unwrap_or_else(|| <StreamAccessLogsMessage_Identifier as ::protobuf::Message>::default_instance())
    }
    pub fn clear_identifier(&mut self) {
        self.identifier.clear();
    }

    pub fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_identifier(&mut self, v: StreamAccessLogsMessage_Identifier) {
        self.identifier = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_identifier(&mut self) -> &mut StreamAccessLogsMessage_Identifier {
        if self.identifier.is_none() {
            self.identifier.set_default();
        }
        self.identifier.as_mut().unwrap()
    }

    // Take field
    pub fn take_identifier(&mut self) -> StreamAccessLogsMessage_Identifier {
        self.identifier.take().unwrap_or_else(|| StreamAccessLogsMessage_Identifier::new())
    }

    // .envoy.service.accesslog.v2.StreamAccessLogsMessage.HTTPAccessLogEntries http_logs = 2;


    pub fn get_http_logs(&self) -> &StreamAccessLogsMessage_HTTPAccessLogEntries {
        match self.log_entries {
            ::std::option::Option::Some(StreamAccessLogsMessage_oneof_log_entries::http_logs(ref v)) => v,
            _ => <StreamAccessLogsMessage_HTTPAccessLogEntries as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_http_logs(&mut self) {
        self.log_entries = ::std::option::Option::None;
    }

    pub fn has_http_logs(&self) -> bool {
        match self.log_entries {
            ::std::option::Option::Some(StreamAccessLogsMessage_oneof_log_entries::http_logs(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_http_logs(&mut self, v: StreamAccessLogsMessage_HTTPAccessLogEntries) {
        self.log_entries = ::std::option::Option::Some(StreamAccessLogsMessage_oneof_log_entries::http_logs(v))
    }

    // Mutable pointer to the field.
    pub fn mut_http_logs(&mut self) -> &mut StreamAccessLogsMessage_HTTPAccessLogEntries {
        if let ::std::option::Option::Some(StreamAccessLogsMessage_oneof_log_entries::http_logs(_)) = self.log_entries {
        } else {
            self.log_entries = ::std::option::Option::Some(StreamAccessLogsMessage_oneof_log_entries::http_logs(StreamAccessLogsMessage_HTTPAccessLogEntries::new()));
        }
        match self.log_entries {
            ::std::option::Option::Some(StreamAccessLogsMessage_oneof_log_entries::http_logs(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_http_logs(&mut self) -> StreamAccessLogsMessage_HTTPAccessLogEntries {
        if self.has_http_logs() {
            match self.log_entries.take() {
                ::std::option::Option::Some(StreamAccessLogsMessage_oneof_log_entries::http_logs(v)) => v,
                _ => panic!(),
            }
        } else {
            StreamAccessLogsMessage_HTTPAccessLogEntries::new()
        }
    }

    // .envoy.service.accesslog.v2.StreamAccessLogsMessage.TCPAccessLogEntries tcp_logs = 3;


    pub fn get_tcp_logs(&self) -> &StreamAccessLogsMessage_TCPAccessLogEntries {
        match self.log_entries {
            ::std::option::Option::Some(StreamAccessLogsMessage_oneof_log_entries::tcp_logs(ref v)) => v,
            _ => <StreamAccessLogsMessage_TCPAccessLogEntries as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_tcp_logs(&mut self) {
        self.log_entries = ::std::option::Option::None;
    }

    pub fn has_tcp_logs(&self) -> bool {
        match self.log_entries {
            ::std::option::Option::Some(StreamAccessLogsMessage_oneof_log_entries::tcp_logs(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tcp_logs(&mut self, v: StreamAccessLogsMessage_TCPAccessLogEntries) {
        self.log_entries = ::std::option::Option::Some(StreamAccessLogsMessage_oneof_log_entries::tcp_logs(v))
    }

    // Mutable pointer to the field.
    pub fn mut_tcp_logs(&mut self) -> &mut StreamAccessLogsMessage_TCPAccessLogEntries {
        if let ::std::option::Option::Some(StreamAccessLogsMessage_oneof_log_entries::tcp_logs(_)) = self.log_entries {
        } else {
            self.log_entries = ::std::option::Option::Some(StreamAccessLogsMessage_oneof_log_entries::tcp_logs(StreamAccessLogsMessage_TCPAccessLogEntries::new()));
        }
        match self.log_entries {
            ::std::option::Option::Some(StreamAccessLogsMessage_oneof_log_entries::tcp_logs(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tcp_logs(&mut self) -> StreamAccessLogsMessage_TCPAccessLogEntries {
        if self.has_tcp_logs() {
            match self.log_entries.take() {
                ::std::option::Option::Some(StreamAccessLogsMessage_oneof_log_entries::tcp_logs(v)) => v,
                _ => panic!(),
            }
        } else {
            StreamAccessLogsMessage_TCPAccessLogEntries::new()
        }
    }
}

impl ::protobuf::Message for StreamAccessLogsMessage {
    fn is_initialized(&self) -> bool {
        for v in &self.identifier {
            if !v.is_initialized() {
                return false;
            }
        };
        if let Some(StreamAccessLogsMessage_oneof_log_entries::http_logs(ref v)) = self.log_entries {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(StreamAccessLogsMessage_oneof_log_entries::tcp_logs(ref v)) = self.log_entries {
            if !v.is_initialized() {
                return false;
            }
        }
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.log_entries = ::std::option::Option::Some(StreamAccessLogsMessage_oneof_log_entries::http_logs(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.log_entries = ::std::option::Option::Some(StreamAccessLogsMessage_oneof_log_entries::tcp_logs(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.log_entries {
            match v {
                &StreamAccessLogsMessage_oneof_log_entries::http_logs(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &StreamAccessLogsMessage_oneof_log_entries::tcp_logs(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
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
        if let ::std::option::Option::Some(ref v) = self.log_entries {
            match v {
                &StreamAccessLogsMessage_oneof_log_entries::http_logs(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &StreamAccessLogsMessage_oneof_log_entries::tcp_logs(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
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

    fn new() -> StreamAccessLogsMessage {
        StreamAccessLogsMessage::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StreamAccessLogsMessage_Identifier>>(
                "identifier",
                |m: &StreamAccessLogsMessage| { &m.identifier },
                |m: &mut StreamAccessLogsMessage| { &mut m.identifier },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, StreamAccessLogsMessage_HTTPAccessLogEntries>(
                "http_logs",
                StreamAccessLogsMessage::has_http_logs,
                StreamAccessLogsMessage::get_http_logs,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, StreamAccessLogsMessage_TCPAccessLogEntries>(
                "tcp_logs",
                StreamAccessLogsMessage::has_tcp_logs,
                StreamAccessLogsMessage::get_tcp_logs,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<StreamAccessLogsMessage>(
                "StreamAccessLogsMessage",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static StreamAccessLogsMessage {
        static instance: ::protobuf::rt::LazyV2<StreamAccessLogsMessage> = ::protobuf::rt::LazyV2::INIT;
        instance.get(StreamAccessLogsMessage::new)
    }
}

impl ::protobuf::Clear for StreamAccessLogsMessage {
    fn clear(&mut self) {
        self.identifier.clear();
        self.log_entries = ::std::option::Option::None;
        self.log_entries = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StreamAccessLogsMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StreamAccessLogsMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StreamAccessLogsMessage_Identifier {
    // message fields
    pub node: ::protobuf::SingularPtrField<super::base::Node>,
    pub log_name: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a StreamAccessLogsMessage_Identifier {
    fn default() -> &'a StreamAccessLogsMessage_Identifier {
        <StreamAccessLogsMessage_Identifier as ::protobuf::Message>::default_instance()
    }
}

impl StreamAccessLogsMessage_Identifier {
    pub fn new() -> StreamAccessLogsMessage_Identifier {
        ::std::default::Default::default()
    }

    // .envoy.api.v2.core.Node node = 1;


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

    // string log_name = 2;


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
}

impl ::protobuf::Message for StreamAccessLogsMessage_Identifier {
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
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.log_name)?;
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
        if !self.log_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.log_name);
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
        if !self.log_name.is_empty() {
            os.write_string(2, &self.log_name)?;
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

    fn new() -> StreamAccessLogsMessage_Identifier {
        StreamAccessLogsMessage_Identifier::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::Node>>(
                "node",
                |m: &StreamAccessLogsMessage_Identifier| { &m.node },
                |m: &mut StreamAccessLogsMessage_Identifier| { &mut m.node },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "log_name",
                |m: &StreamAccessLogsMessage_Identifier| { &m.log_name },
                |m: &mut StreamAccessLogsMessage_Identifier| { &mut m.log_name },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<StreamAccessLogsMessage_Identifier>(
                "StreamAccessLogsMessage.Identifier",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static StreamAccessLogsMessage_Identifier {
        static instance: ::protobuf::rt::LazyV2<StreamAccessLogsMessage_Identifier> = ::protobuf::rt::LazyV2::INIT;
        instance.get(StreamAccessLogsMessage_Identifier::new)
    }
}

impl ::protobuf::Clear for StreamAccessLogsMessage_Identifier {
    fn clear(&mut self) {
        self.node.clear();
        self.log_name.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StreamAccessLogsMessage_Identifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StreamAccessLogsMessage_Identifier {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StreamAccessLogsMessage_HTTPAccessLogEntries {
    // message fields
    pub log_entry: ::protobuf::RepeatedField<super::accesslog::HTTPAccessLogEntry>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a StreamAccessLogsMessage_HTTPAccessLogEntries {
    fn default() -> &'a StreamAccessLogsMessage_HTTPAccessLogEntries {
        <StreamAccessLogsMessage_HTTPAccessLogEntries as ::protobuf::Message>::default_instance()
    }
}

impl StreamAccessLogsMessage_HTTPAccessLogEntries {
    pub fn new() -> StreamAccessLogsMessage_HTTPAccessLogEntries {
        ::std::default::Default::default()
    }

    // repeated .envoy.data.accesslog.v2.HTTPAccessLogEntry log_entry = 1;


    pub fn get_log_entry(&self) -> &[super::accesslog::HTTPAccessLogEntry] {
        &self.log_entry
    }
    pub fn clear_log_entry(&mut self) {
        self.log_entry.clear();
    }

    // Param is passed by value, moved
    pub fn set_log_entry(&mut self, v: ::protobuf::RepeatedField<super::accesslog::HTTPAccessLogEntry>) {
        self.log_entry = v;
    }

    // Mutable pointer to the field.
    pub fn mut_log_entry(&mut self) -> &mut ::protobuf::RepeatedField<super::accesslog::HTTPAccessLogEntry> {
        &mut self.log_entry
    }

    // Take field
    pub fn take_log_entry(&mut self) -> ::protobuf::RepeatedField<super::accesslog::HTTPAccessLogEntry> {
        ::std::mem::replace(&mut self.log_entry, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for StreamAccessLogsMessage_HTTPAccessLogEntries {
    fn is_initialized(&self) -> bool {
        for v in &self.log_entry {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.log_entry)?;
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
        for value in &self.log_entry {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.log_entry {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> StreamAccessLogsMessage_HTTPAccessLogEntries {
        StreamAccessLogsMessage_HTTPAccessLogEntries::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::accesslog::HTTPAccessLogEntry>>(
                "log_entry",
                |m: &StreamAccessLogsMessage_HTTPAccessLogEntries| { &m.log_entry },
                |m: &mut StreamAccessLogsMessage_HTTPAccessLogEntries| { &mut m.log_entry },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<StreamAccessLogsMessage_HTTPAccessLogEntries>(
                "StreamAccessLogsMessage.HTTPAccessLogEntries",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static StreamAccessLogsMessage_HTTPAccessLogEntries {
        static instance: ::protobuf::rt::LazyV2<StreamAccessLogsMessage_HTTPAccessLogEntries> = ::protobuf::rt::LazyV2::INIT;
        instance.get(StreamAccessLogsMessage_HTTPAccessLogEntries::new)
    }
}

impl ::protobuf::Clear for StreamAccessLogsMessage_HTTPAccessLogEntries {
    fn clear(&mut self) {
        self.log_entry.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StreamAccessLogsMessage_HTTPAccessLogEntries {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StreamAccessLogsMessage_HTTPAccessLogEntries {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StreamAccessLogsMessage_TCPAccessLogEntries {
    // message fields
    pub log_entry: ::protobuf::RepeatedField<super::accesslog::TCPAccessLogEntry>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a StreamAccessLogsMessage_TCPAccessLogEntries {
    fn default() -> &'a StreamAccessLogsMessage_TCPAccessLogEntries {
        <StreamAccessLogsMessage_TCPAccessLogEntries as ::protobuf::Message>::default_instance()
    }
}

impl StreamAccessLogsMessage_TCPAccessLogEntries {
    pub fn new() -> StreamAccessLogsMessage_TCPAccessLogEntries {
        ::std::default::Default::default()
    }

    // repeated .envoy.data.accesslog.v2.TCPAccessLogEntry log_entry = 1;


    pub fn get_log_entry(&self) -> &[super::accesslog::TCPAccessLogEntry] {
        &self.log_entry
    }
    pub fn clear_log_entry(&mut self) {
        self.log_entry.clear();
    }

    // Param is passed by value, moved
    pub fn set_log_entry(&mut self, v: ::protobuf::RepeatedField<super::accesslog::TCPAccessLogEntry>) {
        self.log_entry = v;
    }

    // Mutable pointer to the field.
    pub fn mut_log_entry(&mut self) -> &mut ::protobuf::RepeatedField<super::accesslog::TCPAccessLogEntry> {
        &mut self.log_entry
    }

    // Take field
    pub fn take_log_entry(&mut self) -> ::protobuf::RepeatedField<super::accesslog::TCPAccessLogEntry> {
        ::std::mem::replace(&mut self.log_entry, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for StreamAccessLogsMessage_TCPAccessLogEntries {
    fn is_initialized(&self) -> bool {
        for v in &self.log_entry {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.log_entry)?;
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
        for value in &self.log_entry {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.log_entry {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> StreamAccessLogsMessage_TCPAccessLogEntries {
        StreamAccessLogsMessage_TCPAccessLogEntries::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::accesslog::TCPAccessLogEntry>>(
                "log_entry",
                |m: &StreamAccessLogsMessage_TCPAccessLogEntries| { &m.log_entry },
                |m: &mut StreamAccessLogsMessage_TCPAccessLogEntries| { &mut m.log_entry },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<StreamAccessLogsMessage_TCPAccessLogEntries>(
                "StreamAccessLogsMessage.TCPAccessLogEntries",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static StreamAccessLogsMessage_TCPAccessLogEntries {
        static instance: ::protobuf::rt::LazyV2<StreamAccessLogsMessage_TCPAccessLogEntries> = ::protobuf::rt::LazyV2::INIT;
        instance.get(StreamAccessLogsMessage_TCPAccessLogEntries::new)
    }
}

impl ::protobuf::Clear for StreamAccessLogsMessage_TCPAccessLogEntries {
    fn clear(&mut self) {
        self.log_entry.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StreamAccessLogsMessage_TCPAccessLogEntries {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StreamAccessLogsMessage_TCPAccessLogEntries {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n$envoy/service/accesslog/v2/als.proto\x12\x1aenvoy.service.accesslog.v\
    2\x1a\x1cenvoy/api/v2/core/base.proto\x1a'envoy/data/accesslog/v2/access\
    log.proto\x1a\x1dudpa/annotations/status.proto\x1a\x17validate/validate.\
    proto\"\x1a\n\x18StreamAccessLogsResponse\"\x9b\x05\n\x17StreamAccessLog\
    sMessage\x12^\n\nidentifier\x18\x01\x20\x01(\x0b2>.envoy.service.accessl\
    og.v2.StreamAccessLogsMessage.IdentifierR\nidentifier\x12g\n\thttp_logs\
    \x18\x02\x20\x01(\x0b2H.envoy.service.accesslog.v2.StreamAccessLogsMessa\
    ge.HTTPAccessLogEntriesH\0R\x08httpLogs\x12d\n\x08tcp_logs\x18\x03\x20\
    \x01(\x0b2G.envoy.service.accesslog.v2.StreamAccessLogsMessage.TCPAccess\
    LogEntriesH\0R\x07tcpLogs\x1ag\n\nIdentifier\x125\n\x04node\x18\x01\x20\
    \x01(\x0b2\x17.envoy.api.v2.core.NodeR\x04nodeB\x08\xfaB\x05\x8a\x01\x02\
    \x10\x01\x12\"\n\x08log_name\x18\x02\x20\x01(\tR\x07logNameB\x07\xfaB\
    \x04r\x02\x20\x01\x1aj\n\x14HTTPAccessLogEntries\x12R\n\tlog_entry\x18\
    \x01\x20\x03(\x0b2+.envoy.data.accesslog.v2.HTTPAccessLogEntryR\x08logEn\
    tryB\x08\xfaB\x05\x92\x01\x02\x08\x01\x1ah\n\x13TCPAccessLogEntries\x12Q\
    \n\tlog_entry\x18\x01\x20\x03(\x0b2*.envoy.data.accesslog.v2.TCPAccessLo\
    gEntryR\x08logEntryB\x08\xfaB\x05\x92\x01\x02\x08\x01B\x12\n\x0blog_entr\
    ies\x12\x03\xf8B\x012\x96\x01\n\x10AccessLogService\x12\x81\x01\n\x10Str\
    eamAccessLogs\x123.envoy.service.accesslog.v2.StreamAccessLogsMessage\
    \x1a4.envoy.service.accesslog.v2.StreamAccessLogsResponse\"\0(\x01BA\n(i\
    o.envoyproxy.envoy.service.accesslog.v2B\x08AlsProtoP\x01\x88\x01\x01\
    \xba\x80\xc8\xd1\x06\x02\x10\x01b\x06proto3\
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