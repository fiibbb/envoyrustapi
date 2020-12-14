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
//! Generated file from `google/protobuf/struct.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct Struct {
    // message fields
    pub fields: ::std::collections::HashMap<::std::string::String, Value>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Struct {
    fn default() -> &'a Struct {
        <Struct as ::protobuf::Message>::default_instance()
    }
}

impl Struct {
    pub fn new() -> Struct {
        ::std::default::Default::default()
    }

    // repeated .google.protobuf.Struct.FieldsEntry fields = 1;


    pub fn get_fields(&self) -> &::std::collections::HashMap<::std::string::String, Value> {
        &self.fields
    }
    pub fn clear_fields(&mut self) {
        self.fields.clear();
    }

    // Param is passed by value, moved
    pub fn set_fields(&mut self, v: ::std::collections::HashMap<::std::string::String, Value>) {
        self.fields = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fields(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, Value> {
        &mut self.fields
    }

    // Take field
    pub fn take_fields(&mut self) -> ::std::collections::HashMap<::std::string::String, Value> {
        ::std::mem::replace(&mut self.fields, ::std::collections::HashMap::new())
    }
}

impl ::protobuf::Message for Struct {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Value>>(wire_type, is, &mut self.fields)?;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Value>>(1, &self.fields);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Value>>(1, &self.fields, os)?;
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

    fn new() -> Struct {
        Struct::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Value>>(
                "fields",
                |m: &Struct| { &m.fields },
                |m: &mut Struct| { &mut m.fields },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Struct>(
                "Struct",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Struct {
        static instance: ::protobuf::rt::LazyV2<Struct> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Struct::new)
    }
}

impl ::protobuf::Clear for Struct {
    fn clear(&mut self) {
        self.fields.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Struct {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Struct {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Value {
    // message oneof groups
    pub kind: ::std::option::Option<Value_oneof_kind>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Value {
    fn default() -> &'a Value {
        <Value as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum Value_oneof_kind {
    null_value(NullValue),
    number_value(f64),
    string_value(::std::string::String),
    bool_value(bool),
    struct_value(Struct),
    list_value(ListValue),
}

impl Value {
    pub fn new() -> Value {
        ::std::default::Default::default()
    }

    // .google.protobuf.NullValue null_value = 1;


    pub fn get_null_value(&self) -> NullValue {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::null_value(v)) => v,
            _ => NullValue::NULL_VALUE,
        }
    }
    pub fn clear_null_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_null_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::null_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_null_value(&mut self, v: NullValue) {
        self.kind = ::std::option::Option::Some(Value_oneof_kind::null_value(v))
    }

    // double number_value = 2;


    pub fn get_number_value(&self) -> f64 {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::number_value(v)) => v,
            _ => 0.,
        }
    }
    pub fn clear_number_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_number_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::number_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_number_value(&mut self, v: f64) {
        self.kind = ::std::option::Option::Some(Value_oneof_kind::number_value(v))
    }

    // string string_value = 3;


    pub fn get_string_value(&self) -> &str {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::string_value(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_string_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_string_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::string_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_string_value(&mut self, v: ::std::string::String) {
        self.kind = ::std::option::Option::Some(Value_oneof_kind::string_value(v))
    }

    // Mutable pointer to the field.
    pub fn mut_string_value(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Value_oneof_kind::string_value(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(Value_oneof_kind::string_value(::std::string::String::new()));
        }
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::string_value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_string_value(&mut self) -> ::std::string::String {
        if self.has_string_value() {
            match self.kind.take() {
                ::std::option::Option::Some(Value_oneof_kind::string_value(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // bool bool_value = 4;


    pub fn get_bool_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::bool_value(v)) => v,
            _ => false,
        }
    }
    pub fn clear_bool_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_bool_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::bool_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bool_value(&mut self, v: bool) {
        self.kind = ::std::option::Option::Some(Value_oneof_kind::bool_value(v))
    }

    // .google.protobuf.Struct struct_value = 5;


    pub fn get_struct_value(&self) -> &Struct {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::struct_value(ref v)) => v,
            _ => <Struct as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_struct_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_struct_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::struct_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_struct_value(&mut self, v: Struct) {
        self.kind = ::std::option::Option::Some(Value_oneof_kind::struct_value(v))
    }

    // Mutable pointer to the field.
    pub fn mut_struct_value(&mut self) -> &mut Struct {
        if let ::std::option::Option::Some(Value_oneof_kind::struct_value(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(Value_oneof_kind::struct_value(Struct::new()));
        }
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::struct_value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_struct_value(&mut self) -> Struct {
        if self.has_struct_value() {
            match self.kind.take() {
                ::std::option::Option::Some(Value_oneof_kind::struct_value(v)) => v,
                _ => panic!(),
            }
        } else {
            Struct::new()
        }
    }

    // .google.protobuf.ListValue list_value = 6;


    pub fn get_list_value(&self) -> &ListValue {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::list_value(ref v)) => v,
            _ => <ListValue as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_list_value(&mut self) {
        self.kind = ::std::option::Option::None;
    }

    pub fn has_list_value(&self) -> bool {
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::list_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_list_value(&mut self, v: ListValue) {
        self.kind = ::std::option::Option::Some(Value_oneof_kind::list_value(v))
    }

    // Mutable pointer to the field.
    pub fn mut_list_value(&mut self) -> &mut ListValue {
        if let ::std::option::Option::Some(Value_oneof_kind::list_value(_)) = self.kind {
        } else {
            self.kind = ::std::option::Option::Some(Value_oneof_kind::list_value(ListValue::new()));
        }
        match self.kind {
            ::std::option::Option::Some(Value_oneof_kind::list_value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_list_value(&mut self) -> ListValue {
        if self.has_list_value() {
            match self.kind.take() {
                ::std::option::Option::Some(Value_oneof_kind::list_value(v)) => v,
                _ => panic!(),
            }
        } else {
            ListValue::new()
        }
    }
}

impl ::protobuf::Message for Value {
    fn is_initialized(&self) -> bool {
        if let Some(Value_oneof_kind::struct_value(ref v)) = self.kind {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Value_oneof_kind::list_value(ref v)) = self.kind {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Value_oneof_kind::null_value(is.read_enum()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Value_oneof_kind::number_value(is.read_double()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Value_oneof_kind::string_value(is.read_string()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Value_oneof_kind::bool_value(is.read_bool()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Value_oneof_kind::struct_value(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.kind = ::std::option::Option::Some(Value_oneof_kind::list_value(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.kind {
            match v {
                &Value_oneof_kind::null_value(v) => {
                    my_size += ::protobuf::rt::enum_size(1, v);
                },
                &Value_oneof_kind::number_value(v) => {
                    my_size += 9;
                },
                &Value_oneof_kind::string_value(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
                &Value_oneof_kind::bool_value(v) => {
                    my_size += 2;
                },
                &Value_oneof_kind::struct_value(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Value_oneof_kind::list_value(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.kind {
            match v {
                &Value_oneof_kind::null_value(v) => {
                    os.write_enum(1, ::protobuf::ProtobufEnum::value(&v))?;
                },
                &Value_oneof_kind::number_value(v) => {
                    os.write_double(2, v)?;
                },
                &Value_oneof_kind::string_value(ref v) => {
                    os.write_string(3, v)?;
                },
                &Value_oneof_kind::bool_value(v) => {
                    os.write_bool(4, v)?;
                },
                &Value_oneof_kind::struct_value(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Value_oneof_kind::list_value(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> Value {
        Value::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor::<_, NullValue>(
                "null_value",
                Value::has_null_value,
                Value::get_null_value,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor::<_>(
                "number_value",
                Value::has_number_value,
                Value::get_number_value,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "string_value",
                Value::has_string_value,
                Value::get_string_value,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                "bool_value",
                Value::has_bool_value,
                Value::get_bool_value,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Struct>(
                "struct_value",
                Value::has_struct_value,
                Value::get_struct_value,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ListValue>(
                "list_value",
                Value::has_list_value,
                Value::get_list_value,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Value>(
                "Value",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Value {
        static instance: ::protobuf::rt::LazyV2<Value> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Value::new)
    }
}

impl ::protobuf::Clear for Value {
    fn clear(&mut self) {
        self.kind = ::std::option::Option::None;
        self.kind = ::std::option::Option::None;
        self.kind = ::std::option::Option::None;
        self.kind = ::std::option::Option::None;
        self.kind = ::std::option::Option::None;
        self.kind = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Value {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListValue {
    // message fields
    pub values: ::protobuf::RepeatedField<Value>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ListValue {
    fn default() -> &'a ListValue {
        <ListValue as ::protobuf::Message>::default_instance()
    }
}

impl ListValue {
    pub fn new() -> ListValue {
        ::std::default::Default::default()
    }

    // repeated .google.protobuf.Value values = 1;


    pub fn get_values(&self) -> &[Value] {
        &self.values
    }
    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: ::protobuf::RepeatedField<Value>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values(&mut self) -> &mut ::protobuf::RepeatedField<Value> {
        &mut self.values
    }

    // Take field
    pub fn take_values(&mut self) -> ::protobuf::RepeatedField<Value> {
        ::std::mem::replace(&mut self.values, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for ListValue {
    fn is_initialized(&self) -> bool {
        for v in &self.values {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.values)?;
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
        for value in &self.values {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.values {
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

    fn new() -> ListValue {
        ListValue::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Value>>(
                "values",
                |m: &ListValue| { &m.values },
                |m: &mut ListValue| { &mut m.values },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ListValue>(
                "ListValue",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ListValue {
        static instance: ::protobuf::rt::LazyV2<ListValue> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ListValue::new)
    }
}

impl ::protobuf::Clear for ListValue {
    fn clear(&mut self) {
        self.values.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListValue {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NullValue {
    NULL_VALUE = 0,
}

impl ::protobuf::ProtobufEnum for NullValue {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NullValue> {
        match value {
            0 => ::std::option::Option::Some(NullValue::NULL_VALUE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NullValue] = &[
            NullValue::NULL_VALUE,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<NullValue>("NullValue", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for NullValue {
}

impl ::std::default::Default for NullValue {
    fn default() -> Self {
        NullValue::NULL_VALUE
    }
}

impl ::protobuf::reflect::ProtobufValue for NullValue {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cgoogle/protobuf/struct.proto\x12\x0fgoogle.protobuf\"\x98\x01\n\
    \x06Struct\x12;\n\x06fields\x18\x01\x20\x03(\x0b2#.google.protobuf.Struc\
    t.FieldsEntryR\x06fields\x1aQ\n\x0bFieldsEntry\x12\x10\n\x03key\x18\x01\
    \x20\x01(\tR\x03key\x12,\n\x05value\x18\x02\x20\x01(\x0b2\x16.google.pro\
    tobuf.ValueR\x05value:\x028\x01\"\xb2\x02\n\x05Value\x12;\n\nnull_value\
    \x18\x01\x20\x01(\x0e2\x1a.google.protobuf.NullValueH\0R\tnullValue\x12#\
    \n\x0cnumber_value\x18\x02\x20\x01(\x01H\0R\x0bnumberValue\x12#\n\x0cstr\
    ing_value\x18\x03\x20\x01(\tH\0R\x0bstringValue\x12\x1f\n\nbool_value\
    \x18\x04\x20\x01(\x08H\0R\tboolValue\x12<\n\x0cstruct_value\x18\x05\x20\
    \x01(\x0b2\x17.google.protobuf.StructH\0R\x0bstructValue\x12;\n\nlist_va\
    lue\x18\x06\x20\x01(\x0b2\x1a.google.protobuf.ListValueH\0R\tlistValueB\
    \x06\n\x04kind\";\n\tListValue\x12.\n\x06values\x18\x01\x20\x03(\x0b2\
    \x16.google.protobuf.ValueR\x06values*\x1b\n\tNullValue\x12\x0e\n\nNULL_\
    VALUE\x10\0B\x7f\n\x13com.google.protobufB\x0bStructProtoP\x01Z/google.g\
    olang.org/protobuf/types/known/structpb\xf8\x01\x01\xa2\x02\x03GPB\xaa\
    \x02\x1eGoogle.Protobuf.WellKnownTypesb\x06proto3\
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
