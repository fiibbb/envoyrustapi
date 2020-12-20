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
//! Generated file from `envoy/config/route/v4alpha/route.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct RouteConfiguration {
    // message fields
    pub name: ::std::string::String,
    pub virtual_hosts: ::protobuf::RepeatedField<super::route_components::VirtualHost>,
    pub vhds: ::protobuf::SingularPtrField<Vhds>,
    pub internal_only_headers: ::protobuf::RepeatedField<::std::string::String>,
    pub response_headers_to_add: ::protobuf::RepeatedField<super::base::HeaderValueOption>,
    pub response_headers_to_remove: ::protobuf::RepeatedField<::std::string::String>,
    pub request_headers_to_add: ::protobuf::RepeatedField<super::base::HeaderValueOption>,
    pub request_headers_to_remove: ::protobuf::RepeatedField<::std::string::String>,
    pub most_specific_header_mutations_wins: bool,
    pub validate_clusters: ::protobuf::SingularPtrField<::protobuf::well_known_types::BoolValue>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RouteConfiguration {
    fn default() -> &'a RouteConfiguration {
        <RouteConfiguration as ::protobuf::Message>::default_instance()
    }
}

impl RouteConfiguration {
    pub fn new() -> RouteConfiguration {
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

    // repeated .envoy.config.route.v4alpha.VirtualHost virtual_hosts = 2;


    pub fn get_virtual_hosts(&self) -> &[super::route_components::VirtualHost] {
        &self.virtual_hosts
    }
    pub fn clear_virtual_hosts(&mut self) {
        self.virtual_hosts.clear();
    }

    // Param is passed by value, moved
    pub fn set_virtual_hosts(&mut self, v: ::protobuf::RepeatedField<super::route_components::VirtualHost>) {
        self.virtual_hosts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_virtual_hosts(&mut self) -> &mut ::protobuf::RepeatedField<super::route_components::VirtualHost> {
        &mut self.virtual_hosts
    }

    // Take field
    pub fn take_virtual_hosts(&mut self) -> ::protobuf::RepeatedField<super::route_components::VirtualHost> {
        ::std::mem::replace(&mut self.virtual_hosts, ::protobuf::RepeatedField::new())
    }

    // .envoy.config.route.v4alpha.Vhds vhds = 9;


    pub fn get_vhds(&self) -> &Vhds {
        self.vhds.as_ref().unwrap_or_else(|| <Vhds as ::protobuf::Message>::default_instance())
    }
    pub fn clear_vhds(&mut self) {
        self.vhds.clear();
    }

    pub fn has_vhds(&self) -> bool {
        self.vhds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vhds(&mut self, v: Vhds) {
        self.vhds = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vhds(&mut self) -> &mut Vhds {
        if self.vhds.is_none() {
            self.vhds.set_default();
        }
        self.vhds.as_mut().unwrap()
    }

    // Take field
    pub fn take_vhds(&mut self) -> Vhds {
        self.vhds.take().unwrap_or_else(|| Vhds::new())
    }

    // repeated string internal_only_headers = 3;


    pub fn get_internal_only_headers(&self) -> &[::std::string::String] {
        &self.internal_only_headers
    }
    pub fn clear_internal_only_headers(&mut self) {
        self.internal_only_headers.clear();
    }

    // Param is passed by value, moved
    pub fn set_internal_only_headers(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.internal_only_headers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_internal_only_headers(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.internal_only_headers
    }

    // Take field
    pub fn take_internal_only_headers(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.internal_only_headers, ::protobuf::RepeatedField::new())
    }

    // repeated .envoy.config.core.v4alpha.HeaderValueOption response_headers_to_add = 4;


    pub fn get_response_headers_to_add(&self) -> &[super::base::HeaderValueOption] {
        &self.response_headers_to_add
    }
    pub fn clear_response_headers_to_add(&mut self) {
        self.response_headers_to_add.clear();
    }

    // Param is passed by value, moved
    pub fn set_response_headers_to_add(&mut self, v: ::protobuf::RepeatedField<super::base::HeaderValueOption>) {
        self.response_headers_to_add = v;
    }

    // Mutable pointer to the field.
    pub fn mut_response_headers_to_add(&mut self) -> &mut ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        &mut self.response_headers_to_add
    }

    // Take field
    pub fn take_response_headers_to_add(&mut self) -> ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        ::std::mem::replace(&mut self.response_headers_to_add, ::protobuf::RepeatedField::new())
    }

    // repeated string response_headers_to_remove = 5;


    pub fn get_response_headers_to_remove(&self) -> &[::std::string::String] {
        &self.response_headers_to_remove
    }
    pub fn clear_response_headers_to_remove(&mut self) {
        self.response_headers_to_remove.clear();
    }

    // Param is passed by value, moved
    pub fn set_response_headers_to_remove(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.response_headers_to_remove = v;
    }

    // Mutable pointer to the field.
    pub fn mut_response_headers_to_remove(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.response_headers_to_remove
    }

    // Take field
    pub fn take_response_headers_to_remove(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.response_headers_to_remove, ::protobuf::RepeatedField::new())
    }

    // repeated .envoy.config.core.v4alpha.HeaderValueOption request_headers_to_add = 6;


    pub fn get_request_headers_to_add(&self) -> &[super::base::HeaderValueOption] {
        &self.request_headers_to_add
    }
    pub fn clear_request_headers_to_add(&mut self) {
        self.request_headers_to_add.clear();
    }

    // Param is passed by value, moved
    pub fn set_request_headers_to_add(&mut self, v: ::protobuf::RepeatedField<super::base::HeaderValueOption>) {
        self.request_headers_to_add = v;
    }

    // Mutable pointer to the field.
    pub fn mut_request_headers_to_add(&mut self) -> &mut ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        &mut self.request_headers_to_add
    }

