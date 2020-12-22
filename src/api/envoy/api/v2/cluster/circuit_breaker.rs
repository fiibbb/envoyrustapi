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
//! Generated file from `envoy/api/v2/cluster/circuit_breaker.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct CircuitBreakers {
    // message fields
    pub thresholds: ::protobuf::RepeatedField<CircuitBreakers_Thresholds>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CircuitBreakers {
    fn default() -> &'a CircuitBreakers {
        <CircuitBreakers as ::protobuf::Message>::default_instance()
    }
}

impl CircuitBreakers {
    pub fn new() -> CircuitBreakers {
        ::std::default::Default::default()
    }

    // repeated .envoy.api.v2.cluster.CircuitBreakers.Thresholds thresholds = 1;


    pub fn get_thresholds(&self) -> &[CircuitBreakers_Thresholds] {
        &self.thresholds
    }
    pub fn clear_thresholds(&mut self) {
        self.thresholds.clear();
    }

    // Param is passed by value, moved
    pub fn set_thresholds(&mut self, v: ::protobuf::RepeatedField<CircuitBreakers_Thresholds>) {
        self.thresholds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_thresholds(&mut self) -> &mut ::protobuf::RepeatedField<CircuitBreakers_Thresholds> {
        &mut self.thresholds
    }

    // Take field
    pub fn take_thresholds(&mut self) -> ::protobuf::RepeatedField<CircuitBreakers_Thresholds> {
        ::std::mem::replace(&mut self.thresholds, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for CircuitBreakers {
    fn is_initialized(&self) -> bool {
        for v in &self.thresholds {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.thresholds)?;
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
        for value in &self.thresholds {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.thresholds {
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

    fn new() -> CircuitBreakers {
        CircuitBreakers::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CircuitBreakers_Thresholds>>(
                "thresholds",
                |m: &CircuitBreakers| { &m.thresholds },
                |m: &mut CircuitBreakers| { &mut m.thresholds },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CircuitBreakers>(
                "CircuitBreakers",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CircuitBreakers {
        static instance: ::protobuf::rt::LazyV2<CircuitBreakers> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CircuitBreakers::new)
    }
}

impl ::protobuf::Clear for CircuitBreakers {
    fn clear(&mut self) {
        self.thresholds.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CircuitBreakers {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CircuitBreakers {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CircuitBreakers_Thresholds {
    // message fields
    pub priority: super::base::RoutingPriority,
    pub max_connections: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub max_pending_requests: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub max_requests: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub max_retries: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub retry_budget: ::protobuf::SingularPtrField<CircuitBreakers_Thresholds_RetryBudget>,
    pub track_remaining: bool,
    pub max_connection_pools: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CircuitBreakers_Thresholds {
    fn default() -> &'a CircuitBreakers_Thresholds {
        <CircuitBreakers_Thresholds as ::protobuf::Message>::default_instance()
    }
}

impl CircuitBreakers_Thresholds {
    pub fn new() -> CircuitBreakers_Thresholds {
        ::std::default::Default::default()
    }

    // .envoy.api.v2.core.RoutingPriority priority = 1;


    pub fn get_priority(&self) -> super::base::RoutingPriority {
        self.priority
    }
    pub fn clear_priority(&mut self) {
        self.priority = super::base::RoutingPriority::DEFAULT;
    }

    // Param is passed by value, moved
    pub fn set_priority(&mut self, v: super::base::RoutingPriority) {
        self.priority = v;
    }

    // .google.protobuf.UInt32Value max_connections = 2;


    pub fn get_max_connections(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.max_connections.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::UInt32Value as ::protobuf::Message>::default_instance())
    }
    pub fn clear_max_connections(&mut self) {
        self.max_connections.clear();
    }

    pub fn has_max_connections(&self) -> bool {
        self.max_connections.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_connections(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.max_connections = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max_connections(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.max_connections.is_none() {
            self.max_connections.set_default();
        }
        self.max_connections.as_mut().unwrap()
    }

    // Take field
    pub fn take_max_connections(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.max_connections.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    // .google.protobuf.UInt32Value max_pending_requests = 3;


    pub fn get_max_pending_requests(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.max_pending_requests.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::UInt32Value as ::protobuf::Message>::default_instance())
    }
    pub fn clear_max_pending_requests(&mut self) {
        self.max_pending_requests.clear();
    }

    pub fn has_max_pending_requests(&self) -> bool {
        self.max_pending_requests.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_pending_requests(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.max_pending_requests = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max_pending_requests(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.max_pending_requests.is_none() {
            self.max_pending_requests.set_default();
        }
        self.max_pending_requests.as_mut().unwrap()
    }

    // Take field
    pub fn take_max_pending_requests(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.max_pending_requests.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    // .google.protobuf.UInt32Value max_requests = 4;


    pub fn get_max_requests(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.max_requests.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::UInt32Value as ::protobuf::Message>::default_instance())
    }
    pub fn clear_max_requests(&mut self) {
        self.max_requests.clear();
    }

    pub fn has_max_requests(&self) -> bool {
        self.max_requests.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_requests(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.max_requests = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max_requests(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.max_requests.is_none() {
            self.max_requests.set_default();
        }
        self.max_requests.as_mut().unwrap()
    }

    // Take field
    pub fn take_max_requests(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.max_requests.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    // .google.protobuf.UInt32Value max_retries = 5;


    pub fn get_max_retries(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.max_retries.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::UInt32Value as ::protobuf::Message>::default_instance())
    }
    pub fn clear_max_retries(&mut self) {
        self.max_retries.clear();
    }

    pub fn has_max_retries(&self) -> bool {
        self.max_retries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_retries(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.max_retries = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max_retries(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.max_retries.is_none() {
            self.max_retries.set_default();
        }
        self.max_retries.as_mut().unwrap()
    }

    // Take field
    pub fn take_max_retries(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.max_retries.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    // .envoy.api.v2.cluster.CircuitBreakers.Thresholds.RetryBudget retry_budget = 8;


    pub fn get_retry_budget(&self) -> &CircuitBreakers_Thresholds_RetryBudget {
        self.retry_budget.as_ref().unwrap_or_else(|| <CircuitBreakers_Thresholds_RetryBudget as ::protobuf::Message>::default_instance())
    }
    pub fn clear_retry_budget(&mut self) {
        self.retry_budget.clear();
    }

    pub fn has_retry_budget(&self) -> bool {
        self.retry_budget.is_some()
    }

    // Param is passed by value, moved
    pub fn set_retry_budget(&mut self, v: CircuitBreakers_Thresholds_RetryBudget) {
        self.retry_budget = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_retry_budget(&mut self) -> &mut CircuitBreakers_Thresholds_RetryBudget {
        if self.retry_budget.is_none() {
            self.retry_budget.set_default();
        }
        self.retry_budget.as_mut().unwrap()
    }

    // Take field
    pub fn take_retry_budget(&mut self) -> CircuitBreakers_Thresholds_RetryBudget {
        self.retry_budget.take().unwrap_or_else(|| CircuitBreakers_Thresholds_RetryBudget::new())
    }

    // bool track_remaining = 6;


    pub fn get_track_remaining(&self) -> bool {
        self.track_remaining
    }
    pub fn clear_track_remaining(&mut self) {
        self.track_remaining = false;
    }

    // Param is passed by value, moved
    pub fn set_track_remaining(&mut self, v: bool) {
        self.track_remaining = v;
    }

    // .google.protobuf.UInt32Value max_connection_pools = 7;


    pub fn get_max_connection_pools(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.max_connection_pools.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::UInt32Value as ::protobuf::Message>::default_instance())
    }
    pub fn clear_max_connection_pools(&mut self) {
        self.max_connection_pools.clear();
    }

    pub fn has_max_connection_pools(&self) -> bool {
        self.max_connection_pools.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_connection_pools(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.max_connection_pools = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max_connection_pools(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.max_connection_pools.is_none() {
            self.max_connection_pools.set_default();
        }
        self.max_connection_pools.as_mut().unwrap()
    }

    // Take field
    pub fn take_max_connection_pools(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.max_connection_pools.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }
}

impl ::protobuf::Message for CircuitBreakers_Thresholds {
    fn is_initialized(&self) -> bool {
        for v in &self.max_connections {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.max_pending_requests {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.max_requests {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.max_retries {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.retry_budget {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.max_connection_pools {
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
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.priority, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.max_connections)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.max_pending_requests)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.max_requests)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.max_retries)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.retry_budget)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.track_remaining = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.max_connection_pools)?;
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
        if self.priority != super::base::RoutingPriority::DEFAULT {
            my_size += ::protobuf::rt::enum_size(1, self.priority);
        }
        if let Some(ref v) = self.max_connections.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.max_pending_requests.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.max_requests.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.max_retries.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.retry_budget.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.track_remaining != false {
            my_size += 2;
        }
        if let Some(ref v) = self.max_connection_pools.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.priority != super::base::RoutingPriority::DEFAULT {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.priority))?;
        }
        if let Some(ref v) = self.max_connections.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.max_pending_requests.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.max_requests.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.max_retries.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.retry_budget.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.track_remaining != false {
            os.write_bool(6, self.track_remaining)?;
        }
        if let Some(ref v) = self.max_connection_pools.as_ref() {
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

    fn new() -> CircuitBreakers_Thresholds {
        CircuitBreakers_Thresholds::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::base::RoutingPriority>>(
                "priority",
                |m: &CircuitBreakers_Thresholds| { &m.priority },
                |m: &mut CircuitBreakers_Thresholds| { &mut m.priority },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                "max_connections",
                |m: &CircuitBreakers_Thresholds| { &m.max_connections },
                |m: &mut CircuitBreakers_Thresholds| { &mut m.max_connections },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                "max_pending_requests",
                |m: &CircuitBreakers_Thresholds| { &m.max_pending_requests },
                |m: &mut CircuitBreakers_Thresholds| { &mut m.max_pending_requests },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                "max_requests",
                |m: &CircuitBreakers_Thresholds| { &m.max_requests },
                |m: &mut CircuitBreakers_Thresholds| { &mut m.max_requests },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                "max_retries",
                |m: &CircuitBreakers_Thresholds| { &m.max_retries },
                |m: &mut CircuitBreakers_Thresholds| { &mut m.max_retries },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CircuitBreakers_Thresholds_RetryBudget>>(
                "retry_budget",
                |m: &CircuitBreakers_Thresholds| { &m.retry_budget },
                |m: &mut CircuitBreakers_Thresholds| { &mut m.retry_budget },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "track_remaining",
                |m: &CircuitBreakers_Thresholds| { &m.track_remaining },
                |m: &mut CircuitBreakers_Thresholds| { &mut m.track_remaining },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                "max_connection_pools",
                |m: &CircuitBreakers_Thresholds| { &m.max_connection_pools },
                |m: &mut CircuitBreakers_Thresholds| { &mut m.max_connection_pools },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CircuitBreakers_Thresholds>(
                "CircuitBreakers.Thresholds",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CircuitBreakers_Thresholds {
        static instance: ::protobuf::rt::LazyV2<CircuitBreakers_Thresholds> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CircuitBreakers_Thresholds::new)
    }
}

impl ::protobuf::Clear for CircuitBreakers_Thresholds {
    fn clear(&mut self) {
        self.priority = super::base::RoutingPriority::DEFAULT;
        self.max_connections.clear();
        self.max_pending_requests.clear();
        self.max_requests.clear();
        self.max_retries.clear();
        self.retry_budget.clear();
        self.track_remaining = false;
        self.max_connection_pools.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CircuitBreakers_Thresholds {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CircuitBreakers_Thresholds {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CircuitBreakers_Thresholds_RetryBudget {
    // message fields
    pub budget_percent: ::protobuf::SingularPtrField<super::percent::Percent>,
    pub min_retry_concurrency: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CircuitBreakers_Thresholds_RetryBudget {
    fn default() -> &'a CircuitBreakers_Thresholds_RetryBudget {
        <CircuitBreakers_Thresholds_RetryBudget as ::protobuf::Message>::default_instance()
    }
}

impl CircuitBreakers_Thresholds_RetryBudget {
    pub fn new() -> CircuitBreakers_Thresholds_RetryBudget {
        ::std::default::Default::default()
    }

    // .envoy.type.Percent budget_percent = 1;


    pub fn get_budget_percent(&self) -> &super::percent::Percent {
        self.budget_percent.as_ref().unwrap_or_else(|| <super::percent::Percent as ::protobuf::Message>::default_instance())
    }
    pub fn clear_budget_percent(&mut self) {
        self.budget_percent.clear();
    }

    pub fn has_budget_percent(&self) -> bool {
        self.budget_percent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_budget_percent(&mut self, v: super::percent::Percent) {
        self.budget_percent = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_budget_percent(&mut self) -> &mut super::percent::Percent {
        if self.budget_percent.is_none() {
            self.budget_percent.set_default();
        }
        self.budget_percent.as_mut().unwrap()
    }

    // Take field
    pub fn take_budget_percent(&mut self) -> super::percent::Percent {
        self.budget_percent.take().unwrap_or_else(|| super::percent::Percent::new())
    }

    // .google.protobuf.UInt32Value min_retry_concurrency = 2;


    pub fn get_min_retry_concurrency(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.min_retry_concurrency.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::UInt32Value as ::protobuf::Message>::default_instance())
    }
    pub fn clear_min_retry_concurrency(&mut self) {
        self.min_retry_concurrency.clear();
    }

    pub fn has_min_retry_concurrency(&self) -> bool {
        self.min_retry_concurrency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min_retry_concurrency(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.min_retry_concurrency = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_min_retry_concurrency(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.min_retry_concurrency.is_none() {
            self.min_retry_concurrency.set_default();
        }
        self.min_retry_concurrency.as_mut().unwrap()
    }

    // Take field
    pub fn take_min_retry_concurrency(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.min_retry_concurrency.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }
}

impl ::protobuf::Message for CircuitBreakers_Thresholds_RetryBudget {
    fn is_initialized(&self) -> bool {
        for v in &self.budget_percent {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.min_retry_concurrency {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.budget_percent)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.min_retry_concurrency)?;
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
        if let Some(ref v) = self.budget_percent.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.min_retry_concurrency.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.budget_percent.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.min_retry_concurrency.as_ref() {
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

    fn new() -> CircuitBreakers_Thresholds_RetryBudget {
        CircuitBreakers_Thresholds_RetryBudget::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::percent::Percent>>(
                "budget_percent",
                |m: &CircuitBreakers_Thresholds_RetryBudget| { &m.budget_percent },
                |m: &mut CircuitBreakers_Thresholds_RetryBudget| { &mut m.budget_percent },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                "min_retry_concurrency",
                |m: &CircuitBreakers_Thresholds_RetryBudget| { &m.min_retry_concurrency },
                |m: &mut CircuitBreakers_Thresholds_RetryBudget| { &mut m.min_retry_concurrency },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CircuitBreakers_Thresholds_RetryBudget>(
                "CircuitBreakers.Thresholds.RetryBudget",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CircuitBreakers_Thresholds_RetryBudget {
        static instance: ::protobuf::rt::LazyV2<CircuitBreakers_Thresholds_RetryBudget> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CircuitBreakers_Thresholds_RetryBudget::new)
    }
}

impl ::protobuf::Clear for CircuitBreakers_Thresholds_RetryBudget {
    fn clear(&mut self) {
        self.budget_percent.clear();
        self.min_retry_concurrency.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CircuitBreakers_Thresholds_RetryBudget {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CircuitBreakers_Thresholds_RetryBudget {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n*envoy/api/v2/cluster/circuit_breaker.proto\x12\x14envoy.api.v2.cluste\
    r\x1a\x1cenvoy/api/v2/core/base.proto\x1a\x18envoy/type/percent.proto\
    \x1a\x1egoogle/protobuf/wrappers.proto\x1a\x1eudpa/annotations/migrate.p\
    roto\x1a\x1dudpa/annotations/status.proto\x1a\x17validate/validate.proto\
    \"\xcb\x06\n\x0fCircuitBreakers\x12P\n\nthresholds\x18\x01\x20\x03(\x0b2\
    0.envoy.api.v2.cluster.CircuitBreakers.ThresholdsR\nthresholds\x1a\xe5\
    \x05\n\nThresholds\x12H\n\x08priority\x18\x01\x20\x01(\x0e2\".envoy.api.\
    v2.core.RoutingPriorityR\x08priorityB\x08\xfaB\x05\x82\x01\x02\x10\x01\
    \x12E\n\x0fmax_connections\x18\x02\x20\x01(\x0b2\x1c.google.protobuf.UIn\
    t32ValueR\x0emaxConnections\x12N\n\x14max_pending_requests\x18\x03\x20\
    \x01(\x0b2\x1c.google.protobuf.UInt32ValueR\x12maxPendingRequests\x12?\n\
    \x0cmax_requests\x18\x04\x20\x01(\x0b2\x1c.google.protobuf.UInt32ValueR\
    \x0bmaxRequests\x12=\n\x0bmax_retries\x18\x05\x20\x01(\x0b2\x1c.google.p\
    rotobuf.UInt32ValueR\nmaxRetries\x12_\n\x0cretry_budget\x18\x08\x20\x01(\
    \x0b2<.envoy.api.v2.cluster.CircuitBreakers.Thresholds.RetryBudgetR\x0br\
    etryBudget\x12'\n\x0ftrack_remaining\x18\x06\x20\x01(\x08R\x0etrackRemai\
    ning\x12N\n\x14max_connection_pools\x18\x07\x20\x01(\x0b2\x1c.google.pro\
    tobuf.UInt32ValueR\x12maxConnectionPools\x1a\x9b\x01\n\x0bRetryBudget\
    \x12:\n\x0ebudget_percent\x18\x01\x20\x01(\x0b2\x13.envoy.type.PercentR\
    \rbudgetPercent\x12P\n\x15min_retry_concurrency\x18\x02\x20\x01(\x0b2\
    \x1c.google.protobuf.UInt32ValueR\x13minRetryConcurrencyB\x94\x01\n\"io.\
    envoyproxy.envoy.api.v2.clusterB\x13CircuitBreakerProtoP\x01\xaa\x02\x16\
    Envoy.Api.V2.ClusterNS\xea\x02\x16Envoy.Api.V2.ClusterNS\xba\x80\xc8\xd1\
    \x06\x02\x10\x01\xf2\x98\xfe\x8f\x05\x19\x12\x17envoy.config.cluster.v3b\
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
