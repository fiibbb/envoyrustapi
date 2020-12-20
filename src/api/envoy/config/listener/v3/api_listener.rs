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
//! Generated file from `envoy/config/listener/v3/api_listener.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct ApiListener {
    // message fields
    pub api_listener: ::protobuf::SingularPtrField<::protobuf::well_known_types::Any>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ApiListener {
    fn default() -> &'a ApiListener {
        <ApiListener as ::protobuf::Message>::default_instance()
    }
}

impl ApiListener {
    pub fn new() -> ApiListener {
        ::std::default::Default::default()
    }

    // .google.protobuf.Any api_listener = 1;


    pub fn get_api_listener(&self) -> &::protobuf::well_known_types::Any {
        self.api_listener.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Any as ::protobuf::Message>::default_instance())
    }
    pub fn clear_api_listener(&mut self) {
        self.api_listener.clear();
    }

    pub fn has_api_listener(&self) -> bool {
        self.api_listener.is_some()
    }

    // Param is passed by value, moved
    pub fn set_api_listener(&mut self, v: ::protobuf::well_known_types::Any) {
        self.api_listener = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_api_listener(&mut self) -> &mut ::protobuf::well_known_types::Any {
        if self.api_listener.is_none() {
            self.api_listener.set_default();
        }
        self.api_listener.as_mut().unwrap()
    }

    // Take field
    pub fn take_api_listener(&mut self) -> ::protobuf::well_known_types::Any {
        self.api_listener.take().unwrap_or_else(|| ::protobuf::well_known_types::Any::new())
    }
}

impl ::protobuf::Message for ApiListener {
    fn is_initialized(&self) -> bool {
        for v in &self.api_listener {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.api_listener)?;
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
        if let Some(ref v) = self.api_listener.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.api_listener.as_ref() {
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

    fn new() -> ApiListener {
        ApiListener::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Any>>(
                "api_listener",
                |m: &ApiListener| { &m.api_listener },
                |m: &mut ApiListener| { &mut m.api_listener },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ApiListener>(
                "ApiListener",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ApiListener {
        static instance: ::protobuf::rt::LazyV2<ApiListener> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ApiListener::new)
    }
}

impl ::protobuf::Clear for ApiListener {
    fn clear(&mut self) {
        self.api_listener.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ApiListener {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ApiListener {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n+envoy/config/listener/v3/api_listener.proto\x12\x18envoy.config.liste\
    ner.v3\x1a\x19google/protobuf/any.proto\x1a\x1dudpa/annotations/status.p\
    roto\x1a!udpa/annotations/versioning.proto\"s\n\x0bApiListener\x127\n\
    \x0capi_listener\x18\x01\x20\x01(\x0b2\x14.google.protobuf.AnyR\x0bapiLi\
    stener:+\x9a\xc5\x88\x1e&\n$envoy.config.listener.v2.ApiListenerBD\n&io.\
    envoyproxy.envoy.config.listener.v3B\x10ApiListenerProtoP\x01\xba\x80\
    \xc8\xd1\x06\x02\x10\x02b\x06proto3\
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
