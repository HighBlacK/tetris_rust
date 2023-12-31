//! Compression and decompression of the save data.
//! 
//! # Examples
//! 
//! ```
//! let serialized_save = serialize_save(player_info);
//! 
//! let compressed_save = compress_saves(serialized_save);
//! let decompressed_save = decompress_saves(loaded_file);
//! 
//! assert_eq!(serialized_save, decompressed_save);
//! ```
//! 
//! # Note
//! 
//! The compression and decompression of the save data is optional.
//! 
//! # See also
//! 
//! - [encryption.rs][encryption.rs]

use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use std::io::Write;
use std::io::Read;

use super::error::{SaveErrorKind, SaveError, SaveResult, HandleSaveError};

/// Compresses the save data.
pub fn compress_saves<T>(serialized_save: T) -> SaveResult<Vec<u8>> 
where T: AsRef<[u8]>
{
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    write_all_to_buffer(serialized_save, &mut encoder).cathegorize()?;
    let compressed_save = finish_encode(encoder).cathegorize()?;
    return Ok(compressed_save)
}

/// Decompresses the save data.
pub fn decompress_saves(loaded_file: Vec<u8>) -> SaveResult<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(loaded_file.as_slice());
    let mut decompressed_file = Vec::new();
    read_all_to_buffer(&mut decompressed_file, &mut decoder).cathegorize()?;
    return Ok(decompressed_file)
}

/// Finishes the encoding.
fn finish_encode(encoder: flate2::write::ZlibEncoder<Vec<u8>>) -> SaveResult<Vec<u8>> {
    match encoder.finish() {
        Ok(e) => Ok(e),
        Err(_) => {
            return Err(Box::new(SaveError::new(
                SaveErrorKind::CompressionError,
                "error while finishing compression".to_owned(),
            )))
        }
    }
}

/// Writes all the bytes to the buffer.
fn write_all_to_buffer<T>(buffer: T, operator: &mut flate2::write::ZlibEncoder<Vec<u8>>) -> SaveResult<()> 
where T: AsRef<[u8]>
{
    match operator.write_all(buffer.as_ref()) {
        Ok(_) => Ok(()),
        Err(_) => {
            return Err(Box::new(SaveError::new(
                SaveErrorKind::CompressionError,
                "error while writing to buffer".to_owned(),
            )))
        }
    }
}

/// Reads all the bytes and write to the buffer.
fn read_all_to_buffer(buffer: &mut Vec<u8>, operator: &mut flate2::read::ZlibDecoder<&[u8]>) -> SaveResult<()> {
    match operator.read_to_end(buffer) {
        Ok(_) => Ok(()),
        Err(_) => {
            return Err(Box::new(SaveError::new(
                SaveErrorKind::DecompressionError, 
                "error while reading to buffer".to_owned()
            )))
        }
    }
}