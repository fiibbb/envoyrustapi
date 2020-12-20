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
//! Generated file from `envoy/config/filter/http/gzip/v2/gzip.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct Gzip {
    // message fields
    pub memory_level: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub content_length: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub compression_level: Gzip_CompressionLevel_Enum,
    pub compression_strategy: Gzip_CompressionStrategy,
    pub content_type: ::protobuf::RepeatedField<::std::string::String>,
    pub disable_on_etag_header: bool,
    pub remove_accept_encoding_header: bool,
    pub window_bits: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub compressor: ::protobuf::SingularPtrField<super::compressor::Compressor>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Gzip {
    fn default() -> &'a Gzip {
        <Gzip as ::protobuf::Message>::default_instance()
    }
}

impl Gzip {
    pub fn new() -> Gzip {
        ::std::default::Default::default()
    }

    // .google.protobuf.UInt32Value memory_level = 1;


    pub fn get_memory_level(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.memory_level.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::UInt32Value as ::protobuf::Message>::default_instance())
    }
    pub fn clear_memory_level(&mut self) {
        self.memory_level.clear();
    }

    pub fn has_memory_level(&self) -> bool {
        self.memory_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_memory_level(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.memory_level = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_memory_level(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.memory_level.is_none() {
            self.memory_level.set_default();
        }
        self.memory_level.as_mut().unwrap()
    }

    // Take field
    pub fn take_memory_level(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.memory_level.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    // .google.protobuf.UInt32Value content_length = 2;


    pub fn get_content_length(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.content_length.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::UInt32Value as ::protobuf::Message>::default_instance())
    }
    pub fn clear_content_length(&mut self) {
        self.content_length.clear();
    }

    pub fn has_content_length(&self) -> bool {
        self.content_length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content_length(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.content_length = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content_length(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.content_length.is_none() {
            self.content_length.set_default();
        }
        self.content_length.as_mut().unwrap()
    }

    // Take field
    pub fn take_content_length(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.content_length.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    // .envoy.config.filter.http.gzip.v2.Gzip.CompressionLevel.Enum compression_level = 3;


    pub fn get_compression_level(&self) -> Gzip_CompressionLevel_Enum {
        self.compression_level
    }
    pub fn clear_compression_level(&mut self) {
        self.compression_level = Gzip_CompressionLevel_Enum::DEFAULT;
    }

    // Param is passed by value, moved
    pub fn set_compression_level(&mut self, v: Gzip_CompressionLevel_Enum) {
        self.compression_level = v;
    }

    // .envoy.config.filter.http.gzip.v2.Gzip.CompressionStrategy compression_strategy = 4;


    pub fn get_compression_strategy(&self) -> Gzip_CompressionStrategy {
        self.compression_strategy
    }
    pub fn clear_compression_strategy(&mut self) {
        self.compression_strategy = Gzip_CompressionStrategy::DEFAULT;
    }

    // Param is passed by value, moved
    pub fn set_compression_strategy(&mut self, v: Gzip_CompressionStrategy) {
        self.compression_strategy = v;
    }

    // repeated string content_type = 6;


    pub fn get_content_type(&self) -> &[::std::string::String] {
        &self.content_type
    }
    pub fn clear_content_type(&mut self) {
        self.content_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_content_type(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.content_type = v;
    }

    // Mutable pointer to the field.
    pub fn mut_content_type(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.content_type
    }

    // Take field
    pub fn take_content_type(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.content_type, ::protobuf::RepeatedField::new())
    }

    // bool disable_on_etag_header = 7;


    pub fn get_disable_on_etag_header(&self) -> bool {
        self.disable_on_etag_header
    }
    pub fn clear_disable_on_etag_header(&mut self) {
        self.disable_on_etag_header = false;
    }

    // Param is passed by value, moved
    pub fn set_disable_on_etag_header(&mut self, v: bool) {
        self.disable_on_etag_header = v;
    }

    // bool remove_accept_encoding_header = 8;


    pub fn get_remove_accept_encoding_header(&self) -> bool {
        self.remove_accept_encoding_header
    }
    pub fn clear_remove_accept_encoding_header(&mut self) {
        self.remove_accept_encoding_header = false;
    }

    // Param is passed by value, moved
    pub fn set_remove_accept_encoding_header(&mut self, v: bool) {
        self.remove_accept_encoding_header = v;
    }

    // .google.protobuf.UInt32Value window_bits = 9;


    pub fn get_window_bits(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.window_bits.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::UInt32Value as ::protobuf::Message>::default_instance())
    }
    pub fn clear_window_bits(&mut self) {
        self.window_bits.clear();
    }

    pub fn has_window_bits(&self) -> bool {
        self.window_bits.is_some()
    }

    // Param is passed by value, moved
    pub fn set_window_bits(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.window_bits = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_window_bits(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.window_bits.is_none() {
            self.window_bits.set_default();
        }
        self.window_bits.as_mut().unwrap()
    }

    // Take field
    pub fn take_window_bits(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.window_bits.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    // .envoy.config.filter.http.compressor.v2.Compressor compressor = 10;


    pub fn get_compressor(&self) -> &super::compressor::Compressor {
        self.compressor.as_ref().unwrap_or_else(|| <super::compressor::Compressor as ::protobuf::Message>::default_instance())
    }
    pub fn clear_compressor(&mut self) {
        self.compressor.clear();
    }

    pub fn has_compressor(&self) -> bool {
        self.compressor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_compressor(&mut self, v: super::compressor::Compressor) {
        self.compressor = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_compressor(&mut self) -> &mut super::compressor::Compressor {
        if self.compressor.is_none() {
            self.compressor.set_default();
        }
        self.compressor.as_mut().unwrap()
    }

    // Take field
    pub fn take_compressor(&mut self) -> super::compressor::Compressor {
        self.compressor.take().unwrap_or_else(|| super::compressor::Compressor::new())
    }
}

impl ::protobuf::Message for Gzip {
    fn is_initialized(&self) -> bool {
        for v in &self.memory_level {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.content_length {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.window_bits {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.compressor {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.memory_level)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.content_length)?;
                },
                3 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.compression_level, 3, &mut self.unknown_fields)?
                },
                4 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.compression_strategy, 4, &mut self.unknown_fields)?
                },
                6 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.content_type)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.disable_on_etag_header = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.remove_accept_encoding_header = tmp;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.window_bits)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.compressor)?;
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
        if let Some(ref v) = self.memory_level.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.content_length.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.compression_level != Gzip_CompressionLevel_Enum::DEFAULT {
            my_size += ::protobuf::rt::enum_size(3, self.compression_level);
        }
        if self.compression_strategy != Gzip_CompressionStrategy::DEFAULT {
            my_size += ::protobuf::rt::enum_size(4, self.compression_strategy);
        }
        for value in &self.content_type {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        if self.disable_on_etag_header != false {
            my_size += 2;
        }
        if self.remove_accept_encoding_header != false {
            my_size += 2;
        }
        if let Some(ref v) = self.window_bits.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.compressor.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.memory_level.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.content_length.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.compression_level != Gzip_CompressionLevel_Enum::DEFAULT {
            os.write_enum(3, ::protobuf::ProtobufEnum::value(&self.compression_level))?;
        }
        if self.compression_strategy != Gzip_CompressionStrategy::DEFAULT {
            os.write_enum(4, ::protobuf::ProtobufEnum::value(&self.compression_strategy))?;
        }
        for v in &self.content_type {
            os.write_string(6, &v)?;
        };
        if self.disable_on_etag_header != false {
            os.write_bool(7, self.disable_on_etag_header)?;
        }
        if self.remove_accept_encoding_header != false {
            os.write_bool(8, self.remove_accept_encoding_header)?;
        }
        if let Some(ref v) = self.window_bits.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.compressor.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> Gzip {
        Gzip::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                "memory_level",
                |m: &Gzip| { &m.memory_level },
                |m: &mut Gzip| { &mut m.memory_level },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                "content_length",
                |m: &Gzip| { &m.content_length },
                |m: &mut Gzip| { &mut m.content_length },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Gzip_CompressionLevel_Enum>>(
                "compression_level",
                |m: &Gzip| { &m.compression_level },
                |m: &mut Gzip| { &mut m.compression_level },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Gzip_CompressionStrategy>>(
                "compression_strategy",
                |m: &Gzip| { &m.compression_strategy },
                |m: &mut Gzip| { &mut m.compression_strategy },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "content_type",
                |m: &Gzip| { &m.content_type },
                |m: &mut Gzip| { &mut m.content_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "disable_on_etag_header",
                |m: &Gzip| { &m.disable_on_etag_header },
                |m: &mut Gzip| { &mut m.disable_on_etag_header },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "remove_accept_encoding_header",
                |m: &Gzip| { &m.remove_accept_encoding_header },
                |m: &mut Gzip| { &mut m.remove_accept_encoding_header },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                "window_bits",
                |m: &Gzip| { &m.window_bits },
                |m: &mut Gzip| { &mut m.window_bits },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::compressor::Compressor>>(
                "compressor",
                |m: &Gzip| { &m.compressor },
                |m: &mut Gzip| { &mut m.compressor },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Gzip>(
                "Gzip",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Gzip {
        static instance: ::protobuf::rt::LazyV2<Gzip> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Gzip::new)
    }
}

impl ::protobuf::Clear for Gzip {
    fn clear(&mut self) {
        self.memory_level.clear();
        self.content_length.clear();
        self.compression_level = Gzip_CompressionLevel_Enum::DEFAULT;
        self.compression_strategy = Gzip_CompressionStrategy::DEFAULT;
        self.content_type.clear();
        self.disable_on_etag_header = false;
        self.remove_accept_encoding_header = false;
        self.window_bits.clear();
        self.compressor.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Gzip {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Gzip {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Gzip_CompressionLevel {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Gzip_CompressionLevel {
    fn default() -> &'a Gzip_CompressionLevel {
        <Gzip_CompressionLevel as ::protobuf::Message>::default_instance()
    }
}

impl Gzip_CompressionLevel {
    pub fn new() -> Gzip_CompressionLevel {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for Gzip_CompressionLevel {
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

    fn new() -> Gzip_CompressionLevel {
        Gzip_CompressionLevel::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let fields = ::std::vec::Vec::new();
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Gzip_CompressionLevel>(
                "Gzip.CompressionLevel",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Gzip_CompressionLevel {
        static instance: ::protobuf::rt::LazyV2<Gzip_CompressionLevel> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Gzip_CompressionLevel::new)
    }
}

impl ::protobuf::Clear for Gzip_CompressionLevel {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Gzip_CompressionLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Gzip_CompressionLevel {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Gzip_CompressionLevel_Enum {
    DEFAULT = 0,
    BEST = 1,
    SPEED = 2,
}

impl ::protobuf::ProtobufEnum for Gzip_CompressionLevel_Enum {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Gzip_CompressionLevel_Enum> {
        match value {
            0 => ::std::option::Option::Some(Gzip_CompressionLevel_Enum::DEFAULT),
            1 => ::std::option::Option::Some(Gzip_CompressionLevel_Enum::BEST),
            2 => ::std::option::Option::Some(Gzip_CompressionLevel_Enum::SPEED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Gzip_CompressionLevel_Enum] = &[
            Gzip_CompressionLevel_Enum::DEFAULT,
            Gzip_CompressionLevel_Enum::BEST,
            Gzip_CompressionLevel_Enum::SPEED,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<Gzip_CompressionLevel_Enum>("Gzip.CompressionLevel.Enum", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for Gzip_CompressionLevel_Enum {
}

impl ::std::default::Default for Gzip_CompressionLevel_Enum {
    fn default() -> Self {
        Gzip_CompressionLevel_Enum::DEFAULT
    }
}

impl ::protobuf::reflect::ProtobufValue for Gzip_CompressionLevel_Enum {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Gzip_CompressionStrategy {
    DEFAULT = 0,
    FILTERED = 1,
    HUFFMAN = 2,
    RLE = 3,
}

impl ::protobuf::ProtobufEnum for Gzip_CompressionStrategy {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Gzip_CompressionStrategy> {
        match value {
            0 => ::std::option::Option::Some(Gzip_CompressionStrategy::DEFAULT),
            1 => ::std::option::Option::Some(Gzip_CompressionStrategy::FILTERED),
            2 => ::std::option::Option::Some(Gzip_CompressionStrategy::HUFFMAN),
            3 => ::std::option::Option::Some(Gzip_CompressionStrategy::RLE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Gzip_CompressionStrategy] = &[
            Gzip_CompressionStrategy::DEFAULT,
            Gzip_CompressionStrategy::FILTERED,
            Gzip_CompressionStrategy::HUFFMAN,
            Gzip_CompressionStrategy::RLE,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<Gzip_CompressionStrategy>("Gzip.CompressionStrategy", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for Gzip_CompressionStrategy {
}

impl ::std::default::Default for Gzip_CompressionStrategy {
    fn default() -> Self {
        Gzip_CompressionStrategy::DEFAULT
    }
}

impl ::protobuf::reflect::ProtobufValue for Gzip_CompressionStrategy {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n+envoy/config/filter/http/gzip/v2/gzip.proto\x12\x20envoy.config.filte\
    r.http.gzip.v2\x1a7envoy/config/filter/http/compressor/v2/compressor.pro\
    to\x1a\x1egoogle/protobuf/wrappers.proto\x1a\x1eudpa/annotations/migrate\
    .proto\x1a\x1dudpa/annotations/status.proto\x1a\x17validate/validate.pro\
    to\"\xd4\x06\n\x04Gzip\x12J\n\x0cmemory_level\x18\x01\x20\x01(\x0b2\x1c.\
    google.protobuf.UInt32ValueR\x0bmemoryLevelB\t\xfaB\x06*\x04\x18\t(\x01\
    \x12G\n\x0econtent_length\x18\x02\x20\x01(\x0b2\x1c.google.protobuf.UInt\
    32ValueR\rcontentLengthB\x02\x18\x01\x12s\n\x11compression_level\x18\x03\
    \x20\x01(\x0e2<.envoy.config.filter.http.gzip.v2.Gzip.CompressionLevel.E\
    numR\x10compressionLevelB\x08\xfaB\x05\x82\x01\x02\x10\x01\x12w\n\x14com\
    pression_strategy\x18\x04\x20\x01(\x0e2:.envoy.config.filter.http.gzip.v\
    2.Gzip.CompressionStrategyR\x13compressionStrategyB\x08\xfaB\x05\x82\x01\
    \x02\x10\x01\x12%\n\x0ccontent_type\x18\x06\x20\x03(\tR\x0bcontentTypeB\
    \x02\x18\x01\x127\n\x16disable_on_etag_header\x18\x07\x20\x01(\x08R\x13d\
    isableOnEtagHeaderB\x02\x18\x01\x12E\n\x1dremove_accept_encoding_header\
    \x18\x08\x20\x01(\x08R\x1aremoveAcceptEncodingHeaderB\x02\x18\x01\x12H\n\
    \x0bwindow_bits\x18\t\x20\x01(\x0b2\x1c.google.protobuf.UInt32ValueR\nwi\
    ndowBitsB\t\xfaB\x06*\x04\x18\x0f(\t\x12R\n\ncompressor\x18\n\x20\x01(\
    \x0b22.envoy.config.filter.http.compressor.v2.CompressorR\ncompressor\
    \x1a<\n\x10CompressionLevel\"(\n\x04Enum\x12\x0b\n\x07DEFAULT\x10\0\x12\
    \x08\n\x04BEST\x10\x01\x12\t\n\x05SPEED\x10\x02\"F\n\x13CompressionStrat\
    egy\x12\x0b\n\x07DEFAULT\x10\0\x12\x0c\n\x08FILTERED\x10\x01\x12\x0b\n\
    \x07HUFFMAN\x10\x02\x12\x07\n\x03RLE\x10\x03Br\n.io.envoyproxy.envoy.con\
    fig.filter.http.gzip.v2B\tGzipProtoP\x01\xf2\x98\xfe\x8f\x05'\x12%envoy.\
    extensions.filters.http.gzip.v3\xba\x80\xc8\xd1\x06\x02\x10\x01b\x06prot\
    o3\
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
