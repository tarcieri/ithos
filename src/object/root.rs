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

use algorithm;
use objecthash::{self, ObjectHash, ObjectHasher};
use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Root {
    // message fields
    pub digest_alg: algorithm::DigestAlgorithm,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Root {}

impl Root {
    pub fn new() -> Root {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Root {
        static mut instance: ::protobuf::lazy::Lazy<Root> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Root,
        };
        unsafe { instance.get(Root::new) }
    }

    // .ithos.DigestAlgorithm digest_alg = 1;

    pub fn clear_digest_alg(&mut self) {
        self.digest_alg = algorithm::DigestAlgorithm::SHA256;
    }

    // Param is passed by value, moved
    pub fn set_digest_alg(&mut self, v: algorithm::DigestAlgorithm) {
        self.digest_alg = v;
    }

    pub fn get_digest_alg(&self) -> algorithm::DigestAlgorithm {
        self.digest_alg
    }

    fn get_digest_alg_for_reflect(&self) -> &algorithm::DigestAlgorithm {
        &self.digest_alg
    }

    fn mut_digest_alg_for_reflect(&mut self) -> &mut algorithm::DigestAlgorithm {
        &mut self.digest_alg
    }
}

impl ::protobuf::Message for Root {
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
                    self.digest_alg = tmp;
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
        if self.digest_alg != algorithm::DigestAlgorithm::SHA256 {
            my_size += ::protobuf::rt::enum_size(1, self.digest_alg);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self,
                                  os: &mut ::protobuf::CodedOutputStream)
                                  -> ::protobuf::ProtobufResult<()> {
        if self.digest_alg != algorithm::DigestAlgorithm::SHA256 {
            os.write_enum(1, self.digest_alg.value())?;
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

impl ::protobuf::MessageStatic for Root {
    fn new() -> Root {
        Root::new()
    }

    fn descriptor_static(_: ::std::option::Option<Root>)
                         -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<algorithm::DigestAlgorithm>>(
                    "digest_alg",
                    Root::get_digest_alg_for_reflect,
                    Root::mut_digest_alg_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Root>(
                    "Root",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Root {
    fn clear(&mut self) {
        self.clear_digest_alg();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Root {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Root {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] =
    &[0x0a, 0x11, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x2f, 0x72, 0x6f, 0x6f, 0x74, 0x2e, 0x70,
      0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0c, 0x69, 0x74, 0x68, 0x6f, 0x73, 0x2e, 0x6f, 0x62, 0x6a,
      0x65, 0x63, 0x74, 0x1a, 0x0f, 0x61, 0x6c, 0x67, 0x6f, 0x72, 0x69, 0x74, 0x68, 0x6d, 0x2e,
      0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x3d, 0x0a, 0x04, 0x52, 0x6f, 0x6f, 0x74, 0x12, 0x35,
      0x0a, 0x0a, 0x64, 0x69, 0x67, 0x65, 0x73, 0x74, 0x5f, 0x61, 0x6c, 0x67, 0x18, 0x01, 0x20,
      0x01, 0x28, 0x0e, 0x32, 0x16, 0x2e, 0x69, 0x74, 0x68, 0x6f, 0x73, 0x2e, 0x44, 0x69, 0x67,
      0x65, 0x73, 0x74, 0x41, 0x6c, 0x67, 0x6f, 0x72, 0x69, 0x74, 0x68, 0x6d, 0x52, 0x09, 0x64,
      0x69, 0x67, 0x65, 0x73, 0x74, 0x41, 0x6c, 0x67, 0x4a, 0xc5, 0x01, 0x0a, 0x06, 0x12, 0x04,
      0x00, 0x00, 0x09, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a,
      0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x14, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00,
      0x12, 0x03, 0x04, 0x07, 0x18, 0x0a, 0x4a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00,
      0x09, 0x01, 0x1a, 0x3e, 0x20, 0x52, 0x6f, 0x6f, 0x74, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x79,
      0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f,
      0x72, 0x79, 0x20, 0x74, 0x72, 0x65, 0x65, 0x20, 0x28, 0x52, 0x6f, 0x6f, 0x74, 0x20, 0x44,
      0x53, 0x45, 0x20, 0x69, 0x6e, 0x20, 0x4c, 0x44, 0x41, 0x50, 0x20, 0x70, 0x61, 0x72, 0x6c,
      0x61, 0x6e, 0x63, 0x65, 0x29, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
      0x07, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x04,
      0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x08, 0x04, 0x07,
      0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x08, 0x04, 0x13,
      0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x14, 0x1e, 0x0a,
      0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x21, 0x22, 0x62, 0x06,
      0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33];

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
impl ObjectHash for Root {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        objecthash_struct!(hasher, "digest_alg" => self.digest_alg as u32)
    }
}
