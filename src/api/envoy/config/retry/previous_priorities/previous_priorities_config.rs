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
//! Generated file from `envoy/config/retry/previous_priorities/previous_priorities_config.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct PreviousPrioritiesConfig {
    // message fields
    pub update_frequency: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PreviousPrioritiesConfig {
    fn default() -> &'a PreviousPrioritiesConfig {
        <PreviousPrioritiesConfig as ::protobuf::Message>::default_instance()
    }
}

impl PreviousPrioritiesConfig {
    pub fn new() -> PreviousPrioritiesConfig {
        ::std::default::Default::default()
    }

    // int32 update_frequency = 1;


    pub fn get_update_frequency(&self) -> i32 {
        self.update_frequency
    }
    pub fn clear_update_frequency(&mut self) {
        self.update_frequency = 0;
    }

    // Param is passed by value, moved
    pub fn set_update_frequency(&mut self, v: i32) {
        self.update_frequency = v;
    }
}

impl ::protobuf::Message for PreviousPrioritiesConfig {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.update_frequency = tmp;
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
        if self.update_frequency != 0 {
            my_size += ::protobuf::rt::value_size(1, self.update_frequency, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.update_frequency != 0 {
            os.write_int32(1, self.update_frequency)?;
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

    fn new() -> PreviousPrioritiesConfig {
        PreviousPrioritiesConfig::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "update_frequency",
                |m: &PreviousPrioritiesConfig| { &m.update_frequency },
                |m: &mut PreviousPrioritiesConfig| { &mut m.update_frequency },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<PreviousPrioritiesConfig>(
                "PreviousPrioritiesConfig",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static PreviousPrioritiesConfig {
        static instance: ::protobuf::rt::LazyV2<PreviousPrioritiesConfig> = ::protobuf::rt::LazyV2::INIT;
        instance.get(PreviousPrioritiesConfig::new)
    }
}

impl ::protobuf::Clear for PreviousPrioritiesConfig {
    fn clear(&mut self) {
        self.update_frequency = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PreviousPrioritiesConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PreviousPrioritiesConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \nGenvoy/config/retry/previous_priorities/previous_priorities_config.pro\
    to\x12&envoy.config.retry.previous_priorities\x1a\x1dudpa/annotations/st\
    atus.proto\x1a\x17validate/validate.proto\"N\n\x18PreviousPrioritiesConf\
    ig\x122\n\x10update_frequency\x18\x01\x20\x01(\x05R\x0fupdateFrequencyB\
    \x07\xfaB\x04\x1a\x02\x20\0B_\n4io.envoyproxy.envoy.config.retry.previou\
    s_prioritiesB\x1dPreviousPrioritiesConfigProtoP\x01\xba\x80\xc8\xd1\x06\
    \x02\x10\x01b\x06proto3\
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
