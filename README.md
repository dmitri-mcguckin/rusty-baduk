# Rusty Baduk
*A CLI-based Go/Baduk client*

## Contributors
* **Conor Dunlap:** __<conor4@pdx.edu>__
* **Dmitri McGuckin:** __<dmitri3@pdx.edu>__

Our project’s goal is to emulate the Chinese game of Baduk (also known as Go) for the terminal. We are using the Japanese rule set for this project and an ascii style graphics for the display. (@/O for black and white stones respectively, empty board vertices “+”, etc). Our board would be of 3 standard Baduk board sizes: 9, 13, and 19. The boards coordinates are designated by capital English alphabetical characters (A-S for a 19x19 board, A-I for a 9x9) for the X coordinates, and numeric characters for the Y coordinates (1-9, 1-19, etc). The user during the game’s loop will be able to enter in the desired coordinates, and the two players alternate turns. The function “place” of the board struct takes coordinate information, checks it against the bounds of the board and places the stone if valid. At any point the player has the option to quit the game early via a prompt at the end of each turn.

## Build
* `$` `cargo build`

## Run
* `$` `cargo run`

## Tests
* `$` `cargo test`

We created a separate testing document for our tests, and simply doing cargo test on our program. How we tested our program was to take the functions such as place, check_bounds, etc and have a test function for each function. We would then try different inputs and compare them to the expected output. For example for the place function we had 2 different test functions, one for checking if an out of bounds input was entered the function would throw the option “None” rather than panic. The second test function for place was us giving it a valid input and checking to see if it returned the option “Some(true)” showing that the placement was a success.

## Report
Much of the preliminary work was fairly straightforward. Creating the board ended up being just a vec of vecs and while we thought at first each stone would also need positional information we quickly found it to be redundant info that was more a property of the data structure than an individual stone itself. What we were able to do was create a board, place stones on it then check for dead stones, (stones with no liberties left). What was a little tricky, however, was looking at a group of stones liberties rather than one at a time. This is something that is doable but unfortunately did not have the time to implement it. The dead stone checking was fairly easy to achieve, however, it was a simple recursive solution involving checking all four immediate neighbors every time a stone was placed. If the neighboring stones were the opposing color to the one just placed its liberties were checked and if they happened to be zero, it would remove the stone. Bounds checking was probably one of the other more overly-convoluted pieces to this game. While it made sense to make the coordinate types usize’s (since we were dealing with sub indexing vectors), it didn’t when it came to the bounds checking itself. What we’d want during bounds checking was for a pair of coordinates to be potentially invalid (which may or may not have negative numbers) and check the bounds and return a bool. However, due to the inherent properties of a usize as well as rust’s overflow/underflow protection, we would get a panic as soon as we fell out of bounds of the board. The solution to this was not pretty but it ended up working. What ended up having to do was make all of the inputs to the bounds checking i16’s and we’d have to cast our potential coordinates to that type every time we checked. This allowed for the flexibility we were looking for when trying to also implement recursive solutions to liberty and dead stone checking.

Overall we are fairly satisfied with our results. We were able to make a game loop that allows players to either play or pass. We were able to construct a board and enforce the primary dead/live stone rules. (Although we did not implement anything for checking suicidal moves or seki). And we were even able to implement half of a real scoring system that at least tracks the captures each player made. Implementing complicated procedures like static score checking, (including determining moyo), at the end of the game was not something that was in the scope of this project so it was not added in.
