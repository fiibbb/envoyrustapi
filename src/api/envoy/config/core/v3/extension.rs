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
//! Generated file from `envoy/config/core/v3/extension.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct TypedExtensionConfig {
    // message fields
    pub name: ::std::string::String,
    pub typed_config: ::protobuf::SingularPtrField<::protobuf::well_known_types::Any>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TypedExtensionConfig {
    fn default() -> &'a TypedExtensionConfig {
        <TypedExtensionConfig as ::protobuf::Message>::default_instance()
    }
}

impl TypedExtensionConfig {
    pub fn new() -> TypedExtensionConfig {
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

    // .google.protobuf.Any typed_config = 2;


    pub fn get_typed_config(&self) -> &::protobuf::well_known_types::Any {
        self.typed_config.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Any as ::protobuf::Message>::default_instance())
    }
    pub fn clear_typed_config(&mut self) {
        self.typed_config.clear();
    }

    pub fn has_typed_config(&self) -> bool {
        self.typed_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_typed_config(&mut self, v: ::protobuf::well_known_types::Any) {
        self.typed_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_typed_config(&mut self) -> &mut ::protobuf::well_known_types::Any {
        if self.typed_config.is_none() {
            self.typed_config.set_default();
        }
        self.typed_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_typed_config(&mut self) -> ::protobuf::well_known_types::Any {
        self.typed_config.take().unwrap_or_else(|| ::protobuf::well_known_types::Any::new())
    }
}

impl ::protobuf::Message for TypedExtensionConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.typed_config {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.typed_config)?;
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
        if let Some(ref v) = self.typed_config.as_ref() {
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
        if let Some(ref v) = self.typed_config.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> TypedExtensionConfig {
        TypedExtensionConfig::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &TypedExtensionConfig| { &m.name },
                |m: &mut TypedExtensionConfig| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Any>>(
                "typed_config",
                |m: &TypedExtensionConfig| { &m.typed_config },
                |m: &mut TypedExtensionConfig| { &mut m.typed_config },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TypedExtensionConfig>(
                "TypedExtensionConfig",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TypedExtensionConfig {
        static instance: ::protobuf::rt::LazyV2<TypedExtensionConfig> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TypedExtensionConfig::new)
    }
}

impl ::protobuf::Clear for TypedExtensionConfig {
    fn clear(&mut self) {
        self.name.clear();
        self.typed_config.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TypedExtensionConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TypedExtensionConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ExtensionConfigSource {
    // message fields
    pub config_source: ::protobuf::SingularPtrField<super::config_source::ConfigSource>,
    pub default_config: ::protobuf::SingularPtrField<::protobuf::well_known_types::Any>,
    pub apply_default_config_without_warming: bool,
    pub type_urls: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ExtensionConfigSource {
    fn default() -> &'a ExtensionConfigSource {
        <ExtensionConfigSource as ::protobuf::Message>::default_instance()
    }
}

impl ExtensionConfigSource {
    pub fn new() -> ExtensionConfigSource {
        ::std::default::Default::default()
    }

    // .envoy.config.core.v3.ConfigSource config_source = 1;


    pub fn get_config_source(&self) -> &super::config_source::ConfigSource {
        self.config_source.as_ref().unwrap_or_else(|| <super::config_source::ConfigSource as ::protobuf::Message>::default_instance())
    }
    pub fn clear_config_source(&mut self) {
        self.config_source.clear();
    }

    pub fn has_config_source(&self) -> bool {
        self.config_source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_config_source(&mut self, v: super::config_source::ConfigSource) {
        self.config_source = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config_source(&mut self) -> &mut super::config_source::ConfigSource {
        if self.config_source.is_none() {
            self.config_source.set_default();
        }
        self.config_source.as_mut().unwrap()
    }

    // Take field
    pub fn take_config_source(&mut self) -> super::config_source::ConfigSource {
        self.config_source.take().unwrap_or_else(|| super::config_source::ConfigSource::new())
    }

    // .google.protobuf.Any default_config = 2;


    pub fn get_default_config(&self) -> &::protobuf::well_known_types::Any {
        self.default_config.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Any as ::protobuf::Message>::default_instance())
    }
    pub fn clear_default_config(&mut self) {
        self.default_config.clear();
    }

    pub fn has_default_config(&self) -> bool {
        self.default_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_default_config(&mut self, v: ::protobuf::well_known_types::Any) {
        self.default_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_default_config(&mut self) -> &mut ::protobuf::well_known_types::Any {
        if self.default_config.is_none() {
            self.default_config.set_default();
        }
        self.default_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_default_config(&mut self) -> ::protobuf::well_known_types::Any {
        self.default_config.take().unwrap_or_else(|| ::protobuf::well_known_types::Any::new())
    }

    // bool apply_default_config_without_warming = 3;


    pub fn get_apply_default_config_without_warming(&self) -> bool {
        self.apply_default_config_without_warming
    }
    pub fn clear_apply_default_config_without_warming(&mut self) {
        self.apply_default_config_without_warming = false;
    }

    // Param is passed by value, moved
    pub fn set_apply_default_config_without_warming(&mut self, v: bool) {
        self.apply_default_config_without_warming = v;
    }

    // repeated string type_urls = 4;


    pub fn get_type_urls(&self) -> &[::std::string::String] {
        &self.type_urls
    }
    pub fn clear_type_urls(&mut self) {
        self.type_urls.clear();
    }

    // Param is passed by value, moved
    pub fn set_type_urls(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.type_urls = v;
    }

    // Mutable pointer to the field.
    pub fn mut_type_urls(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.type_urls
    }

    // Take field
    pub fn take_type_urls(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.type_urls, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for ExtensionConfigSource {
    fn is_initialized(&self) -> bool {
        for v in &self.config_source {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.default_config {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.config_source)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.default_config)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.apply_default_config_without_warming = tmp;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.type_urls)?;
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
        if let Some(ref v) = self.config_source.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.default_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.apply_default_config_without_warming != false {
            my_size += 2;
        }
        for value in &self.type_urls {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.config_source.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.default_config.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.apply_default_config_without_warming != false {
            os.write_bool(3, self.apply_default_config_without_warming)?;
        }
        for v in &self.type_urls {
            os.write_string(4, &v)?;
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

    fn new() -> ExtensionConfigSource {
        ExtensionConfigSource::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::config_source::ConfigSource>>(
                "config_source",
                |m: &ExtensionConfigSource| { &m.config_source },
                |m: &mut ExtensionConfigSource| { &mut m.config_source },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Any>>(
                "default_config",
                |m: &ExtensionConfigSource| { &m.default_config },
                |m: &mut ExtensionConfigSource| { &mut m.default_config },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "apply_default_config_without_warming",
                |m: &ExtensionConfigSource| { &m.apply_default_config_without_warming },
                |m: &mut ExtensionConfigSource| { &mut m.apply_default_config_without_warming },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "type_urls",
                |m: &ExtensionConfigSource| { &m.type_urls },
                |m: &mut ExtensionConfigSource| { &mut m.type_urls },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ExtensionConfigSource>(
                "ExtensionConfigSource",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ExtensionConfigSource {
        static instance: ::protobuf::rt::LazyV2<ExtensionConfigSource> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ExtensionConfigSource::new)
    }
}

impl ::protobuf::Clear for ExtensionConfigSource {
    fn clear(&mut self) {
        self.config_source.clear();
        self.default_config.clear();
        self.apply_default_config_without_warming = false;
        self.type_urls.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExtensionConfigSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExtensionConfigSource {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n$envoy/config/core/v3/extension.proto\x12\x14envoy.config.core.v3\x1a(\
    envoy/config/core/v3/config_source.proto\x1a\x19google/protobuf/any.prot\
    o\x1a\x1dudpa/annotations/status.proto\x1a\x17validate/validate.proto\"v\
    \n\x14TypedExtensionConfig\x12\x1b\n\x04name\x18\x01\x20\x01(\tR\x04name\
    B\x07\xfaB\x04r\x02\x10\x01\x12A\n\x0ctyped_config\x18\x02\x20\x01(\x0b2\
    \x14.google.protobuf.AnyR\x0btypedConfigB\x08\xfaB\x05\xa2\x01\x02\x08\
    \x01\"\x9e\x02\n\x15ExtensionConfigSource\x12Q\n\rconfig_source\x18\x01\
    \x20\x01(\x0b2\".envoy.config.core.v3.ConfigSourceR\x0cconfigSourceB\x08\
    \xfaB\x05\xa2\x01\x02\x08\x01\x12;\n\x0edefault_config\x18\x02\x20\x01(\
    \x0b2\x14.google.protobuf.AnyR\rdefaultConfig\x12N\n$apply_default_confi\
    g_without_warming\x18\x03\x20\x01(\x08R\x20applyDefaultConfigWithoutWarm\
    ing\x12%\n\ttype_urls\x18\x04\x20\x03(\tR\x08typeUrlsB\x08\xfaB\x05\x92\
    \x01\x02\x08\x01B>\n\"io.envoyproxy.envoy.config.core.v3B\x0eExtensionPr\
    otoP\x01\xba\x80\xc8\xd1\x06\x02\x10\x02b\x06proto3\
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
