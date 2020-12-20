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
//! Generated file from `envoy/service/auth/v2/external_auth.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct CheckRequest {
    // message fields
    pub attributes: ::protobuf::SingularPtrField<super::attribute_context::AttributeContext>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CheckRequest {
    fn default() -> &'a CheckRequest {
        <CheckRequest as ::protobuf::Message>::default_instance()
    }
}

impl CheckRequest {
    pub fn new() -> CheckRequest {
        ::std::default::Default::default()
    }

    // .envoy.service.auth.v2.AttributeContext attributes = 1;


    pub fn get_attributes(&self) -> &super::attribute_context::AttributeContext {
        self.attributes.as_ref().unwrap_or_else(|| <super::attribute_context::AttributeContext as ::protobuf::Message>::default_instance())
    }
    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }

    pub fn has_attributes(&self) -> bool {
        self.attributes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attributes(&mut self, v: super::attribute_context::AttributeContext) {
        self.attributes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_attributes(&mut self) -> &mut super::attribute_context::AttributeContext {
        if self.attributes.is_none() {
            self.attributes.set_default();
        }
        self.attributes.as_mut().unwrap()
    }

    // Take field
    pub fn take_attributes(&mut self) -> super::attribute_context::AttributeContext {
        self.attributes.take().unwrap_or_else(|| super::attribute_context::AttributeContext::new())
    }
}

impl ::protobuf::Message for CheckRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.attributes {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.attributes)?;
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
        if let Some(ref v) = self.attributes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.attributes.as_ref() {
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

    fn new() -> CheckRequest {
        CheckRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::attribute_context::AttributeContext>>(
                "attributes",
                |m: &CheckRequest| { &m.attributes },
                |m: &mut CheckRequest| { &mut m.attributes },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CheckRequest>(
                "CheckRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CheckRequest {
        static instance: ::protobuf::rt::LazyV2<CheckRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CheckRequest::new)
    }
}

impl ::protobuf::Clear for CheckRequest {
    fn clear(&mut self) {
        self.attributes.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CheckRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CheckRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeniedHttpResponse {
    // message fields
    pub status: ::protobuf::SingularPtrField<super::http_status::HttpStatus>,
    pub headers: ::protobuf::RepeatedField<super::base::HeaderValueOption>,
    pub body: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DeniedHttpResponse {
    fn default() -> &'a DeniedHttpResponse {
        <DeniedHttpResponse as ::protobuf::Message>::default_instance()
    }
}

impl DeniedHttpResponse {
    pub fn new() -> DeniedHttpResponse {
        ::std::default::Default::default()
    }

    // .envoy.type.HttpStatus status = 1;


    pub fn get_status(&self) -> &super::http_status::HttpStatus {
        self.status.as_ref().unwrap_or_else(|| <super::http_status::HttpStatus as ::protobuf::Message>::default_instance())
    }
    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: super::http_status::HttpStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut super::http_status::HttpStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> super::http_status::HttpStatus {
        self.status.take().unwrap_or_else(|| super::http_status::HttpStatus::new())
    }

    // repeated .envoy.api.v2.core.HeaderValueOption headers = 2;


    pub fn get_headers(&self) -> &[super::base::HeaderValueOption] {
        &self.headers
    }
    pub fn clear_headers(&mut self) {
        self.headers.clear();
    }

    // Param is passed by value, moved
    pub fn set_headers(&mut self, v: ::protobuf::RepeatedField<super::base::HeaderValueOption>) {
        self.headers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_headers(&mut self) -> &mut ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        &mut self.headers
    }

    // Take field
    pub fn take_headers(&mut self) -> ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        ::std::mem::replace(&mut self.headers, ::protobuf::RepeatedField::new())
    }

    // string body = 3;


    pub fn get_body(&self) -> &str {
        &self.body
    }
    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: ::std::string::String) {
        self.body = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut ::std::string::String {
        &mut self.body
    }

    // Take field
    pub fn take_body(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.body, ::std::string::String::new())
    }
}

impl ::protobuf::Message for DeniedHttpResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.status {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.headers {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.status)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.headers)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.body)?;
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
        if let Some(ref v) = self.status.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.headers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.body.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.body);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.status.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.headers {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.body.is_empty() {
            os.write_string(3, &self.body)?;
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

    fn new() -> DeniedHttpResponse {
        DeniedHttpResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::http_status::HttpStatus>>(
                "status",
                |m: &DeniedHttpResponse| { &m.status },
                |m: &mut DeniedHttpResponse| { &mut m.status },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::HeaderValueOption>>(
                "headers",
                |m: &DeniedHttpResponse| { &m.headers },
                |m: &mut DeniedHttpResponse| { &mut m.headers },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "body",
                |m: &DeniedHttpResponse| { &m.body },
                |m: &mut DeniedHttpResponse| { &mut m.body },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<DeniedHttpResponse>(
                "DeniedHttpResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static DeniedHttpResponse {
        static instance: ::protobuf::rt::LazyV2<DeniedHttpResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(DeniedHttpResponse::new)
    }
}

impl ::protobuf::Clear for DeniedHttpResponse {
    fn clear(&mut self) {
        self.status.clear();
        self.headers.clear();
        self.body.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeniedHttpResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeniedHttpResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OkHttpResponse {
    // message fields
    pub headers: ::protobuf::RepeatedField<super::base::HeaderValueOption>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a OkHttpResponse {
    fn default() -> &'a OkHttpResponse {
        <OkHttpResponse as ::protobuf::Message>::default_instance()
    }
}

impl OkHttpResponse {
    pub fn new() -> OkHttpResponse {
        ::std::default::Default::default()
    }

    // repeated .envoy.api.v2.core.HeaderValueOption headers = 2;


    pub fn get_headers(&self) -> &[super::base::HeaderValueOption] {
        &self.headers
    }
    pub fn clear_headers(&mut self) {
        self.headers.clear();
    }

    // Param is passed by value, moved
    pub fn set_headers(&mut self, v: ::protobuf::RepeatedField<super::base::HeaderValueOption>) {
        self.headers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_headers(&mut self) -> &mut ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        &mut self.headers
    }

    // Take field
    pub fn take_headers(&mut self) -> ::protobuf::RepeatedField<super::base::HeaderValueOption> {
        ::std::mem::replace(&mut self.headers, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for OkHttpResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.headers {
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
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.headers)?;
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
        for value in &self.headers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.headers {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> OkHttpResponse {
        OkHttpResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base::HeaderValueOption>>(
                "headers",
                |m: &OkHttpResponse| { &m.headers },
                |m: &mut OkHttpResponse| { &mut m.headers },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<OkHttpResponse>(
                "OkHttpResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static OkHttpResponse {
        static instance: ::protobuf::rt::LazyV2<OkHttpResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(OkHttpResponse::new)
    }
}

impl ::protobuf::Clear for OkHttpResponse {
    fn clear(&mut self) {
        self.headers.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OkHttpResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OkHttpResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CheckResponse {
    // message fields
    pub status: ::protobuf::SingularPtrField<super::status::Status>,
    // message oneof groups
    pub http_response: ::std::option::Option<CheckResponse_oneof_http_response>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CheckResponse {
    fn default() -> &'a CheckResponse {
        <CheckResponse as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum CheckResponse_oneof_http_response {
    denied_response(DeniedHttpResponse),
    ok_response(OkHttpResponse),
}

impl CheckResponse {
    pub fn new() -> CheckResponse {
        ::std::default::Default::default()
    }

    // .google.rpc.Status status = 1;


    pub fn get_status(&self) -> &super::status::Status {
        self.status.as_ref().unwrap_or_else(|| <super::status::Status as ::protobuf::Message>::default_instance())
    }
    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: super::status::Status) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut super::status::Status {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> super::status::Status {
        self.status.take().unwrap_or_else(|| super::status::Status::new())
    }

    // .envoy.service.auth.v2.DeniedHttpResponse denied_response = 2;


    pub fn get_denied_response(&self) -> &DeniedHttpResponse {
        match self.http_response {
            ::std::option::Option::Some(CheckResponse_oneof_http_response::denied_response(ref v)) => v,
            _ => <DeniedHttpResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_denied_response(&mut self) {
        self.http_response = ::std::option::Option::None;
    }

    pub fn has_denied_response(&self) -> bool {
        match self.http_response {
            ::std::option::Option::Some(CheckResponse_oneof_http_response::denied_response(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_denied_response(&mut self, v: DeniedHttpResponse) {
        self.http_response = ::std::option::Option::Some(CheckResponse_oneof_http_response::denied_response(v))
    }

    // Mutable pointer to the field.
    pub fn mut_denied_response(&mut self) -> &mut DeniedHttpResponse {
        if let ::std::option::Option::Some(CheckResponse_oneof_http_response::denied_response(_)) = self.http_response {
        } else {
            self.http_response = ::std::option::Option::Some(CheckResponse_oneof_http_response::denied_response(DeniedHttpResponse::new()));
        }
        match self.http_response {
            ::std::option::Option::Some(CheckResponse_oneof_http_response::denied_response(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_denied_response(&mut self) -> DeniedHttpResponse {
        if self.has_denied_response() {
            match self.http_response.take() {
                ::std::option::Option::Some(CheckResponse_oneof_http_response::denied_response(v)) => v,
                _ => panic!(),
            }
        } else {
            DeniedHttpResponse::new()
        }
    }

    // .envoy.service.auth.v2.OkHttpResponse ok_response = 3;


    pub fn get_ok_response(&self) -> &OkHttpResponse {
        match self.http_response {
            ::std::option::Option::Some(CheckResponse_oneof_http_response::ok_response(ref v)) => v,
            _ => <OkHttpResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_ok_response(&mut self) {
        self.http_response = ::std::option::Option::None;
    }

    pub fn has_ok_response(&self) -> bool {
        match self.http_response {
            ::std::option::Option::Some(CheckResponse_oneof_http_response::ok_response(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ok_response(&mut self, v: OkHttpResponse) {
        self.http_response = ::std::option::Option::Some(CheckResponse_oneof_http_response::ok_response(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ok_response(&mut self) -> &mut OkHttpResponse {
        if let ::std::option::Option::Some(CheckResponse_oneof_http_response::ok_response(_)) = self.http_response {
        } else {
            self.http_response = ::std::option::Option::Some(CheckResponse_oneof_http_response::ok_response(OkHttpResponse::new()));
        }
        match self.http_response {
            ::std::option::Option::Some(CheckResponse_oneof_http_response::ok_response(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ok_response(&mut self) -> OkHttpResponse {
        if self.has_ok_response() {
            match self.http_response.take() {
                ::std::option::Option::Some(CheckResponse_oneof_http_response::ok_response(v)) => v,
                _ => panic!(),
            }
        } else {
            OkHttpResponse::new()
        }
    }
}

impl ::protobuf::Message for CheckResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.status {
            if !v.is_initialized() {
                return false;
            }
        };
        if let Some(CheckResponse_oneof_http_response::denied_response(ref v)) = self.http_response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(CheckResponse_oneof_http_response::ok_response(ref v)) = self.http_response {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.status)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.http_response = ::std::option::Option::Some(CheckResponse_oneof_http_response::denied_response(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.http_response = ::std::option::Option::Some(CheckResponse_oneof_http_response::ok_response(is.read_message()?));
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
        if let Some(ref v) = self.status.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.http_response {
            match v {
                &CheckResponse_oneof_http_response::denied_response(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &CheckResponse_oneof_http_response::ok_response(ref v) => {
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
        if let Some(ref v) = self.status.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.http_response {
            match v {
                &CheckResponse_oneof_http_response::denied_response(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &CheckResponse_oneof_http_response::ok_response(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> CheckResponse {
        CheckResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::status::Status>>(
                "status",
                |m: &CheckResponse| { &m.status },
                |m: &mut CheckResponse| { &mut m.status },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DeniedHttpResponse>(
                "denied_response",
                CheckResponse::has_denied_response,
                CheckResponse::get_denied_response,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, OkHttpResponse>(
                "ok_response",
                CheckResponse::has_ok_response,
                CheckResponse::get_ok_response,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CheckResponse>(
                "CheckResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CheckResponse {
        static instance: ::protobuf::rt::LazyV2<CheckResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CheckResponse::new)
    }
}

impl ::protobuf::Clear for CheckResponse {
    fn clear(&mut self) {
        self.status.clear();
        self.http_response = ::std::option::Option::None;
        self.http_response = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CheckResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CheckResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n)envoy/service/auth/v2/external_auth.proto\x12\x15envoy.service.auth.v\
    2\x1a\x1cenvoy/api/v2/core/base.proto\x1a-envoy/service/auth/v2/attribut\
    e_context.proto\x1a\x1cenvoy/type/http_status.proto\x1a\x17google/rpc/st\
    atus.proto\x1a\x1dudpa/annotations/status.proto\x1a\x17validate/validate\
    .proto\"W\n\x0cCheckRequest\x12G\n\nattributes\x18\x01\x20\x01(\x0b2'.en\
    voy.service.auth.v2.AttributeContextR\nattributes\"\xa2\x01\n\x12DeniedH\
    ttpResponse\x128\n\x06status\x18\x01\x20\x01(\x0b2\x16.envoy.type.HttpSt\
    atusR\x06statusB\x08\xfaB\x05\x8a\x01\x02\x10\x01\x12>\n\x07headers\x18\
    \x02\x20\x03(\x0b2$.envoy.api.v2.core.HeaderValueOptionR\x07headers\x12\
    \x12\n\x04body\x18\x03\x20\x01(\tR\x04body\"P\n\x0eOkHttpResponse\x12>\n\
    \x07headers\x18\x02\x20\x03(\x0b2$.envoy.api.v2.core.HeaderValueOptionR\
    \x07headers\"\xec\x01\n\rCheckResponse\x12*\n\x06status\x18\x01\x20\x01(\
    \x0b2\x12.google.rpc.StatusR\x06status\x12T\n\x0fdenied_response\x18\x02\
    \x20\x01(\x0b2).envoy.service.auth.v2.DeniedHttpResponseH\0R\x0edeniedRe\
    sponse\x12H\n\x0bok_response\x18\x03\x20\x01(\x0b2%.envoy.service.auth.v\
    2.OkHttpResponseH\0R\nokResponseB\x0f\n\rhttp_response2e\n\rAuthorizatio\
    n\x12T\n\x05Check\x12#.envoy.service.auth.v2.CheckRequest\x1a$.envoy.ser\
    vice.auth.v2.CheckResponse\"\0BE\n#io.envoyproxy.envoy.service.auth.v2B\
    \x11ExternalAuthProtoP\x01\x88\x01\x01\xba\x80\xc8\xd1\x06\x02\x10\x01b\
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