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
//! Generated file from `envoy/config/filter/network/direct_response/v2/config.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct Config {
    // message fields
    pub response: ::protobuf::SingularPtrField<super::base::DataSource>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Config {
    fn default() -> &'a Config {
        <Config as ::protobuf::Message>::default_instance()
    }
}

impl Config {
    pub fn new() -> Config {
        ::std::default::Default::default()
    }

    // .envoy.api.v2.core.DataSource response = 1;


    pub fn get_response(&self) -> &super::base::DataSource {
        self.response.as_ref().unwrap_or_else(|| <super::base::DataSource as ::protobuf::Message>::default_instance())
    }
    pub fn clear_response(&mut self) {
        self.response.clear();
    }

    pub fn has_response(&self) -> bool {
        self.response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: super::base::DataSource) {
        self.response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response(&mut self) -> &mut super::base::DataSource {
        if self.response.is_none() {
            self.response.set_default();
        }
        self.response.as_mut().unwrap()
    }

    // Take field
    pub fn take_response(&mut self) -> super::base::DataSource {
        self.response.take().unwrap_or_else(|| super::base::DataSource::new())
    }
}

impl ::protobuf::Message for Config {
    fn is_initialized(&self) -> bool {
        for v in &self.response {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.response)?;
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
        if let Some(ref v) = self.response.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.response.as_ref() {
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

    fn new() -> Config {
        Config::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::DataSource>>(
                "response",
                |m: &Config| { &m.response },
                |m: &mut Config| { &mut m.response },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Config>(
                "Config",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Config {
        static instance: ::protobuf::rt::LazyV2<Config> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Config::new)
    }
}

impl ::protobuf::Clear for Config {
    fn clear(&mut self) {
        self.response.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Config {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Config {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n;envoy/config/filter/network/direct_response/v2/config.proto\x12.envoy\
    .config.filter.network.direct_response.v2\x1a\x1cenvoy/api/v2/core/base.\
    proto\x1a\x1eudpa/annotations/migrate.proto\x1a\x1dudpa/annotations/stat\
    us.proto\"C\n\x06Config\x129\n\x08response\x18\x01\x20\x01(\x0b2\x1d.env\
    oy.api.v2.core.DataSourceR\x08responseB\x90\x01\n<io.envoyproxy.envoy.co\
    nfig.filter.network.direct_response.v2B\x0bConfigProtoP\x01\xf2\x98\xfe\
    \x8f\x055\x123envoy.extensions.filters.network.direct_response.v3\xba\
    \x80\xc8\xd1\x06\x02\x10\x01b\x06proto3\
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
