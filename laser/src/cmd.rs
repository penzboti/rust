// basic functions in command lines
//? might delete later

use std::process::Command;
fn pause() {
    // 'pause' command on windows
    // https://users.rust-lang.org/t/rusts-equivalent-of-cs-system-pause/4494/2// 
    // maybe also do clearign this way?
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}