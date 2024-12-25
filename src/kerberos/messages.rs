use crypto::rc4::Rc4;
use crypto::symmetriccipher::SynchronousStreamCipher;

/*
    encrypt_with_rc4 : Return u8 byte vector of RC4-encrypted input data.
    This is used in the encryption of timestamps for Kerberos.
 */
fn encrypt_with_rc4(key: &[u8], data: &[u8]) -> Vec<u8> {
    // Create an RC4 cipher with the given key
    let mut rc4 = Rc4::new(key);

    // Encrypt the data
    let mut result : Vec<u8> = vec![];
    let encrypted_data = rc4.process(data, &mut result);

    result
}