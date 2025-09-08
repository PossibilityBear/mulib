# General Tech Stack 
- Axum web server
- Leptos framework for GUI
- Stylance library for scoped external scss for leptos components
- uuid for unique identifiers
- SQLite database with rusqlite Database Driver (should switch to sqlx or something)
# CSS Configurations:
Configuration of project environment is largely controlled by cargo.toml there 
is also a fish script "./watch.fish" that is configured to preprocess stylance 
items from the src folder
the cargo.toml is then confingured for cargo leptos to look for styles in the
./stylance/_index.scss file created from running the stylance CLI tool. 

from here cargo leptos watches for changes and will put the processes scss 
file into ./target/site/package/

# Development serve and watch with:
```fish
fish ./watch.fish
```


# Getting compilation errors from uuid or get_random?
This is likely due to the get_random create not knowing which target to 
compile for. The fix for this is to include the needed features in 
cargo.toml, 
```toml
[dependancies]
.
.
.
uuid = {version="1.18.1", features = ["v4", "std", "js"]}
getrandom = { version = "0.3.3", features = ["wasm_js"] }
.
.
.
```

and to set an environment variable / modify '~/.cargo/config.toml' to include:
```toml
[target.wasm32-unknown-unknown]
rustflags = ['--cfg', 'getrandom_backend="wasm_js"']
```