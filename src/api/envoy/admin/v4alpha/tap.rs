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
//! Generated file from `envoy/admin/v4alpha/tap.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct TapRequest {
    // message fields
    pub config_id: ::std::string::String,
    pub tap_config: ::protobuf::SingularPtrField<super::common::TapConfig>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TapRequest {
    fn default() -> &'a TapRequest {
        <TapRequest as ::protobuf::Message>::default_instance()
    }
}

impl TapRequest {
    pub fn new() -> TapRequest {
        ::std::default::Default::default()
    }

    // string config_id = 1;


    pub fn get_config_id(&self) -> &str {
        &self.config_id
    }
    pub fn clear_config_id(&mut self) {
        self.config_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_config_id(&mut self, v: ::std::string::String) {
        self.config_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config_id(&mut self) -> &mut ::std::string::String {
        &mut self.config_id
    }

    // Take field
    pub fn take_config_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.config_id, ::std::string::String::new())
    }

    // .envoy.config.tap.v4alpha.TapConfig tap_config = 2;


    pub fn get_tap_config(&self) -> &super::common::TapConfig {
        self.tap_config.as_ref().unwrap_or_else(|| <super::common::TapConfig as ::protobuf::Message>::default_instance())
    }
    pub fn clear_tap_config(&mut self) {
        self.tap_config.clear();
    }

    pub fn has_tap_config(&self) -> bool {
        self.tap_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tap_config(&mut self, v: super::common::TapConfig) {
        self.tap_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tap_config(&mut self) -> &mut super::common::TapConfig {
        if self.tap_config.is_none() {
            self.tap_config.set_default();
        }
        self.tap_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_tap_config(&mut self) -> super::common::TapConfig {
        self.tap_config.take().unwrap_or_else(|| super::common::TapConfig::new())
    }
}

impl ::protobuf::Message for TapRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.tap_config {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.config_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tap_config)?;
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
        if !self.config_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.config_id);
        }
        if let Some(ref v) = self.tap_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.config_id.is_empty() {
            os.write_string(1, &self.config_id)?;
        }
        if let Some(ref v) = self.tap_config.as_ref() {
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

    fn new() -> TapRequest {
        TapRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "config_id",
                |m: &TapRequest| { &m.config_id },
                |m: &mut TapRequest| { &mut m.config_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::TapConfig>>(
                "tap_config",
                |m: &TapRequest| { &m.tap_config },
                |m: &mut TapRequest| { &mut m.tap_config },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TapRequest>(
                "TapRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TapRequest {
        static instance: ::protobuf::rt::LazyV2<TapRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TapRequest::new)
    }
}

impl ::protobuf::Clear for TapRequest {
    fn clear(&mut self) {
        self.config_id.clear();
        self.tap_config.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TapRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TapRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1denvoy/admin/v4alpha/tap.proto\x12\x13envoy.admin.v4alpha\x1a%envoy\
    /config/tap/v4alpha/common.proto\x1a\x1dudpa/annotations/status.proto\
    \x1a!udpa/annotations/versioning.proto\x1a\x17validate/validate.proto\"\
    \xa2\x01\n\nTapRequest\x12$\n\tconfig_id\x18\x01\x20\x01(\tR\x08configId\
    B\x07\xfaB\x04r\x02\x10\x01\x12L\n\ntap_config\x18\x02\x20\x01(\x0b2#.en\
    voy.config.tap.v4alpha.TapConfigR\ttapConfigB\x08\xfaB\x05\x8a\x01\x02\
    \x10\x01:\x20\x9a\xc5\x88\x1e\x1b\n\x19envoy.admin.v3.TapRequestB7\n!io.\
    envoyproxy.envoy.admin.v4alphaB\x08TapProtoP\x01\xba\x80\xc8\xd1\x06\x02\
    \x10\x03b\x06proto3\
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
