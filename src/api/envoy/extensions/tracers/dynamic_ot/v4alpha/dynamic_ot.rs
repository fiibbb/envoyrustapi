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
//! Generated file from `envoy/extensions/tracers/dynamic_ot/v4alpha/dynamic_ot.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct DynamicOtConfig {
    // message fields
    pub library: ::std::string::String,
    pub config: ::protobuf::SingularPtrField<::protobuf::well_known_types::Struct>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DynamicOtConfig {
    fn default() -> &'a DynamicOtConfig {
        <DynamicOtConfig as ::protobuf::Message>::default_instance()
    }
}

impl DynamicOtConfig {
    pub fn new() -> DynamicOtConfig {
        ::std::default::Default::default()
    }

    // string library = 1;


    pub fn get_library(&self) -> &str {
        &self.library
    }
    pub fn clear_library(&mut self) {
        self.library.clear();
    }

    // Param is passed by value, moved
    pub fn set_library(&mut self, v: ::std::string::String) {
        self.library = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_library(&mut self) -> &mut ::std::string::String {
        &mut self.library
    }

    // Take field
    pub fn take_library(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.library, ::std::string::String::new())
    }

    // .google.protobuf.Struct config = 2;


    pub fn get_config(&self) -> &::protobuf::well_known_types::Struct {
        self.config.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Struct as ::protobuf::Message>::default_instance())
    }
    pub fn clear_config(&mut self) {
        self.config.clear();
    }

    pub fn has_config(&self) -> bool {
        self.config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_config(&mut self, v: ::protobuf::well_known_types::Struct) {
        self.config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config(&mut self) -> &mut ::protobuf::well_known_types::Struct {
        if self.config.is_none() {
            self.config.set_default();
        }
        self.config.as_mut().unwrap()
    }

    // Take field
    pub fn take_config(&mut self) -> ::protobuf::well_known_types::Struct {
        self.config.take().unwrap_or_else(|| ::protobuf::well_known_types::Struct::new())
    }
}

impl ::protobuf::Message for DynamicOtConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.config {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.library)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.config)?;
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
        if !self.library.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.library);
        }
        if let Some(ref v) = self.config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.library.is_empty() {
            os.write_string(1, &self.library)?;
        }
        if let Some(ref v) = self.config.as_ref() {
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

    fn new() -> DynamicOtConfig {
        DynamicOtConfig::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "library",
                |m: &DynamicOtConfig| { &m.library },
                |m: &mut DynamicOtConfig| { &mut m.library },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Struct>>(
                "config",
                |m: &DynamicOtConfig| { &m.config },
                |m: &mut DynamicOtConfig| { &mut m.config },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<DynamicOtConfig>(
                "DynamicOtConfig",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static DynamicOtConfig {
        static instance: ::protobuf::rt::LazyV2<DynamicOtConfig> = ::protobuf::rt::LazyV2::INIT;
        instance.get(DynamicOtConfig::new)
    }
}

impl ::protobuf::Clear for DynamicOtConfig {
    fn clear(&mut self) {
        self.library.clear();
        self.config.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DynamicOtConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DynamicOtConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n<envoy/extensions/tracers/dynamic_ot/v4alpha/dynamic_ot.proto\x12+envo\
    y.extensions.tracers.dynamic_ot.v4alpha\x1a\x1cgoogle/protobuf/struct.pr\
    oto\x1a\x1dudpa/annotations/status.proto\x1a!udpa/annotations/versioning\
    .proto\x1a\x17validate/validate.proto\"\x93\x01\n\x0fDynamicOtConfig\x12\
    !\n\x07library\x18\x01\x20\x01(\tR\x07libraryB\x07\xfaB\x04r\x02\x10\x01\
    \x12/\n\x06config\x18\x02\x20\x01(\x0b2\x17.google.protobuf.StructR\x06c\
    onfig:,\x9a\xc5\x88\x1e'\n%envoy.config.trace.v3.DynamicOtConfigBU\n9io.\
    envoyproxy.envoy.extensions.tracers.dynamic_ot.v4alphaB\x0eDynamicOtProt\
    oP\x01\xba\x80\xc8\xd1\x06\x02\x10\x03b\x06proto3\
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