extern crate blake2;
extern crate ring;
extern crate untrusted;
extern crate uuid;

use ring::aead::{Algorithm, OpeningKey, SealingKey};
use ring::{aead, error, rand, signature};

use uuid::Uuid;

use blake2::{Blake2b, Digest};
use std::fs;
use std::hash::Hash;
use std::io::{self, Read};

/// # File Encryption Module
///
/// The module creates the encrypted file using the crate ring and the ring и алгоритма `ring::aead::CHACHA20_POLY1305` algorithm.
/// The hash received from the encrypted file is signed using crate `ring::signature::Ed25519KeyPair`.
/// After checking the hash signature of the encrypted file, you can decrypt the file to its original state.
///
/// ## Examples
///
/// Basic usage:
///
/// ```rust
///
///  use encrypt_file::*;
///
///  fn test()->Result<(),encrypt_file::Error>{
///    let path = std::path::Path::new("pic.jpg");
///    let (uuid_name,hash_file) = get_file_name_and_hash(path)?;
///
///    // create an encrypted version of the file
///    let hash_file_encrypt:Vec<u8> = encrypt_file_content(path,&uuid_name)?;
///
///    // sign a hash
///    let (peer_public_key_bytes,sig_bytes) = gen_fingerprint(&hash_file_encrypt)?;
///
///    // check hash
///    if check_key_is_correct(&hash_file_encrypt,&peer_public_key_bytes,&sig_bytes).is_ok(){
///
///      // verify signature
///      deciphering_file_content( std::path::Path::new(&uuid_name) ,std::path::Path::new("pic_deciphering.jpg"));
///    }   
///  Ok(())
///  }
/// ```
mod encrypt_file {

    use super::*;

    /// Contains a set of error types that can occur in the module.
    #[derive(Debug)]
    pub enum Error {
        CryptoError,
        InvalidSignature,
        Unspecified,
        IOError(std::io::Error),
        UuidError(String),
    }
    /// Implementing Unspecified Transformation Types of Errors.
    impl From<ring::error::Unspecified> for Error {
        fn from(err: ring::error::Unspecified) -> Self {
            Error::CryptoError
        }
    }
    /// Implementing io::Error Transformation Types of Errors.  
    impl From<io::Error> for Error {
        fn from(err: io::Error) -> Self {
            Error::IOError(err)
        }
    }

    /// Create a new encrypted version of this file and
    /// return the hash of the encrypted file.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///
    ///  use encrypt_file::*;
    ///
    ///  fn test()->Result<(),encrypt_file::Error>{
    ///    let path = std::path::Path::new("pic.jpg");
    ///    let (uuid_name,hash_file) = get_file_name_and_hash(path)?;
    ///
    ///    // create an encrypted version of the file  
    ///    let hash_file_encrypt:Vec<u8> = encrypt_file_content(path,&uuid_name)?;
    ///  Ok(())
    ///  }
    /// ```
    pub fn encrypt_file_content(
        path: &std::path::Path,
        uuid_name: &str,
    ) -> Result<(Vec<u8>), Error> {
        let aead_alg: &'static aead::Algorithm = &aead::CHACHA20_POLY1305;
        let key_len = aead_alg.key_len();
        let key_data = vec![0u8; key_len];
        let s_key: ring::aead::SealingKey = aead::SealingKey::new(aead_alg, &key_data[..key_len])?;
        let o_key: ring::aead::OpeningKey = aead::OpeningKey::new(aead_alg, &key_data[..key_len])?;

        let nonce_len = aead_alg.nonce_len();
        let nonce = vec![0u8; nonce_len * 2];

        let prefix_len = 0;
        let tag_len = aead_alg.tag_len();
        let ad: [u8; 0] = [];

        let mut to_seal: Vec<u8> = std::fs::read(path)?;

        for _ in 0..tag_len {
            to_seal.push(0);
        }
        let to_seal = &to_seal[..];

        let mut to_open = Vec::from(to_seal);
        let ciphertext_len =
            aead::seal_in_place(&s_key, &nonce[..nonce_len], &ad, &mut to_open, tag_len)?;
        let to_open: &[u8] = &to_open[..ciphertext_len];

        std::fs::write(uuid_name.clone(), to_open)?;

        let (_, hash_file_encrypt) = get_file_name_and_hash(std::path::Path::new(&uuid_name))?;
        Ok(hash_file_encrypt)
    }

