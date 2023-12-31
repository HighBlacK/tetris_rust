//! This module contains functions that are used by the highscores module.
//! 
//! # Examples
//! 
//! ```
//! let path = "src/experiments/tetris/highscores/scores.sav";
//! let loaded_file = lib::load_file(path.to_owned());
//! ```
//! 
//! ```
//! let str = String::from("{\"1\":\"test\"}");
//! let serialized = lib::serialize_save(str);
//! let deserialized = lib::deserialize_save(serialized);
//! assert_eq!(deserialized, str);
//! ```
//! 

use std::collections::{BTreeMap, HashMap};
use std::fs::{self, write, File};
use std::io::{self, ErrorKind, Write};
use std::str;
use std::fs::OpenOptions;
use std::os::windows::fs::OpenOptionsExt;

use winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN;

use serde::{Serialize, Deserialize};

use super::error::{SaveError, SaveErrorKind, SaveResult, HandleSaveError};

/// Holds the player name and score.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub name: String,
    pub score: u32,
}

impl PlayerInfo {
    /// Creates a new PlayerInfo instance.
    pub fn new() -> PlayerInfo {
        PlayerInfo::default() 
    }
}

/// Loads the highscores from the save file.
/// 
/// # TODO
/// 
/// * Move the error handling in this function to a separate function. 
pub fn load_file(path: String) -> Vec<u8> {
    let expect_txt: String = format!("The file '{}' should exist", path);
    let loaded_file: Vec<u8> = fs::read(&path).unwrap_or_else(|error| { 
        if error.kind() == ErrorKind::NotFound {
            create_file(path.clone()).cathegorize().unwrap();
            fs::read(&path).expect(&expect_txt)
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    return loaded_file;
}

/// Deserializes the save data.
pub fn deserialize_save(str: String) -> SaveResult<BTreeMap<u32, String>> {
    let mut map: BTreeMap<u32, String> = BTreeMap::new();

    let deserialize: HashMap<u32, String> = deserialize_to_map(str).cathegorize()?;

    for (k, v) in deserialize {
        map.insert(k, v);
    }

    return Ok(map);
}

/// Serializes the save data.
pub fn serialize_save(scores: &BTreeMap<u32, String>) -> SaveResult<String> {
    match serde_json::to_string(&scores) {
        Ok(saveformat) => Ok(saveformat),
        Err(_) => Err(Box::new(SaveError::new(
            SaveErrorKind::JsonError,
            "Serialization error".to_owned(),
        ))),
    }
}

pub fn delete_save_data(path: String) -> SaveResult<()> {
    remove_file(path).cathegorize()?;
    Ok(())
}

/// Returns to the main menu.
pub fn back_to_main_menu() {
    println!("Returning to main menu");
    //TODO: return to main menu
}

//ERROR HANDLING

/// Write the 'contents' to the file at 'path'.
pub fn write_to_file<T>(path: String,contents: T) -> SaveResult<()>
where T: AsRef<[u8]> 
{
    match  write(path, contents) {
        Ok(written) => Ok(written),
        Err(_) => {
            return Err(Box::new(SaveError::new(
                SaveErrorKind::FileError, 
                "Error while writing file".to_owned()
            )))
        }
    }
}

/// Writes the 'contents' to a hidden file at 'path'.
pub fn write_to_hidden_file<T>(path: &str, contents: T) -> SaveResult<()>
where T: AsRef<[u8]>
{   
    let mut file = match OpenOptions::new().write(true).create(true).attributes(FILE_ATTRIBUTE_HIDDEN).open(path.to_owned()) {
        Ok(file) => file,
        Err(_) => {
            return Err(Box::new(SaveError::new(
                SaveErrorKind::FileError,
                "Error while opening file".to_owned(),
            )))
        }
    };
    match file.write_all(contents.as_ref()) {
        Ok(written) => Ok(written),
        Err(_) => {
            return Err(Box::new(SaveError::new(
                SaveErrorKind::FileError,
                "Error while writing file".to_owned(),
            )))
        }
    }
}

/// Creates a file at 'path'.
pub fn create_file(path: String) -> SaveResult<File> {
    match File::create(path) {
        Ok(created) => Ok(created),
        Err(_) => {
            return Err(Box::new(SaveError::new(
                SaveErrorKind::FileError, 
            "Error while creating file".to_owned()
        )))
        }
    }
}

/// Removes the file at 'path'.
pub fn remove_file(path: String) -> SaveResult<()> {
    match fs::remove_file(path) {
        Ok(removed) => Ok(removed),
        Err(_) =>  {
            return Err(Box::new(SaveError::new(
                SaveErrorKind::FileError,
                "Error while removing file".to_owned(),
            )))
        },
    }
}

/// Reads a line from the standard input and adds it to the buffer.
pub fn read_line(buffer: &mut String) -> SaveResult<usize> {
    match io::stdin().read_line(buffer) {
        Ok(line_read) => Ok(line_read),
        Err(_) => Err(Box::new(SaveError::new(
            SaveErrorKind::Io,
            "Error while reading input".to_owned(),
        ))),
    }
}

/// Converts the decrypted bytes to a string.
pub fn from_bytes_to_str(decrypted: &Vec<u8>) -> SaveResult<&str> {
    match str::from_utf8(&decrypted) {
        Ok(bts) => Ok(bts),
        Err(_) => {
            return Err(Box::new(SaveError::new(
                SaveErrorKind::DecryptionError,
                "error while converting bytes to string".to_owned(),
            )))
        }
    }

}

/// Deserializes the save data and puts it in a map.
fn deserialize_to_map(str: String) -> SaveResult<HashMap<u32, String>> {
    let deserialize: HashMap<u32, String> = match serde_json::from_str(str.as_str()) {
        Ok(deserialized) => deserialized,
        Err(_) => {
            return Err(Box::new(SaveError::new(
                SaveErrorKind::JsonError,
                "Error while deserializing".to_owned(),
            )))
        }
    };

    return Ok(deserialize);
}

