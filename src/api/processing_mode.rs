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
//! Generated file from `envoy/extensions/filters/http/ext_proc/v3alpha/processing_mode.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct ProcessingMode {
    // message fields
    pub request_header_mode: ProcessingMode_HeaderSendMode,
    pub response_header_mode: ProcessingMode_HeaderSendMode,
    pub request_body_mode: ProcessingMode_BodySendMode,
    pub response_body_mode: ProcessingMode_BodySendMode,
    pub request_trailer_mode: ProcessingMode_HeaderSendMode,
    pub response_trailer_mode: ProcessingMode_HeaderSendMode,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ProcessingMode {
    fn default() -> &'a ProcessingMode {
        <ProcessingMode as ::protobuf::Message>::default_instance()
    }
}

impl ProcessingMode {
    pub fn new() -> ProcessingMode {
        ::std::default::Default::default()
    }

    // .envoy.extensions.filters.http.ext_proc.v3alpha.ProcessingMode.HeaderSendMode request_header_mode = 1;


    pub fn get_request_header_mode(&self) -> ProcessingMode_HeaderSendMode {
        self.request_header_mode
    }
    pub fn clear_request_header_mode(&mut self) {
        self.request_header_mode = ProcessingMode_HeaderSendMode::DEFAULT;
    }

    // Param is passed by value, moved
    pub fn set_request_header_mode(&mut self, v: ProcessingMode_HeaderSendMode) {
        self.request_header_mode = v;
    }

    // .envoy.extensions.filters.http.ext_proc.v3alpha.ProcessingMode.HeaderSendMode response_header_mode = 2;


    pub fn get_response_header_mode(&self) -> ProcessingMode_HeaderSendMode {
        self.response_header_mode
    }
    pub fn clear_response_header_mode(&mut self) {
        self.response_header_mode = ProcessingMode_HeaderSendMode::DEFAULT;
    }

    // Param is passed by value, moved
    pub fn set_response_header_mode(&mut self, v: ProcessingMode_HeaderSendMode) {
        self.response_header_mode = v;
    }

    // .envoy.extensions.filters.http.ext_proc.v3alpha.ProcessingMode.BodySendMode request_body_mode = 3;


    pub fn get_request_body_mode(&self) -> ProcessingMode_BodySendMode {
        self.request_body_mode
    }
    pub fn clear_request_body_mode(&mut self) {
        self.request_body_mode = ProcessingMode_BodySendMode::NONE;
    }

    // Param is passed by value, moved
    pub fn set_request_body_mode(&mut self, v: ProcessingMode_BodySendMode) {
        self.request_body_mode = v;
    }

    // .envoy.extensions.filters.http.ext_proc.v3alpha.ProcessingMode.BodySendMode response_body_mode = 4;


    pub fn get_response_body_mode(&self) -> ProcessingMode_BodySendMode {
        self.response_body_mode
    }
    pub fn clear_response_body_mode(&mut self) {
        self.response_body_mode = ProcessingMode_BodySendMode::NONE;
    }

    // Param is passed by value, moved
    pub fn set_response_body_mode(&mut self, v: ProcessingMode_BodySendMode) {
        self.response_body_mode = v;
    }

    // .envoy.extensions.filters.http.ext_proc.v3alpha.ProcessingMode.HeaderSendMode request_trailer_mode = 5;


    pub fn get_request_trailer_mode(&self) -> ProcessingMode_HeaderSendMode {
        self.request_trailer_mode
    }
    pub fn clear_request_trailer_mode(&mut self) {
        self.request_trailer_mode = ProcessingMode_HeaderSendMode::DEFAULT;
    }

    // Param is passed by value, moved
    pub fn set_request_trailer_mode(&mut self, v: ProcessingMode_HeaderSendMode) {
        self.request_trailer_mode = v;
    }

    // .envoy.extensions.filters.http.ext_proc.v3alpha.ProcessingMode.HeaderSendMode response_trailer_mode = 6;


    pub fn get_response_trailer_mode(&self) -> ProcessingMode_HeaderSendMode {
        self.response_trailer_mode
    }
    pub fn clear_response_trailer_mode(&mut self) {
        self.response_trailer_mode = ProcessingMode_HeaderSendMode::DEFAULT;
    }

    // Param is passed by value, moved
    pub fn set_response_trailer_mode(&mut self, v: ProcessingMode_HeaderSendMode) {
        self.response_trailer_mode = v;
    }
}

