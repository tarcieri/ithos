// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

// TODO: Hand edited! Figure out a better solution for objecthash support

use objecthash::{self, ObjectHash, ObjectHasher};
use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Body {
    // message fields
    pub parent_id: ::std::vec::Vec<u8>,
    pub timestamp: u64,
    ops: ::protobuf::RepeatedField<super::op::Op>,
    pub comment: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Body {}

impl Body {
    pub fn new() -> Body {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Body {
        static mut instance: ::protobuf::lazy::Lazy<Body> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Body,
        };
        unsafe { instance.get(Body::new) }
    }

    // bytes parent_id = 1;

    pub fn clear_parent_id(&mut self) {
        self.parent_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_parent_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.parent_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_parent_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.parent_id
    }

    // Take field
    pub fn take_parent_id(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.parent_id, ::std::vec::Vec::new())
    }

    pub fn get_parent_id(&self) -> &[u8] {
        &self.parent_id
    }

    fn get_parent_id_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.parent_id
    }

    fn mut_parent_id_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.parent_id
    }

    // uint64 timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = v;
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    fn get_timestamp_for_reflect(&self) -> &u64 {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut u64 {
        &mut self.timestamp
    }

    // repeated .ithos.Op ops = 3;

    pub fn clear_ops(&mut self) {
        self.ops.clear();
    }

    // Param is passed by value, moved
    pub fn set_ops(&mut self, v: ::protobuf::RepeatedField<super::op::Op>) {
        self.ops = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ops(&mut self) -> &mut ::protobuf::RepeatedField<super::op::Op> {
        &mut self.ops
    }

    // Take field
    pub fn take_ops(&mut self) -> ::protobuf::RepeatedField<super::op::Op> {
        ::std::mem::replace(&mut self.ops, ::protobuf::RepeatedField::new())
    }

    pub fn get_ops(&self) -> &[super::op::Op] {
        &self.ops
    }

    fn get_ops_for_reflect(&self) -> &::protobuf::RepeatedField<super::op::Op> {
        &self.ops
    }

    fn mut_ops_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::op::Op> {
        &mut self.ops
    }

    // string comment = 4;

    pub fn clear_comment(&mut self) {
        self.comment.clear();
    }

    // Param is passed by value, moved
    pub fn set_comment(&mut self, v: ::std::string::String) {
        self.comment = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_comment(&mut self) -> &mut ::std::string::String {
        &mut self.comment
    }

    // Take field
    pub fn take_comment(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.comment, ::std::string::String::new())
    }

    pub fn get_comment(&self) -> &str {
        &self.comment
    }

    fn get_comment_for_reflect(&self) -> &::std::string::String {
        &self.comment
    }

    fn mut_comment_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.comment
    }
}

