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

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DigestAlgorithm {
    SHA256 = 0,
}

impl ::protobuf::ProtobufEnum for DigestAlgorithm {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DigestAlgorithm> {
        match value {
            0 => ::std::option::Option::Some(DigestAlgorithm::SHA256),
            _ => ::std::option::Option::None,
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DigestAlgorithm] = &[DigestAlgorithm::SHA256];
        values
    }

    fn enum_descriptor_static(_: Option<DigestAlgorithm>)
                              -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DigestAlgorithm", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DigestAlgorithm {}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EncryptionAlgorithm {
    AES256GCM = 0,
}

impl ::protobuf::ProtobufEnum for EncryptionAlgorithm {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EncryptionAlgorithm> {
        match value {
            0 => ::std::option::Option::Some(EncryptionAlgorithm::AES256GCM),
            _ => ::std::option::Option::None,
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EncryptionAlgorithm] = &[EncryptionAlgorithm::AES256GCM];
        values
    }

    fn enum_descriptor_static(_: Option<EncryptionAlgorithm>)
                              -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EncryptionAlgorithm",
                                                         file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EncryptionAlgorithm {}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SignatureAlgorithm {
    Ed25519 = 0,
}

impl ::protobuf::ProtobufEnum for SignatureAlgorithm {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SignatureAlgorithm> {
        match value {
            0 => ::std::option::Option::Some(SignatureAlgorithm::Ed25519),
            _ => ::std::option::Option::None,
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SignatureAlgorithm] = &[SignatureAlgorithm::Ed25519];
        values
    }

    fn enum_descriptor_static(_: Option<SignatureAlgorithm>)
                              -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SignatureAlgorithm",
                                                         file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SignatureAlgorithm {}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CipherSuite {
    Ed25519_AES256GCM_SHA256 = 0,
}

impl ::protobuf::ProtobufEnum for CipherSuite {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CipherSuite> {
        match value {
            0 => ::std::option::Option::Some(CipherSuite::Ed25519_AES256GCM_SHA256),
            _ => ::std::option::Option::None,
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CipherSuite] = &[CipherSuite::Ed25519_AES256GCM_SHA256];
        values
    }

    fn enum_descriptor_static(_: Option<CipherSuite>)
                              -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CipherSuite", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CipherSuite {}

static file_descriptor_proto_data: &'static [u8] =
    &[0x0a, 0x0f, 0x61, 0x6c, 0x67, 0x6f, 0x72, 0x69, 0x74, 0x68, 0x6d, 0x2e, 0x70, 0x72, 0x6f,
      0x74, 0x6f, 0x12, 0x05, 0x69, 0x74, 0x68, 0x6f, 0x73, 0x2a, 0x1d, 0x0a, 0x0f, 0x44, 0x69,
      0x67, 0x65, 0x73, 0x74, 0x41, 0x6c, 0x67, 0x6f, 0x72, 0x69, 0x74, 0x68, 0x6d, 0x12, 0x0a,
      0x0a, 0x06, 0x53, 0x48, 0x41, 0x32, 0x35, 0x36, 0x10, 0x00, 0x2a, 0x24, 0x0a, 0x13, 0x45,
      0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x41, 0x6c, 0x67, 0x6f, 0x72, 0x69,
      0x74, 0x68, 0x6d, 0x12, 0x0d, 0x0a, 0x09, 0x41, 0x45, 0x53, 0x32, 0x35, 0x36, 0x47, 0x43,
      0x4d, 0x10, 0x00, 0x2a, 0x21, 0x0a, 0x12, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72,
      0x65, 0x41, 0x6c, 0x67, 0x6f, 0x72, 0x69, 0x74, 0x68, 0x6d, 0x12, 0x0b, 0x0a, 0x07, 0x45,
      0x64, 0x32, 0x35, 0x35, 0x31, 0x39, 0x10, 0x00, 0x2a, 0x2b, 0x0a, 0x0b, 0x43, 0x69, 0x70,
      0x68, 0x65, 0x72, 0x53, 0x75, 0x69, 0x74, 0x65, 0x12, 0x1c, 0x0a, 0x18, 0x45, 0x64, 0x32,
      0x35, 0x35, 0x31, 0x39, 0x5f, 0x41, 0x45, 0x53, 0x32, 0x35, 0x36, 0x47, 0x43, 0x4d, 0x5f,
      0x53, 0x48, 0x41, 0x32, 0x35, 0x36, 0x10, 0x00, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
      0x33];

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
