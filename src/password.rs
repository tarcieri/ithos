use pwhash::scrypt::{self, ScryptParams};
use ring::constant_time;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum PasswordAlgorithm {
    SCRYPT,
}

#[inline(always)]
fn params() -> ScryptParams {
    ScryptParams::new(16, 8, 1)
}

pub fn derive(alg: PasswordAlgorithm, salt: &[u8], password: &str, out: &mut [u8]) {
    // scrypt is the only password hashing algorithm we support for now
    assert!(alg == PasswordAlgorithm::SCRYPT);

    scrypt::scrypt(password.as_bytes(), salt, &params(), out);
}

pub fn verify(alg: PasswordAlgorithm,
              salt: &[u8],
              password: &str,
              previously_derived: &[u8])
              -> bool {
    // scrypt is the only password hashing algorithm we support for now
    assert!(alg == PasswordAlgorithm::SCRYPT);

    let mut out = vec![0u8; previously_derived.len()];
    scrypt::scrypt(password.as_bytes(), &*salt, &params(), &mut out);

    constant_time::verify_slices_are_equal(&previously_derived, &out).is_ok()
}

#[cfg(test)]
mod tests {
    use password;
    use password::PasswordAlgorithm;

    const PASSWORD: &'static str = "The Magic Words are Squeamish Ossifrage";

    #[test]
    fn test_password() {
        let salt = [0u8; 32];
        let mut derived_buf = [0u8; 32];

        password::derive(PasswordAlgorithm::SCRYPT, &salt, PASSWORD, &mut derived_buf);

        assert!(password::verify(PasswordAlgorithm::SCRYPT, &salt, PASSWORD, &derived_buf));
        assert!(!password::verify(PasswordAlgorithm::SCRYPT, &salt, "WRONG", &derived_buf));
    }
}
