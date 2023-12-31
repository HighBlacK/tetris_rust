Clipboard



.unwrap_or_else(|_error| {
            let error_handle = super::delete_corrupt_saves();
            match error_handle {
                Ok(false) => {
                    println!("You did not delete the corrupted save file.\n The game will exit.");
                    process::exit(1);
                }

                Ok(true) => age::Decryptor::new(empty_buffer).unwrap(),

                Err(err) => {
                    println!("Error: {:?}", err);
                    process::exit(1);
                }
            }
        })