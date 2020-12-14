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
//! Generated file from `envoy/extensions/transport_sockets/alts/v3/alts.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct Alts {
    // message fields
    pub handshaker_service: ::std::string::String,
    pub peer_service_accounts: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Alts {
    fn default() -> &'a Alts {
        <Alts as ::protobuf::Message>::default_instance()
    }
}

impl Alts {
    pub fn new() -> Alts {
        ::std::default::Default::default()
    }

    // string handshaker_service = 1;


    pub fn get_handshaker_service(&self) -> &str {
        &self.handshaker_service
    }
    pub fn clear_handshaker_service(&mut self) {
        self.handshaker_service.clear();
    }

    // Param is passed by value, moved
    pub fn set_handshaker_service(&mut self, v: ::std::string::String) {
        self.handshaker_service = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_handshaker_service(&mut self) -> &mut ::std::string::String {
        &mut self.handshaker_service
    }

    // Take field
    pub fn take_handshaker_service(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.handshaker_service, ::std::string::String::new())
    }

    // repeated string peer_service_accounts = 2;


    pub fn get_peer_service_accounts(&self) -> &[::std::string::String] {
        &self.peer_service_accounts
    }
    pub fn clear_peer_service_accounts(&mut self) {
        self.peer_service_accounts.clear();
    }

    // Param is passed by value, moved
    pub fn set_peer_service_accounts(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.peer_service_accounts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_peer_service_accounts(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.peer_service_accounts
    }

    // Take field
    pub fn take_peer_service_accounts(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.peer_service_accounts, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Alts {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.handshaker_service)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.peer_service_accounts)?;
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
        if !self.handshaker_service.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.handshaker_service);
        }
        for value in &self.peer_service_accounts {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.handshaker_service.is_empty() {
            os.write_string(1, &self.handshaker_service)?;
        }
        for v in &self.peer_service_accounts {
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

    fn new() -> Alts {
        Alts::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "handshaker_service",
                |m: &Alts| { &m.handshaker_service },
                |m: &mut Alts| { &mut m.handshaker_service },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "peer_service_accounts",
                |m: &Alts| { &m.peer_service_accounts },
                |m: &mut Alts| { &mut m.peer_service_accounts },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Alts>(
                "Alts",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Alts {
        static instance: ::protobuf::rt::LazyV2<Alts> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Alts::new)
    }
}

impl ::protobuf::Clear for Alts {
    fn clear(&mut self) {
        self.handshaker_service.clear();
        self.peer_service_accounts.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Alts {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Alts {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n5envoy/extensions/transport_sockets/alts/v3/alts.proto\x12*envoy.exten\
    sions.transport_sockets.alts.v3\x1a\x1dudpa/annotations/status.proto\x1a\
    !udpa/annotations/versioning.proto\x1a\x17validate/validate.proto\"\xaa\
    \x01\n\x04Alts\x126\n\x12handshaker_service\x18\x01\x20\x01(\tR\x11hands\
    hakerServiceB\x07\xfaB\x04r\x02\x10\x01\x122\n\x15peer_service_accounts\
    \x18\x02\x20\x03(\tR\x13peerServiceAccounts:6\x9a\xc5\x88\x1e1\n/envoy.c\
    onfig.transport_socket.alts.v2alpha.AltsBO\n8io.envoyproxy.envoy.extensi\
    ons.transport_sockets.alts.v3B\tAltsProtoP\x01\xba\x80\xc8\xd1\x06\x02\
    \x10\x02b\x06proto3\
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