    /// Return the signature of the received data.
    /// It is better to sign a hash file than the file itself.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///
    ///  use encrypt_file::*;
    ///
    ///  fn test()->Result<(),encrypt_file::Error>{
    ///    let path = std::path::Path::new("pic.jpg");
    ///    let (uuid_name,hash_file) = get_file_name_and_hash(path)?;
    ///
    ///    // создать шифрованную версия файла  
    ///    let hash_file_encrypt:Vec<u8> = encrypt_file_content(path,&uuid_name)?;
    ///    // подписать хеш
    ///    // let (peer_public_key_bytes,sig_bytes) = gen_fingerprint(&hash_file_encrypt).unwrap_or((vec![1u8;0],vec![1u8;0]));
    ///
    ///    let (peer_public_key_bytes,sig_bytes) = gen_fingerprint(&hash_file_encrypt)?;
    ///  
    ///  Ok(())
    ///  }
    /// ```
    pub fn gen_fingerprint(message: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)?;
        let key_pair: ring::signature::Ed25519KeyPair =
            signature::Ed25519KeyPair::from_pkcs8(untrusted::Input::from(&pkcs8_bytes))?;

        //let mut message:Vec<u8> = std::fs::read(path).unwrap();

        //Подпишите сообщение.
        let sig: ring::signature::Signature = key_pair.sign(message);

        let peer_public_key_bytes: &[u8] = key_pair.public_key_bytes();
        let sig_bytes: &[u8] = sig.as_ref();

