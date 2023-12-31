//! Tests the tetris module.

use crate::highscores::error::HandleSaveError;

use super::highscores::{print_highscores,add_highscores};
use super::highscores::encryption;
use super::highscores::lib::{self, PlayerInfo, back_to_main_menu};
use super::highscores::error::{self, SaveResult};

/// Tests the highscores module.
pub fn highscores_test(){
    
    match print_highscores() {
        Ok(()) => (),
        Err(_) => println!("Couldn't print highscores"),
    };

    let mut player = PlayerInfo::new();
    println!("Default player name: {}, Default score: {}", player.name, player.score);

    player = PlayerInfo {
        name: String::from("HighBlacK"),
        score: 42069,
    };
    match add_highscores(player.clone()) {
        Ok(()) => println!("Added highscores"),
        Err(_) => match error::try_saving_again(player) {
            Ok(()) => println!("Added highscores"),
            Err(_) => back_to_main_menu(),
        },
    };

    player = PlayerInfo {
        name: String::from("xXX420_69SlayerxXx"),
        score: 42068,
    };
    match add_highscores(player.clone()) {
        Ok(()) => println!("Added highscores"),
        Err(_) => match error::try_saving_again(player) {
            Ok(()) => println!("Added highscores"),
            Err(_) => back_to_main_menu(),
        },
    };

    player = PlayerInfo {
        name: String::from("Ohlolo"),
        score: 42070,
    };
    match add_highscores(player.clone()) {
        Ok(()) => println!("Added highscores"),
        Err(_) => match error::try_saving_again(player) {
            Ok(()) => println!("Added highscores"),
            Err(_) => back_to_main_menu(),
        },
    };


    player = PlayerInfo {
        name: String::from("Don"),
        score: 69420,
    };
    match add_highscores(player.clone()) {
        Ok(()) => println!("Added highscores"),
        Err(_) => match error::try_saving_again(player) {
            Ok(()) => println!("Added highscores"),
            Err(_) => back_to_main_menu(),
        },
    };


    player = PlayerInfo {
        name: String::from("Koupa"),
        score: 42059,
    };
    match add_highscores(player.clone()) {
        Ok(()) => println!("Added highscores"),
        Err(_) => match error::try_saving_again(player) {
            Ok(()) => println!("Added highscores"),
            Err(_) => back_to_main_menu(),
        },
    };


    player = PlayerInfo {
        name: String::from("Ohlolo"),
        score: 70850,
    };
    match add_highscores(player.clone()) {
        Ok(()) => println!("Added highscores"),
        Err(_) => match error::try_saving_again(player) {
            Ok(()) => println!("Added highscores"),
            Err(_) => back_to_main_menu(),
        },
    };

    player = PlayerInfo {
        name: String::from("xXX420_69SlayerxXx"),
        score: 78800,
    };
    match add_highscores(player.clone()) {
        Ok(()) => println!("Added highscores"),
        Err(_) => match error::try_saving_again(player) {
            Ok(()) => println!("Added highscores"),
            Err(_) => back_to_main_menu(),
        },
    };


    //shouldn't be added
    
    player = PlayerInfo {
        name: String::from("xXX420_69SlayerxXx"),
        score: 42050,
    };
    match add_highscores(player.clone()) {
        Ok(()) => println!("Added highscores"),
        Err(_) => match error::try_saving_again(player) {
            Ok(()) => println!("Added highscores"),
            Err(_) => back_to_main_menu(),
        },
    };

    player = PlayerInfo {
        name: String::from("Random_username"),
        score: 92068,
    };
    match add_highscores(player.clone()) {
        Ok(()) => println!("Added highscores"),
        Err(_) => match error::try_saving_again(player) {
            Ok(()) => println!("Added highscores"),
            Err(_) => back_to_main_menu(),
        },
    };


    player = PlayerInfo {
        name: String::from("new_Username"),
        score: 78968,
    };
    match add_highscores(player.clone()) {
        Ok(()) => println!("Added highscores"),
        Err(_) => match error::try_saving_again(player) {
            Ok(()) => println!("Added highscores"),
            Err(_) => back_to_main_menu(),
        },
    };

        
    //should be removed
    player = PlayerInfo {
        name: String::from("451635241865478564"),
        score: 20850,
    };       
    match add_highscores(player.clone()) {
        Ok(()) => println!("Added highscores"),
        Err(_) => match error::try_saving_again(player) {
            Ok(()) => println!("Added highscores"),
            Err(_) => back_to_main_menu(),
        },
    };


    player = PlayerInfo {
        name: String::from("Batman"),
        score: 170850,
    };
    match add_highscores(player.clone()) {
        Ok(()) => println!("Added highscores"),
        Err(_) => match error::try_saving_again(player) {
            Ok(()) => println!("Added highscores"),
            Err(_) => back_to_main_menu(),
        },
    };


    match print_highscores() {
        Ok(()) => (),
        Err(_) => println!("Couldn't print highscores"),
    };

}

/// Creates an empty encrypted file, reads it's contents and prints them.
fn _fetch_data_from_empty_encrypted_file() -> SaveResult<()> {
    let path = String::from("src/experiments/tetris/highscores/.empty");
    encryption::create_empty_encrypted_file().cathegorize()?;
    let loaded_empty_file = lib::load_file(path);
    println!("{:?}", loaded_empty_file);
    return Ok(())
}