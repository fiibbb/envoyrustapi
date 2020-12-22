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
//! Generated file from `envoy/api/v2/core/grpc_method_list.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct GrpcMethodList {
    // message fields
    pub services: ::protobuf::RepeatedField<GrpcMethodList_Service>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GrpcMethodList {
    fn default() -> &'a GrpcMethodList {
        <GrpcMethodList as ::protobuf::Message>::default_instance()
    }
}

impl GrpcMethodList {
    pub fn new() -> GrpcMethodList {
        ::std::default::Default::default()
    }

    // repeated .envoy.api.v2.core.GrpcMethodList.Service services = 1;


    pub fn get_services(&self) -> &[GrpcMethodList_Service] {
        &self.services
    }
    pub fn clear_services(&mut self) {
        self.services.clear();
    }

    // Param is passed by value, moved
    pub fn set_services(&mut self, v: ::protobuf::RepeatedField<GrpcMethodList_Service>) {
        self.services = v;
    }

    // Mutable pointer to the field.
    pub fn mut_services(&mut self) -> &mut ::protobuf::RepeatedField<GrpcMethodList_Service> {
        &mut self.services
    }

    // Take field
    pub fn take_services(&mut self) -> ::protobuf::RepeatedField<GrpcMethodList_Service> {
        ::std::mem::replace(&mut self.services, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for GrpcMethodList {
    fn is_initialized(&self) -> bool {
        for v in &self.services {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.services)?;
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
        for value in &self.services {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.services {
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

    fn new() -> GrpcMethodList {
        GrpcMethodList::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GrpcMethodList_Service>>(
                "services",
                |m: &GrpcMethodList| { &m.services },
                |m: &mut GrpcMethodList| { &mut m.services },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<GrpcMethodList>(
                "GrpcMethodList",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GrpcMethodList {
        static instance: ::protobuf::rt::LazyV2<GrpcMethodList> = ::protobuf::rt::LazyV2::INIT;
        instance.get(GrpcMethodList::new)
    }
}

impl ::protobuf::Clear for GrpcMethodList {
    fn clear(&mut self) {
        self.services.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GrpcMethodList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GrpcMethodList {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GrpcMethodList_Service {
    // message fields
    pub name: ::std::string::String,
    pub method_names: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GrpcMethodList_Service {
    fn default() -> &'a GrpcMethodList_Service {
        <GrpcMethodList_Service as ::protobuf::Message>::default_instance()
    }
}

impl GrpcMethodList_Service {
    pub fn new() -> GrpcMethodList_Service {
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

    // repeated string method_names = 2;


    pub fn get_method_names(&self) -> &[::std::string::String] {
        &self.method_names
    }
    pub fn clear_method_names(&mut self) {
        self.method_names.clear();
    }

    // Param is passed by value, moved
    pub fn set_method_names(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.method_names = v;
    }

    // Mutable pointer to the field.
    pub fn mut_method_names(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.method_names
    }

    // Take field
    pub fn take_method_names(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.method_names, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for GrpcMethodList_Service {
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
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.method_names)?;
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
        for value in &self.method_names {
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
        for v in &self.method_names {
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

    fn new() -> GrpcMethodList_Service {
        GrpcMethodList_Service::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &GrpcMethodList_Service| { &m.name },
                |m: &mut GrpcMethodList_Service| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "method_names",
                |m: &GrpcMethodList_Service| { &m.method_names },
                |m: &mut GrpcMethodList_Service| { &mut m.method_names },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<GrpcMethodList_Service>(
                "GrpcMethodList.Service",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GrpcMethodList_Service {
        static instance: ::protobuf::rt::LazyV2<GrpcMethodList_Service> = ::protobuf::rt::LazyV2::INIT;
        instance.get(GrpcMethodList_Service::new)
    }
}

impl ::protobuf::Clear for GrpcMethodList_Service {
    fn clear(&mut self) {
        self.name.clear();
        self.method_names.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GrpcMethodList_Service {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GrpcMethodList_Service {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(envoy/api/v2/core/grpc_method_list.proto\x12\x11envoy.api.v2.core\x1a\
    \x1eudpa/annotations/migrate.proto\x1a\x1dudpa/annotations/status.proto\
    \x1a\x17validate/validate.proto\"\xac\x01\n\x0eGrpcMethodList\x12E\n\x08\
    services\x18\x01\x20\x03(\x0b2).envoy.api.v2.core.GrpcMethodList.Service\
    R\x08services\x1aS\n\x07Service\x12\x1b\n\x04name\x18\x01\x20\x01(\tR\
    \x04nameB\x07\xfaB\x04r\x02\x20\x01\x12+\n\x0cmethod_names\x18\x02\x20\
    \x03(\tR\x0bmethodNamesB\x08\xfaB\x05\x92\x01\x02\x08\x01B\\\n\x1fio.env\
    oyproxy.envoy.api.v2.coreB\x13GrpcMethodListProtoP\x01\xf2\x98\xfe\x8f\
    \x05\x16\x12\x14envoy.config.core.v3\xba\x80\xc8\xd1\x06\x02\x10\x01b\
    \x06proto3\
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
