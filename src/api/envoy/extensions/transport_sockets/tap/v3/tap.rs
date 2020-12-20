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
//! Generated file from `envoy/extensions/transport_sockets/tap/v3/tap.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct Tap {
    // message fields
    pub common_config: ::protobuf::SingularPtrField<super::common::CommonExtensionConfig>,
    pub transport_socket: ::protobuf::SingularPtrField<super::base::TransportSocket>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Tap {
    fn default() -> &'a Tap {
        <Tap as ::protobuf::Message>::default_instance()
    }
}

impl Tap {
    pub fn new() -> Tap {
        ::std::default::Default::default()
    }

    // .envoy.extensions.common.tap.v3.CommonExtensionConfig common_config = 1;


    pub fn get_common_config(&self) -> &super::common::CommonExtensionConfig {
        self.common_config.as_ref().unwrap_or_else(|| <super::common::CommonExtensionConfig as ::protobuf::Message>::default_instance())
    }
    pub fn clear_common_config(&mut self) {
        self.common_config.clear();
    }

    pub fn has_common_config(&self) -> bool {
        self.common_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_common_config(&mut self, v: super::common::CommonExtensionConfig) {
        self.common_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_common_config(&mut self) -> &mut super::common::CommonExtensionConfig {
        if self.common_config.is_none() {
            self.common_config.set_default();
        }
        self.common_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_common_config(&mut self) -> super::common::CommonExtensionConfig {
        self.common_config.take().unwrap_or_else(|| super::common::CommonExtensionConfig::new())
    }

    // .envoy.config.core.v3.TransportSocket transport_socket = 2;


    pub fn get_transport_socket(&self) -> &super::base::TransportSocket {
        self.transport_socket.as_ref().unwrap_or_else(|| <super::base::TransportSocket as ::protobuf::Message>::default_instance())
    }
    pub fn clear_transport_socket(&mut self) {
        self.transport_socket.clear();
    }

    pub fn has_transport_socket(&self) -> bool {
        self.transport_socket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transport_socket(&mut self, v: super::base::TransportSocket) {
        self.transport_socket = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transport_socket(&mut self) -> &mut super::base::TransportSocket {
        if self.transport_socket.is_none() {
            self.transport_socket.set_default();
        }
        self.transport_socket.as_mut().unwrap()
    }

    // Take field
    pub fn take_transport_socket(&mut self) -> super::base::TransportSocket {
        self.transport_socket.take().unwrap_or_else(|| super::base::TransportSocket::new())
    }
}

impl ::protobuf::Message for Tap {
    fn is_initialized(&self) -> bool {
        for v in &self.common_config {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.transport_socket {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.common_config)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.transport_socket)?;
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
        if let Some(ref v) = self.common_config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.transport_socket.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.common_config.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.transport_socket.as_ref() {
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

    fn new() -> Tap {
        Tap::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::CommonExtensionConfig>>(
                "common_config",
                |m: &Tap| { &m.common_config },
                |m: &mut Tap| { &mut m.common_config },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::TransportSocket>>(
                "transport_socket",
                |m: &Tap| { &m.transport_socket },
                |m: &mut Tap| { &mut m.transport_socket },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Tap>(
                "Tap",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Tap {
        static instance: ::protobuf::rt::LazyV2<Tap> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Tap::new)
    }
}

impl ::protobuf::Clear for Tap {
    fn clear(&mut self) {
        self.common_config.clear();
        self.transport_socket.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Tap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Tap {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n3envoy/extensions/transport_sockets/tap/v3/tap.proto\x12)envoy.extensi\
    ons.transport_sockets.tap.v3\x1a\x1fenvoy/config/core/v3/base.proto\x1a+\
    envoy/extensions/common/tap/v3/common.proto\x1a\x1dudpa/annotations/stat\
    us.proto\x1a!udpa/annotations/versioning.proto\x1a\x17validate/validate.\
    proto\"\xfd\x01\n\x03Tap\x12d\n\rcommon_config\x18\x01\x20\x01(\x0b25.en\
    voy.extensions.common.tap.v3.CommonExtensionConfigR\x0ccommonConfigB\x08\
    \xfaB\x05\x8a\x01\x02\x10\x01\x12Z\n\x10transport_socket\x18\x02\x20\x01\
    (\x0b2%.envoy.config.core.v3.TransportSocketR\x0ftransportSocketB\x08\
    \xfaB\x05\x8a\x01\x02\x10\x01:4\x9a\xc5\x88\x1e/\n-envoy.config.transpor\
    t_socket.tap.v2alpha.TapBM\n7io.envoyproxy.envoy.extensions.transport_so\
    ckets.tap.v3B\x08TapProtoP\x01\xba\x80\xc8\xd1\x06\x02\x10\x02b\x06proto\
    3\
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
