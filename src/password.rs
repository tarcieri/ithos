use ring::pbkdf2;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum PasswordAlgorithm {
    // TODO: support better algorithms than PBKDF2
    PBKDF2,
}

static PBKDF2_PRF: &'static pbkdf2::PRF = &pbkdf2::HMAC_SHA256;
static PBKDF2_ITER: usize = 100_000;

pub fn derive(alg: PasswordAlgorithm, salt: &[u8], password: &str, out: &mut [u8]) {
    // PBKDF2 is the only password hashing algorithm we support for now
    assert!(alg == PasswordAlgorithm::PBKDF2);

    pbkdf2::derive(PBKDF2_PRF, PBKDF2_ITER, salt, password.as_bytes(), out);
}

pub fn verify(alg: PasswordAlgorithm,
              salt: &[u8],
              password: &str,
              previously_derived: &[u8])
              -> bool {
    // PBKDF2 is the only password hashing algorithm we support for now
    assert!(alg == PasswordAlgorithm::PBKDF2);

    pbkdf2::verify(PBKDF2_PRF,
                   PBKDF2_ITER,
                   salt,
                   password.as_bytes(),
                   previously_derived)
        .is_ok()
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

        password::derive(PasswordAlgorithm::PBKDF2, &salt, PASSWORD, &mut derived_buf);

        assert!(password::verify(PasswordAlgorithm::PBKDF2, &salt, PASSWORD, &derived_buf));
        assert!(!password::verify(PasswordAlgorithm::PBKDF2, &salt, "WRONG", &derived_buf));
    }
}
