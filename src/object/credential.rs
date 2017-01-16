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

// TODO: Hand edits! Don't do this!

use algorithm;
use objecthash::{self, ObjectHash, ObjectHasher};

// Back to your regularly scheduled proto
use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Credential {
    // message fields
    keyid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    credential_type: ::std::option::Option<Type>,
    credential_alg: ::protobuf::SingularField<::std::string::String>,
    sealing_alg: ::std::option::Option<algorithm::EncryptionAlgorithm>,
    encrypted_value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    salt: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    public_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    not_before: ::std::option::Option<u64>,
    not_after: ::std::option::Option<u64>,
    description: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
        unsafe {
            instance.get(|| {
                Credential {
                    keyid: ::protobuf::SingularField::none(),
                    credential_type: ::std::option::Option::None,
                    credential_alg: ::protobuf::SingularField::none(),
                    sealing_alg: ::std::option::Option::None,
                    encrypted_value: ::protobuf::SingularField::none(),
                    salt: ::protobuf::SingularField::none(),
                    public_key: ::protobuf::SingularField::none(),
                    not_before: ::std::option::Option::None,
                    not_after: ::std::option::Option::None,
                    description: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes keyid = 1;

    pub fn clear_keyid(&mut self) {
        self.keyid.clear();
    }

    pub fn has_keyid(&self) -> bool {
        self.keyid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyid(&mut self, v: ::std::vec::Vec<u8>) {
        self.keyid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keyid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.keyid.is_none() {
            self.keyid.set_default();
        };
        self.keyid.as_mut().unwrap()
    }

    // Take field
    pub fn take_keyid(&mut self) -> ::std::vec::Vec<u8> {
        self.keyid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_keyid(&self) -> &[u8] {
        match self.keyid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .ithos.object.Type credential_type = 2;

    pub fn clear_credential_type(&mut self) {
        self.credential_type = ::std::option::Option::None;
    }

    pub fn has_credential_type(&self) -> bool {
        self.credential_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_credential_type(&mut self, v: Type) {
        self.credential_type = ::std::option::Option::Some(v);
    }

    pub fn get_credential_type(&self) -> Type {
        self.credential_type.unwrap_or(Type::SIGNATURE_KEY_PAIR)
    }

    // optional string credential_alg = 3;

    pub fn clear_credential_alg(&mut self) {
        self.credential_alg.clear();
    }

    pub fn has_credential_alg(&self) -> bool {
        self.credential_alg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_credential_alg(&mut self, v: ::std::string::String) {
        self.credential_alg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_credential_alg(&mut self) -> &mut ::std::string::String {
        if self.credential_alg.is_none() {
            self.credential_alg.set_default();
        };
        self.credential_alg.as_mut().unwrap()
    }

    // Take field
    pub fn take_credential_alg(&mut self) -> ::std::string::String {
        self.credential_alg.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_credential_alg(&self) -> &str {
        match self.credential_alg.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .ithos.EncryptionAlgorithm sealing_alg = 4;

    pub fn clear_sealing_alg(&mut self) {
        self.sealing_alg = ::std::option::Option::None;
    }

    pub fn has_sealing_alg(&self) -> bool {
        self.sealing_alg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sealing_alg(&mut self, v: algorithm::EncryptionAlgorithm) {
        self.sealing_alg = ::std::option::Option::Some(v);
    }

    pub fn get_sealing_alg(&self) -> algorithm::EncryptionAlgorithm {
        self.sealing_alg.unwrap_or(algorithm::EncryptionAlgorithm::AES256GCM)
    }

    // optional bytes encrypted_value = 5;

    pub fn clear_encrypted_value(&mut self) {
        self.encrypted_value.clear();
    }

    pub fn has_encrypted_value(&self) -> bool {
        self.encrypted_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encrypted_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.encrypted_value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encrypted_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.encrypted_value.is_none() {
            self.encrypted_value.set_default();
        };
        self.encrypted_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_encrypted_value(&mut self) -> ::std::vec::Vec<u8> {
        self.encrypted_value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_encrypted_value(&self) -> &[u8] {
        match self.encrypted_value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes salt = 6;

    pub fn clear_salt(&mut self) {
        self.salt.clear();
    }

    pub fn has_salt(&self) -> bool {
        self.salt.is_some()
    }

    // Param is passed by value, moved
    pub fn set_salt(&mut self, v: ::std::vec::Vec<u8>) {
        self.salt = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_salt(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.salt.is_none() {
            self.salt.set_default();
        };
        self.salt.as_mut().unwrap()
    }

    // Take field
    pub fn take_salt(&mut self) -> ::std::vec::Vec<u8> {
        self.salt.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_salt(&self) -> &[u8] {
        match self.salt.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes public_key = 7;

    pub fn clear_public_key(&mut self) {
        self.public_key.clear();
    }

    pub fn has_public_key(&self) -> bool {
        self.public_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.public_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.public_key.is_none() {
            self.public_key.set_default();
        };
        self.public_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_public_key(&mut self) -> ::std::vec::Vec<u8> {
        self.public_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_public_key(&self) -> &[u8] {
        match self.public_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint64 not_before = 8;

    pub fn clear_not_before(&mut self) {
        self.not_before = ::std::option::Option::None;
    }

    pub fn has_not_before(&self) -> bool {
        self.not_before.is_some()
    }

    // Param is passed by value, moved
    pub fn set_not_before(&mut self, v: u64) {
        self.not_before = ::std::option::Option::Some(v);
    }

    pub fn get_not_before(&self) -> u64 {
        self.not_before.unwrap_or(0)
    }

    // optional uint64 not_after = 9;

    pub fn clear_not_after(&mut self) {
        self.not_after = ::std::option::Option::None;
    }

    pub fn has_not_after(&self) -> bool {
        self.not_after.is_some()
    }

    // Param is passed by value, moved
    pub fn set_not_after(&mut self, v: u64) {
        self.not_after = ::std::option::Option::Some(v);
    }

    pub fn get_not_after(&self) -> u64 {
        self.not_after.unwrap_or(0)
    }

    // optional string description = 10;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        if self.description.is_none() {
            self.description.set_default();
        };
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        match self.description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Credential {
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.keyid));
                }
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.credential_type = ::std::option::Option::Some(tmp);
                }
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type,
                                                                   is,
                                                                   &mut self.credential_alg));
                }
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.sealing_alg = ::std::option::Option::Some(tmp);
                }
                5 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type,
                                                                  is,
                                                                  &mut self.encrypted_value));
                }
                6 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.salt));
                }
                7 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type,
                                                                  is,
                                                                  &mut self.public_key));
                }
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.not_before = ::std::option::Option::Some(tmp);
                }
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.not_after = ::std::option::Option::Some(tmp);
                }
                10 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type,
                                                                   is,
                                                                   &mut self.description));
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
        for value in &self.keyid {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        }
        for value in &self.credential_type {
            my_size += ::protobuf::rt::enum_size(2, *value);
        }
        for value in &self.credential_alg {
            my_size += ::protobuf::rt::string_size(3, &value);
        }
        for value in &self.sealing_alg {
            my_size += ::protobuf::rt::enum_size(4, *value);
        }
        for value in &self.encrypted_value {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        }
        for value in &self.salt {
            my_size += ::protobuf::rt::bytes_size(6, &value);
        }
        for value in &self.public_key {
            my_size += ::protobuf::rt::bytes_size(7, &value);
        }
        for value in &self.not_before {
            my_size +=
                ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.not_after {
            my_size +=
                ::protobuf::rt::value_size(9, *value, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.description {
            my_size += ::protobuf::rt::string_size(10, &value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.keyid.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.credential_type {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.credential_alg.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.sealing_alg {
            try!(os.write_enum(4, v.value()));
        };
        if let Some(v) = self.encrypted_value.as_ref() {
            try!(os.write_bytes(5, &v));
        };
        if let Some(v) = self.salt.as_ref() {
            try!(os.write_bytes(6, &v));
        };
        if let Some(v) = self.public_key.as_ref() {
            try!(os.write_bytes(7, &v));
        };
        if let Some(v) = self.not_before {
            try!(os.write_uint64(8, v));
        };
        if let Some(v) = self.not_after {
            try!(os.write_uint64(9, v));
        };
        if let Some(v) = self.description.as_ref() {
            try!(os.write_string(10, &v));
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
        ::std::any::TypeId::of::<Credential>()
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
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "keyid",
                    Credential::has_keyid,
                    Credential::get_keyid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "credential_type",
                    Credential::has_credential_type,
                    Credential::get_credential_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "credential_alg",
                    Credential::has_credential_alg,
                    Credential::get_credential_alg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "sealing_alg",
                    Credential::has_sealing_alg,
                    Credential::get_sealing_alg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "encrypted_value",
                    Credential::has_encrypted_value,
                    Credential::get_encrypted_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "salt",
                    Credential::has_salt,
                    Credential::get_salt,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "public_key",
                    Credential::has_public_key,
                    Credential::get_public_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "not_before",
                    Credential::has_not_before,
                    Credential::get_not_before,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "not_after",
                    Credential::has_not_after,
                    Credential::get_not_after,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "description",
                    Credential::has_description,
                    Credential::get_description,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Credential>("Credential",
                                                                          fields,
                                                                          file_descriptor_proto())
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

impl ::std::cmp::PartialEq for Credential {
    fn eq(&self, other: &Credential) -> bool {
        self.keyid == other.keyid && self.credential_type == other.credential_type &&
        self.credential_alg == other.credential_alg &&
        self.sealing_alg == other.sealing_alg &&
        self.encrypted_value == other.encrypted_value && self.salt == other.salt &&
        self.public_key == other.public_key && self.not_before == other.not_before &&
        self.not_after == other.not_after &&
        self.description == other.description && self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Credential {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
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
        let mut digests: Vec<Vec<u8>> = Vec::new();

        // digests.push(objecthash_struct_member!("keyid", *self.keyid.get_ref()));
        // digests.push(objecthash_struct_member!("credential_type", self.credential_type.unwrap() as u32));
        // digests.push(objecthash_struct_member!("credential_alg", *self.credential_alg.get_ref()));
        digests.push(objecthash_struct_member!("sealing_alg", self.sealing_alg.unwrap() as u32));
        digests.push(objecthash_struct_member!("encrypted_value", *self.encrypted_value.get_ref()));
        digests.push(objecthash_struct_member!("salt", *self.salt.get_ref()));
        digests.push(objecthash_struct_member!("public_key", *self.public_key.get_ref()));
        digests.push(objecthash_struct_member!("not_before", self.not_before.unwrap() as i64));
        digests.push(objecthash_struct_member!("not_after", self.not_after.unwrap() as i64));
        digests.push(objecthash_struct_member!("description", *self.description.get_ref()));

        digests.sort();

        hasher.update(objecthash::types::DICT_TAG);

        for value in &digests {
            hasher.update(value);
        }

    }
}
