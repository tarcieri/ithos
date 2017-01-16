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

use objecthash::{self, ObjectHash, ObjectHasher};
use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Domain {
    // message fields
    description: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Domain {}

impl Domain {
    pub fn new() -> Domain {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Domain {
        static mut instance: ::protobuf::lazy::Lazy<Domain> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Domain,
        };
        unsafe {
            instance.get(|| {
                Domain {
                    description: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string description = 1;

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

impl ::protobuf::Message for Domain {
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
        for value in &self.description {
            my_size += ::protobuf::rt::string_size(1, &value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.description.as_ref() {
            try!(os.write_string(1, &v));
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
        ::std::any::TypeId::of::<Domain>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Domain {
    fn new() -> Domain {
        Domain::new()
    }

    fn descriptor_static(_: ::std::option::Option<Domain>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "description",
                    Domain::has_description,
                    Domain::get_description,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Domain>("Domain",
                                                                      fields,
                                                                      file_descriptor_proto())
            })
        }
    }
}

impl ::protobuf::Clear for Domain {
    fn clear(&mut self) {
        self.clear_description();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Domain {
    fn eq(&self, other: &Domain) -> bool {
        self.description == other.description && self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Domain {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] =
    &[0x0a, 0x13, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x2f, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e,
      0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0c, 0x69, 0x74, 0x68, 0x6f, 0x73, 0x2e, 0x6f,
      0x62, 0x6a, 0x65, 0x63, 0x74, 0x22, 0x2a, 0x0a, 0x06, 0x44, 0x6f, 0x6d, 0x61, 0x69, 0x6e,
      0x12, 0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e,
      0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70,
      0x74, 0x69, 0x6f, 0x6e, 0x4a, 0xb3, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x07, 0x01,
      0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02,
      0x12, 0x03, 0x02, 0x08, 0x14, 0x0a, 0x43, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x05, 0x00,
      0x07, 0x01, 0x1a, 0x37, 0x20, 0x54, 0x6f, 0x70, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x20, 0x64,
      0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x6e, 0x20, 0x6f, 0x72,
      0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x28, 0x69, 0x2e, 0x65,
      0x2e, 0x20, 0x44, 0x4e, 0x53, 0x20, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x29, 0x0a, 0x0a,
      0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x05, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04,
      0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x04, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
      0x02, 0x00, 0x04, 0x12, 0x04, 0x06, 0x04, 0x05, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
      0x02, 0x00, 0x05, 0x12, 0x03, 0x06, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
      0x00, 0x01, 0x12, 0x03, 0x06, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
      0x03, 0x12, 0x03, 0x06, 0x19, 0x1a, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33];

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
impl ObjectHash for Domain {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        if self.description.is_some() {
            objecthash_struct!(hasher, "description" => *self.description.get_ref())
        }
    }
}
