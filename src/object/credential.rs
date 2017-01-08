use algorithm::{EncryptionAlgorithm, SignatureAlgorithm};
use buffoon::{Serialize, Deserialize, OutputStream, InputStream};
use error::{Error, Result};
use object::{AllowsChild, Object};
use objecthash::{self, ObjectHash, ObjectHasher};
use proto::{ToProto, FromProto};
use signature::KeyPair;
use std::io;
use timestamp::Timestamp;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Type {
    SignatureKeyPair(SignatureAlgorithm),
}

impl Type {
    fn id(&self) -> u32 {
        match *self {
            Type::SignatureKeyPair(_) => 1,
        }
    }

    fn alg(&self) -> String {
        match *self {
            // TODO: factor this into SignatureAlgorithm
            Type::SignatureKeyPair(SignatureAlgorithm::Ed25519) => String::from("Ed25519"),
        }
    }

    // TODO: actually support more algorithms
    fn from_id_and_alg(credential_id: Option<u32>, credential_alg: Option<String>) -> Result<Type> {
        if credential_id != Some(1) {
            return Err(Error::parse(None));
        }

        if credential_alg != Some(String::from("Ed25519")) {
            return Err(Error::parse(None));
        }

        Ok(Type::SignatureKeyPair(SignatureAlgorithm::Ed25519))
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match *self {
            Type::SignatureKeyPair(_) => String::from("SIGNATURE_KEY_PAIR"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CredentialEntry {
    pub keyid: Vec<u8>,
    pub credential_type: Type,
    pub sealing_alg: Option<EncryptionAlgorithm>,
    pub encrypted_value: Option<Vec<u8>>,
    pub salt: Option<Vec<u8>>,
    pub public_key: Option<Vec<u8>>,
    pub not_before: Option<Timestamp>,
    pub not_after: Option<Timestamp>,
    pub description: Option<String>,
}

impl CredentialEntry {
    pub fn from_signature_keypair(signature_alg: SignatureAlgorithm,
                                  sealing_alg: EncryptionAlgorithm,
                                  sealed_keypair: &[u8],
                                  salt: &[u8],
                                  public_key: &[u8],
                                  not_before: Timestamp,
                                  not_after: Timestamp,
                                  description: Option<String>)
                                  -> CredentialEntry {
        // Ed25519 is the only signature algorithm we presently support
        assert!(signature_alg == SignatureAlgorithm::Ed25519);

        CredentialEntry {
            keyid: Vec::from(public_key),
            credential_type: Type::SignatureKeyPair(SignatureAlgorithm::Ed25519),
            sealing_alg: Some(sealing_alg),
            encrypted_value: Some(Vec::from(sealed_keypair)),
            salt: Some(Vec::from(salt)),
            public_key: Some(Vec::from(public_key)),
            not_before: Some(not_before),
            not_after: Some(not_after),
            description: description,
        }
    }

    pub fn unseal_signature_keypair(&self, symmetric_key_bytes: &[u8]) -> Result<KeyPair> {
        // Ed25519 is the only signature algorithm we presently support
        if self.credential_type != Type::SignatureKeyPair(SignatureAlgorithm::Ed25519) {
            return Err(Error::bad_type(None));
        }

        let encrypted_value = match self.encrypted_value {
            Some(ref value) => value,
            None => return Err(Error::corrupt_data(None)),
        };

        let sealing_alg = try!(self.sealing_alg.ok_or(Error::corrupt_data(None)));

        let public_key = match self.public_key {
            Some(ref key) => key,
            None => return Err(Error::corrupt_data(None)),
        };

        KeyPair::unseal(SignatureAlgorithm::Ed25519,
                        sealing_alg,
                        symmetric_key_bytes,
                        encrypted_value,
                        public_key)
    }
}

impl AllowsChild for CredentialEntry {
    #[inline]
    fn allows_child(_child: &Object) -> bool {
        false
    }
}

impl Serialize for CredentialEntry {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        try!(out.write(1, &self.keyid));
        try!(out.write(2, &self.credential_type.id()));
        try!(out.write(3, &self.credential_type.alg()));

        if let Some(ref sealing_alg) = self.sealing_alg {
            try!(out.write(4, sealing_alg));
        }

        if let Some(ref encrypted_value) = self.encrypted_value {
            try!(out.write(5, encrypted_value));
        }

        if let Some(ref salt) = self.salt {
            try!(out.write(6, salt));
        }

        if let Some(ref public_key) = self.public_key {
            try!(out.write(7, public_key));
        }

        if let Some(not_before) = self.not_before {
            try!(out.write(8, &not_before));
        }

        if let Some(not_after) = self.not_after {
            try!(out.write(9, &not_after));
        }

        if let Some(ref description) = self.description {
            try!(out.write(10, description));
        }

        Ok(())
    }
}

impl Deserialize for CredentialEntry {
    fn deserialize<R: io::Read>(i: &mut InputStream<R>) -> io::Result<CredentialEntry> {
        let mut keyid: Option<Vec<u8>> = None;
        let mut credential_id: Option<u32> = None;
        let mut credential_alg: Option<String> = None;
        let mut sealing_alg: Option<EncryptionAlgorithm> = None;
        let mut encrypted_value: Option<Vec<u8>> = None;
        let mut salt: Option<Vec<u8>> = None;
        let mut public_key: Option<Vec<u8>> = None;
        let mut not_before: Option<Timestamp> = None;
        let mut not_after: Option<Timestamp> = None;
        let mut description: Option<String> = None;

        while let Some(f) = try!(i.read_field()) {
            match f.tag() {
                1 => keyid = Some(try!(f.read())),
                2 => credential_id = Some(try!(f.read())),
                3 => credential_alg = Some(try!(f.read())),
                4 => sealing_alg = Some(try!(f.read())),
                5 => encrypted_value = Some(try!(f.read())),
                6 => salt = Some(try!(f.read())),
                7 => public_key = Some(try!(f.read())),
                8 => not_before = Some(try!(f.read())),
                9 => not_after = Some(try!(f.read())),
                10 => description = Some(try!(f.read())),
                _ => try!(f.skip()),
            }
        }

        let credential_type = try!(Type::from_id_and_alg(credential_id, credential_alg)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid credential type")));

        Ok(CredentialEntry {
            keyid: required!(keyid, "CredentialObject::keyid"),
            credential_type: credential_type,
            sealing_alg: sealing_alg,
            encrypted_value: encrypted_value,
            salt: salt,
            public_key: public_key,
            not_before: not_before,
            not_after: not_after,
            description: description,
        })
    }
}

impl ToProto for CredentialEntry {}
impl FromProto for CredentialEntry {}

impl ObjectHash for CredentialEntry {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        let mut digests: Vec<Vec<u8>> = Vec::new();

        let credential_id_string = self.credential_type.to_string();
        let credential_alg_string = self.credential_type.alg();

        digests.push(objecthash_struct_member!("keyid", self.keyid));
        digests.push(objecthash_struct_member!("credential_type", credential_id_string));
        digests.push(objecthash_struct_member!("credential_alg", credential_alg_string));

        if let Some(ref sealing_alg) = self.sealing_alg {
            digests.push(objecthash_struct_member!("sealing_alg", sealing_alg.to_string()));
        }

        if let Some(ref encrypted_value) = self.encrypted_value {
            digests.push(objecthash_struct_member!("encrypted_value", *encrypted_value));
        }

        if let Some(ref salt) = self.salt {
            digests.push(objecthash_struct_member!("salt", *salt));
        }

        if let Some(ref public_key) = self.public_key {
            digests.push(objecthash_struct_member!("public_key", *public_key));
        }

        if let Some(not_before) = self.not_before {
            digests.push(objecthash_struct_member!("not_before", not_before));
        }

        if let Some(not_after) = self.not_after {
            digests.push(objecthash_struct_member!("not_after", not_after));
        }

        if let Some(ref description) = self.description {
            digests.push(objecthash_struct_member!("description", *description));
        }

        digests.sort();

        hasher.update(objecthash::types::DICT_TAG);

        for value in &digests {
            hasher.update(value);
        }
    }
}

#[cfg(test)]
mod tests {

    use algorithm::{EncryptionAlgorithm, SignatureAlgorithm};
    use object::credential::{CredentialEntry, Type};
    use proto::{FromProto, ToProto};
    use timestamp::Timestamp;

    const EXAMPLE_PUBLIC_KEY: &'static [u8] = b"public-key-placeholder";
    const EXAMPLE_SEALED_KEY: &'static [u8] = b"ciphertext-placeholder";
    const EXAMPLE_SALT: &'static [u8] = b"NaCl";

    fn example_timestamp() -> Timestamp {
        Timestamp::at(1_231_006_505)
    }

    fn example_credential() -> CredentialEntry {
        CredentialEntry {
            keyid: Vec::from(EXAMPLE_PUBLIC_KEY),
            credential_type: Type::SignatureKeyPair(SignatureAlgorithm::Ed25519),
            sealing_alg: Some(EncryptionAlgorithm::Aes256Gcm),
            encrypted_value: Some(Vec::from(EXAMPLE_SEALED_KEY)),
            salt: Some(Vec::from(EXAMPLE_SALT)),
            public_key: Some(Vec::from(EXAMPLE_PUBLIC_KEY)),
            not_before: Some(example_timestamp()),
            not_after: Some(example_timestamp()),
            description: Some(String::from("An example credential")),
        }
    }

    #[test]
    fn test_proto_serialization() {
        let proto = example_credential().to_proto();
        assert!(proto.is_ok());

        let credential = CredentialEntry::from_proto(&proto.unwrap());
        assert!(credential.is_ok());
    }
}
