//! This module contains the error types and handling functions for saving highscores.
//!
//! The `SaveError` struct represents an error that occurs during saving highscores.
//! It contains information about the origin of the error and the kind of error.
//! The `SaveErrorKind` enum defines the different kinds of errors that can occur.
//! The module also provides functions for handling errors and returning appropriate values.
//! 
//! # Examples
//! 
//! ```
//! // Create a new SaveError instance
//! let error = SaveError::new(SaveErrorKind::CorruptSave, String::from("save file corrupted"));
//! 
//! // Print the error kind and message
//! println!("{:?}, {}", error.kind, error.message);
//! ```
//!
//! # TODO
//! Finish this module.
//! actually do something with the errors.
//! look into each error and see what can be done.

use std::{fmt::Display, fmt::Formatter};
use std::error::Error;
//use std::any::Any;

use super::lib;
use super::*;

static SAVE_PATH: &str = "src/experiments/tetris/highscores/scores.sav";

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SaveError {
    pub message: String,
    pub kind: SaveErrorKind,
}

impl SaveError{
    ///Creates a new SaveError instance.
    pub fn new(kind: SaveErrorKind, message: String) -> Self{
        SaveError {
            message,
            kind,
        }
    }

    ///Returns the kind of error as a str.
    fn as_str(&self) -> &'static str {
        self.kind.as_str()
    }

    /*///Returns the kind of error as a SaveErrorKind.
    fn kind(&self) -> SaveErrorKind {
        self.kind
    }

    ///Returns the message of the error as a string.
    fn message(&self) -> String {
        self.message.clone()
    }

    ///Converts the error to a Box<dyn Any>.
    fn to_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    ///Converts the error to a Box<dyn Error>.
    fn to_box_error(&self) -> Box<dyn Error> {
        Box::new(self.clone())
    }*/

}

impl Error for SaveError {
    ///Returns the error as an optional source.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}


impl Display for SaveError {
    ///Formats the error.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SaveErrorKind{
    CorruptSave,
    DecryptionError,
    EncrytionError,
    CompressionError,
    DecompressionError,
    JsonError,
    FileError,
    Io,
    Debug,
    Generic,
    #[default]
    Null,
}


impl Display for SaveErrorKind {
    ///Formats the error kind.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}


impl Error for SaveErrorKind {
    ///Returns the error kind as an optional source.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}


impl SaveErrorKind {
    ///Returns the kind of error.
    /*pub fn kind(&self) -> SaveErrorKind {
        *self
    }*/

    ///Returns the kind of error as a str.
    pub fn as_str(&self) -> &'static str {
        match *self {
            SaveErrorKind::CorruptSave => "save file corrupted",
            SaveErrorKind::DecryptionError => "decryption error",
            SaveErrorKind::EncrytionError => "encryption error",
            SaveErrorKind::CompressionError => "compression error",
            SaveErrorKind::DecompressionError => "decompression error",
            SaveErrorKind::JsonError => "json error",
            SaveErrorKind::FileError => "file error",
            SaveErrorKind::Io => "input/output error",
            SaveErrorKind::Generic => "generic error",
            SaveErrorKind::Debug => "debuging function error",
            SaveErrorKind::Null => "null",
        }
    }

    ///Returns the kind of error as a string.
    pub fn as_string(&self) -> String {
        self.as_str().to_owned()
    }
}

///The result of a save operation.
pub type SaveResult<T> = Result<T, Box<dyn Error>>;

pub trait HandleSaveError<T> {
    
    ///Cathogorizes the error and returns the appropriate value.
    fn cathegorize(self) -> SaveResult<T>;
}

impl<T> HandleSaveError<T> for SaveResult<T> {
    
    fn cathegorize(self) -> SaveResult<T> {
        let error: T = match self {
            Ok(result) => result,

            Err(mut err) => {
                let mut new_save_error: SaveError = SaveError::new(SaveErrorKind::Null, SaveErrorKind::Null.as_string());
                let save_error: &mut SaveError = match err.downcast_mut::<SaveError>(){
                    Some(error) => error,
                    None => &mut new_save_error
                };
                match save_error.kind {
                    SaveErrorKind::CorruptSave => {println!("{:?}", save_error.message); 
                        delete_corrupt_saves(true, SAVE_PATH.to_owned())
                        .cathegorize()?;
                        return Err(Box::new(SaveError::new(
                            SaveErrorKind::Generic,
                            "save deleted".to_owned(),
                        )))
                    },

                    SaveErrorKind::DecryptionError => {println!("{:?}", save_error.message);
                        delete_corrupt_saves(true, SAVE_PATH.to_owned())
                        .cathegorize()?;
                        return Err(Box::new(SaveError::new(
                            SaveErrorKind::Generic,
                            "save deleted".to_owned(),
                        )))
                    },
                    SaveErrorKind::EncrytionError => {println!("{:?}", save_error.message);},
                    SaveErrorKind::CompressionError => {println!("{:?}", save_error.message);},
                    SaveErrorKind::DecompressionError => {println!("{:?}", save_error.message);},
                    SaveErrorKind::JsonError => {println!("{:?}", save_error.message);},
                    SaveErrorKind::FileError => {println!("{:?}", save_error.message);},
                    SaveErrorKind::Io => {println!("{:?}", save_error.message);},
                    SaveErrorKind::Generic => {println!("{:?}", save_error.message);},
                    SaveErrorKind::Debug => {println!("{:?}", save_error.message);},
                    SaveErrorKind::Null => {println!("{:?}", save_error.message);},
                };
                return Err(err);
            },
        };
        return Ok(error);
    }
}

/// Deletes the corrupted save data if the user wants to.
fn delete_corrupt_saves(encryption: bool, path: String) -> SaveResult<bool> {
    let corrupt_save_prompt: String = String::from("The save data in the file 'scores.sav' seems to be corrupted\n Do you want to delete the save data? (y/n)");
    println!("{}", corrupt_save_prompt);
    let mut user_answer: String = String::new();
    let mut deleted: bool = false;
    loop {
        
        lib::read_line(&mut user_answer).cathegorize()?;     

        match user_answer.trim().to_lowercase().as_str() {
                "y" => {

                    lib::delete_save_data(path.clone()).cathegorize()?;

                    if encryption {
                        encryption::create_empty_encrypted_file().cathegorize()?
                    } else {
                        lib::create_file(path)?;
                    }
                    println!("The save data has been deleted");
                    deleted = true;
                    break;
                }

                "n" => break,

                _ => {
                    println!("Do you want to delete the save data? (y/n)");
                    continue;
                }
        }
    }
    Ok(deleted)
}

///Retries saving the highscores.
pub fn try_saving_again(player_info: PlayerInfo) -> SaveResult<()>{
    println!("Your highscore couldn't be saved\n");
    println!("Do you want to try saving again? (y/n)");
    let mut user_answer: String = String::new();
    loop {
        lib::read_line(&mut user_answer).cathegorize()?;     

        match user_answer.trim().to_lowercase().as_str() {
                "y" => { match add_highscores(player_info.clone()){
                            Ok(()) => println!("Score successfully saved"), 
                            Err(_) => {try_saving_again(player_info).cathegorize()?;}
                        };
                        break},

                "n" => {return Err(Box::new(SaveError::new(
                            SaveErrorKind::Generic, 
                            "Couldn't save highscores".to_owned()
                        )));
                    },

                _ => {
                    println!("Do you want to try saving again? (y/n)");
                    continue;
                }
        }
    }
    Ok(())
}

