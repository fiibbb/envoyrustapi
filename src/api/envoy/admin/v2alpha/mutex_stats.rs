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
//! Generated file from `envoy/admin/v2alpha/mutex_stats.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct MutexStats {
    // message fields
    pub num_contentions: u64,
    pub current_wait_cycles: u64,
    pub lifetime_wait_cycles: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MutexStats {
    fn default() -> &'a MutexStats {
        <MutexStats as ::protobuf::Message>::default_instance()
    }
}

impl MutexStats {
    pub fn new() -> MutexStats {
        ::std::default::Default::default()
    }

    // uint64 num_contentions = 1;


    pub fn get_num_contentions(&self) -> u64 {
        self.num_contentions
    }
    pub fn clear_num_contentions(&mut self) {
        self.num_contentions = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_contentions(&mut self, v: u64) {
        self.num_contentions = v;
    }

    // uint64 current_wait_cycles = 2;


    pub fn get_current_wait_cycles(&self) -> u64 {
        self.current_wait_cycles
    }
    pub fn clear_current_wait_cycles(&mut self) {
        self.current_wait_cycles = 0;
    }

    // Param is passed by value, moved
    pub fn set_current_wait_cycles(&mut self, v: u64) {
        self.current_wait_cycles = v;
    }

    // uint64 lifetime_wait_cycles = 3;


    pub fn get_lifetime_wait_cycles(&self) -> u64 {
        self.lifetime_wait_cycles
    }
    pub fn clear_lifetime_wait_cycles(&mut self) {
        self.lifetime_wait_cycles = 0;
    }

    // Param is passed by value, moved
    pub fn set_lifetime_wait_cycles(&mut self, v: u64) {
        self.lifetime_wait_cycles = v;
    }
}

impl ::protobuf::Message for MutexStats {
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
                    let tmp = is.read_uint64()?;
                    self.num_contentions = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.current_wait_cycles = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.lifetime_wait_cycles = tmp;
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
        if self.num_contentions != 0 {
            my_size += ::protobuf::rt::value_size(1, self.num_contentions, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.current_wait_cycles != 0 {
            my_size += ::protobuf::rt::value_size(2, self.current_wait_cycles, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.lifetime_wait_cycles != 0 {
            my_size += ::protobuf::rt::value_size(3, self.lifetime_wait_cycles, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.num_contentions != 0 {
            os.write_uint64(1, self.num_contentions)?;
        }
        if self.current_wait_cycles != 0 {
            os.write_uint64(2, self.current_wait_cycles)?;
        }
        if self.lifetime_wait_cycles != 0 {
            os.write_uint64(3, self.lifetime_wait_cycles)?;
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

    fn new() -> MutexStats {
        MutexStats::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "num_contentions",
                |m: &MutexStats| { &m.num_contentions },
                |m: &mut MutexStats| { &mut m.num_contentions },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "current_wait_cycles",
                |m: &MutexStats| { &m.current_wait_cycles },
                |m: &mut MutexStats| { &mut m.current_wait_cycles },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "lifetime_wait_cycles",
                |m: &MutexStats| { &m.lifetime_wait_cycles },
                |m: &mut MutexStats| { &mut m.lifetime_wait_cycles },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MutexStats>(
                "MutexStats",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MutexStats {
        static instance: ::protobuf::rt::LazyV2<MutexStats> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MutexStats::new)
    }
}

impl ::protobuf::Clear for MutexStats {
    fn clear(&mut self) {
        self.num_contentions = 0;
        self.current_wait_cycles = 0;
        self.lifetime_wait_cycles = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MutexStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MutexStats {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%envoy/admin/v2alpha/mutex_stats.proto\x12\x13envoy.admin.v2alpha\x1a\
    \x1dudpa/annotations/status.proto\"\x97\x01\n\nMutexStats\x12'\n\x0fnum_\
    contentions\x18\x01\x20\x01(\x04R\x0enumContentions\x12.\n\x13current_wa\
    it_cycles\x18\x02\x20\x01(\x04R\x11currentWaitCycles\x120\n\x14lifetime_\
    wait_cycles\x18\x03\x20\x01(\x04R\x12lifetimeWaitCyclesB>\n!io.envoyprox\
    y.envoy.admin.v2alphaB\x0fMutexStatsProtoP\x01\xba\x80\xc8\xd1\x06\x02\
    \x10\x01b\x06proto3\
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
