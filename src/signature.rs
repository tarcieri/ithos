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
pub struct Signature {
    // message fields
    pub algorithm: super::alg::SignatureAlg,
    pub public_key: ::std::vec::Vec<u8>,
    pub value: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Signature {}

impl Signature {
    pub fn new() -> Signature {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Signature {
        static mut instance: ::protobuf::lazy::Lazy<Signature> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Signature,
        };
        unsafe { instance.get(Signature::new) }
    }

    // .ithos.SignatureAlgorithm algorithm = 1;

    pub fn clear_algorithm(&mut self) {
        self.algorithm = super::alg::SignatureAlg::Ed25519;
    }

    // Param is passed by value, moved
    pub fn set_algorithm(&mut self, v: super::alg::SignatureAlg) {
        self.algorithm = v;
    }

    pub fn get_algorithm(&self) -> super::alg::SignatureAlg {
        self.algorithm
    }

    fn get_algorithm_for_reflect(&self) -> &super::alg::SignatureAlg {
        &self.algorithm
    }

    fn mut_algorithm_for_reflect(&mut self) -> &mut super::alg::SignatureAlg {
        &mut self.algorithm
    }

    // bytes public_key = 2;

    pub fn clear_public_key(&mut self) {
        self.public_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.public_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.public_key
    }

    // Take field
    pub fn take_public_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.public_key, ::std::vec::Vec::new())
    }

    pub fn get_public_key(&self) -> &[u8] {
        &self.public_key
    }

    fn get_public_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.public_key
    }

    fn mut_public_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.public_key
    }

    // bytes value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }
}

impl ::protobuf::Message for Signature {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.algorithm = tmp;
                }
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type,
                                                                    is,
                                                                    &mut self.public_key)?;
                }
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type,
                                                                    is,
                                                                    &mut self.value)?;
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
        if self.algorithm != super::alg::SignatureAlg::Ed25519 {
            my_size += ::protobuf::rt::enum_size(1, self.algorithm);
        };
        if self.public_key != ::std::vec::Vec::new() {
            my_size += ::protobuf::rt::bytes_size(2, &self.public_key);
        };
        if self.value != ::std::vec::Vec::new() {
            my_size += ::protobuf::rt::bytes_size(3, &self.value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if self.algorithm != super::alg::SignatureAlg::Ed25519 {
            os.write_enum(1, self.algorithm.value())?;
        };
        if self.public_key != ::std::vec::Vec::new() {
            os.write_bytes(2, &self.public_key)?;
        };
        if self.value != ::std::vec::Vec::new() {
            os.write_bytes(3, &self.value)?;
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

impl ::protobuf::MessageStatic for Signature {
    fn new() -> Signature {
        Signature::new()
    }

    fn descriptor_static(_: ::std::option::Option<Signature>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::alg::SignatureAlg>>(
                    "algorithm",
                    Signature::get_algorithm_for_reflect,
                    Signature::mut_algorithm_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "public_key",
                    Signature::get_public_key_for_reflect,
                    Signature::mut_public_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    Signature::get_value_for_reflect,
                    Signature::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Signature>(
                    "Signature",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Signature {
    fn clear(&mut self) {
        self.clear_algorithm();
        self.clear_public_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Signature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Signature {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] =
    &[0x0a, 0x0f, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x2e, 0x70, 0x72, 0x6f,
      0x74, 0x6f, 0x12, 0x05, 0x69, 0x74, 0x68, 0x6f, 0x73, 0x1a, 0x0f, 0x61, 0x6c, 0x67, 0x6f,
      0x72, 0x69, 0x74, 0x68, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x79, 0x0a, 0x09,
      0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x12, 0x37, 0x0a, 0x09, 0x61, 0x6c,
      0x67, 0x6f, 0x72, 0x69, 0x74, 0x68, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x19,
      0x2e, 0x69, 0x74, 0x68, 0x6f, 0x73, 0x2e, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72,
      0x65, 0x41, 0x6c, 0x67, 0x6f, 0x72, 0x69, 0x74, 0x68, 0x6d, 0x52, 0x09, 0x61, 0x6c, 0x67,
      0x6f, 0x72, 0x69, 0x74, 0x68, 0x6d, 0x12, 0x1d, 0x0a, 0x0a, 0x70, 0x75, 0x62, 0x6c, 0x69,
      0x63, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x70, 0x75,
      0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75,
      0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x4a,
      0xbd, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0b, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c,
      0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0d,
      0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x07, 0x18, 0x0a, 0x36, 0x0a, 0x02,
      0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0b, 0x01, 0x1a, 0x2a, 0x20, 0x41, 0x6e, 0x20, 0x69,
      0x6e, 0x64, 0x69, 0x76, 0x69, 0x64, 0x75, 0x61, 0x6c, 0x20, 0x73, 0x69, 0x67, 0x6e, 0x61,
      0x74, 0x75, 0x72, 0x65, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e,
      0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
      0x03, 0x07, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08,
      0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x08, 0x04,
      0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x08, 0x04,
      0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x17, 0x20,
      0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x23, 0x24, 0x0a,
      0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x04, 0x19, 0x0a, 0x0d, 0x0a,
      0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x09, 0x04, 0x08, 0x25, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x09, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05,
      0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x0a, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
      0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x09, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
      0x02, 0x02, 0x12, 0x03, 0x0a, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
      0x04, 0x12, 0x04, 0x0a, 0x04, 0x09, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
      0x05, 0x12, 0x03, 0x0a, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01,
      0x12, 0x03, 0x0a, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12,
      0x03, 0x0a, 0x12, 0x13, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33];

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
impl ObjectHash for Signature {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(
            hasher,
            "algorithm" => &(self.algorithm as u32),
            "public_key" => &self.public_key,
            "value" => &self.value
        )
    }
}
