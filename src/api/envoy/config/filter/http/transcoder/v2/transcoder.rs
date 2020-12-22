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
//! Generated file from `envoy/config/filter/http/transcoder/v2/transcoder.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct GrpcJsonTranscoder {
    // message fields
    pub services: ::protobuf::RepeatedField<::std::string::String>,
    pub print_options: ::protobuf::SingularPtrField<GrpcJsonTranscoder_PrintOptions>,
    pub match_incoming_request_route: bool,
    pub ignored_query_parameters: ::protobuf::RepeatedField<::std::string::String>,
    pub auto_mapping: bool,
    pub ignore_unknown_query_parameters: bool,
    pub convert_grpc_status: bool,
    // message oneof groups
    pub descriptor_set: ::std::option::Option<GrpcJsonTranscoder_oneof_descriptor_set>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GrpcJsonTranscoder {
    fn default() -> &'a GrpcJsonTranscoder {
        <GrpcJsonTranscoder as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum GrpcJsonTranscoder_oneof_descriptor_set {
    proto_descriptor(::std::string::String),
    proto_descriptor_bin(::std::vec::Vec<u8>),
}

impl GrpcJsonTranscoder {
    pub fn new() -> GrpcJsonTranscoder {
        ::std::default::Default::default()
    }

    // string proto_descriptor = 1;


    pub fn get_proto_descriptor(&self) -> &str {
        match self.descriptor_set {
            ::std::option::Option::Some(GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_proto_descriptor(&mut self) {
        self.descriptor_set = ::std::option::Option::None;
    }

    pub fn has_proto_descriptor(&self) -> bool {
        match self.descriptor_set {
            ::std::option::Option::Some(GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_proto_descriptor(&mut self, v: ::std::string::String) {
        self.descriptor_set = ::std::option::Option::Some(GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor(v))
    }

    // Mutable pointer to the field.
    pub fn mut_proto_descriptor(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor(_)) = self.descriptor_set {
        } else {
            self.descriptor_set = ::std::option::Option::Some(GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor(::std::string::String::new()));
        }
        match self.descriptor_set {
            ::std::option::Option::Some(GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_proto_descriptor(&mut self) -> ::std::string::String {
        if self.has_proto_descriptor() {
            match self.descriptor_set.take() {
                ::std::option::Option::Some(GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // bytes proto_descriptor_bin = 4;


    pub fn get_proto_descriptor_bin(&self) -> &[u8] {
        match self.descriptor_set {
            ::std::option::Option::Some(GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor_bin(ref v)) => v,
            _ => &[],
        }
    }
    pub fn clear_proto_descriptor_bin(&mut self) {
        self.descriptor_set = ::std::option::Option::None;
    }

    pub fn has_proto_descriptor_bin(&self) -> bool {
        match self.descriptor_set {
            ::std::option::Option::Some(GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor_bin(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_proto_descriptor_bin(&mut self, v: ::std::vec::Vec<u8>) {
        self.descriptor_set = ::std::option::Option::Some(GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor_bin(v))
    }

    // Mutable pointer to the field.
    pub fn mut_proto_descriptor_bin(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor_bin(_)) = self.descriptor_set {
        } else {
            self.descriptor_set = ::std::option::Option::Some(GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor_bin(::std::vec::Vec::new()));
        }
        match self.descriptor_set {
            ::std::option::Option::Some(GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor_bin(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_proto_descriptor_bin(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_proto_descriptor_bin() {
            match self.descriptor_set.take() {
                ::std::option::Option::Some(GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor_bin(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    // repeated string services = 2;


    pub fn get_services(&self) -> &[::std::string::String] {
        &self.services
    }
    pub fn clear_services(&mut self) {
        self.services.clear();
    }

    // Param is passed by value, moved
    pub fn set_services(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.services = v;
    }

    // Mutable pointer to the field.
    pub fn mut_services(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.services
    }

    // Take field
    pub fn take_services(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.services, ::protobuf::RepeatedField::new())
    }

    // .envoy.config.filter.http.transcoder.v2.GrpcJsonTranscoder.PrintOptions print_options = 3;


    pub fn get_print_options(&self) -> &GrpcJsonTranscoder_PrintOptions {
        self.print_options.as_ref().unwrap_or_else(|| <GrpcJsonTranscoder_PrintOptions as ::protobuf::Message>::default_instance())
    }
    pub fn clear_print_options(&mut self) {
        self.print_options.clear();
    }

    pub fn has_print_options(&self) -> bool {
        self.print_options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_print_options(&mut self, v: GrpcJsonTranscoder_PrintOptions) {
        self.print_options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_print_options(&mut self) -> &mut GrpcJsonTranscoder_PrintOptions {
        if self.print_options.is_none() {
            self.print_options.set_default();
        }
        self.print_options.as_mut().unwrap()
    }

    // Take field
    pub fn take_print_options(&mut self) -> GrpcJsonTranscoder_PrintOptions {
        self.print_options.take().unwrap_or_else(|| GrpcJsonTranscoder_PrintOptions::new())
    }

    // bool match_incoming_request_route = 5;


    pub fn get_match_incoming_request_route(&self) -> bool {
        self.match_incoming_request_route
    }
    pub fn clear_match_incoming_request_route(&mut self) {
        self.match_incoming_request_route = false;
    }

    // Param is passed by value, moved
    pub fn set_match_incoming_request_route(&mut self, v: bool) {
        self.match_incoming_request_route = v;
    }

    // repeated string ignored_query_parameters = 6;


    pub fn get_ignored_query_parameters(&self) -> &[::std::string::String] {
        &self.ignored_query_parameters
    }
    pub fn clear_ignored_query_parameters(&mut self) {
        self.ignored_query_parameters.clear();
    }

    // Param is passed by value, moved
    pub fn set_ignored_query_parameters(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.ignored_query_parameters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ignored_query_parameters(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.ignored_query_parameters
    }

    // Take field
    pub fn take_ignored_query_parameters(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.ignored_query_parameters, ::protobuf::RepeatedField::new())
    }

    // bool auto_mapping = 7;


    pub fn get_auto_mapping(&self) -> bool {
        self.auto_mapping
    }
    pub fn clear_auto_mapping(&mut self) {
        self.auto_mapping = false;
    }

    // Param is passed by value, moved
    pub fn set_auto_mapping(&mut self, v: bool) {
        self.auto_mapping = v;
    }

    // bool ignore_unknown_query_parameters = 8;


    pub fn get_ignore_unknown_query_parameters(&self) -> bool {
        self.ignore_unknown_query_parameters
    }
    pub fn clear_ignore_unknown_query_parameters(&mut self) {
        self.ignore_unknown_query_parameters = false;
    }

    // Param is passed by value, moved
    pub fn set_ignore_unknown_query_parameters(&mut self, v: bool) {
        self.ignore_unknown_query_parameters = v;
    }

    // bool convert_grpc_status = 9;


    pub fn get_convert_grpc_status(&self) -> bool {
        self.convert_grpc_status
    }
    pub fn clear_convert_grpc_status(&mut self) {
        self.convert_grpc_status = false;
    }

    // Param is passed by value, moved
    pub fn set_convert_grpc_status(&mut self, v: bool) {
        self.convert_grpc_status = v;
    }
}

impl ::protobuf::Message for GrpcJsonTranscoder {
    fn is_initialized(&self) -> bool {
        for v in &self.print_options {
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.descriptor_set = ::std::option::Option::Some(GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor(is.read_string()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.descriptor_set = ::std::option::Option::Some(GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor_bin(is.read_bytes()?));
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.services)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.print_options)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.match_incoming_request_route = tmp;
                },
                6 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.ignored_query_parameters)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.auto_mapping = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.ignore_unknown_query_parameters = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.convert_grpc_status = tmp;
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
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if let Some(ref v) = self.print_options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.match_incoming_request_route != false {
            my_size += 2;
        }
        for value in &self.ignored_query_parameters {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        if self.auto_mapping != false {
            my_size += 2;
        }
        if self.ignore_unknown_query_parameters != false {
            my_size += 2;
        }
        if self.convert_grpc_status != false {
            my_size += 2;
        }
        if let ::std::option::Option::Some(ref v) = self.descriptor_set {
            match v {
                &GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor_bin(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(4, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.services {
            os.write_string(2, &v)?;
        };
        if let Some(ref v) = self.print_options.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.match_incoming_request_route != false {
            os.write_bool(5, self.match_incoming_request_route)?;
        }
        for v in &self.ignored_query_parameters {
            os.write_string(6, &v)?;
        };
        if self.auto_mapping != false {
            os.write_bool(7, self.auto_mapping)?;
        }
        if self.ignore_unknown_query_parameters != false {
            os.write_bool(8, self.ignore_unknown_query_parameters)?;
        }
        if self.convert_grpc_status != false {
            os.write_bool(9, self.convert_grpc_status)?;
        }
        if let ::std::option::Option::Some(ref v) = self.descriptor_set {
            match v {
                &GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor(ref v) => {
                    os.write_string(1, v)?;
                },
                &GrpcJsonTranscoder_oneof_descriptor_set::proto_descriptor_bin(ref v) => {
                    os.write_bytes(4, v)?;
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

    fn new() -> GrpcJsonTranscoder {
        GrpcJsonTranscoder::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "proto_descriptor",
                GrpcJsonTranscoder::has_proto_descriptor,
                GrpcJsonTranscoder::get_proto_descriptor,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                "proto_descriptor_bin",
                GrpcJsonTranscoder::has_proto_descriptor_bin,
                GrpcJsonTranscoder::get_proto_descriptor_bin,
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "services",
                |m: &GrpcJsonTranscoder| { &m.services },
                |m: &mut GrpcJsonTranscoder| { &mut m.services },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GrpcJsonTranscoder_PrintOptions>>(
                "print_options",
                |m: &GrpcJsonTranscoder| { &m.print_options },
                |m: &mut GrpcJsonTranscoder| { &mut m.print_options },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "match_incoming_request_route",
                |m: &GrpcJsonTranscoder| { &m.match_incoming_request_route },
                |m: &mut GrpcJsonTranscoder| { &mut m.match_incoming_request_route },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ignored_query_parameters",
                |m: &GrpcJsonTranscoder| { &m.ignored_query_parameters },
                |m: &mut GrpcJsonTranscoder| { &mut m.ignored_query_parameters },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "auto_mapping",
                |m: &GrpcJsonTranscoder| { &m.auto_mapping },
                |m: &mut GrpcJsonTranscoder| { &mut m.auto_mapping },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "ignore_unknown_query_parameters",
                |m: &GrpcJsonTranscoder| { &m.ignore_unknown_query_parameters },
                |m: &mut GrpcJsonTranscoder| { &mut m.ignore_unknown_query_parameters },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "convert_grpc_status",
                |m: &GrpcJsonTranscoder| { &m.convert_grpc_status },
                |m: &mut GrpcJsonTranscoder| { &mut m.convert_grpc_status },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<GrpcJsonTranscoder>(
                "GrpcJsonTranscoder",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GrpcJsonTranscoder {
        static instance: ::protobuf::rt::LazyV2<GrpcJsonTranscoder> = ::protobuf::rt::LazyV2::INIT;
        instance.get(GrpcJsonTranscoder::new)
    }
}

impl ::protobuf::Clear for GrpcJsonTranscoder {
    fn clear(&mut self) {
        self.descriptor_set = ::std::option::Option::None;
        self.descriptor_set = ::std::option::Option::None;
        self.services.clear();
        self.print_options.clear();
        self.match_incoming_request_route = false;
        self.ignored_query_parameters.clear();
        self.auto_mapping = false;
        self.ignore_unknown_query_parameters = false;
        self.convert_grpc_status = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GrpcJsonTranscoder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GrpcJsonTranscoder {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GrpcJsonTranscoder_PrintOptions {
    // message fields
    pub add_whitespace: bool,
    pub always_print_primitive_fields: bool,
    pub always_print_enums_as_ints: bool,
    pub preserve_proto_field_names: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GrpcJsonTranscoder_PrintOptions {
    fn default() -> &'a GrpcJsonTranscoder_PrintOptions {
        <GrpcJsonTranscoder_PrintOptions as ::protobuf::Message>::default_instance()
    }
}

impl GrpcJsonTranscoder_PrintOptions {
    pub fn new() -> GrpcJsonTranscoder_PrintOptions {
        ::std::default::Default::default()
    }

    // bool add_whitespace = 1;


    pub fn get_add_whitespace(&self) -> bool {
        self.add_whitespace
    }
    pub fn clear_add_whitespace(&mut self) {
        self.add_whitespace = false;
    }

    // Param is passed by value, moved
    pub fn set_add_whitespace(&mut self, v: bool) {
        self.add_whitespace = v;
    }

    // bool always_print_primitive_fields = 2;


    pub fn get_always_print_primitive_fields(&self) -> bool {
        self.always_print_primitive_fields
    }
    pub fn clear_always_print_primitive_fields(&mut self) {
        self.always_print_primitive_fields = false;
    }

    // Param is passed by value, moved
    pub fn set_always_print_primitive_fields(&mut self, v: bool) {
        self.always_print_primitive_fields = v;
    }

    // bool always_print_enums_as_ints = 3;


    pub fn get_always_print_enums_as_ints(&self) -> bool {
        self.always_print_enums_as_ints
    }
    pub fn clear_always_print_enums_as_ints(&mut self) {
        self.always_print_enums_as_ints = false;
    }

    // Param is passed by value, moved
    pub fn set_always_print_enums_as_ints(&mut self, v: bool) {
        self.always_print_enums_as_ints = v;
    }

    // bool preserve_proto_field_names = 4;


    pub fn get_preserve_proto_field_names(&self) -> bool {
        self.preserve_proto_field_names
    }
    pub fn clear_preserve_proto_field_names(&mut self) {
        self.preserve_proto_field_names = false;
    }

    // Param is passed by value, moved
    pub fn set_preserve_proto_field_names(&mut self, v: bool) {
        self.preserve_proto_field_names = v;
    }
}

impl ::protobuf::Message for GrpcJsonTranscoder_PrintOptions {
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
                    let tmp = is.read_bool()?;
                    self.add_whitespace = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.always_print_primitive_fields = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.always_print_enums_as_ints = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.preserve_proto_field_names = tmp;
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
        if self.add_whitespace != false {
            my_size += 2;
        }
        if self.always_print_primitive_fields != false {
            my_size += 2;
        }
        if self.always_print_enums_as_ints != false {
            my_size += 2;
        }
        if self.preserve_proto_field_names != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.add_whitespace != false {
            os.write_bool(1, self.add_whitespace)?;
        }
        if self.always_print_primitive_fields != false {
            os.write_bool(2, self.always_print_primitive_fields)?;
        }
        if self.always_print_enums_as_ints != false {
            os.write_bool(3, self.always_print_enums_as_ints)?;
        }
        if self.preserve_proto_field_names != false {
            os.write_bool(4, self.preserve_proto_field_names)?;
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

    fn new() -> GrpcJsonTranscoder_PrintOptions {
        GrpcJsonTranscoder_PrintOptions::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "add_whitespace",
                |m: &GrpcJsonTranscoder_PrintOptions| { &m.add_whitespace },
                |m: &mut GrpcJsonTranscoder_PrintOptions| { &mut m.add_whitespace },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "always_print_primitive_fields",
                |m: &GrpcJsonTranscoder_PrintOptions| { &m.always_print_primitive_fields },
                |m: &mut GrpcJsonTranscoder_PrintOptions| { &mut m.always_print_primitive_fields },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "always_print_enums_as_ints",
                |m: &GrpcJsonTranscoder_PrintOptions| { &m.always_print_enums_as_ints },
                |m: &mut GrpcJsonTranscoder_PrintOptions| { &mut m.always_print_enums_as_ints },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "preserve_proto_field_names",
                |m: &GrpcJsonTranscoder_PrintOptions| { &m.preserve_proto_field_names },
                |m: &mut GrpcJsonTranscoder_PrintOptions| { &mut m.preserve_proto_field_names },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<GrpcJsonTranscoder_PrintOptions>(
                "GrpcJsonTranscoder.PrintOptions",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GrpcJsonTranscoder_PrintOptions {
        static instance: ::protobuf::rt::LazyV2<GrpcJsonTranscoder_PrintOptions> = ::protobuf::rt::LazyV2::INIT;
        instance.get(GrpcJsonTranscoder_PrintOptions::new)
    }
}

impl ::protobuf::Clear for GrpcJsonTranscoder_PrintOptions {
    fn clear(&mut self) {
        self.add_whitespace = false;
        self.always_print_primitive_fields = false;
        self.always_print_enums_as_ints = false;
        self.preserve_proto_field_names = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GrpcJsonTranscoder_PrintOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GrpcJsonTranscoder_PrintOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n7envoy/config/filter/http/transcoder/v2/transcoder.proto\x12&envoy.con\
    fig.filter.http.transcoder.v2\x1a\x1eudpa/annotations/migrate.proto\x1a\
    \x1dudpa/annotations/status.proto\x1a\x17validate/validate.proto\"\xa9\
    \x06\n\x12GrpcJsonTranscoder\x12+\n\x10proto_descriptor\x18\x01\x20\x01(\
    \tH\0R\x0fprotoDescriptor\x122\n\x14proto_descriptor_bin\x18\x04\x20\x01\
    (\x0cH\0R\x12protoDescriptorBin\x12$\n\x08services\x18\x02\x20\x03(\tR\
    \x08servicesB\x08\xfaB\x05\x92\x01\x02\x08\x01\x12l\n\rprint_options\x18\
    \x03\x20\x01(\x0b2G.envoy.config.filter.http.transcoder.v2.GrpcJsonTrans\
    coder.PrintOptionsR\x0cprintOptions\x12?\n\x1cmatch_incoming_request_rou\
    te\x18\x05\x20\x01(\x08R\x19matchIncomingRequestRoute\x128\n\x18ignored_\
    query_parameters\x18\x06\x20\x03(\tR\x16ignoredQueryParameters\x12!\n\
    \x0cauto_mapping\x18\x07\x20\x01(\x08R\x0bautoMapping\x12E\n\x1fignore_u\
    nknown_query_parameters\x18\x08\x20\x01(\x08R\x1cignoreUnknownQueryParam\
    eters\x12.\n\x13convert_grpc_status\x18\t\x20\x01(\x08R\x11convertGrpcSt\
    atus\x1a\xf1\x01\n\x0cPrintOptions\x12%\n\x0eadd_whitespace\x18\x01\x20\
    \x01(\x08R\raddWhitespace\x12A\n\x1dalways_print_primitive_fields\x18\
    \x02\x20\x01(\x08R\x1aalwaysPrintPrimitiveFields\x12:\n\x1aalways_print_\
    enums_as_ints\x18\x03\x20\x01(\x08R\x16alwaysPrintEnumsAsInts\x12;\n\x1a\
    preserve_proto_field_names\x18\x04\x20\x01(\x08R\x17preserveProtoFieldNa\
    mesB\x15\n\x0edescriptor_set\x12\x03\xf8B\x01B\x8e\x01\n4io.envoyproxy.e\
    nvoy.config.filter.http.transcoder.v2B\x0fTranscoderProtoP\x01\xf2\x98\
    \xfe\x8f\x057\x125envoy.extensions.filters.http.grpc_json_transcoder.v3\
    \xba\x80\xc8\xd1\x06\x02\x10\x01b\x06proto3\
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
