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
//! Generated file from `envoy/extensions/filters/listener/original_src/v3/original_src.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct OriginalSrc {
    // message fields
    pub bind_port: bool,
    pub mark: u32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a OriginalSrc {
    fn default() -> &'a OriginalSrc {
        <OriginalSrc as ::protobuf::Message>::default_instance()
    }
}

impl OriginalSrc {
    pub fn new() -> OriginalSrc {
        ::std::default::Default::default()
    }

    // bool bind_port = 1;


    pub fn get_bind_port(&self) -> bool {
        self.bind_port
    }
    pub fn clear_bind_port(&mut self) {
        self.bind_port = false;
    }

    // Param is passed by value, moved
    pub fn set_bind_port(&mut self, v: bool) {
        self.bind_port = v;
    }

    // uint32 mark = 2;


    pub fn get_mark(&self) -> u32 {
        self.mark
    }
    pub fn clear_mark(&mut self) {
        self.mark = 0;
    }

    // Param is passed by value, moved
    pub fn set_mark(&mut self, v: u32) {
        self.mark = v;
    }
}

impl ::protobuf::Message for OriginalSrc {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_bool()?;
                    self.bind_port = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.mark = tmp;
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
        if self.bind_port != false {
            my_size += 2;
        }
        if self.mark != 0 {
            my_size += ::protobuf::rt::value_size(2, self.mark, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.bind_port != false {
            os.write_bool(1, self.bind_port)?;
        }
        if self.mark != 0 {
            os.write_uint32(2, self.mark)?;
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

    fn new() -> OriginalSrc {
        OriginalSrc::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "bind_port",
                |m: &OriginalSrc| { &m.bind_port },
                |m: &mut OriginalSrc| { &mut m.bind_port },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "mark",
                |m: &OriginalSrc| { &m.mark },
                |m: &mut OriginalSrc| { &mut m.mark },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<OriginalSrc>(
                "OriginalSrc",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static OriginalSrc {
        static instance: ::protobuf::rt::LazyV2<OriginalSrc> = ::protobuf::rt::LazyV2::INIT;
        instance.get(OriginalSrc::new)
    }
}

impl ::protobuf::Clear for OriginalSrc {
    fn clear(&mut self) {
        self.bind_port = false;
        self.mark = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OriginalSrc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OriginalSrc {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \nDenvoy/extensions/filters/listener/original_src/v3/original_src.proto\
    \x121envoy.extensions.filters.listener.original_src.v3\x1a\x1dudpa/annot\
    ations/status.proto\x1a!udpa/annotations/versioning.proto\x1a\x17validat\
    e/validate.proto\"\x85\x01\n\x0bOriginalSrc\x12\x1b\n\tbind_port\x18\x01\
    \x20\x01(\x08R\x08bindPort\x12\x12\n\x04mark\x18\x02\x20\x01(\rR\x04mark\
    :E\x9a\xc5\x88\x1e@\n>envoy.config.filter.listener.original_src.v2alpha1\
    .OriginalSrcB]\n?io.envoyproxy.envoy.extensions.filters.listener.origina\
    l_src.v3B\x10OriginalSrcProtoP\x01\xba\x80\xc8\xd1\x06\x02\x10\x02b\x06p\
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