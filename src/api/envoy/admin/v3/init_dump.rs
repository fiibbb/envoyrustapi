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
//! Generated file from `envoy/admin/v3/init_dump.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct UnreadyTargetsDumps {
    // message fields
    pub unready_targets_dumps: ::protobuf::RepeatedField<UnreadyTargetsDumps_UnreadyTargetsDump>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UnreadyTargetsDumps {
    fn default() -> &'a UnreadyTargetsDumps {
        <UnreadyTargetsDumps as ::protobuf::Message>::default_instance()
    }
}

impl UnreadyTargetsDumps {
    pub fn new() -> UnreadyTargetsDumps {
        ::std::default::Default::default()
    }

    // repeated .envoy.admin.v3.UnreadyTargetsDumps.UnreadyTargetsDump unready_targets_dumps = 1;


    pub fn get_unready_targets_dumps(&self) -> &[UnreadyTargetsDumps_UnreadyTargetsDump] {
        &self.unready_targets_dumps
    }
    pub fn clear_unready_targets_dumps(&mut self) {
        self.unready_targets_dumps.clear();
    }

    // Param is passed by value, moved
    pub fn set_unready_targets_dumps(&mut self, v: ::protobuf::RepeatedField<UnreadyTargetsDumps_UnreadyTargetsDump>) {
        self.unready_targets_dumps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_unready_targets_dumps(&mut self) -> &mut ::protobuf::RepeatedField<UnreadyTargetsDumps_UnreadyTargetsDump> {
        &mut self.unready_targets_dumps
    }

    // Take field
    pub fn take_unready_targets_dumps(&mut self) -> ::protobuf::RepeatedField<UnreadyTargetsDumps_UnreadyTargetsDump> {
        ::std::mem::replace(&mut self.unready_targets_dumps, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for UnreadyTargetsDumps {
    fn is_initialized(&self) -> bool {
        for v in &self.unready_targets_dumps {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.unready_targets_dumps)?;
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
        for value in &self.unready_targets_dumps {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.unready_targets_dumps {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> UnreadyTargetsDumps {
        UnreadyTargetsDumps::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UnreadyTargetsDumps_UnreadyTargetsDump>>(
                "unready_targets_dumps",
                |m: &UnreadyTargetsDumps| { &m.unready_targets_dumps },
                |m: &mut UnreadyTargetsDumps| { &mut m.unready_targets_dumps },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UnreadyTargetsDumps>(
                "UnreadyTargetsDumps",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UnreadyTargetsDumps {
        static instance: ::protobuf::rt::LazyV2<UnreadyTargetsDumps> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UnreadyTargetsDumps::new)
    }
}

impl ::protobuf::Clear for UnreadyTargetsDumps {
    fn clear(&mut self) {
        self.unready_targets_dumps.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnreadyTargetsDumps {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnreadyTargetsDumps {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnreadyTargetsDumps_UnreadyTargetsDump {
    // message fields
    pub name: ::std::string::String,
    pub target_names: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UnreadyTargetsDumps_UnreadyTargetsDump {
    fn default() -> &'a UnreadyTargetsDumps_UnreadyTargetsDump {
        <UnreadyTargetsDumps_UnreadyTargetsDump as ::protobuf::Message>::default_instance()
    }
}

impl UnreadyTargetsDumps_UnreadyTargetsDump {
    pub fn new() -> UnreadyTargetsDumps_UnreadyTargetsDump {
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

    // repeated string target_names = 2;


    pub fn get_target_names(&self) -> &[::std::string::String] {
        &self.target_names
    }
    pub fn clear_target_names(&mut self) {
        self.target_names.clear();
    }

    // Param is passed by value, moved
    pub fn set_target_names(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.target_names = v;
    }

    // Mutable pointer to the field.
    pub fn mut_target_names(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.target_names
    }

    // Take field
    pub fn take_target_names(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.target_names, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for UnreadyTargetsDumps_UnreadyTargetsDump {
    fn is_initialized(&self) -> bool {
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
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.target_names)?;
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
        for value in &self.target_names {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        for v in &self.target_names {
            os.write_string(2, &v)?;
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

    fn new() -> UnreadyTargetsDumps_UnreadyTargetsDump {
        UnreadyTargetsDumps_UnreadyTargetsDump::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &UnreadyTargetsDumps_UnreadyTargetsDump| { &m.name },
                |m: &mut UnreadyTargetsDumps_UnreadyTargetsDump| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "target_names",
                |m: &UnreadyTargetsDumps_UnreadyTargetsDump| { &m.target_names },
                |m: &mut UnreadyTargetsDumps_UnreadyTargetsDump| { &mut m.target_names },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UnreadyTargetsDumps_UnreadyTargetsDump>(
                "UnreadyTargetsDumps.UnreadyTargetsDump",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UnreadyTargetsDumps_UnreadyTargetsDump {
        static instance: ::protobuf::rt::LazyV2<UnreadyTargetsDumps_UnreadyTargetsDump> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UnreadyTargetsDumps_UnreadyTargetsDump::new)
    }
}

impl ::protobuf::Clear for UnreadyTargetsDumps_UnreadyTargetsDump {
    fn clear(&mut self) {
        self.name.clear();
        self.target_names.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnreadyTargetsDumps_UnreadyTargetsDump {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnreadyTargetsDumps_UnreadyTargetsDump {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eenvoy/admin/v3/init_dump.proto\x12\x0eenvoy.admin.v3\x1a\x1dudpa/a\
    nnotations/status.proto\x1a!udpa/annotations/versioning.proto\"\xce\x01\
    \n\x13UnreadyTargetsDumps\x12j\n\x15unready_targets_dumps\x18\x01\x20\
    \x03(\x0b26.envoy.admin.v3.UnreadyTargetsDumps.UnreadyTargetsDumpR\x13un\
    readyTargetsDumps\x1aK\n\x12UnreadyTargetsDump\x12\x12\n\x04name\x18\x01\
    \x20\x01(\tR\x04name\x12!\n\x0ctarget_names\x18\x02\x20\x03(\tR\x0btarge\
    tNamesB7\n\x1cio.envoyproxy.envoy.admin.v3B\rInitDumpProtoP\x01\xba\x80\
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
