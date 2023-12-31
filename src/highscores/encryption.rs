//! This module is used to encrypt and decrypt the highscores file.
//! 
//! # Example
//! 
//! ```
//! let saveformat = String::from("{\"1\":\"test\"}");
//! 
//! let encrypted = encryption::encrypt_saves(saveformat).cathegorize()?;
//! let decrypted = encryption::decrypt_saves(encrypted).cathegorize()?;
//! 
//! assert_eq!(decrypted, "{\"1\":\"test\"}");
//! ```
//! 
//! # Note
//! 
//! It's completely optional and can be disabled by setting the ENABLE_ENCRYPTION constant to false.
//! The module is not flexible because it uses a hardcoded encryption key.
//! This option mainly exists to prevent the laymen user from modifying the highscores file. It's not meant to be a secure encryption.
//! It's computationally expensive. It adds about 0.5 seconds to the program's runtime.
//! Compression is recommended since it would accomplish the same goal and has the added benefit of reducing the save file size.
//! 
//! # Safety
//! 
//! The module is not safe because it uses a hardcoded encryption key.
//! 
//! # Todo
//! 
//! * Consider making the empty file hidden
//! * Look for faster encryption
//! 
//! # See also
//! 
//! - [compression.rs][compression.rs]

use age::decryptor::PassphraseDecryptor;
use age::stream::{StreamReader, StreamWriter};
use age::{Decryptor, Encryptor};
use age::secrecy::Secret;

use std::io::{Read, Write};
use std::str;
use std::sync::Arc;
use super::error::{SaveError, HandleSaveError, SaveResult};
use super::error::SaveErrorKind;
use super::lib;


static ENCRYPTION_KEY: &str = "o2Os#XJo&c1IP2@p5TOv9z@Cl5qw?9XejR2M0Di#";
static EMPTY_FILE_PATH: &str = "src/experiments/tetris/highscores/.empty";

/// Encrypts the save data.
pub fn encrypt_saves(saveformat: String) -> SaveResult<Vec<u8>> {
    let to_str: &[u8] = saveformat.as_str().as_bytes();
    let encrypted: Vec<u8> = {
        let encryptor: Encryptor =
            age::Encryptor::with_user_passphrase(Secret::new(ENCRYPTION_KEY.to_owned()));

        let mut encrypted: Vec<u8> = vec![];
        let mut writer:StreamWriter<&mut Vec<u8>> = write_encrypted_bytes(encryptor, &mut encrypted)?;
        write_all_to_buffer(to_str, &mut writer)?;
        writer.finish().unwrap(); //To fix: finish_writing(&mut writer).handle_error();

        encrypted
    };

    return Ok(encrypted);
}

/// Decrypts the save data.
pub fn decrypt_saves(loaded_save: Vec<u8>) -> SaveResult<String> {
    let buffer: Vec<u8> = loaded_save;
    let decrypted: Vec<u8> = {
        let decryptor: Decryptor<&[u8]> = create_decryptor(&buffer).cathegorize()?;
        let decrypted: PassphraseDecryptor<&[u8]> = {
            match decryptor {
                age::Decryptor::Passphrase(d) => d,
                _ => unreachable!(),
            }
        };

        let mut decrypted_bytes: Vec<u8> = vec![];
        let mut reader: StreamReader<&[u8]> = decryption(decrypted).cathegorize()?;
        reading_decrypted_bytes(&mut reader, &mut decrypted_bytes).cathegorize()?;

        decrypted_bytes
    };

    let from_bytes_to_str: String = lib::from_bytes_to_str(&decrypted).cathegorize()?.to_owned();
    return Ok(from_bytes_to_str);
}



// EMPTY FILE FUNCTIONS

/// Creates an empty encrypted file.
pub fn create_empty_encrypted_file() -> SaveResult<()> {
    let empty: String = String::new();
    let encrypted: Vec<u8> = encrypt_saves(empty)?;

    lib::write_to_file(EMPTY_FILE_PATH.to_owned(), encrypted)?;
    Ok(())
}


