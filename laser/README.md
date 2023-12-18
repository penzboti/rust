# info
Controll two points on the screen
One of which, is a start of the laser pointer.
And the other one, is a transparent target.

Logically, the laser pointer points to the target, but the actual laser goes trough, and touches the border of the window.

# how will the inputs work
Move them with wasd, and arrow keys. Switch inputs with the space & tab key. Exit with enter & ctrl+c.

# todo
## inputs stuff:
 - [x] get screen size
 - [x] keyboard input detection (i mean it only prints out inputs, but it still detects them)
 - [x] switch input types

## variable stuff:
 - [x] storing laser variables
 - [x] changing laser variables
## rendering stuff:
 - [x] rendering pointer and target
 - [ ] rendering line
 - [ ] fix raw input mode. (println! still works, even tough they said it won't) ((use alternate screen! maybe?))

# refactor
from my phone's notes.

- Create struct with x and y coordinates; make 2 of them: target and pointer
- Make keyboard return a string (change char 'w' to "w_key")
- Make another set of events that handle release
- Then handle the key from the main function
- Also make variables that handle continuous button pressing [^1]
> Q: how fast does it handle it? Maybe we have to put some sort of a delay in that?
- Then return the diff we have to change the coordinates
- Then IN THE MAIN FUNCTION change the position variables
- And render based on these variables.

[^1]: Sorry, not happening. See in [this file](async.md)
<!-- i hate to search for everything https://www.markdownguide.org/cheat-sheet/ -->