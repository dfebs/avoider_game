# Avoider Game Made in Rust Bevy
To use, make sure you have rust installed and then do a `cargo run`. These instructions and general description of the project are not great, and as such making this readme better is part of the todo list below.

## TODO
* Add a game over text thing when the state changes to GameOver
* Make different types of enemies spawn on some levels
* Make the movement of components a more shared functionality, rather than scattered
* Add controller support
* Make flames come out the ships backs
* Make readme more robust
* Add a paused state
* Add cooldown for player gun

 ## Not Urgent, should do at some point
* Use `insert` when spawning things with multiple attributes (aka basically everything) 
* Add a slowdown effect when the ship stops
* Add a planet or moon or something
* Getting consecutive hits without misses could result in a bad guy exploding into multiple pieces that can kill other bad guys 
* Special ability where you can shoot 3 shots at the dudes, high ish cooldown 
* Clean up imports
* Load all assets at setup time instead of needing to use the asset server every time <- this will be annoying to do and maybe I won't and call it a lesson learned

 ## Nice-to-Haves-But-Probably-Won't-Happen
* arbitrary camera rotation for jokes? 
* music related sync stuff, this is super out there though