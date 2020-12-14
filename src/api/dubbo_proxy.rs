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
//! Generated file from `envoy/extensions/filters/network/dubbo_proxy/v4alpha/dubbo_proxy.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct DubboProxy {
    // message fields
    pub stat_prefix: ::std::string::String,
    pub protocol_type: ProtocolType,
    pub serialization_type: SerializationType,
    pub route_config: ::protobuf::RepeatedField<super::route::RouteConfiguration>,
    pub dubbo_filters: ::protobuf::RepeatedField<DubboFilter>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DubboProxy {
    fn default() -> &'a DubboProxy {
        <DubboProxy as ::protobuf::Message>::default_instance()
    }
}

impl DubboProxy {
    pub fn new() -> DubboProxy {
        ::std::default::Default::default()
    }

    // string stat_prefix = 1;


    pub fn get_stat_prefix(&self) -> &str {
        &self.stat_prefix
    }
    pub fn clear_stat_prefix(&mut self) {
        self.stat_prefix.clear();
    }

    // Param is passed by value, moved
    pub fn set_stat_prefix(&mut self, v: ::std::string::String) {
        self.stat_prefix = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stat_prefix(&mut self) -> &mut ::std::string::String {
        &mut self.stat_prefix
    }

    // Take field
    pub fn take_stat_prefix(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.stat_prefix, ::std::string::String::new())
    }

    // .envoy.extensions.filters.network.dubbo_proxy.v4alpha.ProtocolType protocol_type = 2;


    pub fn get_protocol_type(&self) -> ProtocolType {
        self.protocol_type
    }
    pub fn clear_protocol_type(&mut self) {
        self.protocol_type = ProtocolType::Dubbo;
    }

    // Param is passed by value, moved
    pub fn set_protocol_type(&mut self, v: ProtocolType) {
        self.protocol_type = v;
    }

    // .envoy.extensions.filters.network.dubbo_proxy.v4alpha.SerializationType serialization_type = 3;


    pub fn get_serialization_type(&self) -> SerializationType {
        self.serialization_type
    }
    pub fn clear_serialization_type(&mut self) {
        self.serialization_type = SerializationType::Hessian2;
    }

    // Param is passed by value, moved
    pub fn set_serialization_type(&mut self, v: SerializationType) {
        self.serialization_type = v;
    }

    // repeated .envoy.extensions.filters.network.dubbo_proxy.v4alpha.RouteConfiguration route_config = 4;


    pub fn get_route_config(&self) -> &[super::route::RouteConfiguration] {
        &self.route_config
    }
    pub fn clear_route_config(&mut self) {
        self.route_config.clear();
    }

    // Param is passed by value, moved
    pub fn set_route_config(&mut self, v: ::protobuf::RepeatedField<super::route::RouteConfiguration>) {
        self.route_config = v;
    }

    // Mutable pointer to the field.
    pub fn mut_route_config(&mut self) -> &mut ::protobuf::RepeatedField<super::route::RouteConfiguration> {
        &mut self.route_config
    }

    // Take field
    pub fn take_route_config(&mut self) -> ::protobuf::RepeatedField<super::route::RouteConfiguration> {
        ::std::mem::replace(&mut self.route_config, ::protobuf::RepeatedField::new())
    }

    // repeated .envoy.extensions.filters.network.dubbo_proxy.v4alpha.DubboFilter dubbo_filters = 5;


    pub fn get_dubbo_filters(&self) -> &[DubboFilter] {
        &self.dubbo_filters
    }
    pub fn clear_dubbo_filters(&mut self) {
        self.dubbo_filters.clear();
    }

