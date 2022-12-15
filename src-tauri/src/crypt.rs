//! AES256 CBC、CTR mode encrypt decrypt demo
// use std::str;
use crypto::{symmetriccipher,buffer,aes,blockmodes};
use crypto::buffer::{ReadBuffer,WriteBuffer,BufferResult};
use crypto::aessafe::*;
use crypto::blockmodes::*;
use crypto::symmetriccipher::*;
// use rand::{Rng,OsRng};

// fn main(){
//     let message="Hello World!";

    // let mut key:[u8;32]=[0;32];
    // let mut iv:[u8;16]=[0;16];

    // In a real program, the key and iv may be determined
    // using some other mechanism. If a password is to be used
    // as a key, an algorithm like PBKDF2, Bcrypt, or Scrypt (all
    // supported by Rust-Crypto!) would be a good choice to derive
    // a password. For the purposes of this example, the key and
    // iv are just random values.
    // let mut rng=OsRng::new().ok().unwrap();
    // rng.fill_bytes(&mut key);
    // rng.fill_bytes(&mut iv);

    // let key = [2, 79, 183, 20, 91, 252, 102, 68, 5, 16, 248, 145, 16, 198, 42, 26, 28, 83, 78, 177, 178, 59, 157, 82, 209, 5, 126, 66, 180, 190, 225, 117];
    // let iv = [120, 47, 249, 122, 121, 130, 29, 98, 190, 142, 223, 196, 102, 238, 164, 111];

    // println!("{:?}", key);
    // println!("{:?}", iv);

//     let encrypted_data=aes256_encrypt(message.as_bytes());
//     let decrypted_data=aes256_decrypt(&encrypted_data[..]);

//     let crypt_message=str::from_utf8(decrypted_data.as_slice()).unwrap();

//     assert_eq!(message,crypt_message);
//     println!("{}",crypt_message);
// }

pub fn aes256_encrypt(data: &[u8])-> Vec<u8> {
    let key = [2, 79, 183, 20, 91, 252, 102, 68, 5, 16, 248, 145, 16, 198, 42, 26, 28, 83, 78, 177, 178, 59, 157, 82, 209, 5, 126, 66, 180, 190, 225, 117];
    let iv = [120, 47, 249, 122, 121, 130, 29, 98, 190, 142, 223, 196, 102, 238, 164, 111];
    aes256_cbc_encrypt(data, &key, &iv).unwrap()
}

pub fn aes256_decrypt(encrypted_data: &[u8])-> Vec<u8> {
    let key = [2, 79, 183, 20, 91, 252, 102, 68, 5, 16, 248, 145, 16, 198, 42, 26, 28, 83, 78, 177, 178, 59, 157, 82, 209, 5, 126, 66, 180, 190, 225, 117];
    let iv = [120, 47, 249, 122, 121, 130, 29, 98, 190, 142, 223, 196, 102, 238, 164, 111];
    aes256_cbc_decrypt(encrypted_data, &key, &iv).unwrap()
}


// Encrypt a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
fn aes256_cbc_encrypt(data: &[u8],key: &[u8], iv: &[u8])->Result<Vec<u8>,symmetriccipher::SymmetricCipherError>{
    let mut encryptor=aes::cbc_encryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result=Vec::<u8>::new();
    let mut read_buffer=buffer::RefReadBuffer::new(data);
    let mut buffer=[0;4096];
    let mut write_buffer=buffer::RefWriteBuffer::new(&mut buffer);

    loop{
        let result=encryptor.encrypt(&mut read_buffer,&mut write_buffer,true).unwrap();

        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow=>break,
            BufferResult::BufferOverflow=>{},
        }
    }

    Ok(final_result)
}

// Decrypts a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
fn aes256_cbc_decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}

// pub fn aes_ctr_mode(){
//     let message="Hello World! AES CTR MODE.";

//     let mut key:[u8;32]=[0;32];
//     let mut iv:[u8;16]=[0;16];

//     let mut rng=OsRng::new().ok().unwrap();
//     rng.fill_bytes(&mut key);
//     rng.fill_bytes(&mut iv);

//     let encrypted_data=aes256_ctr_encrypt(message.as_bytes(),&key,&iv).ok().unwrap();
//     let decrypted_data=aes256_ctr_decrypt(&encrypted_data[..],&key,&iv).ok().unwrap();

//     let crypt_message=str::from_utf8(decrypted_data.as_slice()).unwrap();

//     assert_eq!(message,crypt_message);
//     println!("{}",crypt_message);
// }

fn aes256_ctr_encrypt(data: &[u8],key: &[u8],iv: &[u8])->Result<Vec<u8>,symmetriccipher::SymmetricCipherError>{
    let mut final_result=Vec::<u8>::new();
    let mut read_buffer=buffer::RefReadBuffer::new(data);
    let mut buffer=[0;4096];
    let mut write_buffer=buffer::RefWriteBuffer::new(&mut buffer);

    let mut encoder=CtrMode::new(AesSafe256Encryptor::new(key),iv.to_vec());
    encoder.encrypt(&mut read_buffer,&mut write_buffer,true)?;

    final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
    Ok(final_result)
}

fn aes256_ctr_decrypt(encrypted_data: &[u8],key: &[u8], iv: &[u8])->Result<Vec<u8>,symmetriccipher::SymmetricCipherError>{
    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    let mut decoder=CtrMode::new(AesSafe256Encryptor::new(key),iv.to_vec());
    decoder.decrypt(&mut read_buffer,&mut write_buffer,true)?;

    final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
    Ok(final_result)
}
