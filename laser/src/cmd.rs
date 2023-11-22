// basic functions in command lines
//? might delete later

use std::process::Command;
fn pause() {
    // 'pause' command on windows
    // https://users.rust-lang.org/t/rusts-equivalent-of-cs-system-pause/4494/2// 
    // maybe also do clearign this way?
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

use std::io::stdout;
use crossterm::terminal::{Clear, ClearType};
fn clear() -> std::io::Result<()> {
    // basically adds a couple of rows, so that earlier information gets off the screen lol. 
    // I don't think this is how it's supposed to work, but at this point i'm afraid to ask
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All))?;
    Ok(())
}
