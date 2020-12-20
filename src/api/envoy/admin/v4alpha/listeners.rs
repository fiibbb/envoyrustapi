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
//! Generated file from `envoy/admin/v4alpha/listeners.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct Listeners {
    // message fields
    pub listener_statuses: ::protobuf::RepeatedField<ListenerStatus>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Listeners {
    fn default() -> &'a Listeners {
        <Listeners as ::protobuf::Message>::default_instance()
    }
}

impl Listeners {
    pub fn new() -> Listeners {
        ::std::default::Default::default()
    }

    // repeated .envoy.admin.v4alpha.ListenerStatus listener_statuses = 1;


    pub fn get_listener_statuses(&self) -> &[ListenerStatus] {
        &self.listener_statuses
    }
    pub fn clear_listener_statuses(&mut self) {
        self.listener_statuses.clear();
    }

    // Param is passed by value, moved
    pub fn set_listener_statuses(&mut self, v: ::protobuf::RepeatedField<ListenerStatus>) {
        self.listener_statuses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_listener_statuses(&mut self) -> &mut ::protobuf::RepeatedField<ListenerStatus> {
        &mut self.listener_statuses
    }

    // Take field
    pub fn take_listener_statuses(&mut self) -> ::protobuf::RepeatedField<ListenerStatus> {
        ::std::mem::replace(&mut self.listener_statuses, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Listeners {
    fn is_initialized(&self) -> bool {
        for v in &self.listener_statuses {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.listener_statuses)?;
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
        for value in &self.listener_statuses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.listener_statuses {
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

    fn new() -> Listeners {
        Listeners::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ListenerStatus>>(
                "listener_statuses",
                |m: &Listeners| { &m.listener_statuses },
                |m: &mut Listeners| { &mut m.listener_statuses },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Listeners>(
                "Listeners",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Listeners {
        static instance: ::protobuf::rt::LazyV2<Listeners> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Listeners::new)
    }
}

impl ::protobuf::Clear for Listeners {
    fn clear(&mut self) {
        self.listener_statuses.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Listeners {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Listeners {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListenerStatus {
    // message fields
    pub name: ::std::string::String,
    pub local_address: ::protobuf::SingularPtrField<super::address::Address>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ListenerStatus {
    fn default() -> &'a ListenerStatus {
        <ListenerStatus as ::protobuf::Message>::default_instance()
    }
}

impl ListenerStatus {
    pub fn new() -> ListenerStatus {
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

    // .envoy.config.core.v4alpha.Address local_address = 2;


    pub fn get_local_address(&self) -> &super::address::Address {
        self.local_address.as_ref().unwrap_or_else(|| <super::address::Address as ::protobuf::Message>::default_instance())
    }
    pub fn clear_local_address(&mut self) {
        self.local_address.clear();
    }

    pub fn has_local_address(&self) -> bool {
        self.local_address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_local_address(&mut self, v: super::address::Address) {
        self.local_address = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_local_address(&mut self) -> &mut super::address::Address {
        if self.local_address.is_none() {
            self.local_address.set_default();
        }
        self.local_address.as_mut().unwrap()
    }

    // Take field
    pub fn take_local_address(&mut self) -> super::address::Address {
        self.local_address.take().unwrap_or_else(|| super::address::Address::new())
    }
}

impl ::protobuf::Message for ListenerStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.local_address {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.local_address)?;
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
        if let Some(ref v) = self.local_address.as_ref() {
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
        if let Some(ref v) = self.local_address.as_ref() {
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

    fn new() -> ListenerStatus {
        ListenerStatus::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &ListenerStatus| { &m.name },
                |m: &mut ListenerStatus| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::address::Address>>(
                "local_address",
                |m: &ListenerStatus| { &m.local_address },
                |m: &mut ListenerStatus| { &mut m.local_address },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ListenerStatus>(
                "ListenerStatus",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ListenerStatus {
        static instance: ::protobuf::rt::LazyV2<ListenerStatus> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ListenerStatus::new)
    }
}

impl ::protobuf::Clear for ListenerStatus {
    fn clear(&mut self) {
        self.name.clear();
        self.local_address.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListenerStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListenerStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#envoy/admin/v4alpha/listeners.proto\x12\x13envoy.admin.v4alpha\x1a'en\
    voy/config/core/v4alpha/address.proto\x1a\x1dudpa/annotations/status.pro\
    to\x1a!udpa/annotations/versioning.proto\"~\n\tListeners\x12P\n\x11liste\
    ner_statuses\x18\x01\x20\x03(\x0b2#.envoy.admin.v4alpha.ListenerStatusR\
    \x10listenerStatuses:\x1f\x9a\xc5\x88\x1e\x1a\n\x18envoy.admin.v3.Listen\
    ers\"\x93\x01\n\x0eListenerStatus\x12\x12\n\x04name\x18\x01\x20\x01(\tR\
    \x04name\x12G\n\rlocal_address\x18\x02\x20\x01(\x0b2\".envoy.config.core\
    .v4alpha.AddressR\x0clocalAddress:$\x9a\xc5\x88\x1e\x1f\n\x1denvoy.admin\
    .v3.ListenerStatusB=\n!io.envoyproxy.envoy.admin.v4alphaB\x0eListenersPr\
    otoP\x01\xba\x80\xc8\xd1\x06\x02\x10\x03b\x06proto3\
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