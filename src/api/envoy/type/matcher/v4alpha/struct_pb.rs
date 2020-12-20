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
//! Generated file from `envoy/type/matcher/v4alpha/struct.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct StructMatcher {
    // message fields
    pub path: ::protobuf::RepeatedField<StructMatcher_PathSegment>,
    pub value: ::protobuf::SingularPtrField<super::value::ValueMatcher>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a StructMatcher {
    fn default() -> &'a StructMatcher {
        <StructMatcher as ::protobuf::Message>::default_instance()
    }
}

impl StructMatcher {
    pub fn new() -> StructMatcher {
        ::std::default::Default::default()
    }

    // repeated .envoy.type.matcher.v4alpha.StructMatcher.PathSegment path = 2;


    pub fn get_path(&self) -> &[StructMatcher_PathSegment] {
        &self.path
    }
    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::protobuf::RepeatedField<StructMatcher_PathSegment>) {
        self.path = v;
    }

    // Mutable pointer to the field.
    pub fn mut_path(&mut self) -> &mut ::protobuf::RepeatedField<StructMatcher_PathSegment> {
        &mut self.path
    }

    // Take field
    pub fn take_path(&mut self) -> ::protobuf::RepeatedField<StructMatcher_PathSegment> {
        ::std::mem::replace(&mut self.path, ::protobuf::RepeatedField::new())
    }

    // .envoy.type.matcher.v4alpha.ValueMatcher value = 3;


    pub fn get_value(&self) -> &super::value::ValueMatcher {
        self.value.as_ref().unwrap_or_else(|| <super::value::ValueMatcher as ::protobuf::Message>::default_instance())
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: super::value::ValueMatcher) {
        self.value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut super::value::ValueMatcher {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> super::value::ValueMatcher {
        self.value.take().unwrap_or_else(|| super::value::ValueMatcher::new())
    }
}

impl ::protobuf::Message for StructMatcher {
    fn is_initialized(&self) -> bool {
        for v in &self.path {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.value {
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
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.path)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value)?;
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
        for value in &self.path {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.value.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.path {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.value.as_ref() {
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

    fn new() -> StructMatcher {
        StructMatcher::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StructMatcher_PathSegment>>(
                "path",
                |m: &StructMatcher| { &m.path },
                |m: &mut StructMatcher| { &mut m.path },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::value::ValueMatcher>>(
                "value",
                |m: &StructMatcher| { &m.value },
                |m: &mut StructMatcher| { &mut m.value },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<StructMatcher>(
                "StructMatcher",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static StructMatcher {
        static instance: ::protobuf::rt::LazyV2<StructMatcher> = ::protobuf::rt::LazyV2::INIT;
        instance.get(StructMatcher::new)
    }
}

impl ::protobuf::Clear for StructMatcher {
    fn clear(&mut self) {
        self.path.clear();
        self.value.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StructMatcher {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StructMatcher {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StructMatcher_PathSegment {
    // message oneof groups
    pub segment: ::std::option::Option<StructMatcher_PathSegment_oneof_segment>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a StructMatcher_PathSegment {
    fn default() -> &'a StructMatcher_PathSegment {
        <StructMatcher_PathSegment as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum StructMatcher_PathSegment_oneof_segment {
    key(::std::string::String),
}

impl StructMatcher_PathSegment {
    pub fn new() -> StructMatcher_PathSegment {
        ::std::default::Default::default()
    }

    // string key = 1;


    pub fn get_key(&self) -> &str {
        match self.segment {
            ::std::option::Option::Some(StructMatcher_PathSegment_oneof_segment::key(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_key(&mut self) {
        self.segment = ::std::option::Option::None;
    }

    pub fn has_key(&self) -> bool {
        match self.segment {
            ::std::option::Option::Some(StructMatcher_PathSegment_oneof_segment::key(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.segment = ::std::option::Option::Some(StructMatcher_PathSegment_oneof_segment::key(v))
    }

    // Mutable pointer to the field.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(StructMatcher_PathSegment_oneof_segment::key(_)) = self.segment {
        } else {
            self.segment = ::std::option::Option::Some(StructMatcher_PathSegment_oneof_segment::key(::std::string::String::new()));
        }
        match self.segment {
            ::std::option::Option::Some(StructMatcher_PathSegment_oneof_segment::key(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        if self.has_key() {
            match self.segment.take() {
                ::std::option::Option::Some(StructMatcher_PathSegment_oneof_segment::key(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }
}

impl ::protobuf::Message for StructMatcher_PathSegment {
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
                    self.segment = ::std::option::Option::Some(StructMatcher_PathSegment_oneof_segment::key(is.read_string()?));
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
        if let ::std::option::Option::Some(ref v) = self.segment {
            match v {
                &StructMatcher_PathSegment_oneof_segment::key(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.segment {
            match v {
                &StructMatcher_PathSegment_oneof_segment::key(ref v) => {
                    os.write_string(1, v)?;
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

    fn new() -> StructMatcher_PathSegment {
        StructMatcher_PathSegment::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "key",
                StructMatcher_PathSegment::has_key,
                StructMatcher_PathSegment::get_key,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<StructMatcher_PathSegment>(
                "StructMatcher.PathSegment",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static StructMatcher_PathSegment {
        static instance: ::protobuf::rt::LazyV2<StructMatcher_PathSegment> = ::protobuf::rt::LazyV2::INIT;
        instance.get(StructMatcher_PathSegment::new)
    }
}

impl ::protobuf::Clear for StructMatcher_PathSegment {
    fn clear(&mut self) {
        self.segment = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StructMatcher_PathSegment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StructMatcher_PathSegment {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'envoy/type/matcher/v4alpha/struct.proto\x12\x1aenvoy.type.matcher.v4a\
    lpha\x1a&envoy/type/matcher/v4alpha/value.proto\x1a\x1dudpa/annotations/\
    status.proto\x1a!udpa/annotations/versioning.proto\x1a\x17validate/valid\
    ate.proto\"\xce\x02\n\rStructMatcher\x12S\n\x04path\x18\x02\x20\x03(\x0b\
    25.envoy.type.matcher.v4alpha.StructMatcher.PathSegmentR\x04pathB\x08\
    \xfaB\x05\x92\x01\x02\x08\x01\x12H\n\x05value\x18\x03\x20\x01(\x0b2(.env\
    oy.type.matcher.v4alpha.ValueMatcherR\x05valueB\x08\xfaB\x05\x8a\x01\x02\
    \x10\x01\x1ar\n\x0bPathSegment\x12\x1b\n\x03key\x18\x01\x20\x01(\tH\0R\
    \x03keyB\x07\xfaB\x04r\x02\x10\x01B\x0e\n\x07segment\x12\x03\xf8B\x01:6\
    \x9a\xc5\x88\x1e1\n/envoy.type.matcher.v3.StructMatcher.PathSegment:*\
    \x9a\xc5\x88\x1e%\n#envoy.type.matcher.v3.StructMatcherBA\n(io.envoyprox\
    y.envoy.type.matcher.v4alphaB\x0bStructProtoP\x01\xba\x80\xc8\xd1\x06\
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