/// Contains the contents of the empty encrypted file.
/// 
/// # Note
/// 
/// This function should only be used for testing purposes.
fn _empty_encryted_string() -> Arc<[u8]> {
    if ENCRYPTION_KEY == "o2Os#XJo&c1IP2@p5TOv9z@Cl5qw?9XejR2M0Di#" {
        let _empty_encryted_string: Arc<[u8]> = vec!(97, 103, 101, 45, 101, 110, 99, 114, 121, 112, 116, 105, 111, 110, 46, 111, 114, 103, 47, 118, 49, 10, 45, 62, 32, 115, 99, 114, 121, 112, 116, 32, 75, 113, 113, 81, 57, 113, 77, 114, 118, 112, 104, 54, 116, 83, 105, 104, 68, 97, 89, 55, 75, 81, 32, 49, 52, 10, 109, 84, 90, 53, 115, 111, 52, 118, 74, 116, 77, 89, 105, 80, 111, 113, 47, 72, 66, 98, 69, 48, 43, 114, 78, 73, 68, 113, 99, 111, 72, 79, 110, 73, 47, 83, 68, 71, 55, 72, 121, 109, 48, 10, 45, 45, 45, 32, 98, 89, 117, 52, 89, 80, 122, 87, 76, 49, 79, 83, 84, 48, 72, 84, 104, 65, 108, 100, 111, 102, 100, 68, 70, 116, 80, 108, 117, 69, 99, 80, 100, 101, 74, 80, 52, 83, 77, 77, 72, 79, 119, 10, 146, 172, 155, 154, 81, 145, 161, 199, 127, 230, 31, 158, 115, 255, 100, 31, 218, 108, 181, 55, 86, 23, 39, 9, 86, 94, 104, 129, 229, 163, 158, 36).into();
        return _empty_encryted_string;
    }
    else {
        panic!("The encryption key has been changed. The empty encrypted string needs to be updated.");
    }
    
}

//ERROR HANDLING FUNCTIONS 

// Encryption functions

/// Creates an encryptor.
fn write_encrypted_bytes(encryptor: Encryptor, encrypted: &mut Vec<u8>) -> SaveResult<StreamWriter<&mut Vec<u8>>> {
    match encryptor.wrap_output(encrypted){
        Ok(written) => Ok(written),
        Err(_) => {
            return Err(Box::new(SaveError::new(
                SaveErrorKind::EncrytionError, 
                "error while writing encryted bytes to buffer".to_owned()
            )))
        }
    }
}

/// Writes all the bytes to the buffer.
fn write_all_to_buffer(to_str: &[u8], writer: &mut StreamWriter<&mut Vec<u8>>) -> SaveResult<()> {
    match writer.write_all(to_str) {
        Ok(_) => Ok(()),
        Err(_) => {
            return Err(Box::new(SaveError::new(
                SaveErrorKind::EncrytionError,
                "error while writing to buffer".to_owned(),
            )))
        }
    }
}

//FIXME: This doesn't work
/*fn finish_writing<'a>(writer: &mut StreamWriter<&'a mut Vec<u8>>) -> SaveResult<&'a mut Vec<u8>> {
    match writer.finish() {
        Ok(finished) => Ok(finished),
        Err(_) => {
            return Err(SaveError::new(
                SaveErrorKind::EncrytionError,
                "error while finishing writing".to_owned(),
            ))
        }
    }
}*/

// Decryption functions

/// Reads the decrypted bytes and adds them to the decrypted bytes vector.
fn reading_decrypted_bytes(reader: &mut StreamReader<&[u8]>, decrypted_bytes: &mut Vec<u8>) -> SaveResult<usize> {
    match reader.read_to_end(decrypted_bytes) {
        Ok(read) => Ok(read),
        Err(_) => {
            return Err(Box::new(SaveError::new(
                SaveErrorKind::DecryptionError, 
                "error while reading decrypted bytes".to_owned()
            )))
        }
    }
}

/// Decrypts the save data.
fn decryption(decrypted: PassphraseDecryptor<&[u8]>) -> SaveResult<StreamReader<&[u8]>> {
    match decrypted.decrypt(&Secret::new(ENCRYPTION_KEY.to_owned()), None) {
        Ok(reader) => Ok(reader),
        Err(_) => {
            return Err(Box::new(SaveError::new(
                SaveErrorKind::DecryptionError,
                "error during decryption".to_owned()
            )))
        }
    }

}

/// Creates a decryptor.
fn create_decryptor(buffer: &[u8]) -> SaveResult<Decryptor<&[u8]>> {
    match age::Decryptor::new_buffered(buffer) {
        Ok(decryptor) => Ok(decryptor),
        Err(_error) => {
            return Err(Box::new(SaveError::new(
                SaveErrorKind::CorruptSave,
                "save file corrupted".to_owned(),
            )))
        }
    }
}