// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

// TODO: Hand edited! Figure out a better solution for objecthash support

use crypto::signing::KeyPair;
use error::{Error, Result};
use objecthash::{self, ObjectHash, ObjectHasher};
use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;
use rustc_serialize::base64::{self, ToBase64};
use witness::Witness;

// TODO: HAND EDITS START HERE! REFACTOR THIS CODE!!!
const DIGEST_SIZE: usize = 32;

// Block IDs are presently SHA-256 only
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Id([u8; DIGEST_SIZE]);

impl Id {
    // Parent ID of the initial block (256-bits of zero)
    pub fn zero() -> Id {
        Id([0u8; DIGEST_SIZE])
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Id> {
        if bytes.len() != DIGEST_SIZE {
            return Err(Error::parse(None));
        }

        let mut id = [0u8; DIGEST_SIZE];
        id.copy_from_slice(&bytes[0..DIGEST_SIZE]);

        Ok(Id(id))
    }

    pub fn of(block: &Block) -> Id {
        Id::from_bytes(objecthash::digest(block).as_ref()).unwrap()
    }
}

impl AsRef<[u8]> for Id {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl ObjectHash for Id {
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        self.0.objecthash(hasher);
    }
}

// TODO: HAND EDITS END HERE!!!

#[derive(Clone,Default)]
pub struct Body {
    // message fields
    parent_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    timestamp: ::std::option::Option<u64>,
    ops: ::protobuf::RepeatedField<super::op::Op>,
    comment: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
        unsafe {
            instance.get(|| {
                Body {
                    parent_id: ::protobuf::SingularField::none(),
                    timestamp: ::std::option::Option::None,
                    ops: ::protobuf::RepeatedField::new(),
                    comment: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes parent_id = 1;

    pub fn clear_parent_id(&mut self) {
        self.parent_id.clear();
    }

    pub fn has_parent_id(&self) -> bool {
        self.parent_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parent_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.parent_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_parent_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.parent_id.is_none() {
            self.parent_id.set_default();
        };
        self.parent_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_parent_id(&mut self) -> ::std::vec::Vec<u8> {
        self.parent_id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_parent_id(&self) -> &[u8] {
        match self.parent_id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint64 timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp.unwrap_or(0)
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

    // optional string comment = 4;

    pub fn clear_comment(&mut self) {
        self.comment.clear();
    }

    pub fn has_comment(&self) -> bool {
        self.comment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_comment(&mut self, v: ::std::string::String) {
        self.comment = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_comment(&mut self) -> &mut ::std::string::String {
        if self.comment.is_none() {
            self.comment.set_default();
        };
        self.comment.as_mut().unwrap()
    }

    // Take field
    pub fn take_comment(&mut self) -> ::std::string::String {
        self.comment.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_comment(&self) -> &str {
        match self.comment.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Body {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type,
                                                                  is,
                                                                  &mut self.parent_id));
                }
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.timestamp = ::std::option::Option::Some(tmp);
                }
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ops));
                }
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type,
                                                                   is,
                                                                   &mut self.comment));
                }
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                                    wire_type,
                                                                    is,
                                                                    self.mut_unknown_fields()));
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.parent_id {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        }
        for value in &self.timestamp {
            my_size +=
                ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.ops {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.comment {
            my_size += ::protobuf::rt::string_size(4, &value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.parent_id.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.timestamp {
            try!(os.write_uint64(2, v));
        };
        for v in &self.ops {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        }
        if let Some(v) = self.comment.as_ref() {
            try!(os.write_string(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Body>()
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
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "parent_id",
                    Body::has_parent_id,
                    Body::get_parent_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "timestamp",
                    Body::has_timestamp,
                    Body::get_timestamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "ops",
                    Body::get_ops,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "comment",
                    Body::has_comment,
                    Body::get_comment,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Body>("Body",
                                                                    fields,
                                                                    file_descriptor_proto())
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

impl ::std::cmp::PartialEq for Body {
    fn eq(&self, other: &Body) -> bool {
        self.parent_id == other.parent_id && self.timestamp == other.timestamp &&
        self.ops == other.ops && self.comment == other.comment &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Body {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Block {
    // message fields
    body: ::protobuf::SingularPtrField<Body>,
    witness: ::protobuf::SingularPtrField<super::witness::Witness>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
        unsafe {
            instance.get(|| {
                Block {
                    body: ::protobuf::SingularPtrField::none(),
                    witness: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .ithos.Body body = 1;

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

    // optional .ithos.Witness witness = 2;

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
}

impl ::protobuf::Message for Block {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self,
                  is: &mut ::protobuf::CodedInputStream)
                  -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.body));
                }
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type,
                                                                    is,
                                                                    &mut self.witness));
                }
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number,
                                                                    wire_type,
                                                                    is,
                                                                    self.mut_unknown_fields()));
                }
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.body {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.witness {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.body.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.witness.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Block>()
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "body",
                    Block::has_body,
                    Block::get_body,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "witness",
                    Block::has_witness,
                    Block::get_witness,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Block>("Block",
                                                                     fields,
                                                                     file_descriptor_proto())
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

impl ::std::cmp::PartialEq for Block {
    fn eq(&self, other: &Block) -> bool {
        self.body == other.body && self.witness == other.witness &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Block {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
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

// TODO: Hand edited! Move this elsewhere!
pub fn sign(body: Body, keypair: &KeyPair) -> Block {
    let mut message = String::from("ithos.block.body.ni:///sha-256;");
    message.push_str(&objecthash::digest(&body).as_ref().to_base64(base64::URL_SAFE));

    let signature = keypair.sign(&message.as_bytes());
    let mut witness = Witness::new();
    witness.set_signatures(::protobuf::RepeatedField::from_vec(vec![signature]));

    let mut block = Block::new();
    block.set_body(body);
    block.set_witness(witness);
    block
}

// TODO: Hand edited! Figure out a better solution for objecthash support
impl ObjectHash for Body {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        // TODO: Don't Panic
        objecthash_struct!(
            hasher,
            "parent_id" => *self.parent_id.get_ref(),
            "timestamp" => self.timestamp.unwrap() as i64,
            "ops" => Vec::from(self.get_ops()),
            "comment" => *self.comment.get_ref()
        )
    }
}

impl ObjectHash for Block {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        // TODO: Don't Panic
        objecthash_struct!(
            hasher,
            "body" => *self.body.get_ref(),
            "witness" => *self.witness.get_ref()
        )
    }
}
