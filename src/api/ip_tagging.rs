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
//! Generated file from `envoy/extensions/filters/http/ip_tagging/v3/ip_tagging.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct IPTagging {
    // message fields
    pub request_type: IPTagging_RequestType,
    pub ip_tags: ::protobuf::RepeatedField<IPTagging_IPTag>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a IPTagging {
    fn default() -> &'a IPTagging {
        <IPTagging as ::protobuf::Message>::default_instance()
    }
}

impl IPTagging {
    pub fn new() -> IPTagging {
        ::std::default::Default::default()
    }

    // .envoy.extensions.filters.http.ip_tagging.v3.IPTagging.RequestType request_type = 1;


    pub fn get_request_type(&self) -> IPTagging_RequestType {
        self.request_type
    }
    pub fn clear_request_type(&mut self) {
        self.request_type = IPTagging_RequestType::BOTH;
    }

    // Param is passed by value, moved
    pub fn set_request_type(&mut self, v: IPTagging_RequestType) {
        self.request_type = v;
    }

    // repeated .envoy.extensions.filters.http.ip_tagging.v3.IPTagging.IPTag ip_tags = 4;


    pub fn get_ip_tags(&self) -> &[IPTagging_IPTag] {
        &self.ip_tags
    }
    pub fn clear_ip_tags(&mut self) {
        self.ip_tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_ip_tags(&mut self, v: ::protobuf::RepeatedField<IPTagging_IPTag>) {
        self.ip_tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ip_tags(&mut self) -> &mut ::protobuf::RepeatedField<IPTagging_IPTag> {
        &mut self.ip_tags
    }

    // Take field
    pub fn take_ip_tags(&mut self) -> ::protobuf::RepeatedField<IPTagging_IPTag> {
        ::std::mem::replace(&mut self.ip_tags, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for IPTagging {
    fn is_initialized(&self) -> bool {
        for v in &self.ip_tags {
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
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.request_type, 1, &mut self.unknown_fields)?
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ip_tags)?;
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
        if self.request_type != IPTagging_RequestType::BOTH {
            my_size += ::protobuf::rt::enum_size(1, self.request_type);
        }
        for value in &self.ip_tags {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.request_type != IPTagging_RequestType::BOTH {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.request_type))?;
        }
        for v in &self.ip_tags {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> IPTagging {
        IPTagging::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<IPTagging_RequestType>>(
                "request_type",
                |m: &IPTagging| { &m.request_type },
                |m: &mut IPTagging| { &mut m.request_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<IPTagging_IPTag>>(
                "ip_tags",
                |m: &IPTagging| { &m.ip_tags },
                |m: &mut IPTagging| { &mut m.ip_tags },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<IPTagging>(
                "IPTagging",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static IPTagging {
        static instance: ::protobuf::rt::LazyV2<IPTagging> = ::protobuf::rt::LazyV2::INIT;
        instance.get(IPTagging::new)
    }
}

impl ::protobuf::Clear for IPTagging {
    fn clear(&mut self) {
        self.request_type = IPTagging_RequestType::BOTH;
        self.ip_tags.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IPTagging {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IPTagging {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IPTagging_IPTag {
    // message fields
    pub ip_tag_name: ::std::string::String,
    pub ip_list: ::protobuf::RepeatedField<super::address::CidrRange>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a IPTagging_IPTag {
    fn default() -> &'a IPTagging_IPTag {
        <IPTagging_IPTag as ::protobuf::Message>::default_instance()
    }
}

impl IPTagging_IPTag {
    pub fn new() -> IPTagging_IPTag {
        ::std::default::Default::default()
    }

    // string ip_tag_name = 1;


    pub fn get_ip_tag_name(&self) -> &str {
        &self.ip_tag_name
    }
    pub fn clear_ip_tag_name(&mut self) {
        self.ip_tag_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_ip_tag_name(&mut self, v: ::std::string::String) {
        self.ip_tag_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ip_tag_name(&mut self) -> &mut ::std::string::String {
        &mut self.ip_tag_name
    }

    // Take field
    pub fn take_ip_tag_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ip_tag_name, ::std::string::String::new())
    }

    // repeated .envoy.config.core.v3.CidrRange ip_list = 2;


    pub fn get_ip_list(&self) -> &[super::address::CidrRange] {
        &self.ip_list
    }
    pub fn clear_ip_list(&mut self) {
        self.ip_list.clear();
    }

    // Param is passed by value, moved
    pub fn set_ip_list(&mut self, v: ::protobuf::RepeatedField<super::address::CidrRange>) {
        self.ip_list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ip_list(&mut self) -> &mut ::protobuf::RepeatedField<super::address::CidrRange> {
        &mut self.ip_list
    }

    // Take field
    pub fn take_ip_list(&mut self) -> ::protobuf::RepeatedField<super::address::CidrRange> {
        ::std::mem::replace(&mut self.ip_list, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for IPTagging_IPTag {
    fn is_initialized(&self) -> bool {
        for v in &self.ip_list {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ip_tag_name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ip_list)?;
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
        if !self.ip_tag_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.ip_tag_name);
        }
        for value in &self.ip_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.ip_tag_name.is_empty() {
            os.write_string(1, &self.ip_tag_name)?;
        }
        for v in &self.ip_list {
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

    fn new() -> IPTagging_IPTag {
        IPTagging_IPTag::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ip_tag_name",
                |m: &IPTagging_IPTag| { &m.ip_tag_name },
                |m: &mut IPTagging_IPTag| { &mut m.ip_tag_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::address::CidrRange>>(
                "ip_list",
                |m: &IPTagging_IPTag| { &m.ip_list },
                |m: &mut IPTagging_IPTag| { &mut m.ip_list },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<IPTagging_IPTag>(
                "IPTagging.IPTag",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static IPTagging_IPTag {
        static instance: ::protobuf::rt::LazyV2<IPTagging_IPTag> = ::protobuf::rt::LazyV2::INIT;
        instance.get(IPTagging_IPTag::new)
    }
}

impl ::protobuf::Clear for IPTagging_IPTag {
    fn clear(&mut self) {
        self.ip_tag_name.clear();
        self.ip_list.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IPTagging_IPTag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IPTagging_IPTag {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum IPTagging_RequestType {
    BOTH = 0,
    INTERNAL = 1,
    EXTERNAL = 2,
}

impl ::protobuf::ProtobufEnum for IPTagging_RequestType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<IPTagging_RequestType> {
        match value {
            0 => ::std::option::Option::Some(IPTagging_RequestType::BOTH),
            1 => ::std::option::Option::Some(IPTagging_RequestType::INTERNAL),
            2 => ::std::option::Option::Some(IPTagging_RequestType::EXTERNAL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [IPTagging_RequestType] = &[
            IPTagging_RequestType::BOTH,
            IPTagging_RequestType::INTERNAL,
            IPTagging_RequestType::EXTERNAL,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<IPTagging_RequestType>("IPTagging.RequestType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for IPTagging_RequestType {
}

impl ::std::default::Default for IPTagging_RequestType {
    fn default() -> Self {
        IPTagging_RequestType::BOTH
    }
}

impl ::protobuf::reflect::ProtobufValue for IPTagging_RequestType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n<envoy/extensions/filters/http/ip_tagging/v3/ip_tagging.proto\x12+envo\
    y.extensions.filters.http.ip_tagging.v3\x1a\"envoy/config/core/v3/addres\
    s.proto\x1a\x1dudpa/annotations/status.proto\x1a!udpa/annotations/versio\
    ning.proto\x1a\x17validate/validate.proto\"\xee\x03\n\tIPTagging\x12o\n\
    \x0crequest_type\x18\x01\x20\x01(\x0e2B.envoy.extensions.filters.http.ip\
    _tagging.v3.IPTagging.RequestTypeR\x0brequestTypeB\x08\xfaB\x05\x82\x01\
    \x02\x10\x01\x12_\n\x07ip_tags\x18\x04\x20\x03(\x0b2<.envoy.extensions.f\
    ilters.http.ip_tagging.v3.IPTagging.IPTagR\x06ipTagsB\x08\xfaB\x05\x92\
    \x01\x02\x08\x01\x1a\xa0\x01\n\x05IPTag\x12\x1e\n\x0bip_tag_name\x18\x01\
    \x20\x01(\tR\tipTagName\x128\n\x07ip_list\x18\x02\x20\x03(\x0b2\x1f.envo\
    y.config.core.v3.CidrRangeR\x06ipList:=\x9a\xc5\x88\x1e8\n6envoy.config.\
    filter.http.ip_tagging.v2.IPTagging.IPTag\"3\n\x0bRequestType\x12\x08\n\
    \x04BOTH\x10\0\x12\x0c\n\x08INTERNAL\x10\x01\x12\x0c\n\x08EXTERNAL\x10\
    \x02:7\x9a\xc5\x88\x1e2\n0envoy.config.filter.http.ip_tagging.v2.IPTaggi\
    ngBU\n9io.envoyproxy.envoy.extensions.filters.http.ip_tagging.v3B\x0eIpT\
    aggingProtoP\x01\xba\x80\xc8\xd1\x06\x02\x10\x02b\x06proto3\
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
