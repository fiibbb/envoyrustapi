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
//! Generated file from `envoy/config/route/v3/scoped_route.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct ScopedRouteConfiguration {
    // message fields
    pub on_demand: bool,
    pub name: ::std::string::String,
    pub route_configuration_name: ::std::string::String,
    pub key: ::protobuf::SingularPtrField<ScopedRouteConfiguration_Key>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ScopedRouteConfiguration {
    fn default() -> &'a ScopedRouteConfiguration {
        <ScopedRouteConfiguration as ::protobuf::Message>::default_instance()
    }
}

impl ScopedRouteConfiguration {
    pub fn new() -> ScopedRouteConfiguration {
        ::std::default::Default::default()
    }

    // bool on_demand = 4;


    pub fn get_on_demand(&self) -> bool {
        self.on_demand
    }
    pub fn clear_on_demand(&mut self) {
        self.on_demand = false;
    }

    // Param is passed by value, moved
    pub fn set_on_demand(&mut self, v: bool) {
        self.on_demand = v;
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

    // string route_configuration_name = 2;


    pub fn get_route_configuration_name(&self) -> &str {
        &self.route_configuration_name
    }
    pub fn clear_route_configuration_name(&mut self) {
        self.route_configuration_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_route_configuration_name(&mut self, v: ::std::string::String) {
        self.route_configuration_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_route_configuration_name(&mut self) -> &mut ::std::string::String {
        &mut self.route_configuration_name
    }

    // Take field
    pub fn take_route_configuration_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.route_configuration_name, ::std::string::String::new())
    }

    // .envoy.config.route.v3.ScopedRouteConfiguration.Key key = 3;


    pub fn get_key(&self) -> &ScopedRouteConfiguration_Key {
        self.key.as_ref().unwrap_or_else(|| <ScopedRouteConfiguration_Key as ::protobuf::Message>::default_instance())
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ScopedRouteConfiguration_Key) {
        self.key = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ScopedRouteConfiguration_Key {
        if self.key.is_none() {
            self.key.set_default();
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ScopedRouteConfiguration_Key {
        self.key.take().unwrap_or_else(|| ScopedRouteConfiguration_Key::new())
    }
}

impl ::protobuf::Message for ScopedRouteConfiguration {
    fn is_initialized(&self) -> bool {
        for v in &self.key {
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
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.on_demand = tmp;
                },
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.route_configuration_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.key)?;
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
        if self.on_demand != false {
            my_size += 2;
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.route_configuration_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.route_configuration_name);
        }
        if let Some(ref v) = self.key.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.on_demand != false {
            os.write_bool(4, self.on_demand)?;
        }
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.route_configuration_name.is_empty() {
            os.write_string(2, &self.route_configuration_name)?;
        }
        if let Some(ref v) = self.key.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> ScopedRouteConfiguration {
        ScopedRouteConfiguration::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "on_demand",
                |m: &ScopedRouteConfiguration| { &m.on_demand },
                |m: &mut ScopedRouteConfiguration| { &mut m.on_demand },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &ScopedRouteConfiguration| { &m.name },
                |m: &mut ScopedRouteConfiguration| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "route_configuration_name",
                |m: &ScopedRouteConfiguration| { &m.route_configuration_name },
                |m: &mut ScopedRouteConfiguration| { &mut m.route_configuration_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ScopedRouteConfiguration_Key>>(
                "key",
                |m: &ScopedRouteConfiguration| { &m.key },
                |m: &mut ScopedRouteConfiguration| { &mut m.key },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ScopedRouteConfiguration>(
                "ScopedRouteConfiguration",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ScopedRouteConfiguration {
        static instance: ::protobuf::rt::LazyV2<ScopedRouteConfiguration> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ScopedRouteConfiguration::new)
    }
}

impl ::protobuf::Clear for ScopedRouteConfiguration {
    fn clear(&mut self) {
        self.on_demand = false;
        self.name.clear();
        self.route_configuration_name.clear();
        self.key.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScopedRouteConfiguration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScopedRouteConfiguration {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScopedRouteConfiguration_Key {
    // message fields
    pub fragments: ::protobuf::RepeatedField<ScopedRouteConfiguration_Key_Fragment>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ScopedRouteConfiguration_Key {
    fn default() -> &'a ScopedRouteConfiguration_Key {
        <ScopedRouteConfiguration_Key as ::protobuf::Message>::default_instance()
    }
}

impl ScopedRouteConfiguration_Key {
    pub fn new() -> ScopedRouteConfiguration_Key {
        ::std::default::Default::default()
    }

    // repeated .envoy.config.route.v3.ScopedRouteConfiguration.Key.Fragment fragments = 1;


    pub fn get_fragments(&self) -> &[ScopedRouteConfiguration_Key_Fragment] {
        &self.fragments
    }
    pub fn clear_fragments(&mut self) {
        self.fragments.clear();
    }

    // Param is passed by value, moved
    pub fn set_fragments(&mut self, v: ::protobuf::RepeatedField<ScopedRouteConfiguration_Key_Fragment>) {
        self.fragments = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fragments(&mut self) -> &mut ::protobuf::RepeatedField<ScopedRouteConfiguration_Key_Fragment> {
        &mut self.fragments
    }

    // Take field
    pub fn take_fragments(&mut self) -> ::protobuf::RepeatedField<ScopedRouteConfiguration_Key_Fragment> {
        ::std::mem::replace(&mut self.fragments, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for ScopedRouteConfiguration_Key {
    fn is_initialized(&self) -> bool {
        for v in &self.fragments {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.fragments)?;
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
        for value in &self.fragments {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.fragments {
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

    fn new() -> ScopedRouteConfiguration_Key {
        ScopedRouteConfiguration_Key::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ScopedRouteConfiguration_Key_Fragment>>(
                "fragments",
                |m: &ScopedRouteConfiguration_Key| { &m.fragments },
                |m: &mut ScopedRouteConfiguration_Key| { &mut m.fragments },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ScopedRouteConfiguration_Key>(
                "ScopedRouteConfiguration.Key",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ScopedRouteConfiguration_Key {
        static instance: ::protobuf::rt::LazyV2<ScopedRouteConfiguration_Key> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ScopedRouteConfiguration_Key::new)
    }
}

impl ::protobuf::Clear for ScopedRouteConfiguration_Key {
    fn clear(&mut self) {
        self.fragments.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScopedRouteConfiguration_Key {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScopedRouteConfiguration_Key {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScopedRouteConfiguration_Key_Fragment {
    // message oneof groups
    pub field_type: ::std::option::Option<ScopedRouteConfiguration_Key_Fragment_oneof_type>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ScopedRouteConfiguration_Key_Fragment {
    fn default() -> &'a ScopedRouteConfiguration_Key_Fragment {
        <ScopedRouteConfiguration_Key_Fragment as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum ScopedRouteConfiguration_Key_Fragment_oneof_type {
    string_key(::std::string::String),
}

impl ScopedRouteConfiguration_Key_Fragment {
    pub fn new() -> ScopedRouteConfiguration_Key_Fragment {
        ::std::default::Default::default()
    }

    // string string_key = 1;


    pub fn get_string_key(&self) -> &str {
        match self.field_type {
            ::std::option::Option::Some(ScopedRouteConfiguration_Key_Fragment_oneof_type::string_key(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_string_key(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_string_key(&self) -> bool {
        match self.field_type {
            ::std::option::Option::Some(ScopedRouteConfiguration_Key_Fragment_oneof_type::string_key(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_string_key(&mut self, v: ::std::string::String) {
        self.field_type = ::std::option::Option::Some(ScopedRouteConfiguration_Key_Fragment_oneof_type::string_key(v))
    }

    // Mutable pointer to the field.
    pub fn mut_string_key(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(ScopedRouteConfiguration_Key_Fragment_oneof_type::string_key(_)) = self.field_type {
        } else {
            self.field_type = ::std::option::Option::Some(ScopedRouteConfiguration_Key_Fragment_oneof_type::string_key(::std::string::String::new()));
        }
        match self.field_type {
            ::std::option::Option::Some(ScopedRouteConfiguration_Key_Fragment_oneof_type::string_key(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_string_key(&mut self) -> ::std::string::String {
        if self.has_string_key() {
            match self.field_type.take() {
                ::std::option::Option::Some(ScopedRouteConfiguration_Key_Fragment_oneof_type::string_key(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }
}

impl ::protobuf::Message for ScopedRouteConfiguration_Key_Fragment {
    fn is_initialized(&self) -> bool {
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
                    self.field_type = ::std::option::Option::Some(ScopedRouteConfiguration_Key_Fragment_oneof_type::string_key(is.read_string()?));
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
        if let ::std::option::Option::Some(ref v) = self.field_type {
            match v {
                &ScopedRouteConfiguration_Key_Fragment_oneof_type::string_key(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.field_type {
            match v {
                &ScopedRouteConfiguration_Key_Fragment_oneof_type::string_key(ref v) => {
                    os.write_string(1, v)?;
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

    fn new() -> ScopedRouteConfiguration_Key_Fragment {
        ScopedRouteConfiguration_Key_Fragment::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "string_key",
                ScopedRouteConfiguration_Key_Fragment::has_string_key,
                ScopedRouteConfiguration_Key_Fragment::get_string_key,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ScopedRouteConfiguration_Key_Fragment>(
                "ScopedRouteConfiguration.Key.Fragment",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ScopedRouteConfiguration_Key_Fragment {
        static instance: ::protobuf::rt::LazyV2<ScopedRouteConfiguration_Key_Fragment> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ScopedRouteConfiguration_Key_Fragment::new)
    }
}

impl ::protobuf::Clear for ScopedRouteConfiguration_Key_Fragment {
    fn clear(&mut self) {
        self.field_type = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScopedRouteConfiguration_Key_Fragment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScopedRouteConfiguration_Key_Fragment {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(envoy/config/route/v3/scoped_route.proto\x12\x15envoy.config.route.v3\
    \x1a\x1dudpa/annotations/status.proto\x1a!udpa/annotations/versioning.pr\
    oto\x1a\x17validate/validate.proto\"\xab\x04\n\x18ScopedRouteConfigurati\
    on\x12\x1b\n\ton_demand\x18\x04\x20\x01(\x08R\x08onDemand\x12\x1b\n\x04n\
    ame\x18\x01\x20\x01(\tR\x04nameB\x07\xfaB\x04r\x02\x10\x01\x12A\n\x18rou\
    te_configuration_name\x18\x02\x20\x01(\tR\x16routeConfigurationNameB\x07\
    \xfaB\x04r\x02\x10\x01\x12O\n\x03key\x18\x03\x20\x01(\x0b23.envoy.config\
    .route.v3.ScopedRouteConfiguration.KeyR\x03keyB\x08\xfaB\x05\x8a\x01\x02\
    \x10\x01\x1a\x92\x02\n\x03Key\x12d\n\tfragments\x18\x01\x20\x03(\x0b2<.e\
    nvoy.config.route.v3.ScopedRouteConfiguration.Key.FragmentR\tfragmentsB\
    \x08\xfaB\x05\x92\x01\x02\x08\x01\x1as\n\x08Fragment\x12\x1f\n\nstring_k\
    ey\x18\x01\x20\x01(\tH\0R\tstringKeyB\x0b\n\x04type\x12\x03\xf8B\x01:9\
    \x9a\xc5\x88\x1e4\n2envoy.api.v2.ScopedRouteConfiguration.Key.Fragment:0\
    \x9a\xc5\x88\x1e+\n)envoy.api.v2.ScopedRouteConfiguration.Key:,\x9a\xc5\
    \x88\x1e'\n%envoy.api.v2.ScopedRouteConfigurationBA\n#io.envoyproxy.envo\
    y.config.route.v3B\x10ScopedRouteProtoP\x01\xba\x80\xc8\xd1\x06\x02\x10\
    \x02b\x06proto3\
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