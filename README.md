# General Tech Stack 
Axum web server
Leptos framework for GUI
Stylance library for scoped external scss for leptos components

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