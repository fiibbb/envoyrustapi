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
//! Generated file from `envoy/config/common/tap/v2alpha/common.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct CommonExtensionConfig {
    // message oneof groups
    pub config_type: ::std::option::Option<CommonExtensionConfig_oneof_config_type>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CommonExtensionConfig {
    fn default() -> &'a CommonExtensionConfig {
        <CommonExtensionConfig as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum CommonExtensionConfig_oneof_config_type {
    admin_config(AdminConfig),
    static_config(super::common::TapConfig),
}

impl CommonExtensionConfig {
    pub fn new() -> CommonExtensionConfig {
        ::std::default::Default::default()
    }

    // .envoy.config.common.tap.v2alpha.AdminConfig admin_config = 1;


    pub fn get_admin_config(&self) -> &AdminConfig {
        match self.config_type {
            ::std::option::Option::Some(CommonExtensionConfig_oneof_config_type::admin_config(ref v)) => v,
            _ => <AdminConfig as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_admin_config(&mut self) {
        self.config_type = ::std::option::Option::None;
    }

    pub fn has_admin_config(&self) -> bool {
        match self.config_type {
            ::std::option::Option::Some(CommonExtensionConfig_oneof_config_type::admin_config(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_admin_config(&mut self, v: AdminConfig) {
        self.config_type = ::std::option::Option::Some(CommonExtensionConfig_oneof_config_type::admin_config(v))
    }

    // Mutable pointer to the field.
    pub fn mut_admin_config(&mut self) -> &mut AdminConfig {
        if let ::std::option::Option::Some(CommonExtensionConfig_oneof_config_type::admin_config(_)) = self.config_type {
        } else {
            self.config_type = ::std::option::Option::Some(CommonExtensionConfig_oneof_config_type::admin_config(AdminConfig::new()));
        }
        match self.config_type {
            ::std::option::Option::Some(CommonExtensionConfig_oneof_config_type::admin_config(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_admin_config(&mut self) -> AdminConfig {
        if self.has_admin_config() {
            match self.config_type.take() {
                ::std::option::Option::Some(CommonExtensionConfig_oneof_config_type::admin_config(v)) => v,
                _ => panic!(),
            }
        } else {
            AdminConfig::new()
        }
    }

    // .envoy.service.tap.v2alpha.TapConfig static_config = 2;


    pub fn get_static_config(&self) -> &super::common::TapConfig {
        match self.config_type {
            ::std::option::Option::Some(CommonExtensionConfig_oneof_config_type::static_config(ref v)) => v,
            _ => <super::common::TapConfig as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_static_config(&mut self) {
        self.config_type = ::std::option::Option::None;
    }

    pub fn has_static_config(&self) -> bool {
        match self.config_type {
            ::std::option::Option::Some(CommonExtensionConfig_oneof_config_type::static_config(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_static_config(&mut self, v: super::common::TapConfig) {
        self.config_type = ::std::option::Option::Some(CommonExtensionConfig_oneof_config_type::static_config(v))
    }

    // Mutable pointer to the field.
    pub fn mut_static_config(&mut self) -> &mut super::common::TapConfig {
        if let ::std::option::Option::Some(CommonExtensionConfig_oneof_config_type::static_config(_)) = self.config_type {
        } else {
            self.config_type = ::std::option::Option::Some(CommonExtensionConfig_oneof_config_type::static_config(super::common::TapConfig::new()));
        }
        match self.config_type {
            ::std::option::Option::Some(CommonExtensionConfig_oneof_config_type::static_config(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_static_config(&mut self) -> super::common::TapConfig {
        if self.has_static_config() {
            match self.config_type.take() {
                ::std::option::Option::Some(CommonExtensionConfig_oneof_config_type::static_config(v)) => v,
                _ => panic!(),
            }
        } else {
            super::common::TapConfig::new()
        }
    }
}

impl ::protobuf::Message for CommonExtensionConfig {
    fn is_initialized(&self) -> bool {
        if let Some(CommonExtensionConfig_oneof_config_type::admin_config(ref v)) = self.config_type {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(CommonExtensionConfig_oneof_config_type::static_config(ref v)) = self.config_type {
            if !v.is_initialized() {
                return false;
            }
        }
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
                    self.config_type = ::std::option::Option::Some(CommonExtensionConfig_oneof_config_type::admin_config(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.config_type = ::std::option::Option::Some(CommonExtensionConfig_oneof_config_type::static_config(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.config_type {
            match v {
                &CommonExtensionConfig_oneof_config_type::admin_config(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &CommonExtensionConfig_oneof_config_type::static_config(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.config_type {
            match v {
                &CommonExtensionConfig_oneof_config_type::admin_config(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &CommonExtensionConfig_oneof_config_type::static_config(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
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

    fn new() -> CommonExtensionConfig {
        CommonExtensionConfig::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, AdminConfig>(
                "admin_config",
                CommonExtensionConfig::has_admin_config,
                CommonExtensionConfig::get_admin_config,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::common::TapConfig>(
                "static_config",
                CommonExtensionConfig::has_static_config,
                CommonExtensionConfig::get_static_config,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CommonExtensionConfig>(
                "CommonExtensionConfig",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CommonExtensionConfig {
        static instance: ::protobuf::rt::LazyV2<CommonExtensionConfig> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CommonExtensionConfig::new)
    }
}

impl ::protobuf::Clear for CommonExtensionConfig {
    fn clear(&mut self) {
        self.config_type = ::std::option::Option::None;
        self.config_type = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CommonExtensionConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommonExtensionConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AdminConfig {
    // message fields
    pub config_id: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a AdminConfig {
    fn default() -> &'a AdminConfig {
        <AdminConfig as ::protobuf::Message>::default_instance()
    }
}

impl AdminConfig {
    pub fn new() -> AdminConfig {
        ::std::default::Default::default()
    }

    // string config_id = 1;


    pub fn get_config_id(&self) -> &str {
        &self.config_id
    }
    pub fn clear_config_id(&mut self) {
        self.config_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_config_id(&mut self, v: ::std::string::String) {
        self.config_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_config_id(&mut self) -> &mut ::std::string::String {
        &mut self.config_id
    }

    // Take field
    pub fn take_config_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.config_id, ::std::string::String::new())
    }
}

impl ::protobuf::Message for AdminConfig {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.config_id)?;
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
        if !self.config_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.config_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.config_id.is_empty() {
            os.write_string(1, &self.config_id)?;
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

    fn new() -> AdminConfig {
        AdminConfig::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "config_id",
                |m: &AdminConfig| { &m.config_id },
                |m: &mut AdminConfig| { &mut m.config_id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<AdminConfig>(
                "AdminConfig",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static AdminConfig {
        static instance: ::protobuf::rt::LazyV2<AdminConfig> = ::protobuf::rt::LazyV2::INIT;
        instance.get(AdminConfig::new)
    }
}

impl ::protobuf::Clear for AdminConfig {
    fn clear(&mut self) {
        self.config_id.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AdminConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AdminConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n,envoy/config/common/tap/v2alpha/common.proto\x12\x1fenvoy.config.comm\
    on.tap.v2alpha\x1a&envoy/service/tap/v2alpha/common.proto\x1a\x1eudpa/an\
    notations/migrate.proto\x1a\x1dudpa/annotations/status.proto\x1a\x17vali\
    date/validate.proto\"\xcb\x01\n\x15CommonExtensionConfig\x12Q\n\x0cadmin\
    _config\x18\x01\x20\x01(\x0b2,.envoy.config.common.tap.v2alpha.AdminConf\
    igH\0R\x0badminConfig\x12K\n\rstatic_config\x18\x02\x20\x01(\x0b2$.envoy\
    .service.tap.v2alpha.TapConfigH\0R\x0cstaticConfigB\x12\n\x0bconfig_type\
    \x12\x03\xf8B\x01\"3\n\x0bAdminConfig\x12$\n\tconfig_id\x18\x01\x20\x01(\
    \tR\x08configIdB\x07\xfaB\x04r\x02\x20\x01Bl\n-io.envoyproxy.envoy.confi\
    g.common.tap.v2alphaB\x0bCommonProtoP\x01\xf2\x98\xfe\x8f\x05\x20\x12\
    \x1eenvoy.extensions.common.tap.v3\xba\x80\xc8\xd1\x06\x02\x10\x01b\x06p\
    roto3\
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
