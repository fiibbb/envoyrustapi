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
//! Generated file from `envoy/config/core/v3/socket_option.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct SocketOption {
    // message fields
    pub description: ::std::string::String,
    pub level: i64,
    pub name: i64,
    pub state: SocketOption_SocketState,
    // message oneof groups
    pub value: ::std::option::Option<SocketOption_oneof_value>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SocketOption {
    fn default() -> &'a SocketOption {
        <SocketOption as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum SocketOption_oneof_value {
    int_value(i64),
    buf_value(::std::vec::Vec<u8>),
}

impl SocketOption {
    pub fn new() -> SocketOption {
        ::std::default::Default::default()
    }

    // string description = 1;


    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    // int64 level = 2;


    pub fn get_level(&self) -> i64 {
        self.level
    }
    pub fn clear_level(&mut self) {
        self.level = 0;
    }

    // Param is passed by value, moved
    pub fn set_level(&mut self, v: i64) {
        self.level = v;
    }

    // int64 name = 3;


    pub fn get_name(&self) -> i64 {
        self.name
    }
    pub fn clear_name(&mut self) {
        self.name = 0;
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: i64) {
        self.name = v;
    }

    // int64 int_value = 4;


    pub fn get_int_value(&self) -> i64 {
        match self.value {
            ::std::option::Option::Some(SocketOption_oneof_value::int_value(v)) => v,
            _ => 0,
        }
    }
    pub fn clear_int_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_int_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(SocketOption_oneof_value::int_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int_value(&mut self, v: i64) {
        self.value = ::std::option::Option::Some(SocketOption_oneof_value::int_value(v))
    }

    // bytes buf_value = 5;


    pub fn get_buf_value(&self) -> &[u8] {
        match self.value {
            ::std::option::Option::Some(SocketOption_oneof_value::buf_value(ref v)) => v,
            _ => &[],
        }
    }
    pub fn clear_buf_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_buf_value(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(SocketOption_oneof_value::buf_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_buf_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::std::option::Option::Some(SocketOption_oneof_value::buf_value(v))
    }

    // Mutable pointer to the field.
    pub fn mut_buf_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(SocketOption_oneof_value::buf_value(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(SocketOption_oneof_value::buf_value(::std::vec::Vec::new()));
        }
        match self.value {
            ::std::option::Option::Some(SocketOption_oneof_value::buf_value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_buf_value(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_buf_value() {
            match self.value.take() {
                ::std::option::Option::Some(SocketOption_oneof_value::buf_value(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    // .envoy.config.core.v3.SocketOption.SocketState state = 6;


    pub fn get_state(&self) -> SocketOption_SocketState {
        self.state
    }
    pub fn clear_state(&mut self) {
        self.state = SocketOption_SocketState::STATE_PREBIND;
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: SocketOption_SocketState) {
        self.state = v;
    }
}

impl ::protobuf::Message for SocketOption {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.level = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.name = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(SocketOption_oneof_value::int_value(is.read_int64()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(SocketOption_oneof_value::buf_value(is.read_bytes()?));
                },
                6 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.state, 6, &mut self.unknown_fields)?
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
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.description);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::value_size(2, self.level, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.name != 0 {
            my_size += ::protobuf::rt::value_size(3, self.name, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.state != SocketOption_SocketState::STATE_PREBIND {
            my_size += ::protobuf::rt::enum_size(6, self.state);
        }
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &SocketOption_oneof_value::int_value(v) => {
                    my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &SocketOption_oneof_value::buf_value(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(5, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.description.is_empty() {
            os.write_string(1, &self.description)?;
        }
        if self.level != 0 {
            os.write_int64(2, self.level)?;
        }
        if self.name != 0 {
            os.write_int64(3, self.name)?;
        }
        if self.state != SocketOption_SocketState::STATE_PREBIND {
            os.write_enum(6, ::protobuf::ProtobufEnum::value(&self.state))?;
        }
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &SocketOption_oneof_value::int_value(v) => {
                    os.write_int64(4, v)?;
                },
                &SocketOption_oneof_value::buf_value(ref v) => {
                    os.write_bytes(5, v)?;
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

    fn new() -> SocketOption {
        SocketOption::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "description",
                |m: &SocketOption| { &m.description },
                |m: &mut SocketOption| { &mut m.description },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "level",
                |m: &SocketOption| { &m.level },
                |m: &mut SocketOption| { &mut m.level },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "name",
                |m: &SocketOption| { &m.name },
                |m: &mut SocketOption| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor::<_>(
                "int_value",
                SocketOption::has_int_value,
                SocketOption::get_int_value,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                "buf_value",
                SocketOption::has_buf_value,
                SocketOption::get_buf_value,
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<SocketOption_SocketState>>(
                "state",
                |m: &SocketOption| { &m.state },
                |m: &mut SocketOption| { &mut m.state },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SocketOption>(
                "SocketOption",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SocketOption {
        static instance: ::protobuf::rt::LazyV2<SocketOption> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SocketOption::new)
    }
}

impl ::protobuf::Clear for SocketOption {
    fn clear(&mut self) {
        self.description.clear();
        self.level = 0;
        self.name = 0;
        self.value = ::std::option::Option::None;
        self.value = ::std::option::Option::None;
        self.state = SocketOption_SocketState::STATE_PREBIND;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SocketOption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SocketOption {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SocketOption_SocketState {
    STATE_PREBIND = 0,
    STATE_BOUND = 1,
    STATE_LISTENING = 2,
}

impl ::protobuf::ProtobufEnum for SocketOption_SocketState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SocketOption_SocketState> {
        match value {
            0 => ::std::option::Option::Some(SocketOption_SocketState::STATE_PREBIND),
            1 => ::std::option::Option::Some(SocketOption_SocketState::STATE_BOUND),
            2 => ::std::option::Option::Some(SocketOption_SocketState::STATE_LISTENING),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SocketOption_SocketState] = &[
            SocketOption_SocketState::STATE_PREBIND,
            SocketOption_SocketState::STATE_BOUND,
            SocketOption_SocketState::STATE_LISTENING,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<SocketOption_SocketState>("SocketOption.SocketState", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for SocketOption_SocketState {
}

impl ::std::default::Default for SocketOption_SocketState {
    fn default() -> Self {
        SocketOption_SocketState::STATE_PREBIND
    }
}

impl ::protobuf::reflect::ProtobufValue for SocketOption_SocketState {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(envoy/config/core/v3/socket_option.proto\x12\x14envoy.config.core.v3\
    \x1a\x1dudpa/annotations/status.proto\x1a!udpa/annotations/versioning.pr\
    oto\x1a\x17validate/validate.proto\"\xe5\x02\n\x0cSocketOption\x12\x20\n\
    \x0bdescription\x18\x01\x20\x01(\tR\x0bdescription\x12\x14\n\x05level\
    \x18\x02\x20\x01(\x03R\x05level\x12\x12\n\x04name\x18\x03\x20\x01(\x03R\
    \x04name\x12\x1d\n\tint_value\x18\x04\x20\x01(\x03H\0R\x08intValue\x12\
    \x1d\n\tbuf_value\x18\x05\x20\x01(\x0cH\0R\x08bufValue\x12N\n\x05state\
    \x18\x06\x20\x01(\x0e2..envoy.config.core.v3.SocketOption.SocketStateR\
    \x05stateB\x08\xfaB\x05\x82\x01\x02\x10\x01\"F\n\x0bSocketState\x12\x11\
    \n\rSTATE_PREBIND\x10\0\x12\x0f\n\x0bSTATE_BOUND\x10\x01\x12\x13\n\x0fST\
    ATE_LISTENING\x10\x02B\x0c\n\x05value\x12\x03\xf8B\x01:%\x9a\xc5\x88\x1e\
    \x20\n\x1eenvoy.api.v2.core.SocketOptionBA\n\"io.envoyproxy.envoy.config\
    .core.v3B\x11SocketOptionProtoP\x01\xba\x80\xc8\xd1\x06\x02\x10\x02b\x06\
    proto3\
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
