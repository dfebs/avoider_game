# Avoider Game Made in Rust Bevy
To use, make sure you have rust installed and then do a `cargo run`. These instructions and general description of the project are not great, and as such making this readme better is part of the todo list below.

## TODO
* Make the player's laser gun actually hit enemies. I need to learn how the scope of commands and their spawning in different modules. A spawned thing in player.rs is not necessarily known in main.rs so that is making some things difficult in terms of collision detection
* Add a game over text thing when the state changes to GameOver
* Add increasing difficulty over time (probably ad infinitum). This would most likely involve a timer
* Add controller support
* Make readme more robust
* Make hitboxes better
* make the particle effects for explosions
* make flames come out the ships backs

 ## Not Urgent, should do at some point
* Look up what `insert` method does (it seems to be a follow up to `commands.spawn()`)
* Enemies move through each other so I will want to figure out a way for them to not spawn on the same y-axis of an existing one (but only if it is slower or equal speed)
* Getting consecutive hits without misses could result in a bad guy exploding into multiple pieces that can kill other bad guys 
* Special ability where you can shoot 3 shots at the dudes, high ish cooldown 

 ## Nice-to-Haves-But-Probably-Won't-Happen
* arbitrary camera rotation for jokes? 
* music related sync stuff, this is super out there though