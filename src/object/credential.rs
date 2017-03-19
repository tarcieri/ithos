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

use alg;
use objecthash::{self, ObjectHash, ObjectHasher};
use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Credential {
    // message fields
    pub keyid: ::std::vec::Vec<u8>,
    pub credential_type: Type,
    pub credential_alg: ::std::string::String,
    pub sealing_alg: alg::EncryptionAlg,
    pub encrypted_value: ::std::vec::Vec<u8>,
    pub salt: ::std::vec::Vec<u8>,
    pub public_key: ::std::vec::Vec<u8>,
    pub not_before: u64,
    pub not_after: u64,
    pub description: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Credential {}

impl Credential {
    pub fn new() -> Credential {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Credential {
        static mut instance: ::protobuf::lazy::Lazy<Credential> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Credential,
        };
        unsafe { instance.get(Credential::new) }
    }

    // bytes keyid = 1;

    pub fn clear_keyid(&mut self) {
        self.keyid.clear();
    }

    // Param is passed by value, moved
    pub fn set_keyid(&mut self, v: ::std::vec::Vec<u8>) {
        self.keyid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keyid(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.keyid
    }

    // Take field
    pub fn take_keyid(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.keyid, ::std::vec::Vec::new())
    }

    pub fn get_keyid(&self) -> &[u8] {
        &self.keyid
    }

    fn get_keyid_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.keyid
    }

    fn mut_keyid_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.keyid
    }

    // .ithos.object.Type credential_type = 2;

    pub fn clear_credential_type(&mut self) {
        self.credential_type = Type::SIGNATURE_KEY_PAIR;
    }

    // Param is passed by value, moved
    pub fn set_credential_type(&mut self, v: Type) {
        self.credential_type = v;
    }

    pub fn get_credential_type(&self) -> Type {
        self.credential_type
    }

    fn get_credential_type_for_reflect(&self) -> &Type {
        &self.credential_type
    }

    fn mut_credential_type_for_reflect(&mut self) -> &mut Type {
        &mut self.credential_type
    }

    // string credential_alg = 3;

    pub fn clear_credential_alg(&mut self) {
        self.credential_alg.clear();
    }

    // Param is passed by value, moved
    pub fn set_credential_alg(&mut self, v: ::std::string::String) {
        self.credential_alg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_credential_alg(&mut self) -> &mut ::std::string::String {
        &mut self.credential_alg
    }

    // Take field
    pub fn take_credential_alg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.credential_alg, ::std::string::String::new())
    }

    pub fn get_credential_alg(&self) -> &str {
        &self.credential_alg
    }

    fn get_credential_alg_for_reflect(&self) -> &::std::string::String {
        &self.credential_alg
    }

    fn mut_credential_alg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.credential_alg
    }

    // .ithos.EncryptionAlgorithm sealing_alg = 4;

    pub fn clear_sealing_alg(&mut self) {
        self.sealing_alg = alg::EncryptionAlg::AES256GCM;
    }

    // Param is passed by value, moved
    pub fn set_sealing_alg(&mut self, v: alg::EncryptionAlg) {
        self.sealing_alg = v;
    }

    pub fn get_sealing_alg(&self) -> alg::EncryptionAlg {
        self.sealing_alg
    }

    fn get_sealing_alg_for_reflect(&self) -> &alg::EncryptionAlg {
        &self.sealing_alg
    }

    fn mut_sealing_alg_for_reflect(&mut self) -> &mut alg::EncryptionAlg {
        &mut self.sealing_alg
    }

    // bytes encrypted_value = 5;

    pub fn clear_encrypted_value(&mut self) {
        self.encrypted_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_encrypted_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.encrypted_value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encrypted_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.encrypted_value
    }

    // Take field
    pub fn take_encrypted_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.encrypted_value, ::std::vec::Vec::new())
    }

    pub fn get_encrypted_value(&self) -> &[u8] {
        &self.encrypted_value
    }

    fn get_encrypted_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.encrypted_value
    }

    fn mut_encrypted_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.encrypted_value
    }

    // bytes salt = 6;

    pub fn clear_salt(&mut self) {
        self.salt.clear();
    }

    // Param is passed by value, moved
    pub fn set_salt(&mut self, v: ::std::vec::Vec<u8>) {
        self.salt = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_salt(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.salt
    }

    // Take field
    pub fn take_salt(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.salt, ::std::vec::Vec::new())
    }

    pub fn get_salt(&self) -> &[u8] {
        &self.salt
    }

    fn get_salt_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.salt
    }

    fn mut_salt_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.salt
    }

    // bytes public_key = 7;

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

    // uint64 not_before = 8;

    pub fn clear_not_before(&mut self) {
        self.not_before = 0;
    }

    // Param is passed by value, moved
    pub fn set_not_before(&mut self, v: u64) {
        self.not_before = v;
    }

    pub fn get_not_before(&self) -> u64 {
        self.not_before
    }

    fn get_not_before_for_reflect(&self) -> &u64 {
        &self.not_before
    }

    fn mut_not_before_for_reflect(&mut self) -> &mut u64 {
        &mut self.not_before
    }

    // uint64 not_after = 9;

    pub fn clear_not_after(&mut self) {
        self.not_after = 0;
    }

    // Param is passed by value, moved
    pub fn set_not_after(&mut self, v: u64) {
        self.not_after = v;
    }

    pub fn get_not_after(&self) -> u64 {
        self.not_after
    }

    fn get_not_after_for_reflect(&self) -> &u64 {
        &self.not_after
    }

    fn mut_not_after_for_reflect(&mut self) -> &mut u64 {
        &mut self.not_after
    }

    // string description = 10;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    fn get_description_for_reflect(&self) -> &::std::string::String {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }
}

