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
 - [ ] holding input results in a constant position change
```
this means refactoring a whole lot of code:
```
see in [refactor](#refactor)

## variable stuff:
 - [x] storing laser variables
 - [x] changing laser variables
## rendering stuff:
 - [x] rendering pointer and target
 - [ ] rendering line
 - [ ] fix raw input mode. (println! still works, even tough they said it won't)
```
use this link as guidance into the raw input fetching.
https://stackoverflow.com/a/67593482/12706133

oh, and also crack down on if i continuous press, then it then quickly switches between press and release. PLEASE disable that!
```

# refactor
from my phone's notes.

- Create struct with x and y coordinates; make 2 of them: target and pointer
- Make keyboard return a string (change char 'w' to "w_key")
- Make another set of events that handle release
- Then handle the key from the main function
- Also make variables that handle continuous button pressing
- Q: how fast does it handle it? Maybe we have to put some sort of a delay in that?
- Then return the diff we have to change the coordinates
- Then IN THE MAIN FUNCTION change the position variables
- And render based on these variables.