        Ok((peer_public_key_bytes.to_vec(), sig_bytes.to_vec()))
    }

    /// Verification of a signature.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///
    ///  use encrypt_file::*;
    ///
    ///  fn test()->Result<(),encrypt_file::Error>{
    ///    let path = std::path::Path::new("pic.jpg");
    ///    let (uuid_name,hash_file) = get_file_name_and_hash(path)?;
    ///
    ///    // создать шифрованную версия файла  
    ///    let hash_file_encrypt:Vec<u8> = encrypt_file_content(path,&uuid_name)?;
    ///    // подписать хеш
    ///    // let (peer_public_key_bytes,sig_bytes) = gen_fingerprint(&hash_file_encrypt).unwrap_or((vec![1u8;0],vec![1u8;0]));
    ///
    ///    let (peer_public_key_bytes,sig_bytes) = gen_fingerprint(&hash_file_encrypt)?;
    ///
    ///    // проверить хеш
    ///
    ///    if check_key_is_correct(&hash_file_encrypt,&peer_public_key_bytes,&sig_bytes).is_ok(){
    ///
    ///      println!("Можно расшифровывать в исходную картинку");
    ///
    ///      deciphering_file_content( std::path::Path::new(&uuid_name) ,std::path::Path::new("pic_deciphering.jpg"));
    ///    }   
    ///  Ok(())
    ///  }
    /// ```
    pub fn check_key_is_correct(
        to_open: &[u8],
        peer_public_key_bytes: &[u8],
        sig_bytes: &[u8],
    ) -> Result<(), Error> {
        let peer_public_key = untrusted::Input::from(peer_public_key_bytes);
        let msg = untrusted::Input::from(to_open);
        let sig = untrusted::Input::from(sig_bytes);

        signature::verify(&signature::ED25519, peer_public_key, msg, sig)
            .map_err(|_| Error::InvalidSignature)
    }

    /// Return a new unique name for the file and hash of its contents.
    /// The contents of the file are created using crate `Blake2b`.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///
    ///  use encrypt_file::*;
    ///
    ///  fn test()->Result<(),encrypt_file::Error>{
    ///
    ///    let path = std::path::Path::new("pic.jpg");
    ///
    ///    let (uuid_name,hash_file) = get_file_name_and_hash(path)?;
    ///
    ///  Ok(())
    ///  }
    /// ```
    pub fn get_file_name_and_hash(path: &std::path::Path) -> Result<(String, Vec<u8>), Error> {
        let uuid =
            Uuid::new(uuid::UuidVersion::Random).ok_or(Error::UuidError("Error Uuid".to_string()))?;

        let mut file = fs::File::open(&path)?;
        let output = Blake2b::digest_reader(&mut file)?;

        let uuid_name: String = format!("{:x}.jpg", uuid.simple());
        //let hash_file:String  =  format!("{:x}" , output);

        Ok((uuid_name, output.to_vec()))
    }

    /// Decipher the received data.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///
    ///  use encrypt_file::*;
    ///
    ///  fn test()->Result<(),encrypt_file::Error>{
    ///    let path = std::path::Path::new("pic.jpg");
    ///    let (uuid_name,hash_file) = get_file_name_and_hash(path)?;
    ///
    ///    // создать шифрованную версия файла  
    ///    let hash_file_encrypt:Vec<u8> = encrypt_file_content(path,&uuid_name)?;
    ///    // подписать хеш
    ///    // let (peer_public_key_bytes,sig_bytes) = gen_fingerprint(&hash_file_encrypt).unwrap_or((vec![1u8;0],vec![1u8;0]));
    ///
    ///    let (peer_public_key_bytes,sig_bytes) = gen_fingerprint(&hash_file_encrypt)?;
    ///
    ///    // проверить хеш
    ///
    ///    if check_key_is_correct(&hash_file_encrypt,&peer_public_key_bytes,&sig_bytes).is_ok(){
    ///
    ///      println!("Можно расшифровывать в исходную картинку");
    ///
    ///      deciphering_file_content( std::path::Path::new(&uuid_name) ,std::path::Path::new("pic_deciphering.jpg"));
    ///    }   
    ///  Ok(())
    ///  }
    /// ```
    pub fn deciphering_file_content(
        path_open: &std::path::Path,
        path: &std::path::Path,
    ) -> Result<(), Error> {
        let to_open: std::vec::Vec<u8> = std::fs::read(path_open)?;
        let aead_alg: &'static aead::Algorithm = &aead::CHACHA20_POLY1305;

        let nonce_len = aead_alg.nonce_len();
        let nonce = vec![0u8; nonce_len * 2];
        let ad: [u8; 0] = [];
        let prefix_len = 0;

        let key_len = aead_alg.key_len();
        let key_data = vec![0u8; key_len];
        let o_key: ring::aead::OpeningKey = aead::OpeningKey::new(aead_alg, &key_data[..key_len])?;

        let mut in_out: Vec<u8> = Vec::from(to_open);
        let o_result: &mut [u8] =
            aead::open_in_place(&o_key, &nonce[..nonce_len], &ad, prefix_len, &mut in_out)?;

        std::fs::write(path, o_result)?;
        Ok(())
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_get_file_name_and_hash() {
            let path = std::path::Path::new("test.txt");
            assert!(fs::File::create(&path).is_ok());

            assert!(get_file_name_and_hash(path).is_ok());

            fs::remove_file(path);
        }

        #[test]
        fn test_encrypt_file_content() {
            let path = std::path::Path::new("test2.txt");
            assert!(fs::File::create(&path).is_ok());

            if let Ok(uuid) = Uuid::new(uuid::UuidVersion::Random)
                .ok_or(Error::UuidError("Error Uuid".to_string()))
            {
                let uuid_name: String = format!("{:x}.txt", uuid.simple());

                assert!(encrypt_file_content(path, &uuid_name).is_ok());

                fs::remove_file(uuid_name);
            } else {
                assert!(false);
            }
            fs::remove_file(path);
        }

        #[test]
        fn test_check_key_is_correct() {
            let path = std::path::Path::new("test_check.txt");
            assert!(fs::File::create(&path).is_ok());
            if let Ok(uuid) = Uuid::new(uuid::UuidVersion::Random)
                .ok_or(Error::UuidError("Error Uuid".to_string()))
            {
                let uuid_name: String = format!("{:x}.txt", uuid.simple());

                if let Ok(hash_file_encrypt) = encrypt_file_content(path, &uuid_name) {
                    if let Ok((peer_public_key_bytes, sig_bytes)) =
                        gen_fingerprint(&hash_file_encrypt)
                    {
                        assert!(
                            check_key_is_correct(
                                &hash_file_encrypt,
                                &peer_public_key_bytes,
                                &sig_bytes
                            ).is_ok()
                        );
                    } else {
                        assert!(false);
                    }
                } else {
                    assert!(false);
                }
                fs::remove_file(uuid_name);
            } else {
                assert!(false);
            }
            fs::remove_file(path);
        }
    }

}

use encrypt_file::*;

fn main() -> Result<(), encrypt_file::Error> {
    let path = std::path::Path::new("pic.jpg");

    let (uuid_name, hash_file) = get_file_name_and_hash(path)?;

    // создание подписи на хеш
    //let (peer_public_key_bytes,sig_bytes) = gen_fingerprint(&hash_file);

    // шифрованная версия файла
    let hash_file_encrypt: Vec<u8> = encrypt_file_content(path, &uuid_name)?;

    let (peer_public_key_bytes, sig_bytes) = gen_fingerprint(&hash_file_encrypt)?;

    // проверить хеш
    if check_key_is_correct(&hash_file_encrypt, &peer_public_key_bytes, &sig_bytes).is_ok() {
        println!("Можно расшифровывать в исходную картинку");
        deciphering_file_content(
            std::path::Path::new(&uuid_name),
            std::path::Path::new("pic_deciphering.jpg"),
        );
    }

    Ok(())
}