impl ::protobuf::Message for Credential {
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
                                                                    &mut self.keyid)?;
                }
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.credential_type = tmp;
                }
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type,
                                                                     is,
                                                                     &mut self.credential_alg)?;
                }
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.sealing_alg = tmp;
                }
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type,
                                                                    is,
                                                                    &mut self.encrypted_value)?;
                }
                6 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.salt)?;
                }
                7 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type,
                                                                    is,
                                                                    &mut self.public_key)?;
                }
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.not_before = tmp;
                }
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.not_after = tmp;
                }
                10 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type,
                                                                     is,
                                                                     &mut self.description)?;
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
        if self.keyid != ::std::vec::Vec::new() {
            my_size += ::protobuf::rt::bytes_size(1, &self.keyid);
        };
        if self.credential_type != Type::SIGNATURE_KEY_PAIR {
            my_size += ::protobuf::rt::enum_size(2, self.credential_type);
        };
        if self.credential_alg != ::std::string::String::new() {
            my_size += ::protobuf::rt::string_size(3, &self.credential_alg);
        };
        if self.sealing_alg != alg::EncryptionAlg::AES256GCM {
            my_size += ::protobuf::rt::enum_size(4, self.sealing_alg);
        };
        if self.encrypted_value != ::std::vec::Vec::new() {
            my_size += ::protobuf::rt::bytes_size(5, &self.encrypted_value);
        };
        if self.salt != ::std::vec::Vec::new() {
            my_size += ::protobuf::rt::bytes_size(6, &self.salt);
        };
        if self.public_key != ::std::vec::Vec::new() {
            my_size += ::protobuf::rt::bytes_size(7, &self.public_key);
        };
        if self.not_before != 0 {
            my_size += ::protobuf::rt::value_size(8,
                                                  self.not_before,
                                                  ::protobuf::wire_format::WireTypeVarint);
        };
        if self.not_after != 0 {
            my_size += ::protobuf::rt::value_size(9,
                                                  self.not_after,
                                                  ::protobuf::wire_format::WireTypeVarint);
        };
        if self.description != ::std::string::String::new() {
            my_size += ::protobuf::rt::string_size(10, &self.description);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if self.keyid != ::std::vec::Vec::new() {
            os.write_bytes(1, &self.keyid)?;
        };
        if self.credential_type != Type::SIGNATURE_KEY_PAIR {
            os.write_enum(2, self.credential_type.value())?;
        };
        if self.credential_alg != ::std::string::String::new() {
            os.write_string(3, &self.credential_alg)?;
        };
        if self.sealing_alg != alg::EncryptionAlg::AES256GCM {
            os.write_enum(4, self.sealing_alg.value())?;
        };
        if self.encrypted_value != ::std::vec::Vec::new() {
            os.write_bytes(5, &self.encrypted_value)?;
        };
        if self.salt != ::std::vec::Vec::new() {
            os.write_bytes(6, &self.salt)?;
        };
        if self.public_key != ::std::vec::Vec::new() {
            os.write_bytes(7, &self.public_key)?;
        };
        if self.not_before != 0 {
            os.write_uint64(8, self.not_before)?;
        };
        if self.not_after != 0 {
            os.write_uint64(9, self.not_after)?;
        };
        if self.description != ::std::string::String::new() {
            os.write_string(10, &self.description)?;
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

impl ::protobuf::MessageStatic for Credential {
    fn new() -> Credential {
        Credential::new()
    }

    fn descriptor_static(_: ::std::option::Option<Credential>)
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
                    "keyid",
                    Credential::get_keyid_for_reflect,
                    Credential::mut_keyid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Type>>(
                    "credential_type",
                    Credential::get_credential_type_for_reflect,
                    Credential::mut_credential_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "credential_alg",
                    Credential::get_credential_alg_for_reflect,
                    Credential::mut_credential_alg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<alg::EncryptionAlg>>(
                    "sealing_alg",
                    Credential::get_sealing_alg_for_reflect,
                    Credential::mut_sealing_alg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "encrypted_value",
                    Credential::get_encrypted_value_for_reflect,
                    Credential::mut_encrypted_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "salt",
                    Credential::get_salt_for_reflect,
                    Credential::mut_salt_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "public_key",
                    Credential::get_public_key_for_reflect,
                    Credential::mut_public_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "not_before",
                    Credential::get_not_before_for_reflect,
                    Credential::mut_not_before_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "not_after",
                    Credential::get_not_after_for_reflect,
                    Credential::mut_not_after_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    Credential::get_description_for_reflect,
                    Credential::mut_description_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Credential>(
                    "Credential",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Credential {
    fn clear(&mut self) {
        self.clear_keyid();
        self.clear_credential_type();
        self.clear_credential_alg();
        self.clear_sealing_alg();
        self.clear_encrypted_value();
        self.clear_salt();
        self.clear_public_key();
        self.clear_not_before();
        self.clear_not_after();
        self.clear_description();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Credential {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Credential {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Type {
    SIGNATURE_KEY_PAIR = 0,
}

impl ::protobuf::ProtobufEnum for Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Type> {
        match value {
            0 => ::std::option::Option::Some(Type::SIGNATURE_KEY_PAIR),
            _ => ::std::option::Option::None,
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Type] = &[Type::SIGNATURE_KEY_PAIR];
        values
    }

    fn enum_descriptor_static(_: Option<Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Type {}

impl ::std::default::Default for Type {
    fn default() -> Self {
        Type::SIGNATURE_KEY_PAIR
    }
}

impl ::protobuf::reflect::ProtobufValue for Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] =
    &[0x0a, 0x17, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x2f, 0x63, 0x72, 0x65, 0x64, 0x65, 0x6e,
      0x74, 0x69, 0x61, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0c, 0x69, 0x74, 0x68,
      0x6f, 0x73, 0x2e, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x1a, 0x0f, 0x61, 0x6c, 0x67, 0x6f,
      0x72, 0x69, 0x74, 0x68, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xfd, 0x02, 0x0a,
      0x0a, 0x43, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x12, 0x14, 0x0a, 0x05,
      0x6b, 0x65, 0x79, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x6b, 0x65,
      0x79, 0x69, 0x64, 0x12, 0x3b, 0x0a, 0x0f, 0x63, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69,
      0x61, 0x6c, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x12,
      0x2e, 0x69, 0x74, 0x68, 0x6f, 0x73, 0x2e, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x2e, 0x54,
      0x79, 0x70, 0x65, 0x52, 0x0e, 0x63, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c,
      0x54, 0x79, 0x70, 0x65, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74,
      0x69, 0x61, 0x6c, 0x5f, 0x61, 0x6c, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d,
      0x63, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x41, 0x6c, 0x67, 0x12, 0x3b,
      0x0a, 0x0b, 0x73, 0x65, 0x61, 0x6c, 0x69, 0x6e, 0x67, 0x5f, 0x61, 0x6c, 0x67, 0x18, 0x04,
      0x20, 0x01, 0x28, 0x0e, 0x32, 0x1a, 0x2e, 0x69, 0x74, 0x68, 0x6f, 0x73, 0x2e, 0x45, 0x6e,
      0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x41, 0x6c, 0x67, 0x6f, 0x72, 0x69, 0x74,
      0x68, 0x6d, 0x52, 0x0a, 0x73, 0x65, 0x61, 0x6c, 0x69, 0x6e, 0x67, 0x41, 0x6c, 0x67, 0x12,
      0x27, 0x0a, 0x0f, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x5f, 0x76, 0x61,
      0x6c, 0x75, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0e, 0x65, 0x6e, 0x63, 0x72,
      0x79, 0x70, 0x74, 0x65, 0x64, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x73,
      0x61, 0x6c, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x73, 0x61, 0x6c, 0x74,
      0x12, 0x1d, 0x0a, 0x0a, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x6b, 0x65, 0x79, 0x18,
      0x07, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65,
      0x79, 0x12, 0x1d, 0x0a, 0x0a, 0x6e, 0x6f, 0x74, 0x5f, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65,
      0x18, 0x08, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x6e, 0x6f, 0x74, 0x42, 0x65, 0x66, 0x6f,
      0x72, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x6e, 0x6f, 0x74, 0x5f, 0x61, 0x66, 0x74, 0x65, 0x72,
      0x18, 0x09, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x6e, 0x6f, 0x74, 0x41, 0x66, 0x74, 0x65,
      0x72, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f,
      0x6e, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69,
      0x70, 0x74, 0x69, 0x6f, 0x6e, 0x2a, 0x1e, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x16,
      0x0a, 0x12, 0x53, 0x49, 0x47, 0x4e, 0x41, 0x54, 0x55, 0x52, 0x45, 0x5f, 0x4b, 0x45, 0x59,
      0x5f, 0x50, 0x41, 0x49, 0x52, 0x10, 0x00, 0x4a, 0xf6, 0x06, 0x0a, 0x06, 0x12, 0x04, 0x00,
      0x00, 0x16, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08,
      0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x14, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
      0x03, 0x04, 0x07, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x06, 0x00, 0x08,
      0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x06, 0x05, 0x09, 0x0a, 0x25,
      0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x04, 0x1b, 0x22, 0x18, 0x20, 0x50,
      0x75, 0x62, 0x6c, 0x69, 0x63, 0x2f, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x20, 0x6b,
      0x65, 0x79, 0x70, 0x61, 0x69, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00,
      0x01, 0x12, 0x03, 0x07, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02,
      0x12, 0x03, 0x07, 0x19, 0x1a, 0x0a, 0x2a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0b, 0x00,
      0x16, 0x01, 0x1a, 0x1e, 0x20, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x20,
      0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x20, 0x63, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69,
      0x61, 0x6c, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x08,
      0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x04, 0x14, 0x0a,
      0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0c, 0x04, 0x0b, 0x14, 0x0a,
      0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0c, 0x04, 0x09, 0x0a, 0x0c,
      0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04,
      0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
      0x02, 0x01, 0x04, 0x12, 0x04, 0x0d, 0x04, 0x0c, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
      0x02, 0x01, 0x06, 0x12, 0x03, 0x0d, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
      0x01, 0x01, 0x12, 0x03, 0x0d, 0x09, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
      0x03, 0x12, 0x03, 0x0d, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12,
      0x03, 0x0e, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04,
      0x0e, 0x04, 0x0d, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03,
      0x0e, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0e,
      0x0b, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0e, 0x1c,
      0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x04, 0x28, 0x0a,
      0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x0f, 0x04, 0x0e, 0x1e, 0x0a,
      0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x0f, 0x04, 0x17, 0x0a, 0x0c,
      0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0f, 0x18, 0x23, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0f, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04,
      0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x10, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
      0x02, 0x04, 0x04, 0x12, 0x04, 0x10, 0x04, 0x0f, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
      0x02, 0x04, 0x05, 0x12, 0x03, 0x10, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
      0x04, 0x01, 0x12, 0x03, 0x10, 0x0a, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
      0x03, 0x12, 0x03, 0x10, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12,
      0x03, 0x11, 0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x04,
      0x11, 0x04, 0x10, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03,
      0x11, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x11,
      0x0a, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x11, 0x11,
      0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x12, 0x04, 0x19, 0x0a,
      0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x04, 0x12, 0x04, 0x11, 0x13, 0x0a,
      0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x12, 0x04, 0x09, 0x0a, 0x0c,
      0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x12, 0x0a, 0x14, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x12, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
      0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x13, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
      0x02, 0x07, 0x04, 0x12, 0x04, 0x13, 0x04, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
      0x02, 0x07, 0x05, 0x12, 0x03, 0x13, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
      0x07, 0x01, 0x12, 0x03, 0x13, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
      0x03, 0x12, 0x03, 0x13, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12,
      0x03, 0x14, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x04,
      0x14, 0x04, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03,
      0x14, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x14,
      0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x14, 0x17,
      0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x15, 0x04, 0x1c, 0x0a,
      0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x04, 0x15, 0x04, 0x14, 0x19, 0x0a,
      0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x15, 0x04, 0x0a, 0x0a, 0x0c,
      0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x15, 0x0b, 0x16, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x15, 0x19, 0x1b, 0x62, 0x06, 0x70, 0x72,
      0x6f, 0x74, 0x6f, 0x33];

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
impl ObjectHash for Credential {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(
            hasher,
            "keyid" => &self.keyid,
            "credential_type" => &(self.credential_type as u32),
            "credential_alg" => &self.credential_alg,
            "sealing_alg" => &(self.sealing_alg as u32),
            "encrypted_value" => &self.encrypted_value,
            "salt" => &self.salt,
            "public_key" => &self.public_key,
            "not_before" => &(self.not_before as i64),
            "not_after" => &(self.not_after as i64),
            "description" => &self.description
        );
    }
}
