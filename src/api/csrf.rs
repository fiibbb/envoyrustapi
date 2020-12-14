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
//! Generated file from `envoy/extensions/filters/http/csrf/v4alpha/csrf.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct CsrfPolicy {
    // message fields
    pub filter_enabled: ::protobuf::SingularPtrField<super::base::RuntimeFractionalPercent>,
    pub shadow_enabled: ::protobuf::SingularPtrField<super::base::RuntimeFractionalPercent>,
    pub additional_origins: ::protobuf::RepeatedField<super::string::StringMatcher>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CsrfPolicy {
    fn default() -> &'a CsrfPolicy {
        <CsrfPolicy as ::protobuf::Message>::default_instance()
    }
}

impl CsrfPolicy {
    pub fn new() -> CsrfPolicy {
        ::std::default::Default::default()
    }

    // .envoy.config.core.v4alpha.RuntimeFractionalPercent filter_enabled = 1;


    pub fn get_filter_enabled(&self) -> &super::base::RuntimeFractionalPercent {
        self.filter_enabled.as_ref().unwrap_or_else(|| <super::base::RuntimeFractionalPercent as ::protobuf::Message>::default_instance())
    }
    pub fn clear_filter_enabled(&mut self) {
        self.filter_enabled.clear();
    }

    pub fn has_filter_enabled(&self) -> bool {
        self.filter_enabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filter_enabled(&mut self, v: super::base::RuntimeFractionalPercent) {
        self.filter_enabled = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filter_enabled(&mut self) -> &mut super::base::RuntimeFractionalPercent {
        if self.filter_enabled.is_none() {
            self.filter_enabled.set_default();
        }
        self.filter_enabled.as_mut().unwrap()
    }

    // Take field
    pub fn take_filter_enabled(&mut self) -> super::base::RuntimeFractionalPercent {
        self.filter_enabled.take().unwrap_or_else(|| super::base::RuntimeFractionalPercent::new())
    }

    // .envoy.config.core.v4alpha.RuntimeFractionalPercent shadow_enabled = 2;


    pub fn get_shadow_enabled(&self) -> &super::base::RuntimeFractionalPercent {
        self.shadow_enabled.as_ref().unwrap_or_else(|| <super::base::RuntimeFractionalPercent as ::protobuf::Message>::default_instance())
    }
    pub fn clear_shadow_enabled(&mut self) {
        self.shadow_enabled.clear();
    }

    pub fn has_shadow_enabled(&self) -> bool {
        self.shadow_enabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shadow_enabled(&mut self, v: super::base::RuntimeFractionalPercent) {
        self.shadow_enabled = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shadow_enabled(&mut self) -> &mut super::base::RuntimeFractionalPercent {
        if self.shadow_enabled.is_none() {
            self.shadow_enabled.set_default();
        }
        self.shadow_enabled.as_mut().unwrap()
    }

    // Take field
    pub fn take_shadow_enabled(&mut self) -> super::base::RuntimeFractionalPercent {
        self.shadow_enabled.take().unwrap_or_else(|| super::base::RuntimeFractionalPercent::new())
    }

    // repeated .envoy.type.matcher.v4alpha.StringMatcher additional_origins = 3;


    pub fn get_additional_origins(&self) -> &[super::string::StringMatcher] {
        &self.additional_origins
    }
    pub fn clear_additional_origins(&mut self) {
        self.additional_origins.clear();
    }

    // Param is passed by value, moved
    pub fn set_additional_origins(&mut self, v: ::protobuf::RepeatedField<super::string::StringMatcher>) {
        self.additional_origins = v;
    }

    // Mutable pointer to the field.
    pub fn mut_additional_origins(&mut self) -> &mut ::protobuf::RepeatedField<super::string::StringMatcher> {
        &mut self.additional_origins
    }

    // Take field
    pub fn take_additional_origins(&mut self) -> ::protobuf::RepeatedField<super::string::StringMatcher> {
        ::std::mem::replace(&mut self.additional_origins, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for CsrfPolicy {
    fn is_initialized(&self) -> bool {
        for v in &self.filter_enabled {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.shadow_enabled {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.additional_origins {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.filter_enabled)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.shadow_enabled)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.additional_origins)?;
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
        if let Some(ref v) = self.filter_enabled.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.shadow_enabled.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.additional_origins {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.filter_enabled.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.shadow_enabled.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.additional_origins {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> CsrfPolicy {
        CsrfPolicy::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::RuntimeFractionalPercent>>(
                "filter_enabled",
                |m: &CsrfPolicy| { &m.filter_enabled },
                |m: &mut CsrfPolicy| { &mut m.filter_enabled },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::RuntimeFractionalPercent>>(
                "shadow_enabled",
                |m: &CsrfPolicy| { &m.shadow_enabled },
                |m: &mut CsrfPolicy| { &mut m.shadow_enabled },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::string::StringMatcher>>(
                "additional_origins",
                |m: &CsrfPolicy| { &m.additional_origins },
                |m: &mut CsrfPolicy| { &mut m.additional_origins },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CsrfPolicy>(
                "CsrfPolicy",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CsrfPolicy {
        static instance: ::protobuf::rt::LazyV2<CsrfPolicy> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CsrfPolicy::new)
    }
}

impl ::protobuf::Clear for CsrfPolicy {
    fn clear(&mut self) {
        self.filter_enabled.clear();
        self.shadow_enabled.clear();
        self.additional_origins.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CsrfPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CsrfPolicy {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n5envoy/extensions/filters/http/csrf/v4alpha/csrf.proto\x12*envoy.exten\
    sions.filters.http.csrf.v4alpha\x1a$envoy/config/core/v4alpha/base.proto\
    \x1a'envoy/type/matcher/v4alpha/string.proto\x1a\x1dudpa/annotations/sta\
    tus.proto\x1a!udpa/annotations/versioning.proto\x1a\x17validate/validate\
    .proto\"\xe1\x02\n\nCsrfPolicy\x12d\n\x0efilter_enabled\x18\x01\x20\x01(\
    \x0b23.envoy.config.core.v4alpha.RuntimeFractionalPercentR\rfilterEnable\
    dB\x08\xfaB\x05\x8a\x01\x02\x10\x01\x12Z\n\x0eshadow_enabled\x18\x02\x20\
    \x01(\x0b23.envoy.config.core.v4alpha.RuntimeFractionalPercentR\rshadowE\
    nabled\x12X\n\x12additional_origins\x18\x03\x20\x03(\x0b2).envoy.type.ma\
    tcher.v4alpha.StringMatcherR\x11additionalOrigins:7\x9a\xc5\x88\x1e2\n0e\
    nvoy.extensions.filters.http.csrf.v3.CsrfPolicyBO\n8io.envoyproxy.envoy.\
    extensions.filters.http.csrf.v4alphaB\tCsrfProtoP\x01\xba\x80\xc8\xd1\
    \x06\x02\x10\x03b\x06proto3\
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
