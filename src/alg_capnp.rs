// Generated by the capnpc-rust plugin to the Cap'n Proto schema compiler.
// DO NOT EDIT.
// source: alg.capnp


#[repr(u16)]
#[derive(Clone, Copy, PartialEq)]
pub enum DigestAlg {
  Sha256 = 0,
}
impl ::capnp::traits::FromU16 for DigestAlg {
  #[inline]
  fn from_u16(value: u16) -> ::std::result::Result<DigestAlg, ::capnp::NotInSchema> {
    match value {
      0 => ::std::result::Result::Ok(DigestAlg::Sha256),
      n => ::std::result::Result::Err(::capnp::NotInSchema(n)),
    }
  }
}
impl ::capnp::traits::ToU16 for DigestAlg {
  #[inline]
  fn to_u16(self) -> u16 { self as u16 }
}
impl ::capnp::traits::HasTypeId for DigestAlg {
  #[inline]
  fn type_id() -> u64 { 0xff9a7f608e491cfeu64 }
}

#[repr(u16)]
#[derive(Clone, Copy, PartialEq)]
pub enum EncryptionAlg {
  Aes256gcm = 0,
}
impl ::capnp::traits::FromU16 for EncryptionAlg {
  #[inline]
  fn from_u16(value: u16) -> ::std::result::Result<EncryptionAlg, ::capnp::NotInSchema> {
    match value {
      0 => ::std::result::Result::Ok(EncryptionAlg::Aes256gcm),
      n => ::std::result::Result::Err(::capnp::NotInSchema(n)),
    }
  }
}
impl ::capnp::traits::ToU16 for EncryptionAlg {
  #[inline]
  fn to_u16(self) -> u16 { self as u16 }
}
impl ::capnp::traits::HasTypeId for EncryptionAlg {
  #[inline]
  fn type_id() -> u64 { 0xf6683835abd77c49u64 }
}

#[repr(u16)]
#[derive(Clone, Copy, PartialEq)]
pub enum SignatureAlg {
  Ed25519 = 0,
}
impl ::capnp::traits::FromU16 for SignatureAlg {
  #[inline]
  fn from_u16(value: u16) -> ::std::result::Result<SignatureAlg, ::capnp::NotInSchema> {
    match value {
      0 => ::std::result::Result::Ok(SignatureAlg::Ed25519),
      n => ::std::result::Result::Err(::capnp::NotInSchema(n)),
    }
  }
}
impl ::capnp::traits::ToU16 for SignatureAlg {
  #[inline]
  fn to_u16(self) -> u16 { self as u16 }
}
impl ::capnp::traits::HasTypeId for SignatureAlg {
  #[inline]
  fn type_id() -> u64 { 0xfbac4abdbe0880d3u64 }
}

#[repr(u16)]
#[derive(Clone, Copy, PartialEq)]
pub enum PasswordAlg {
  Scrypt = 0,
}
impl ::capnp::traits::FromU16 for PasswordAlg {
  #[inline]
  fn from_u16(value: u16) -> ::std::result::Result<PasswordAlg, ::capnp::NotInSchema> {
    match value {
      0 => ::std::result::Result::Ok(PasswordAlg::Scrypt),
      n => ::std::result::Result::Err(::capnp::NotInSchema(n)),
    }
  }
}
impl ::capnp::traits::ToU16 for PasswordAlg {
  #[inline]
  fn to_u16(self) -> u16 { self as u16 }
}
impl ::capnp::traits::HasTypeId for PasswordAlg {
  #[inline]
  fn type_id() -> u64 { 0x853f25503dd52821u64 }
}

#[repr(u16)]
#[derive(Clone, Copy, PartialEq)]
pub enum CipherSuite {
  V0 = 0,
}
impl ::capnp::traits::FromU16 for CipherSuite {
  #[inline]
  fn from_u16(value: u16) -> ::std::result::Result<CipherSuite, ::capnp::NotInSchema> {
    match value {
      0 => ::std::result::Result::Ok(CipherSuite::V0),
      n => ::std::result::Result::Err(::capnp::NotInSchema(n)),
    }
  }
}
impl ::capnp::traits::ToU16 for CipherSuite {
  #[inline]
  fn to_u16(self) -> u16 { self as u16 }
}
impl ::capnp::traits::HasTypeId for CipherSuite {
  #[inline]
  fn type_id() -> u64 { 0xe599b1793c3b1012u64 }
}
