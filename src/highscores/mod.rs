//! This module contains the highscores system.
//!
//! # Examples
//!
//! ```
//! highscores::print_highscores();
//! ```
//!
//! ```
//! let new_player = PlayerInfo {
//!     name: String::from("random_player_name"),
//!     score: 42069,
//! }
//!
//! highscores::add_highscores(new_player);
//! ```
//!
//! # Note
//!
//! The highscores system is not flexible because it uses a hardcoded path.
//!
//! # TODO
//!
//! * Add a prompt to ask the user if they want to delete the save data.
//! * Look for faster encryption.

pub mod lib;
pub mod error;
pub mod encryption;
pub mod compression;

use std::collections::BTreeMap;
use std::time::{ SystemTime, Duration, SystemTimeError };
use std::str;

use self::error::{ SaveResult, HandleSaveError };
use self::lib::PlayerInfo;

static DEBUG: bool = true;

static ENABLE_ENCRYPTION: bool = false;
static ENABLE_COMPRESSION: bool = true;

static SAVE_PATH: &str = "src/highscores/scores.sav";

///Prints the highscores.
pub fn print_highscores() -> SaveResult<()> {
    let highscores: BTreeMap<u32, String> = load_highscores()?;

    if highscores.is_empty() {
        println!("No highscores yet!\n");
    } else {
        for (score, player_name) in highscores.iter().rev() {
            println!("{}: {}", player_name, score);
        }
    }
    return Ok(());
}

///Adds the highscores.
pub fn add_highscores(player_info: PlayerInfo) -> SaveResult<()> {
    let mut highscores: BTreeMap<u32, String> = load_highscores()?;

    highscores.entry(player_info.score).or_insert(player_info.name.clone());

    if
        let Some(existing_score) = highscores
            .clone()
            .iter()
            .find(|&(_, v)| v == &player_info.name)
    {
        if player_info.score > *existing_score.0 {
            highscores.remove(existing_score.0);
        }
    }

    highscores.insert(player_info.score, player_info.name);

    let check_highscores: BTreeMap<u32, String> = trim_highscores(highscores, 10)?;

    save_highscores(check_highscores)?;

    return Ok(());
}

///Loads the highscores.
pub fn load_highscores() -> SaveResult<BTreeMap<u32, String>> {
    let fn_run_time: SystemTime = SystemTime::now();

    let loaded_file: Vec<u8> = lib::load_file(SAVE_PATH.to_owned());

    if loaded_file.is_empty() {
        return Ok(BTreeMap::<u32, String>::new());
    }

    if ENABLE_ENCRYPTION {
        if ENABLE_COMPRESSION {
            let decompressed: Vec<u8> = compression::decompress_saves(loaded_file).cathegorize()?;
            let decrypted: String = encryption::decrypt_saves(decompressed).cathegorize()?;
            let map: BTreeMap<u32, String> = lib::deserialize_save(decrypted).cathegorize()?;

            if DEBUG {
                debug(true, false, &map, fn_run_time.elapsed(), &0).cathegorize()?;
            }

            return Ok(map);
        } else {
            let decrypted: String = encryption::decrypt_saves(loaded_file).cathegorize()?;
            let map: BTreeMap<u32, String> = lib::deserialize_save(decrypted).cathegorize()?;

            if DEBUG {
                debug(true, false, &map, fn_run_time.elapsed(), &0).cathegorize()?;
            }

            return Ok(map);
        }
    } else {
        if ENABLE_COMPRESSION {
            let decompressed: Vec<u8> = compression::decompress_saves(loaded_file).cathegorize()?;
            let to_str: &str = lib::from_bytes_to_str(&decompressed).cathegorize()?;
            let map: BTreeMap<u32, String> = lib
                ::deserialize_save(to_str.to_owned())
                .cathegorize()?;

            if DEBUG {
                debug(true, false, &map, fn_run_time.elapsed(), &0).cathegorize()?;
            }

            return Ok(map);
        } else {
            let to_str: &str = lib::from_bytes_to_str(&loaded_file).cathegorize()?;
            let map: BTreeMap<u32, String> = lib
                ::deserialize_save(to_str.to_owned())
                .cathegorize()?;

            if DEBUG {
                debug(true, false, &map, fn_run_time.elapsed(), &0).cathegorize()?;
            }

            return Ok(map);
        }
    }
}

///Trims the highscores to the specified amount.
fn trim_highscores(
    mut highscores: BTreeMap<u32, String>,
    max_count: i32
) -> SaveResult<BTreeMap<u32, String>> {
    let truncated_highscores: BTreeMap<u32, String> = highscores.clone();
    let mut count: i32 = 0;
    for (s, _) in truncated_highscores.iter().rev() {
        if count >= max_count {
            if DEBUG {
                debug(false, true, &highscores, SystemTime::now().elapsed(), s).cathegorize()?;
            }
            highscores.remove(s);
        }
        count += 1;
    }

    return Ok(highscores);
}

///Saves the highscores.
fn save_highscores(scores: BTreeMap<u32, String>) -> SaveResult<()> {
    let fn_run_time: SystemTime = SystemTime::now();

    let saveformat: String = lib::serialize_save(&scores)?;

    if ENABLE_ENCRYPTION {
        let encrypted: Vec<u8> = encryption::encrypt_saves(saveformat).cathegorize()?;
        if ENABLE_COMPRESSION {
            let compressed: Vec<u8> = compression::compress_saves(encrypted).cathegorize()?;
            lib::write_to_file(SAVE_PATH.to_owned(), compressed).cathegorize()?;
        } else {
            lib::write_to_file(SAVE_PATH.to_owned(), encrypted).cathegorize()?;
        }
    } else {
        if ENABLE_COMPRESSION {
            let compressed: Vec<u8> = compression::compress_saves(saveformat).cathegorize()?;
            lib::write_to_file(SAVE_PATH.to_owned(), compressed).cathegorize()?;
        } else {
            lib::write_to_file(SAVE_PATH.to_owned(), saveformat).cathegorize()?;
        }
    }

    if DEBUG {
        debug(false, false, &scores, fn_run_time.elapsed(), &0).cathegorize()?;
    }

    return Ok(());
}

//DEBUGING

///A debug function. Prints the time it took to execute the function and the values that have been loaded or saved.
fn debug(
    load: bool,
    trim: bool,
    map: &BTreeMap<u32, String>,
    time: Result<Duration, SystemTimeError>,
    key: &u32
) -> SaveResult<()> {
    if load {
        let function_duration: Duration = match time {
            Ok(e) => e,
            Err(_) => {
                return Err(
                    Box::new(
                        error::SaveError::new(
                            error::SaveErrorKind::Debug,
                            "error while getting the load_highscores() function duration".to_owned()
                        )
                    )
                );
            }
        };
        println!(
            "The load_highscores function execution lasted: {:?} nanoseconds.\n",
            function_duration.as_nanos()
        );
        println!("The folowing values have been loaded: \n {:?} \n", map);
    } else if !load && !trim {
        let function_duration: Duration = match time {
            Ok(e) => e,
            Err(_) => {
                return Err(
                    Box::new(
                        error::SaveError::new(
                            error::SaveErrorKind::Debug,
                            "error while getting the save_highscores() function duration".to_owned()
                        )
                    )
                );
            }
        };
        println!(
            "The save_highscores function execution lasted: {:?} nanoseconds.\n",
            function_duration.as_nanos()
        );
        println!("The folowing values have been saved: \n {:?} \n", map);
    } else if trim {
        println!("The folowing values has been removed: \n {:?} \n", map.get_key_value(key));
    } else {
        print!("This function should not have been called");
    }

    Ok(())
}
