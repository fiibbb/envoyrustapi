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
//! Generated file from `xds/core/v3/collection_entry.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct CollectionEntry {
    // message oneof groups
    pub resource_specifier: ::std::option::Option<CollectionEntry_oneof_resource_specifier>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CollectionEntry {
    fn default() -> &'a CollectionEntry {
        <CollectionEntry as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum CollectionEntry_oneof_resource_specifier {
    locator(super::resource_locator::ResourceLocator),
    inline_entry(CollectionEntry_InlineEntry),
}

impl CollectionEntry {
    pub fn new() -> CollectionEntry {
        ::std::default::Default::default()
    }

    // .xds.core.v3.ResourceLocator locator = 1;


    pub fn get_locator(&self) -> &super::resource_locator::ResourceLocator {
        match self.resource_specifier {
            ::std::option::Option::Some(CollectionEntry_oneof_resource_specifier::locator(ref v)) => v,
            _ => <super::resource_locator::ResourceLocator as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_locator(&mut self) {
        self.resource_specifier = ::std::option::Option::None;
    }

    pub fn has_locator(&self) -> bool {
        match self.resource_specifier {
            ::std::option::Option::Some(CollectionEntry_oneof_resource_specifier::locator(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_locator(&mut self, v: super::resource_locator::ResourceLocator) {
        self.resource_specifier = ::std::option::Option::Some(CollectionEntry_oneof_resource_specifier::locator(v))
    }

    // Mutable pointer to the field.
    pub fn mut_locator(&mut self) -> &mut super::resource_locator::ResourceLocator {
        if let ::std::option::Option::Some(CollectionEntry_oneof_resource_specifier::locator(_)) = self.resource_specifier {
        } else {
            self.resource_specifier = ::std::option::Option::Some(CollectionEntry_oneof_resource_specifier::locator(super::resource_locator::ResourceLocator::new()));
        }
        match self.resource_specifier {
            ::std::option::Option::Some(CollectionEntry_oneof_resource_specifier::locator(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_locator(&mut self) -> super::resource_locator::ResourceLocator {
        if self.has_locator() {
            match self.resource_specifier.take() {
                ::std::option::Option::Some(CollectionEntry_oneof_resource_specifier::locator(v)) => v,
                _ => panic!(),
            }
        } else {
            super::resource_locator::ResourceLocator::new()
        }
    }

    // .xds.core.v3.CollectionEntry.InlineEntry inline_entry = 2;


    pub fn get_inline_entry(&self) -> &CollectionEntry_InlineEntry {
        match self.resource_specifier {
            ::std::option::Option::Some(CollectionEntry_oneof_resource_specifier::inline_entry(ref v)) => v,
            _ => <CollectionEntry_InlineEntry as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_inline_entry(&mut self) {
        self.resource_specifier = ::std::option::Option::None;
    }

    pub fn has_inline_entry(&self) -> bool {
        match self.resource_specifier {
            ::std::option::Option::Some(CollectionEntry_oneof_resource_specifier::inline_entry(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_inline_entry(&mut self, v: CollectionEntry_InlineEntry) {
        self.resource_specifier = ::std::option::Option::Some(CollectionEntry_oneof_resource_specifier::inline_entry(v))
    }

    // Mutable pointer to the field.
    pub fn mut_inline_entry(&mut self) -> &mut CollectionEntry_InlineEntry {
        if let ::std::option::Option::Some(CollectionEntry_oneof_resource_specifier::inline_entry(_)) = self.resource_specifier {
        } else {
            self.resource_specifier = ::std::option::Option::Some(CollectionEntry_oneof_resource_specifier::inline_entry(CollectionEntry_InlineEntry::new()));
        }
        match self.resource_specifier {
            ::std::option::Option::Some(CollectionEntry_oneof_resource_specifier::inline_entry(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_inline_entry(&mut self) -> CollectionEntry_InlineEntry {
        if self.has_inline_entry() {
            match self.resource_specifier.take() {
                ::std::option::Option::Some(CollectionEntry_oneof_resource_specifier::inline_entry(v)) => v,
                _ => panic!(),
            }
        } else {
            CollectionEntry_InlineEntry::new()
        }
    }
}

impl ::protobuf::Message for CollectionEntry {
    fn is_initialized(&self) -> bool {
        if let Some(CollectionEntry_oneof_resource_specifier::locator(ref v)) = self.resource_specifier {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(CollectionEntry_oneof_resource_specifier::inline_entry(ref v)) = self.resource_specifier {
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.resource_specifier = ::std::option::Option::Some(CollectionEntry_oneof_resource_specifier::locator(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.resource_specifier = ::std::option::Option::Some(CollectionEntry_oneof_resource_specifier::inline_entry(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.resource_specifier {
            match v {
                &CollectionEntry_oneof_resource_specifier::locator(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &CollectionEntry_oneof_resource_specifier::inline_entry(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.resource_specifier {
            match v {
                &CollectionEntry_oneof_resource_specifier::locator(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &CollectionEntry_oneof_resource_specifier::inline_entry(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> CollectionEntry {
        CollectionEntry::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::resource_locator::ResourceLocator>(
                "locator",
                CollectionEntry::has_locator,
                CollectionEntry::get_locator,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CollectionEntry_InlineEntry>(
                "inline_entry",
                CollectionEntry::has_inline_entry,
                CollectionEntry::get_inline_entry,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CollectionEntry>(
                "CollectionEntry",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CollectionEntry {
        static instance: ::protobuf::rt::LazyV2<CollectionEntry> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CollectionEntry::new)
    }
}

impl ::protobuf::Clear for CollectionEntry {
    fn clear(&mut self) {
        self.resource_specifier = ::std::option::Option::None;
        self.resource_specifier = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CollectionEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CollectionEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CollectionEntry_InlineEntry {
    // message fields
    pub name: ::std::string::String,
    pub version: ::std::string::String,
    pub resource: ::protobuf::SingularPtrField<::protobuf::well_known_types::Any>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CollectionEntry_InlineEntry {
    fn default() -> &'a CollectionEntry_InlineEntry {
        <CollectionEntry_InlineEntry as ::protobuf::Message>::default_instance()
    }
}

impl CollectionEntry_InlineEntry {
    pub fn new() -> CollectionEntry_InlineEntry {
        ::std::default::Default::default()
    }

    // string name = 1;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // string version = 2;


    pub fn get_version(&self) -> &str {
        &self.version
    }
    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.version, ::std::string::String::new())
    }

    // .google.protobuf.Any resource = 3;


    pub fn get_resource(&self) -> &::protobuf::well_known_types::Any {
        self.resource.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Any as ::protobuf::Message>::default_instance())
    }
    pub fn clear_resource(&mut self) {
        self.resource.clear();
    }

    pub fn has_resource(&self) -> bool {
        self.resource.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resource(&mut self, v: ::protobuf::well_known_types::Any) {
        self.resource = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resource(&mut self) -> &mut ::protobuf::well_known_types::Any {
        if self.resource.is_none() {
            self.resource.set_default();
        }
        self.resource.as_mut().unwrap()
    }

    // Take field
    pub fn take_resource(&mut self) -> ::protobuf::well_known_types::Any {
        self.resource.take().unwrap_or_else(|| ::protobuf::well_known_types::Any::new())
    }
}

impl ::protobuf::Message for CollectionEntry_InlineEntry {
    fn is_initialized(&self) -> bool {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.version)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.version.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.version);
        }
        if let Some(ref v) = self.resource.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.version.is_empty() {
            os.write_string(2, &self.version)?;
        }
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

    fn new() -> CollectionEntry_InlineEntry {
        CollectionEntry_InlineEntry::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &CollectionEntry_InlineEntry| { &m.name },
                |m: &mut CollectionEntry_InlineEntry| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "version",
                |m: &CollectionEntry_InlineEntry| { &m.version },
                |m: &mut CollectionEntry_InlineEntry| { &mut m.version },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Any>>(
                "resource",
                |m: &CollectionEntry_InlineEntry| { &m.resource },
                |m: &mut CollectionEntry_InlineEntry| { &mut m.resource },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CollectionEntry_InlineEntry>(
                "CollectionEntry.InlineEntry",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CollectionEntry_InlineEntry {
        static instance: ::protobuf::rt::LazyV2<CollectionEntry_InlineEntry> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CollectionEntry_InlineEntry::new)
    }
}

impl ::protobuf::Clear for CollectionEntry_InlineEntry {
    fn clear(&mut self) {
        self.name.clear();
        self.version.clear();
        self.resource.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CollectionEntry_InlineEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CollectionEntry_InlineEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"xds/core/v3/collection_entry.proto\x12\x0bxds.core.v3\x1a\x19google/\
    protobuf/any.proto\x1a\x1dudpa/annotations/status.proto\x1a\"xds/core/v3\
    /resource_locator.proto\x1a\x17validate/validate.proto\"\xc3\x02\n\x0fCo\
    llectionEntry\x128\n\x07locator\x18\x01\x20\x01(\x0b2\x1c.xds.core.v3.Re\
    sourceLocatorH\0R\x07locator\x12M\n\x0cinline_entry\x18\x02\x20\x01(\x0b\
    2(.xds.core.v3.CollectionEntry.InlineEntryH\0R\x0binlineEntry\x1a\x8b\
    \x01\n\x0bInlineEntry\x120\n\x04name\x18\x01\x20\x01(\tR\x04nameB\x1c\
    \xfaB\x19r\x172\x15^[0-9a-zA-Z_\\-\\.~:]+$\x12\x18\n\x07version\x18\x02\
    \x20\x01(\tR\x07version\x120\n\x08resource\x18\x03\x20\x01(\x0b2\x14.goo\
    gle.protobuf.AnyR\x08resourceB\x19\n\x12resource_specifier\x12\x03\xf8B\
    \x01B=\n\x1bcom.github.udpa.xds.core.v3B\x14CollectionEntryProtoP\x01\
    \xba\x80\xc8\xd1\x06\x02\x08\x01b\x06proto3\
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