    // Take field
    pub fn take_request_headers_to_add(&mut self) -> ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        ::std::mem::replace(&mut self.request_headers_to_add, ::protobuf::RepeatedField::new())
    }

    // repeated string request_headers_to_remove = 8;


    pub fn get_request_headers_to_remove(&self) -> &[::std::string::String] {
        &self.request_headers_to_remove
    }
    pub fn clear_request_headers_to_remove(&mut self) {
        self.request_headers_to_remove.clear();
    }

    // Param is passed by value, moved
    pub fn set_request_headers_to_remove(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.request_headers_to_remove = v;
    }

    // Mutable pointer to the field.
    pub fn mut_request_headers_to_remove(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.request_headers_to_remove
    }

    // Take field
    pub fn take_request_headers_to_remove(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.request_headers_to_remove, ::protobuf::RepeatedField::new())
    }

    // bool most_specific_header_mutations_wins = 10;


    pub fn get_most_specific_header_mutations_wins(&self) -> bool {
        self.most_specific_header_mutations_wins
    }
    pub fn clear_most_specific_header_mutations_wins(&mut self) {
        self.most_specific_header_mutations_wins = false;
    }

    // Param is passed by value, moved
    pub fn set_most_specific_header_mutations_wins(&mut self, v: bool) {
        self.most_specific_header_mutations_wins = v;
    }

    // .google.protobuf.BoolValue validate_clusters = 7;


    pub fn get_validate_clusters(&self) -> &::protobuf::well_known_types::BoolValue {
        self.validate_clusters.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::BoolValue as ::protobuf::Message>::default_instance())
    }
    pub fn clear_validate_clusters(&mut self) {
        self.validate_clusters.clear();
    }

    pub fn has_validate_clusters(&self) -> bool {
        self.validate_clusters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_validate_clusters(&mut self, v: ::protobuf::well_known_types::BoolValue) {
        self.validate_clusters = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_validate_clusters(&mut self) -> &mut ::protobuf::well_known_types::BoolValue {
        if self.validate_clusters.is_none() {
            self.validate_clusters.set_default();
        }
        self.validate_clusters.as_mut().unwrap()
    }

    // Take field
    pub fn take_validate_clusters(&mut self) -> ::protobuf::well_known_types::BoolValue {
        self.validate_clusters.take().unwrap_or_else(|| ::protobuf::well_known_types::BoolValue::new())
    }
}

impl ::protobuf::Message for RouteConfiguration {
    fn is_initialized(&self) -> bool {
        for v in &self.virtual_hosts {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.vhds {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.response_headers_to_add {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.request_headers_to_add {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.validate_clusters {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.virtual_hosts)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.vhds)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.internal_only_headers)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.response_headers_to_add)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.response_headers_to_remove)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.request_headers_to_add)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.request_headers_to_remove)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.most_specific_header_mutations_wins = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.validate_clusters)?;
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
        for value in &self.virtual_hosts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.vhds.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.internal_only_headers {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.response_headers_to_add {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.response_headers_to_remove {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in &self.request_headers_to_add {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.request_headers_to_remove {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        if self.most_specific_header_mutations_wins != false {
            my_size += 2;
        }
        if let Some(ref v) = self.validate_clusters.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        for v in &self.virtual_hosts {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.vhds.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.internal_only_headers {
            os.write_string(3, &v)?;
        };
        for v in &self.response_headers_to_add {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.response_headers_to_remove {
            os.write_string(5, &v)?;
        };
        for v in &self.request_headers_to_add {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.request_headers_to_remove {
            os.write_string(8, &v)?;
        };
        if self.most_specific_header_mutations_wins != false {
            os.write_bool(10, self.most_specific_header_mutations_wins)?;
        }
        if let Some(ref v) = self.validate_clusters.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> RouteConfiguration {
        RouteConfiguration::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &RouteConfiguration| { &m.name },
                |m: &mut RouteConfiguration| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::route_components::VirtualHost>>(
                "virtual_hosts",
                |m: &RouteConfiguration| { &m.virtual_hosts },
                |m: &mut RouteConfiguration| { &mut m.virtual_hosts },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Vhds>>(
                "vhds",
                |m: &RouteConfiguration| { &m.vhds },
                |m: &mut RouteConfiguration| { &mut m.vhds },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "internal_only_headers",
                |m: &RouteConfiguration| { &m.internal_only_headers },
                |m: &mut RouteConfiguration| { &mut m.internal_only_headers },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::HeaderValueOption>>(
                "response_headers_to_add",
                |m: &RouteConfiguration| { &m.response_headers_to_add },
                |m: &mut RouteConfiguration| { &mut m.response_headers_to_add },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "response_headers_to_remove",
                |m: &RouteConfiguration| { &m.response_headers_to_remove },
                |m: &mut RouteConfiguration| { &mut m.response_headers_to_remove },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::HeaderValueOption>>(
                "request_headers_to_add",
                |m: &RouteConfiguration| { &m.request_headers_to_add },
                |m: &mut RouteConfiguration| { &mut m.request_headers_to_add },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "request_headers_to_remove",
                |m: &RouteConfiguration| { &m.request_headers_to_remove },
                |m: &mut RouteConfiguration| { &mut m.request_headers_to_remove },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "most_specific_header_mutations_wins",
                |m: &RouteConfiguration| { &m.most_specific_header_mutations_wins },
                |m: &mut RouteConfiguration| { &mut m.most_specific_header_mutations_wins },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::BoolValue>>(
                "validate_clusters",
                |m: &RouteConfiguration| { &m.validate_clusters },
                |m: &mut RouteConfiguration| { &mut m.validate_clusters },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RouteConfiguration>(
                "RouteConfiguration",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RouteConfiguration {
        static instance: ::protobuf::rt::LazyV2<RouteConfiguration> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RouteConfiguration::new)
    }
}

impl ::protobuf::Clear for RouteConfiguration {
    fn clear(&mut self) {
        self.name.clear();
        self.virtual_hosts.clear();
        self.vhds.clear();
        self.internal_only_headers.clear();
        self.response_headers_to_add.clear();
        self.response_headers_to_remove.clear();
        self.request_headers_to_add.clear();
        self.request_headers_to_remove.clear();
        self.most_specific_header_mutations_wins = false;
        self.validate_clusters.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RouteConfiguration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RouteConfiguration {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Vhds {
    // message fields
    pub config_source: ::protobuf::SingularPtrField<super::config_source::ConfigSource>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Vhds {
    fn default() -> &'a Vhds {
        <Vhds as ::protobuf::Message>::default_instance()
    }
}

impl Vhds {
    pub fn new() -> Vhds {
        ::std::default::Default::default()
    }

    // .envoy.config.core.v4alpha.ConfigSource config_source = 1;


    pub fn get_config_source(&self) -> &super::config_source::ConfigSource {
        self.config_source.as_ref().unwrap_or_else(|| <super::config_source::ConfigSource as ::protobuf::Message>::default_instance())
    }
    pub fn clear_config_source(&mut self) {
        self.config_source.clear();
    }

    pub fn has_config_source(&self) -> bool {
        self.config_source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_config_source(&mut self, v: super::config_source::ConfigSource) {
        self.config_source = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config_source(&mut self) -> &mut super::config_source::ConfigSource {
        if self.config_source.is_none() {
            self.config_source.set_default();
        }
        self.config_source.as_mut().unwrap()
    }

    // Take field
    pub fn take_config_source(&mut self) -> super::config_source::ConfigSource {
        self.config_source.take().unwrap_or_else(|| super::config_source::ConfigSource::new())
    }
}

impl ::protobuf::Message for Vhds {
    fn is_initialized(&self) -> bool {
        for v in &self.config_source {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.config_source)?;
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
        if let Some(ref v) = self.config_source.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.config_source.as_ref() {
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

    fn new() -> Vhds {
        Vhds::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::config_source::ConfigSource>>(
                "config_source",
                |m: &Vhds| { &m.config_source },
                |m: &mut Vhds| { &mut m.config_source },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Vhds>(
                "Vhds",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Vhds {
        static instance: ::protobuf::rt::LazyV2<Vhds> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Vhds::new)
    }
}

impl ::protobuf::Clear for Vhds {
    fn clear(&mut self) {
        self.config_source.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Vhds {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Vhds {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&envoy/config/route/v4alpha/route.proto\x12\x1aenvoy.config.route.v4al\
    pha\x1a$envoy/config/core/v4alpha/base.proto\x1a-envoy/config/core/v4alp\
    ha/config_source.proto\x1a1envoy/config/route/v4alpha/route_components.p\
    roto\x1a\x1egoogle/protobuf/wrappers.proto\x1a\x1dudpa/annotations/statu\
    s.proto\x1a!udpa/annotations/versioning.proto\x1a\x17validate/validate.p\
    roto\"\xb4\x06\n\x12RouteConfiguration\x12\x12\n\x04name\x18\x01\x20\x01\
    (\tR\x04name\x12L\n\rvirtual_hosts\x18\x02\x20\x03(\x0b2'.envoy.config.r\
    oute.v4alpha.VirtualHostR\x0cvirtualHosts\x124\n\x04vhds\x18\t\x20\x01(\
    \x0b2\x20.envoy.config.route.v4alpha.VhdsR\x04vhds\x12D\n\x15internal_on\
    ly_headers\x18\x03\x20\x03(\tR\x13internalOnlyHeadersB\x10\xfaB\r\x92\
    \x01\n\"\x08r\x06\xc0\x01\x01\xc8\x01\0\x12n\n\x17response_headers_to_ad\
    d\x18\x04\x20\x03(\x0b2,.envoy.config.core.v4alpha.HeaderValueOptionR\
    \x14responseHeadersToAddB\t\xfaB\x06\x92\x01\x03\x10\xe8\x07\x12M\n\x1ar\
    esponse_headers_to_remove\x18\x05\x20\x03(\tR\x17responseHeadersToRemove\
    B\x10\xfaB\r\x92\x01\n\"\x08r\x06\xc0\x01\x01\xc8\x01\0\x12l\n\x16reques\
    t_headers_to_add\x18\x06\x20\x03(\x0b2,.envoy.config.core.v4alpha.Header\
    ValueOptionR\x13requestHeadersToAddB\t\xfaB\x06\x92\x01\x03\x10\xe8\x07\
    \x12K\n\x19request_headers_to_remove\x18\x08\x20\x03(\tR\x16requestHeade\
    rsToRemoveB\x10\xfaB\r\x92\x01\n\"\x08r\x06\xc0\x01\x01\xc8\x01\0\x12L\n\
    #most_specific_header_mutations_wins\x18\n\x20\x01(\x08R\x1fmostSpecific\
    HeaderMutationsWins\x12G\n\x11validate_clusters\x18\x07\x20\x01(\x0b2\
    \x1a.google.protobuf.BoolValueR\x10validateClusters:/\x9a\xc5\x88\x1e*\n\
    (envoy.config.route.v3.RouteConfiguration\"\x81\x01\n\x04Vhds\x12V\n\rco\
    nfig_source\x18\x01\x20\x01(\x0b2'.envoy.config.core.v4alpha.ConfigSourc\
    eR\x0cconfigSourceB\x08\xfaB\x05\x8a\x01\x02\x10\x01:!\x9a\xc5\x88\x1e\
    \x1c\n\x1aenvoy.config.route.v3.VhdsB@\n(io.envoyproxy.envoy.config.rout\
    e.v4alphaB\nRouteProtoP\x01\xba\x80\xc8\xd1\x06\x02\x10\x03b\x06proto3\
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