impl ::protobuf::Message for Body {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type,
                                                                    is,
                                                                    &mut self.parent_id)?;
                }
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.timestamp = tmp;
                }
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ops)?;
                }
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type,
                                                                     is,
                                                                     &mut self.comment)?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                               wire_type,
                                                               is,
                                                               self.mut_unknown_fields())?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.parent_id != ::std::vec::Vec::new() {
            my_size += ::protobuf::rt::bytes_size(1, &self.parent_id);
        };
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(2,
                                                  self.timestamp,
                                                  ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.ops {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.comment != ::std::string::String::new() {
            my_size += ::protobuf::rt::string_size(4, &self.comment);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if self.parent_id != ::std::vec::Vec::new() {
            os.write_bytes(1, &self.parent_id)?;
        };
        if self.timestamp != 0 {
            os.write_uint64(2, self.timestamp)?;
        };
        for v in &self.ops {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.comment != ::std::string::String::new() {
            os.write_string(4, &self.comment)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Body {
    fn new() -> Body {
        Body::new()
    }

    fn descriptor_static(_: ::std::option::Option<Body>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "parent_id",
                    Body::get_parent_id_for_reflect,
                    Body::mut_parent_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    Body::get_timestamp_for_reflect,
                    Body::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::op::Op>>(
                    "ops",
                    Body::get_ops_for_reflect,
                    Body::mut_ops_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "comment",
                    Body::get_comment_for_reflect,
                    Body::mut_comment_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Body>(
                    "Body",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Body {
    fn clear(&mut self) {
        self.clear_parent_id();
        self.clear_timestamp();
        self.clear_ops();
        self.clear_comment();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Body {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Body {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Block {
    // message fields
    body: ::protobuf::SingularPtrField<Body>,
    witness: ::protobuf::SingularPtrField<super::witness::Witness>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Block {}

impl Block {
    pub fn new() -> Block {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Block {
        static mut instance: ::protobuf::lazy::Lazy<Block> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Block,
        };
        unsafe { instance.get(Block::new) }
    }

    // .ithos.Body body = 1;

    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: Body) {
        self.body = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut Body {
        if self.body.is_none() {
            self.body.set_default();
        };
        self.body.as_mut().unwrap()
    }

    // Take field
    pub fn take_body(&mut self) -> Body {
        self.body.take().unwrap_or_else(|| Body::new())
    }

    pub fn get_body(&self) -> &Body {
        self.body.as_ref().unwrap_or_else(|| Body::default_instance())
    }

    fn get_body_for_reflect(&self) -> &::protobuf::SingularPtrField<Body> {
        &self.body
    }

    fn mut_body_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Body> {
        &mut self.body
    }

    // .ithos.Witness witness = 2;

    pub fn clear_witness(&mut self) {
        self.witness.clear();
    }

    pub fn has_witness(&self) -> bool {
        self.witness.is_some()
    }

    // Param is passed by value, moved
    pub fn set_witness(&mut self, v: super::witness::Witness) {
        self.witness = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_witness(&mut self) -> &mut super::witness::Witness {
        if self.witness.is_none() {
            self.witness.set_default();
        };
        self.witness.as_mut().unwrap()
    }

    // Take field
    pub fn take_witness(&mut self) -> super::witness::Witness {
        self.witness.take().unwrap_or_else(|| super::witness::Witness::new())
    }

    pub fn get_witness(&self) -> &super::witness::Witness {
        self.witness.as_ref().unwrap_or_else(|| super::witness::Witness::default_instance())
    }

    fn get_witness_for_reflect(&self) -> &::protobuf::SingularPtrField<super::witness::Witness> {
        &self.witness
    }

    fn mut_witness_for_reflect(&mut self)
                               -> &mut ::protobuf::SingularPtrField<super::witness::Witness> {
        &mut self.witness
    }
}

impl ::protobuf::Message for Block {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.body)?;
                }
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.witness)?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                               wire_type,
                                                               is,
                                                               self.mut_unknown_fields())?;
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.body.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.witness.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.body.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.witness.as_ref() {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Block {
    fn new() -> Block {
        Block::new()
    }

    fn descriptor_static(_: ::std::option::Option<Block>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Body>>(
                    "body",
                    Block::get_body_for_reflect,
                    Block::mut_body_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::witness::Witness>>(
                    "witness",
                    Block::get_witness_for_reflect,
                    Block::mut_witness_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Block>(
                    "Block",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Block {
    fn clear(&mut self) {
        self.clear_body();
        self.clear_witness();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Block {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Block {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] =
    &[0x0a, 0x0b, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05,
      0x69, 0x74, 0x68, 0x6f, 0x73, 0x1a, 0x08, 0x6f, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
      0x1a, 0x0d, 0x77, 0x69, 0x74, 0x6e, 0x65, 0x73, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
      0x22, 0x78, 0x0a, 0x04, 0x42, 0x6f, 0x64, 0x79, 0x12, 0x1b, 0x0a, 0x09, 0x70, 0x61, 0x72,
      0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x08, 0x70,
      0x61, 0x72, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65,
      0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x74, 0x69,
      0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x1b, 0x0a, 0x03, 0x6f, 0x70, 0x73, 0x18,
      0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x09, 0x2e, 0x69, 0x74, 0x68, 0x6f, 0x73, 0x2e, 0x4f,
      0x70, 0x52, 0x03, 0x6f, 0x70, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x6f, 0x6d, 0x6d, 0x65,
      0x6e, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x63, 0x6f, 0x6d, 0x6d, 0x65,
      0x6e, 0x74, 0x22, 0x52, 0x0a, 0x05, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x1f, 0x0a, 0x04,
      0x62, 0x6f, 0x64, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x69, 0x74,
      0x68, 0x6f, 0x73, 0x2e, 0x42, 0x6f, 0x64, 0x79, 0x52, 0x04, 0x62, 0x6f, 0x64, 0x79, 0x12,
      0x28, 0x0a, 0x07, 0x77, 0x69, 0x74, 0x6e, 0x65, 0x73, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28,
      0x0b, 0x32, 0x0e, 0x2e, 0x69, 0x74, 0x68, 0x6f, 0x73, 0x2e, 0x57, 0x69, 0x74, 0x6e, 0x65,
      0x73, 0x73, 0x52, 0x07, 0x77, 0x69, 0x74, 0x6e, 0x65, 0x73, 0x73, 0x4a, 0x85, 0x04, 0x0a,
      0x06, 0x12, 0x04, 0x00, 0x00, 0x11, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
      0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0d, 0x0a, 0x09, 0x0a,
      0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x07, 0x11, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12,
      0x03, 0x05, 0x07, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0c,
      0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x0c, 0x0a, 0x0b,
      0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x02, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
      0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x08, 0x02, 0x07, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
      0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x08, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
      0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
      0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
      0x01, 0x12, 0x03, 0x09, 0x02, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04,
      0x12, 0x04, 0x09, 0x02, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
      0x12, 0x03, 0x09, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
      0x03, 0x09, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
      0x09, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0a, 0x02,
      0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a,
      0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0a, 0x0b, 0x0d, 0x0a,
      0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x0e, 0x11, 0x0a, 0x0c,
      0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0a, 0x15, 0x16, 0x0a, 0x0b, 0x0a,
      0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x02, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
      0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x0b, 0x02, 0x0a, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
      0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
      0x02, 0x03, 0x01, 0x12, 0x03, 0x0b, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
      0x03, 0x03, 0x12, 0x03, 0x0b, 0x15, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
      0x0e, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x08,
      0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x02, 0x16, 0x0a,
      0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0f, 0x02, 0x0e, 0x0f, 0x0a,
      0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0f, 0x02, 0x06, 0x0a, 0x0c,
      0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x07, 0x0b, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04,
      0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x10, 0x02, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01,
      0x02, 0x01, 0x04, 0x12, 0x04, 0x10, 0x02, 0x0f, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
      0x02, 0x01, 0x06, 0x12, 0x03, 0x10, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
      0x01, 0x01, 0x12, 0x03, 0x10, 0x0a, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
      0x03, 0x12, 0x03, 0x10, 0x14, 0x15, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe { file_descriptor_proto_lazy.get(|| parse_descriptor_proto()) }
}

// TODO: Hand edited! Figure out a better solution for objecthash support
impl ObjectHash for Body {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(
            hasher,
            "parent_id" => &self.parent_id,
            "timestamp" => &(self.timestamp as i64),
            "ops" => &Vec::from(self.get_ops()),
            "comment" => &self.comment
        )
    }
}

impl ObjectHash for Block {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(
            hasher,
            "body" => self.get_body(),
            "witness" => self.get_witness()
        )
    }
}
