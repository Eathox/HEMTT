use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    time::Duration,
};

use enigo::{Enigo, MouseButton, MouseControllable};

use hemtt_tests::*;

fn main() {
    let mut proc = Command::new(arma_path().unwrap().to_str().unwrap())
        .args(ARMA_STARTUP)
        .spawn()
        .expect("failed to execute process");

    let wait_time = Duration::from_secs(1);

    let mut enigo = Enigo::new();

    // loop {
    //     enigo.mouse_down(MouseButton::Left);
    //     std::thread::sleep(wait_time);

    //     enigo.mouse_move_relative(100, 100);
    //     std::thread::sleep(wait_time);

    //     enigo.mouse_up(MouseButton::Left);
    //     std::thread::sleep(wait_time);
    // }

    std::thread::sleep(Duration::from_secs(30));

    proc.kill();
}