impl ::protobuf::Message for ProcessingMode {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.request_header_mode, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.response_header_mode, 2, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.request_body_mode, 3, &mut self.unknown_fields)?
                },
                4 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.response_body_mode, 4, &mut self.unknown_fields)?
                },
                5 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.request_trailer_mode, 5, &mut self.unknown_fields)?
                },
                6 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.response_trailer_mode, 6, &mut self.unknown_fields)?
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
        if self.request_header_mode != ProcessingMode_HeaderSendMode::DEFAULT {
            my_size += ::protobuf::rt::enum_size(1, self.request_header_mode);
        }
        if self.response_header_mode != ProcessingMode_HeaderSendMode::DEFAULT {
            my_size += ::protobuf::rt::enum_size(2, self.response_header_mode);
        }
        if self.request_body_mode != ProcessingMode_BodySendMode::NONE {
            my_size += ::protobuf::rt::enum_size(3, self.request_body_mode);
        }
        if self.response_body_mode != ProcessingMode_BodySendMode::NONE {
            my_size += ::protobuf::rt::enum_size(4, self.response_body_mode);
        }
        if self.request_trailer_mode != ProcessingMode_HeaderSendMode::DEFAULT {
            my_size += ::protobuf::rt::enum_size(5, self.request_trailer_mode);
        }
        if self.response_trailer_mode != ProcessingMode_HeaderSendMode::DEFAULT {
            my_size += ::protobuf::rt::enum_size(6, self.response_trailer_mode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.request_header_mode != ProcessingMode_HeaderSendMode::DEFAULT {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.request_header_mode))?;
        }
        if self.response_header_mode != ProcessingMode_HeaderSendMode::DEFAULT {
            os.write_enum(2, ::protobuf::ProtobufEnum::value(&self.response_header_mode))?;
        }
        if self.request_body_mode != ProcessingMode_BodySendMode::NONE {
            os.write_enum(3, ::protobuf::ProtobufEnum::value(&self.request_body_mode))?;
        }
        if self.response_body_mode != ProcessingMode_BodySendMode::NONE {
            os.write_enum(4, ::protobuf::ProtobufEnum::value(&self.response_body_mode))?;
        }
        if self.request_trailer_mode != ProcessingMode_HeaderSendMode::DEFAULT {
            os.write_enum(5, ::protobuf::ProtobufEnum::value(&self.request_trailer_mode))?;
        }
        if self.response_trailer_mode != ProcessingMode_HeaderSendMode::DEFAULT {
            os.write_enum(6, ::protobuf::ProtobufEnum::value(&self.response_trailer_mode))?;
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

    fn new() -> ProcessingMode {
        ProcessingMode::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ProcessingMode_HeaderSendMode>>(
                "request_header_mode",
                |m: &ProcessingMode| { &m.request_header_mode },
                |m: &mut ProcessingMode| { &mut m.request_header_mode },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ProcessingMode_HeaderSendMode>>(
                "response_header_mode",
                |m: &ProcessingMode| { &m.response_header_mode },
                |m: &mut ProcessingMode| { &mut m.response_header_mode },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ProcessingMode_BodySendMode>>(
                "request_body_mode",
                |m: &ProcessingMode| { &m.request_body_mode },
                |m: &mut ProcessingMode| { &mut m.request_body_mode },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ProcessingMode_BodySendMode>>(
                "response_body_mode",
                |m: &ProcessingMode| { &m.response_body_mode },
                |m: &mut ProcessingMode| { &mut m.response_body_mode },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ProcessingMode_HeaderSendMode>>(
                "request_trailer_mode",
                |m: &ProcessingMode| { &m.request_trailer_mode },
                |m: &mut ProcessingMode| { &mut m.request_trailer_mode },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ProcessingMode_HeaderSendMode>>(
                "response_trailer_mode",
                |m: &ProcessingMode| { &m.response_trailer_mode },
                |m: &mut ProcessingMode| { &mut m.response_trailer_mode },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ProcessingMode>(
                "ProcessingMode",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ProcessingMode {
        static instance: ::protobuf::rt::LazyV2<ProcessingMode> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ProcessingMode::new)
    }
}

impl ::protobuf::Clear for ProcessingMode {
    fn clear(&mut self) {
        self.request_header_mode = ProcessingMode_HeaderSendMode::DEFAULT;
        self.response_header_mode = ProcessingMode_HeaderSendMode::DEFAULT;
        self.request_body_mode = ProcessingMode_BodySendMode::NONE;
        self.response_body_mode = ProcessingMode_BodySendMode::NONE;
        self.request_trailer_mode = ProcessingMode_HeaderSendMode::DEFAULT;
        self.response_trailer_mode = ProcessingMode_HeaderSendMode::DEFAULT;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProcessingMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProcessingMode {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ProcessingMode_HeaderSendMode {
    DEFAULT = 0,
    SEND = 1,
    SKIP = 2,
}

impl ::protobuf::ProtobufEnum for ProcessingMode_HeaderSendMode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ProcessingMode_HeaderSendMode> {
        match value {
            0 => ::std::option::Option::Some(ProcessingMode_HeaderSendMode::DEFAULT),
            1 => ::std::option::Option::Some(ProcessingMode_HeaderSendMode::SEND),
            2 => ::std::option::Option::Some(ProcessingMode_HeaderSendMode::SKIP),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ProcessingMode_HeaderSendMode] = &[
            ProcessingMode_HeaderSendMode::DEFAULT,
            ProcessingMode_HeaderSendMode::SEND,
            ProcessingMode_HeaderSendMode::SKIP,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<ProcessingMode_HeaderSendMode>("ProcessingMode.HeaderSendMode", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for ProcessingMode_HeaderSendMode {
}

impl ::std::default::Default for ProcessingMode_HeaderSendMode {
    fn default() -> Self {
        ProcessingMode_HeaderSendMode::DEFAULT
    }
}

impl ::protobuf::reflect::ProtobufValue for ProcessingMode_HeaderSendMode {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ProcessingMode_BodySendMode {
    NONE = 0,
    STREAMED = 1,
    BUFFERED = 2,
    BUFFERED_PARTIAL = 3,
}

impl ::protobuf::ProtobufEnum for ProcessingMode_BodySendMode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ProcessingMode_BodySendMode> {
        match value {
            0 => ::std::option::Option::Some(ProcessingMode_BodySendMode::NONE),
            1 => ::std::option::Option::Some(ProcessingMode_BodySendMode::STREAMED),
            2 => ::std::option::Option::Some(ProcessingMode_BodySendMode::BUFFERED),
            3 => ::std::option::Option::Some(ProcessingMode_BodySendMode::BUFFERED_PARTIAL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ProcessingMode_BodySendMode] = &[
            ProcessingMode_BodySendMode::NONE,
            ProcessingMode_BodySendMode::STREAMED,
            ProcessingMode_BodySendMode::BUFFERED,
            ProcessingMode_BodySendMode::BUFFERED_PARTIAL,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<ProcessingMode_BodySendMode>("ProcessingMode.BodySendMode", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for ProcessingMode_BodySendMode {
}

impl ::std::default::Default for ProcessingMode_BodySendMode {
    fn default() -> Self {
        ProcessingMode_BodySendMode::NONE
    }
}

impl ::protobuf::reflect::ProtobufValue for ProcessingMode_BodySendMode {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \nDenvoy/extensions/filters/http/ext_proc/v3alpha/processing_mode.proto\
    \x12.envoy.extensions.filters.http.ext_proc.v3alpha\x1a\x1dudpa/annotati\
    ons/status.proto\x1a\x17validate/validate.proto\"\xc9\x07\n\x0eProcessin\
    gMode\x12\x87\x01\n\x13request_header_mode\x18\x01\x20\x01(\x0e2M.envoy.\
    extensions.filters.http.ext_proc.v3alpha.ProcessingMode.HeaderSendModeR\
    \x11requestHeaderModeB\x08\xfaB\x05\x82\x01\x02\x10\x01\x12\x89\x01\n\
    \x14response_header_mode\x18\x02\x20\x01(\x0e2M.envoy.extensions.filters\
    .http.ext_proc.v3alpha.ProcessingMode.HeaderSendModeR\x12responseHeaderM\
    odeB\x08\xfaB\x05\x82\x01\x02\x10\x01\x12\x81\x01\n\x11request_body_mode\
    \x18\x03\x20\x01(\x0e2K.envoy.extensions.filters.http.ext_proc.v3alpha.P\
    rocessingMode.BodySendModeR\x0frequestBodyModeB\x08\xfaB\x05\x82\x01\x02\
    \x10\x01\x12\x83\x01\n\x12response_body_mode\x18\x04\x20\x01(\x0e2K.envo\
    y.extensions.filters.http.ext_proc.v3alpha.ProcessingMode.BodySendModeR\
    \x10responseBodyModeB\x08\xfaB\x05\x82\x01\x02\x10\x01\x12\x89\x01\n\x14\
    request_trailer_mode\x18\x05\x20\x01(\x0e2M.envoy.extensions.filters.htt\
    p.ext_proc.v3alpha.ProcessingMode.HeaderSendModeR\x12requestTrailerModeB\
    \x08\xfaB\x05\x82\x01\x02\x10\x01\x12\x8b\x01\n\x15response_trailer_mode\
    \x18\x06\x20\x01(\x0e2M.envoy.extensions.filters.http.ext_proc.v3alpha.P\
    rocessingMode.HeaderSendModeR\x13responseTrailerModeB\x08\xfaB\x05\x82\
    \x01\x02\x10\x01\"1\n\x0eHeaderSendMode\x12\x0b\n\x07DEFAULT\x10\0\x12\
    \x08\n\x04SEND\x10\x01\x12\x08\n\x04SKIP\x10\x02\"J\n\x0cBodySendMode\
    \x12\x08\n\x04NONE\x10\0\x12\x0c\n\x08STREAMED\x10\x01\x12\x0c\n\x08BUFF\
    ERED\x10\x02\x12\x14\n\x10BUFFERED_PARTIAL\x10\x03Be\n<io.envoyproxy.env\
    oy.extensions.filters.http.ext_proc.v3alphaB\x13ProcessingModeProtoP\x01\
    \xba\x80\xc8\xd1\x06\x02\x08\x01\xba\x80\xc8\xd1\x06\x02\x10\x02b\x06pro\
    to3\
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
