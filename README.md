# Avoider Game Made in Rust Bevy
To use, make sure you have rust installed and then do a `cargo run`. These instructions and general description of the project are not great, and as such making this readme better is part of the todo list below.

## TODO

* Make readme more robust
* Get this bad boi to run in the browser
* Make controller able to pause and restart game
    * On game over, prompt user to press \[space\] or \[a/x\]
* fix flaky bug where moving with wasd is messed up while gamepad is plugged in
* Create a win condition (i.e. surviving till last level and killing remainder of ships)

 ## Things that would improve this game but weren't implemented
 * Make flames come out the ships backs (The asset was added but didn't end up implementing it)
* Scale game assets based on window size
* Use `insert` when spawning things with multiple attributes (aka basically everything) 
* Add a slowdown effect when the ship stops
* Add a planet or moon or something
* Getting consecutive hits without misses could result in a bad guy exploding into multiple pieces that can kill other bad guys 
* Special ability where you can shoot 3 shots at the dudes, high ish cooldown 
* Clean up imports
* Load all assets at setup time instead of needing to use the asset server every time <- this will be annoying to do and maybe I won't and call it a lesson learned
* Make the movement of components a more shared functionality, rather than scattered

## Lessons Learned
* Instead of having a separate listener when state changes, state changes can be listened to themselves e.g. 
```
app
    .add_system(load_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
    .add_system(cleanup_main_menu.in_schedule(OnExit(AppState::MainMenu)))
```
* Assets should be loaded earlier on with the asset server, so the server doesnt have to be arbitrarily passed around. Instead we can use the collectively-loaded assets as a global resource. 
* I'll need to review how the cargo dependency system works, it is in fact less simple than I thought.