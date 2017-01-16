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

use objecthash::{self, ObjectHash, ObjectHasher};
use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Signature {
    // message fields
    algorithm: ::std::option::Option<super::algorithm::SignatureAlgorithm>,
    public_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
        unsafe {
            instance.get(|| {
                Signature {
                    algorithm: ::std::option::Option::None,
                    public_key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .ithos.SignatureAlgorithm algorithm = 1;

    pub fn clear_algorithm(&mut self) {
        self.algorithm = ::std::option::Option::None;
    }

    pub fn has_algorithm(&self) -> bool {
        self.algorithm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_algorithm(&mut self, v: super::algorithm::SignatureAlgorithm) {
        self.algorithm = ::std::option::Option::Some(v);
    }

    pub fn get_algorithm(&self) -> super::algorithm::SignatureAlgorithm {
        self.algorithm.unwrap_or(super::algorithm::SignatureAlgorithm::Ed25519)
    }

    // optional bytes public_key = 2;

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

    // optional bytes value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for Signature {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.algorithm = ::std::option::Option::Some(tmp);
                }
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type,
                                                                  is,
                                                                  &mut self.public_key));
                }
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value));
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
        for value in &self.algorithm {
            my_size += ::protobuf::rt::enum_size(1, *value);
        }
        for value in &self.public_key {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        }
        for value in &self.value {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.algorithm {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.public_key.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_bytes(3, &v));
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
        ::std::any::TypeId::of::<Signature>()
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
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "algorithm",
                    Signature::has_algorithm,
                    Signature::get_algorithm,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "public_key",
                    Signature::has_public_key,
                    Signature::get_public_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    Signature::has_value,
                    Signature::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Signature>("Signature",
                                                                         fields,
                                                                         file_descriptor_proto())
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

impl ::std::cmp::PartialEq for Signature {
    fn eq(&self, other: &Signature) -> bool {
        self.algorithm == other.algorithm && self.public_key == other.public_key &&
        self.value == other.value && self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Signature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
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
        // TODO: Don't Panic!
        objecthash_struct!(
            hasher,
            "algorithm" => self.algorithm.unwrap() as u32,
            "public_key" => *self.public_key.get_ref(),
            "value" => *self.value.get_ref()
        )
    }
}
