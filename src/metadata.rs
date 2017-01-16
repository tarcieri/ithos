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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Metadata {
    // message fields
    pub created_id: ::std::vec::Vec<u8>,
    pub updated_id: ::std::vec::Vec<u8>,
    pub created_at: u64,
    pub updated_at: u64,
    pub version: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Metadata {}

impl Metadata {
    pub fn new() -> Metadata {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Metadata {
        static mut instance: ::protobuf::lazy::Lazy<Metadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Metadata,
        };
        unsafe { instance.get(Metadata::new) }
    }

    // bytes created_id = 1;

    pub fn clear_created_id(&mut self) {
        self.created_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_created_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.created_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_created_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.created_id
    }

    // Take field
    pub fn take_created_id(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.created_id, ::std::vec::Vec::new())
    }

    pub fn get_created_id(&self) -> &[u8] {
        &self.created_id
    }

    fn get_created_id_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.created_id
    }

    fn mut_created_id_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.created_id
    }

    // bytes updated_id = 2;

    pub fn clear_updated_id(&mut self) {
        self.updated_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_updated_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.updated_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_updated_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.updated_id
    }

    // Take field
    pub fn take_updated_id(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.updated_id, ::std::vec::Vec::new())
    }

    pub fn get_updated_id(&self) -> &[u8] {
        &self.updated_id
    }

    fn get_updated_id_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.updated_id
    }

    fn mut_updated_id_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.updated_id
    }

    // uint64 created_at = 3;

    pub fn clear_created_at(&mut self) {
        self.created_at = 0;
    }

    // Param is passed by value, moved
    pub fn set_created_at(&mut self, v: u64) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> u64 {
        self.created_at
    }

    fn get_created_at_for_reflect(&self) -> &u64 {
        &self.created_at
    }

    fn mut_created_at_for_reflect(&mut self) -> &mut u64 {
        &mut self.created_at
    }

    // uint64 updated_at = 4;

    pub fn clear_updated_at(&mut self) {
        self.updated_at = 0;
    }

    // Param is passed by value, moved
    pub fn set_updated_at(&mut self, v: u64) {
        self.updated_at = v;
    }

    pub fn get_updated_at(&self) -> u64 {
        self.updated_at
    }

    fn get_updated_at_for_reflect(&self) -> &u64 {
        &self.updated_at
    }

    fn mut_updated_at_for_reflect(&mut self) -> &mut u64 {
        &mut self.updated_at
    }

    // uint64 version = 5;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }

    pub fn get_version(&self) -> u64 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &u64 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut u64 {
        &mut self.version
    }
}

impl ::protobuf::Message for Metadata {
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
                                                                    &mut self.created_id)?;
                }
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type,
                                                                    is,
                                                                    &mut self.updated_id)?;
                }
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.created_at = tmp;
                }
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.updated_at = tmp;
                }
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.version = tmp;
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
        if self.created_id != ::std::vec::Vec::new() {
            my_size += ::protobuf::rt::bytes_size(1, &self.created_id);
        };
        if self.updated_id != ::std::vec::Vec::new() {
            my_size += ::protobuf::rt::bytes_size(2, &self.updated_id);
        };
        if self.created_at != 0 {
            my_size += ::protobuf::rt::value_size(3,
                                                  self.created_at,
                                                  ::protobuf::wire_format::WireTypeVarint);
        };
        if self.updated_at != 0 {
            my_size += ::protobuf::rt::value_size(4,
                                                  self.updated_at,
                                                  ::protobuf::wire_format::WireTypeVarint);
        };
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(5,
                                                  self.version,
                                                  ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if self.created_id != ::std::vec::Vec::new() {
            os.write_bytes(1, &self.created_id)?;
        };
        if self.updated_id != ::std::vec::Vec::new() {
            os.write_bytes(2, &self.updated_id)?;
        };
        if self.created_at != 0 {
            os.write_uint64(3, self.created_at)?;
        };
        if self.updated_at != 0 {
            os.write_uint64(4, self.updated_at)?;
        };
        if self.version != 0 {
            os.write_uint64(5, self.version)?;
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

impl ::protobuf::MessageStatic for Metadata {
    fn new() -> Metadata {
        Metadata::new()
    }

    fn descriptor_static(_: ::std::option::Option<Metadata>)
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
                    "created_id",
                    Metadata::get_created_id_for_reflect,
                    Metadata::mut_created_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "updated_id",
                    Metadata::get_updated_id_for_reflect,
                    Metadata::mut_updated_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "created_at",
                    Metadata::get_created_at_for_reflect,
                    Metadata::mut_created_at_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "updated_at",
                    Metadata::get_updated_at_for_reflect,
                    Metadata::mut_updated_at_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "version",
                    Metadata::get_version_for_reflect,
                    Metadata::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Metadata>(
                    "Metadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Metadata {
    fn clear(&mut self) {
        self.clear_created_id();
        self.clear_updated_id();
        self.clear_created_at();
        self.clear_updated_at();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Metadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Metadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] =
    &[0x0a, 0x0e, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74,
      0x6f, 0x12, 0x05, 0x69, 0x74, 0x68, 0x6f, 0x73, 0x22, 0xa0, 0x01, 0x0a, 0x08, 0x4d, 0x65,
      0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x72, 0x65, 0x61, 0x74,
      0x65, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x63, 0x72,
      0x65, 0x61, 0x74, 0x65, 0x64, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x75, 0x70, 0x64, 0x61,
      0x74, 0x65, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x75,
      0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x72, 0x65,
      0x61, 0x74, 0x65, 0x64, 0x5f, 0x61, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09,
      0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x41, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x75, 0x70,
      0x64, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x61, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52,
      0x09, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x41, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x76,
      0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x76,
      0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x4a, 0xbd, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
      0x0b, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a,
      0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0d, 0x0a, 0x35, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
      0x05, 0x00, 0x0b, 0x01, 0x1a, 0x29, 0x20, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61,
      0x20, 0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74,
      0x68, 0x20, 0x61, 0x6e, 0x20, 0x69, 0x74, 0x68, 0x6f, 0x73, 0x20, 0x65, 0x6e, 0x74, 0x72,
      0x79, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x05, 0x08, 0x10, 0x0a,
      0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x02, 0x18, 0x0a, 0x0d, 0x0a,
      0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x06, 0x02, 0x05, 0x12, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x06, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05,
      0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x09, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
      0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x06, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
      0x02, 0x01, 0x12, 0x03, 0x07, 0x02, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
      0x04, 0x12, 0x04, 0x07, 0x02, 0x06, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
      0x05, 0x12, 0x03, 0x07, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
      0x12, 0x03, 0x07, 0x09, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
      0x03, 0x07, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x08,
      0x02, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x08, 0x02,
      0x07, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x08, 0x02,
      0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x08, 0x09, 0x13,
      0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x08, 0x16, 0x17, 0x0a,
      0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x09, 0x02, 0x18, 0x0a, 0x0d, 0x0a,
      0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x09, 0x02, 0x08, 0x18, 0x0a, 0x0c, 0x0a,
      0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x09, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
      0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x09, 0x09, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
      0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x09, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
      0x02, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
      0x04, 0x12, 0x04, 0x0a, 0x02, 0x09, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
      0x05, 0x12, 0x03, 0x0a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01,
      0x12, 0x03, 0x0a, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12,
      0x03, 0x0a, 0x16, 0x17, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33];

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
