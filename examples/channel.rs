use tokio::sync::mpsc;
use tokio;
use tokio::task;

use explorer700::explorer700::{
    Explorer700, JoystickState
};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let worker = task::spawn_blocking(move || {
        let board = Explorer700::default();
        let joystick = board.init_joystick();

        let mut previous_state = joystick.state();
        loop {
            let state = joystick.state();
            if state != previous_state {
                tx.blocking_send(state.clone()).unwrap();
                previous_state = state;
                if JoystickState::Up == previous_state { break; } // exit on up signal
            }
        }
    });

    while let Some(message) = rx.recv().await {
        match message {
            JoystickState::Up => println!("Up"),
            JoystickState::Down => println!("Down"),
            JoystickState::Left => println!("Left"),
            JoystickState::Right => println!("Right"),
            JoystickState::NoAction => println!("No_Action"),
        }
    }
    worker.await.unwrap();
}