    // Param is passed by value, moved
    pub fn set_dubbo_filters(&mut self, v: ::protobuf::RepeatedField<DubboFilter>) {
        self.dubbo_filters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_dubbo_filters(&mut self) -> &mut ::protobuf::RepeatedField<DubboFilter> {
        &mut self.dubbo_filters
    }

    // Take field
    pub fn take_dubbo_filters(&mut self) -> ::protobuf::RepeatedField<DubboFilter> {
        ::std::mem::replace(&mut self.dubbo_filters, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for DubboProxy {
    fn is_initialized(&self) -> bool {
        for v in &self.route_config {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.dubbo_filters {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.stat_prefix)?;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.protocol_type, 2, &mut self.unknown_fields)?
                },
                3 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.serialization_type, 3, &mut self.unknown_fields)?
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.route_config)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.dubbo_filters)?;
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
        if !self.stat_prefix.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.stat_prefix);
        }
        if self.protocol_type != ProtocolType::Dubbo {
            my_size += ::protobuf::rt::enum_size(2, self.protocol_type);
        }
        if self.serialization_type != SerializationType::Hessian2 {
            my_size += ::protobuf::rt::enum_size(3, self.serialization_type);
        }
        for value in &self.route_config {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.dubbo_filters {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.stat_prefix.is_empty() {
            os.write_string(1, &self.stat_prefix)?;
        }
        if self.protocol_type != ProtocolType::Dubbo {
            os.write_enum(2, ::protobuf::ProtobufEnum::value(&self.protocol_type))?;
        }
        if self.serialization_type != SerializationType::Hessian2 {
            os.write_enum(3, ::protobuf::ProtobufEnum::value(&self.serialization_type))?;
        }
        for v in &self.route_config {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.dubbo_filters {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> DubboProxy {
        DubboProxy::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "stat_prefix",
                |m: &DubboProxy| { &m.stat_prefix },
                |m: &mut DubboProxy| { &mut m.stat_prefix },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ProtocolType>>(
                "protocol_type",
                |m: &DubboProxy| { &m.protocol_type },
                |m: &mut DubboProxy| { &mut m.protocol_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<SerializationType>>(
                "serialization_type",
                |m: &DubboProxy| { &m.serialization_type },
                |m: &mut DubboProxy| { &mut m.serialization_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::route::RouteConfiguration>>(
                "route_config",
                |m: &DubboProxy| { &m.route_config },
                |m: &mut DubboProxy| { &mut m.route_config },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DubboFilter>>(
                "dubbo_filters",
                |m: &DubboProxy| { &m.dubbo_filters },
                |m: &mut DubboProxy| { &mut m.dubbo_filters },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<DubboProxy>(
                "DubboProxy",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static DubboProxy {
        static instance: ::protobuf::rt::LazyV2<DubboProxy> = ::protobuf::rt::LazyV2::INIT;
        instance.get(DubboProxy::new)
    }
}

impl ::protobuf::Clear for DubboProxy {
    fn clear(&mut self) {
        self.stat_prefix.clear();
        self.protocol_type = ProtocolType::Dubbo;
        self.serialization_type = SerializationType::Hessian2;
        self.route_config.clear();
        self.dubbo_filters.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DubboProxy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DubboProxy {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DubboFilter {
    // message fields
    pub name: ::std::string::String,
    pub config: ::protobuf::SingularPtrField<::protobuf::well_known_types::Any>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DubboFilter {
    fn default() -> &'a DubboFilter {
        <DubboFilter as ::protobuf::Message>::default_instance()
    }
}

impl DubboFilter {
    pub fn new() -> DubboFilter {
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

    // .google.protobuf.Any config = 2;


    pub fn get_config(&self) -> &::protobuf::well_known_types::Any {
        self.config.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Any as ::protobuf::Message>::default_instance())
    }
    pub fn clear_config(&mut self) {
        self.config.clear();
    }

    pub fn has_config(&self) -> bool {
        self.config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_config(&mut self, v: ::protobuf::well_known_types::Any) {
        self.config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config(&mut self) -> &mut ::protobuf::well_known_types::Any {
        if self.config.is_none() {
            self.config.set_default();
        }
        self.config.as_mut().unwrap()
    }

    // Take field
    pub fn take_config(&mut self) -> ::protobuf::well_known_types::Any {
        self.config.take().unwrap_or_else(|| ::protobuf::well_known_types::Any::new())
    }
}

impl ::protobuf::Message for DubboFilter {
    fn is_initialized(&self) -> bool {
        for v in &self.config {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.config)?;
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
        if let Some(ref v) = self.config.as_ref() {
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
        if let Some(ref v) = self.config.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> DubboFilter {
        DubboFilter::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &DubboFilter| { &m.name },
                |m: &mut DubboFilter| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Any>>(
                "config",
                |m: &DubboFilter| { &m.config },
                |m: &mut DubboFilter| { &mut m.config },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<DubboFilter>(
                "DubboFilter",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static DubboFilter {
        static instance: ::protobuf::rt::LazyV2<DubboFilter> = ::protobuf::rt::LazyV2::INIT;
        instance.get(DubboFilter::new)
    }
}

impl ::protobuf::Clear for DubboFilter {
    fn clear(&mut self) {
        self.name.clear();
        self.config.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DubboFilter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DubboFilter {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ProtocolType {
    Dubbo = 0,
}

impl ::protobuf::ProtobufEnum for ProtocolType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ProtocolType> {
        match value {
            0 => ::std::option::Option::Some(ProtocolType::Dubbo),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ProtocolType] = &[
            ProtocolType::Dubbo,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<ProtocolType>("ProtocolType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for ProtocolType {
}

impl ::std::default::Default for ProtocolType {
    fn default() -> Self {
        ProtocolType::Dubbo
    }
}

impl ::protobuf::reflect::ProtobufValue for ProtocolType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SerializationType {
    Hessian2 = 0,
}

impl ::protobuf::ProtobufEnum for SerializationType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SerializationType> {
        match value {
            0 => ::std::option::Option::Some(SerializationType::Hessian2),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SerializationType] = &[
            SerializationType::Hessian2,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<SerializationType>("SerializationType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for SerializationType {
}

impl ::std::default::Default for SerializationType {
    fn default() -> Self {
        SerializationType::Hessian2
    }
}

impl ::protobuf::reflect::ProtobufValue for SerializationType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \nFenvoy/extensions/filters/network/dubbo_proxy/v4alpha/dubbo_proxy.prot\
    o\x124envoy.extensions.filters.network.dubbo_proxy.v4alpha\x1a@envoy/ext\
    ensions/filters/network/dubbo_proxy/v4alpha/route.proto\x1a\x19google/pr\
    otobuf/any.proto\x1a\x1dudpa/annotations/status.proto\x1a!udpa/annotatio\
    ns/versioning.proto\x1a\x17validate/validate.proto\"\xc4\x04\n\nDubboPro\
    xy\x12(\n\x0bstat_prefix\x18\x01\x20\x01(\tR\nstatPrefixB\x07\xfaB\x04r\
    \x02\x10\x01\x12q\n\rprotocol_type\x18\x02\x20\x01(\x0e2B.envoy.extensio\
    ns.filters.network.dubbo_proxy.v4alpha.ProtocolTypeR\x0cprotocolTypeB\
    \x08\xfaB\x05\x82\x01\x02\x10\x01\x12\x80\x01\n\x12serialization_type\
    \x18\x03\x20\x01(\x0e2G.envoy.extensions.filters.network.dubbo_proxy.v4a\
    lpha.SerializationTypeR\x11serializationTypeB\x08\xfaB\x05\x82\x01\x02\
    \x10\x01\x12k\n\x0croute_config\x18\x04\x20\x03(\x0b2H.envoy.extensions.\
    filters.network.dubbo_proxy.v4alpha.RouteConfigurationR\x0brouteConfig\
    \x12f\n\rdubbo_filters\x18\x05\x20\x03(\x0b2A.envoy.extensions.filters.n\
    etwork.dubbo_proxy.v4alpha.DubboFilterR\x0cdubboFilters:A\x9a\xc5\x88\
    \x1e<\n:envoy.extensions.filters.network.dubbo_proxy.v3.DubboProxy\"\x9c\
    \x01\n\x0bDubboFilter\x12\x1b\n\x04name\x18\x01\x20\x01(\tR\x04nameB\x07\
    \xfaB\x04r\x02\x10\x01\x12,\n\x06config\x18\x02\x20\x01(\x0b2\x14.google\
    .protobuf.AnyR\x06config:B\x9a\xc5\x88\x1e=\n;envoy.extensions.filters.n\
    etwork.dubbo_proxy.v3.DubboFilter*\x19\n\x0cProtocolType\x12\t\n\x05Dubb\
    o\x10\0*!\n\x11SerializationType\x12\x0c\n\x08Hessian2\x10\0B_\nBio.envo\
    yproxy.envoy.extensions.filters.network.dubbo_proxy.v4alphaB\x0fDubboPro\
    xyProtoP\x01\xba\x80\xc8\xd1\x06\x02\x10\x03b\x06proto3\
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
