use std::io;

use buffoon::{Serialize, Deserialize, OutputStream, InputStream};
use rustc_serialize::base64::{self, ToBase64};
use serde_json::builder::ObjectBuilder;

use algorithm::{EncryptionAlgorithm, SignatureAlgorithm};
use error::{Error, Result};
use proto::{ToProto, FromProto};
use object::{AllowsChild, Object};
use objecthash::{self, ObjectHash, ObjectHasher};

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
            return Err(Error::Parse);
        }

        if credential_alg != Some(String::from("Ed25519")) {
            return Err(Error::Parse);
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CredentialEntry {
    keyid: Vec<u8>,
    credential_type: Type,
    sealing_alg: EncryptionAlgorithm,
    encrypted_value: Vec<u8>,
    public_key: Option<Vec<u8>>,
    not_before: Option<u64>,
    not_after: Option<u64>,
    description: Option<String>,
}

impl CredentialEntry {
    pub fn signature_keypair(sealing_alg: EncryptionAlgorithm,
                             signature_alg: SignatureAlgorithm,
                             sealed_keypair: &[u8],
                             public_key: &[u8],
                             not_before: u64,
                             not_after: u64,
                             description: Option<String>)
                             -> CredentialEntry {
        // Ed25519 is the only signature algorithm we presently support
        assert!(signature_alg == SignatureAlgorithm::Ed25519);

        CredentialEntry {
            keyid: Vec::from(public_key),
            credential_type: Type::SignatureKeyPair(SignatureAlgorithm::Ed25519),
            sealing_alg: sealing_alg,
            encrypted_value: Vec::from(sealed_keypair),
            public_key: Some(Vec::from(public_key)),
            not_before: Some(not_before),
            not_after: Some(not_after),
            description: description,
        }
    }

    pub fn build_json(&self, builder: ObjectBuilder) -> ObjectBuilder {
        let builder = builder.insert("keyid", self.keyid.to_base64(base64::URL_SAFE))
            .insert("credential_type", self.credential_type.to_string())
            .insert("credential_alg", self.credential_type.alg())
            .insert("sealing_alg", self.sealing_alg.to_string())
            .insert("encrypted_value",
                    self.encrypted_value.to_base64(base64::URL_SAFE));

        let builder = match self.public_key {
            Some(ref public_key) => {
                builder.insert("public_key", public_key.to_base64(base64::URL_SAFE))
            }
            None => builder,
        };

        let builder = match self.not_before {
            Some(ref not_before) => builder.insert("not_before", not_before),
            None => builder,
        };

        let builder = match self.not_after {
            Some(ref not_after) => builder.insert("not_after", not_after),
            None => builder,
        };

        let builder = match self.description {
            Some(ref description) => builder.insert("description", description),
            None => builder,
        };

        builder
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
        try!(out.write(4, &self.sealing_alg.id()));
        try!(out.write(5, &self.encrypted_value));

        if let Some(ref public_key) = self.public_key {
            try!(out.write(6, public_key));
        }

        if let Some(not_before) = self.not_before {
            try!(out.write(7, &not_before));
        }

        if let Some(not_after) = self.not_after {
            try!(out.write(8, &not_after));
        }

        if let Some(ref description) = self.description {
            try!(out.write(9, description));
        }

        Ok(())
    }
}

impl Deserialize for CredentialEntry {
    fn deserialize<R: io::Read>(i: &mut InputStream<R>) -> io::Result<CredentialEntry> {
        let mut keyid: Option<Vec<u8>> = None;
        let mut credential_id: Option<u32> = None;
        let mut credential_alg: Option<String> = None;
        let mut sealing_alg: Option<u32> = None;
        let mut encrypted_value: Option<Vec<u8>> = None;
        let mut public_key: Option<Vec<u8>> = None;
        let mut not_before: Option<u64> = None;
        let mut not_after: Option<u64> = None;
        let mut description: Option<String> = None;

        while let Some(f) = try!(i.read_field()) {
            match f.tag() {
                1 => keyid = Some(try!(f.read())),
                2 => credential_id = Some(try!(f.read())),
                3 => credential_alg = Some(try!(f.read())),
                4 => sealing_alg = Some(try!(f.read())),
                5 => encrypted_value = Some(try!(f.read())),
                6 => public_key = Some(try!(f.read())),
                7 => not_before = Some(try!(f.read())),
                8 => not_after = Some(try!(f.read())),
                9 => description = Some(try!(f.read())),
                _ => try!(f.skip()),
            }
        }

        let credential_type = try!(Type::from_id_and_alg(credential_id, credential_alg)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid credential type")));

        // Ensure sealing algorithm is Aes256Gcm
        if sealing_alg != Some(EncryptionAlgorithm::Aes256Gcm.id()) {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid sealing algorithm"));
        }

        Ok(CredentialEntry {
            keyid: required!(keyid, "CredentialObject::keyid"),
            credential_type: credential_type,
            sealing_alg: EncryptionAlgorithm::Aes256Gcm,
            encrypted_value: required!(encrypted_value, "CredentialObject::encrypted_value"),
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
        digests.push(objecthash_struct_member!("sealing_alg", self.sealing_alg.to_string()));
        digests.push(objecthash_struct_member!("encrypted_value", self.encrypted_value));

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
            hasher.update(&value);
        }
    }
}
