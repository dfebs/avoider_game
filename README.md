# Avoider Game Made in Rust Bevy
To use, make sure you have rust installed and then do a `cargo run`. These instructions and general description of the project are not great, and as such making this readme better is part of the todo list below.

## TODO

* Make readme more robust
* Get this bad boi to run in the browser
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
* Load all assets at setup time instead of needing to use the asset server every time
* Make the movement of components a more shared functionality, rather than scattered
* Starting positions of enemies are less than ideal.

## Lessons Learned
* Instead of having a separate listener when state changes, state changes can be listened to themselves e.g. 
```
app
    .add_system(load_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
    .add_system(cleanup_main_menu.in_schedule(OnExit(AppState::MainMenu)))
```
* Also on the topic of states, updating the state should be calling `set` on a `ResMut<NextState<AppState>>`, not directly setting the application state. Otherwise, the above listeners will not be called.
* Assets should be loaded earlier on with the asset server, so the server doesnt have to be arbitrarily passed around. Instead we can use the collectively-loaded assets as a global resource. 
* It also may be possible to handle some of the controller boilerplate and use the buttons as a resource
* Dumb numbers about the window size should be a global resource
* I'll need to review how the cargo dependency system works, it is in fact less simple than I thought.
* There's definitely a few chains of as_ref().unwrap() that I had to do with level/stage management that I shouldn't be doing. What may have been a better approach? 
