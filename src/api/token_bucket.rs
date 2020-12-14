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
//! Generated file from `envoy/type/v3/token_bucket.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct TokenBucket {
    // message fields
    pub max_tokens: u32,
    pub tokens_per_fill: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    pub fill_interval: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TokenBucket {
    fn default() -> &'a TokenBucket {
        <TokenBucket as ::protobuf::Message>::default_instance()
    }
}

impl TokenBucket {
    pub fn new() -> TokenBucket {
        ::std::default::Default::default()
    }

    // uint32 max_tokens = 1;


    pub fn get_max_tokens(&self) -> u32 {
        self.max_tokens
    }
    pub fn clear_max_tokens(&mut self) {
        self.max_tokens = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_tokens(&mut self, v: u32) {
        self.max_tokens = v;
    }

    // .google.protobuf.UInt32Value tokens_per_fill = 2;


    pub fn get_tokens_per_fill(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.tokens_per_fill.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::UInt32Value as ::protobuf::Message>::default_instance())
    }
    pub fn clear_tokens_per_fill(&mut self) {
        self.tokens_per_fill.clear();
    }

    pub fn has_tokens_per_fill(&self) -> bool {
        self.tokens_per_fill.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tokens_per_fill(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.tokens_per_fill = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tokens_per_fill(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.tokens_per_fill.is_none() {
            self.tokens_per_fill.set_default();
        }
        self.tokens_per_fill.as_mut().unwrap()
    }

    // Take field
    pub fn take_tokens_per_fill(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.tokens_per_fill.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }

    // .google.protobuf.Duration fill_interval = 3;


    pub fn get_fill_interval(&self) -> &::protobuf::well_known_types::Duration {
        self.fill_interval.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Duration as ::protobuf::Message>::default_instance())
    }
    pub fn clear_fill_interval(&mut self) {
        self.fill_interval.clear();
    }

    pub fn has_fill_interval(&self) -> bool {
        self.fill_interval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fill_interval(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.fill_interval = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fill_interval(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.fill_interval.is_none() {
            self.fill_interval.set_default();
        }
        self.fill_interval.as_mut().unwrap()
    }

    // Take field
    pub fn take_fill_interval(&mut self) -> ::protobuf::well_known_types::Duration {
        self.fill_interval.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }
}

impl ::protobuf::Message for TokenBucket {
    fn is_initialized(&self) -> bool {
        for v in &self.tokens_per_fill {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.fill_interval {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.max_tokens = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tokens_per_fill)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fill_interval)?;
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
        if self.max_tokens != 0 {
            my_size += ::protobuf::rt::value_size(1, self.max_tokens, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.tokens_per_fill.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.fill_interval.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.max_tokens != 0 {
            os.write_uint32(1, self.max_tokens)?;
        }
        if let Some(ref v) = self.tokens_per_fill.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.fill_interval.as_ref() {
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

    fn new() -> TokenBucket {
        TokenBucket::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "max_tokens",
                |m: &TokenBucket| { &m.max_tokens },
                |m: &mut TokenBucket| { &mut m.max_tokens },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                "tokens_per_fill",
                |m: &TokenBucket| { &m.tokens_per_fill },
                |m: &mut TokenBucket| { &mut m.tokens_per_fill },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                "fill_interval",
                |m: &TokenBucket| { &m.fill_interval },
                |m: &mut TokenBucket| { &mut m.fill_interval },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TokenBucket>(
                "TokenBucket",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TokenBucket {
        static instance: ::protobuf::rt::LazyV2<TokenBucket> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TokenBucket::new)
    }
}

impl ::protobuf::Clear for TokenBucket {
    fn clear(&mut self) {
        self.max_tokens = 0;
        self.tokens_per_fill.clear();
        self.fill_interval.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TokenBucket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TokenBucket {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20envoy/type/v3/token_bucket.proto\x12\renvoy.type.v3\x1a\x1egoogle/\
    protobuf/duration.proto\x1a\x1egoogle/protobuf/wrappers.proto\x1a\x1dudp\
    a/annotations/status.proto\x1a!udpa/annotations/versioning.proto\x1a\x17\
    validate/validate.proto\"\xef\x01\n\x0bTokenBucket\x12&\n\nmax_tokens\
    \x18\x01\x20\x01(\rR\tmaxTokensB\x07\xfaB\x04*\x02\x20\0\x12M\n\x0ftoken\
    s_per_fill\x18\x02\x20\x01(\x0b2\x1c.google.protobuf.UInt32ValueR\rtoken\
    sPerFillB\x07\xfaB\x04*\x02\x20\0\x12J\n\rfill_interval\x18\x03\x20\x01(\
    \x0b2\x19.google.protobuf.DurationR\x0cfillIntervalB\n\xfaB\x07\xaa\x01\
    \x04\x08\x01*\0:\x1d\x9a\xc5\x88\x1e\x18\n\x16envoy.type.TokenBucketB9\n\
    \x1bio.envoyproxy.envoy.type.v3B\x10TokenBucketProtoP\x01\xba\x80\xc8\
    \xd1\x06\x02\x10\x02b\x06proto3\
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