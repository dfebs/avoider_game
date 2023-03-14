# Avoider Game Made in Rust Bevy
To use, make sure you have rust installed and then do a `cargo run`. These instructions and general description of the project are not great, and as such making this readme better is part of the todo list below.

## TODO
* Add a game over text thing when the state changes to GameOver
* Particle effects exist, but needed to be added when ships are exploded
* Add stars to the background! Maybe add a planet or moon or something too
* Add increasing difficulty over time (probably ad infinitum). This would most likely involve a timer
* Make the movement of components a more shared functionality, rather than scattered
* Add controller support
* Make flames come out the ships backs
* Make readme more robust
* Load all assets at setup time instead of needing to use the asset server every time

 ## Not Urgent, should do at some point
* Use `insert` when spawning things with multiple attributes (aka basically everything) 
* Enemies move through each other so I will want to figure out a way for them to not spawn on the same y-axis of an existing one (but only if it is slower or equal speed)
* Getting consecutive hits without misses could result in a bad guy exploding into multiple pieces that can kill other bad guys 
* Special ability where you can shoot 3 shots at the dudes, high ish cooldown 

 ## Nice-to-Haves-But-Probably-Won't-Happen
* arbitrary camera rotation for jokes? 
* music related sync stuff, this is super out there though