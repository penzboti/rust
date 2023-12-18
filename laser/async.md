# info
I wanted to do async here.
When crossterm reads for input, it blocks threads. So i can't render. I only have this problem, is because i wanted to do video game style movement. 

This means that changing position from getting keydown to getting keyup. But typing normally doesn't work this way. When you type, and hold down a key, it has a short threshold, where it doesn't do anything, just sits after the initial keydown.
Then it rapid fires keydown and up events, until the moment you get keyup.

So my idea was having a variable to store if a key is down, and then rendering a bunch until we get keyup. This still doesn't fix the normal typing behaviour, but it's a start.

But I just don't have enough experience and brain power to solve this task. Tought I might get some answers on the internet, from people having the same problems, but no.
So i'm just dumping the information I found, and leaving it here.

# Linkdump

Probably one of the first [things](https://stackoverflow.com/questions/67592635/how-to-get-keyboard-input-with-termion-in-rust/67593482#67593482) i found. Very raw keyboard input help.

[This](https://github.com/crossterm-rs/crossterm-input) sound like something I'm looking for, but it's deprecated. :p

Found [this 'trait'](https://docs.rs/tokio/1.9.0/tokio/io/trait.AsyncBufReadExt.html) over [here](https://www.reddit.com/r/rust/comments/ovxrd6/help_needed_reading_user_input_with_tokio/), and they said it was the right direction, so I'm putting this here.

Official crossterm [examples](https://github.com/crossterm-rs/crossterm/blob/master/examples/event-stream-tokio.rs), but for some reason, they didn't work for me. The **official** demos, yes.

Found a pretty good [thread](https://users.rust-lang.org/t/text-mode-terminal-application-with-asynchronous-input-output/74760) about some guy being in a *similar* situtation.

Also some [cool code](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=36357007b2224a46f7b832e49a97c1d3) about async i found.

Tokio, which is the all-async crate has cool [documentation](https://tokio.rs/tokio/tutorial/spawning), which I didn't understand.

Also could use tui for fixing raw_input_mode?
Idk, you prob could. But try using crossterm's alternate screen first.

# chatgpt
it did help me write some code, that i did not understand, so after i finish this project with sync, i might just do an async version aswell.