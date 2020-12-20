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
//! Generated file from `envoy/admin/v3/memory.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct Memory {
    // message fields
    pub allocated: u64,
    pub heap_size: u64,
    pub pageheap_unmapped: u64,
    pub pageheap_free: u64,
    pub total_thread_cache: u64,
    pub total_physical_bytes: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Memory {
    fn default() -> &'a Memory {
        <Memory as ::protobuf::Message>::default_instance()
    }
}

impl Memory {
    pub fn new() -> Memory {
        ::std::default::Default::default()
    }

    // uint64 allocated = 1;


    pub fn get_allocated(&self) -> u64 {
        self.allocated
    }
    pub fn clear_allocated(&mut self) {
        self.allocated = 0;
    }

    // Param is passed by value, moved
    pub fn set_allocated(&mut self, v: u64) {
        self.allocated = v;
    }

    // uint64 heap_size = 2;


    pub fn get_heap_size(&self) -> u64 {
        self.heap_size
    }
    pub fn clear_heap_size(&mut self) {
        self.heap_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_heap_size(&mut self, v: u64) {
        self.heap_size = v;
    }

    // uint64 pageheap_unmapped = 3;


    pub fn get_pageheap_unmapped(&self) -> u64 {
        self.pageheap_unmapped
    }
    pub fn clear_pageheap_unmapped(&mut self) {
        self.pageheap_unmapped = 0;
    }

    // Param is passed by value, moved
    pub fn set_pageheap_unmapped(&mut self, v: u64) {
        self.pageheap_unmapped = v;
    }

    // uint64 pageheap_free = 4;


    pub fn get_pageheap_free(&self) -> u64 {
        self.pageheap_free
    }
    pub fn clear_pageheap_free(&mut self) {
        self.pageheap_free = 0;
    }

    // Param is passed by value, moved
    pub fn set_pageheap_free(&mut self, v: u64) {
        self.pageheap_free = v;
    }

    // uint64 total_thread_cache = 5;


    pub fn get_total_thread_cache(&self) -> u64 {
        self.total_thread_cache
    }
    pub fn clear_total_thread_cache(&mut self) {
        self.total_thread_cache = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_thread_cache(&mut self, v: u64) {
        self.total_thread_cache = v;
    }

    // uint64 total_physical_bytes = 6;


    pub fn get_total_physical_bytes(&self) -> u64 {
        self.total_physical_bytes
    }
    pub fn clear_total_physical_bytes(&mut self) {
        self.total_physical_bytes = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_physical_bytes(&mut self, v: u64) {
        self.total_physical_bytes = v;
    }
}

impl ::protobuf::Message for Memory {
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
                    self.allocated = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.heap_size = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.pageheap_unmapped = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.pageheap_free = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.total_thread_cache = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.total_physical_bytes = tmp;
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
        if self.allocated != 0 {
            my_size += ::protobuf::rt::value_size(1, self.allocated, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.heap_size != 0 {
            my_size += ::protobuf::rt::value_size(2, self.heap_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.pageheap_unmapped != 0 {
            my_size += ::protobuf::rt::value_size(3, self.pageheap_unmapped, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.pageheap_free != 0 {
            my_size += ::protobuf::rt::value_size(4, self.pageheap_free, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.total_thread_cache != 0 {
            my_size += ::protobuf::rt::value_size(5, self.total_thread_cache, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.total_physical_bytes != 0 {
            my_size += ::protobuf::rt::value_size(6, self.total_physical_bytes, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.allocated != 0 {
            os.write_uint64(1, self.allocated)?;
        }
        if self.heap_size != 0 {
            os.write_uint64(2, self.heap_size)?;
        }
        if self.pageheap_unmapped != 0 {
            os.write_uint64(3, self.pageheap_unmapped)?;
        }
        if self.pageheap_free != 0 {
            os.write_uint64(4, self.pageheap_free)?;
        }
        if self.total_thread_cache != 0 {
            os.write_uint64(5, self.total_thread_cache)?;
        }
        if self.total_physical_bytes != 0 {
            os.write_uint64(6, self.total_physical_bytes)?;
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

    fn new() -> Memory {
        Memory::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "allocated",
                |m: &Memory| { &m.allocated },
                |m: &mut Memory| { &mut m.allocated },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "heap_size",
                |m: &Memory| { &m.heap_size },
                |m: &mut Memory| { &mut m.heap_size },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "pageheap_unmapped",
                |m: &Memory| { &m.pageheap_unmapped },
                |m: &mut Memory| { &mut m.pageheap_unmapped },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "pageheap_free",
                |m: &Memory| { &m.pageheap_free },
                |m: &mut Memory| { &mut m.pageheap_free },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "total_thread_cache",
                |m: &Memory| { &m.total_thread_cache },
                |m: &mut Memory| { &mut m.total_thread_cache },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "total_physical_bytes",
                |m: &Memory| { &m.total_physical_bytes },
                |m: &mut Memory| { &mut m.total_physical_bytes },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Memory>(
                "Memory",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Memory {
        static instance: ::protobuf::rt::LazyV2<Memory> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Memory::new)
    }
}

impl ::protobuf::Clear for Memory {
    fn clear(&mut self) {
        self.allocated = 0;
        self.heap_size = 0;
        self.pageheap_unmapped = 0;
        self.pageheap_free = 0;
        self.total_thread_cache = 0;
        self.total_physical_bytes = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Memory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Memory {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1benvoy/admin/v3/memory.proto\x12\x0eenvoy.admin.v3\x1a\x1dudpa/anno\
    tations/status.proto\x1a!udpa/annotations/versioning.proto\"\x98\x02\n\
    \x06Memory\x12\x1c\n\tallocated\x18\x01\x20\x01(\x04R\tallocated\x12\x1b\
    \n\theap_size\x18\x02\x20\x01(\x04R\x08heapSize\x12+\n\x11pageheap_unmap\
    ped\x18\x03\x20\x01(\x04R\x10pageheapUnmapped\x12#\n\rpageheap_free\x18\
    \x04\x20\x01(\x04R\x0cpageheapFree\x12,\n\x12total_thread_cache\x18\x05\
    \x20\x01(\x04R\x10totalThreadCache\x120\n\x14total_physical_bytes\x18\
    \x06\x20\x01(\x04R\x12totalPhysicalBytes:!\x9a\xc5\x88\x1e\x1c\n\x1aenvo\
    y.admin.v2alpha.MemoryB5\n\x1cio.envoyproxy.envoy.admin.v3B\x0bMemoryPro\
    toP\x01\xba\x80\xc8\xd1\x06\x02\x10\x02b\x06proto3\
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
