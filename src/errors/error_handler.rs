use bevy::prelude::*;

use crate::errors::my_errors::MyErrors;

pub fn handle_my_errors(
    In(result): In<Result<(), MyErrors>>,
    mut exit_event: MessageWriter<AppExit>,
) {
    match result {
        Ok(_) => {},
        Err(er) => {
            println!("{er}"); 
            exit_event.write(AppExit::Success);
        },
    }
}