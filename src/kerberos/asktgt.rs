use connector::KerberosConnection;
use crate::kerberos::connector;
use ntlm_hash::ntlm_hash;
use crypto::rc4::Rc4;
use crypto::symmetriccipher::SynchronousStreamCipher;
use hex::decode;

/*
    ASKTGT
    - Open kerberos connection

 */


fn encrypt_with_rc4(key: &[u8], data: &[u8]) -> Vec<u8> {
    // Create an RC4 cipher with the given key
    let mut rc4 = Rc4::new(key);

    // Encrypt the data
    let encrypted_data = rc4.process(data, &mut result);

    result
}
pub fn asktgt(args: Vec<String>)
{
    let mut connection = KerberosConnection::new(args);
    println!("{:x?}", ntlm_hash("password123!"));
    // Given NTLM hash (hex) as a string
    let ntlm_hash_hex = "AE894FF0D092153701A0A06CE1864D62";

    // Convert the NTLM hash from hex to bytes
    let ntlm_hash = decode(ntlm_hash_hex).expect("Failed to decode NTLM hash");

    // Given encrypted timestamp (hex) as a string
    let encrypted_timestamp_hex = "557a9a8910057a861983e098cf28cb161e30a40ef130e3cc33904399c5854f90049feb6ef73f9e73bdf37f64e5";

    // Convert the encrypted timestamp from hex to bytes
    let encrypted_timestamp = decode(encrypted_timestamp_hex).expect("Failed to decode encrypted timestamp");

    // Decrypt the timestamp using RC4 with the NTLM hash as the key
    let decrypted_timestamp = encrypt_with_rc4(&ntlm_hash, &encrypted_timestamp);

    // Print the decrypted timestamp in UTF-8 string format
    if let Ok(decrypted_str) = String::from_utf8(decrypted_timestamp) {
        println!("Decrypted Timestamp: {}", decrypted_str);
    } else {
        println!("Decrypted data is not valid UTF-8.");
    }

    let mut resp : Vec<u8> = Vec::new();
    connection.read_stream(&mut resp);
    println!("{:x?}",resp);

}