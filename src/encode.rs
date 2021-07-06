/// Encrypt a plaintext by xoring it with a password
///
/// # Example
/// ```
/// use machiavelli::encode::xor;
///
/// let plaintext: Vec<u8> = vec![1,2,3,4,5];
/// let password: Vec<u8> = vec![0,1];
///
/// let cipher = xor(&plaintext, &password);
///
/// assert_eq!(vec![1,3,3,5,5], cipher);
///
/// ```
pub fn xor(plaintext: &[u8], password: &[u8]) -> Vec<u8> {
    let mut cipher = Vec::<u8>::new();
    let n = password.len();
    for i in 0..plaintext.len() {
        cipher.push(plaintext[i] ^ password[i%n])
    }
    cipher
}

/// Encrypt a string
///
/// # Example
/// ```
/// use machiavelli::encode::encrypt_str;
///
/// let message = "I am a string literal!";
/// let password = "passw0rd";
///
/// let cipher = encrypt_str(&message, &password);
///
/// ```
pub fn encrypt_str(message: &str, password: &str) -> Vec<u8> {
    let plaintext_u8 = message.as_bytes();
    let password_u8 = password.as_bytes();
    xor(&plaintext_u8, &password_u8)
}

/// Decrypt an array of bytes into a string
///
/// # Example
/// ```
/// use machiavelli::encode::{ encrypt_str, decrypt_str };
///
/// let message = "I am a string literal!";
/// let password = "passw0rd";
///
/// let cipher = encrypt_str(&message, &password);
/// let decrypted = decrypt_str(&cipher, &password).unwrap();
///
/// assert_eq!(message.to_string(), decrypted);
///
/// ```
pub fn decrypt_str(cipher: &[u8], password: &str) -> Result<String, std::str::Utf8Error> {
    let password_u8 = password.as_bytes();
    match std::str::from_utf8(&xor(&cipher, &password_u8)) {
        Ok(s) => return Ok(s.to_string()),
        Err(e) => return Err(e)
    };
}
