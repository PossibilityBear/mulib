# Mulib
A personal web app to stream and manage a local music library as a replacement
for spotify, currently in early development as a barebones web music player.


# General Tech Stack 
- Axum web server
- Leptos framework for GUI
- Stylance library for scoped external scss for leptos components
- uuid for unique identifiers
- SQLite database with sqlx Database Driver 

# CSS Configurations:
Configuration of project environment is largely controlled by cargo.toml there 
is also a fish script "./watch.fish" that is configured to preprocess stylance 
items from the src folder
the cargo.toml is then confingured for cargo leptos to look for styles in the
./stylance/_index.scss file created from running the stylance CLI tool. 

from here dart sass (either from cargo leptos or fish watch script) watches for changes and will put the processed scss 
file into ./target/site/package/

## @use imports in .scss
To @use within .scss you the relative paths are rooted from the 
```
<cargo root dir>/stylance 
```
so you will need a relative path starting with 
```
../src/\<your file within source\>
```

A side note here is that currently changes to these 'library' scss 
files aren't caught by dart watch so to trigger dart to process just save the file you are using the @use in.


# .Env
There is a .env file at the project root which points sqlx to the 
location of the music database for compile time validation of 
sql exectued through sqlx macros.

# Development serve and watch with:
```fish
fish ./watch.fish
```
This does 3 things:
1. runs sqlx migration prior to startup (otherwise macros get mad)
2. starts stylance with watch
3. starts leptos with watch

# Issues + Fixes
## Getting compilation errors from uuid or get_random?
This is likely due to the get_random create not knowing which target to 
compile for. The fix for this is to include the needed features in 
cargo.toml, 
```toml
[dependancies]
.
.
uuid = {version="1.18.1", features = ["v4", "std", "js"]}
getrandom = { version = "0.3.3", features = ["wasm_js"] }
.
.
```

and to set an environment variable / modify '~/.cargo/config.toml' to include:
```toml
[target.wasm32-unknown-unknown]
rustflags = ['--cfg', 'getrandom_backend="wasm_js"']
```

 ## Getting build errors from wasm-bindgen version mismatch?
 ```sh
   0: at `/home/ian/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cargo-leptos-0.2.41/src/compile/front.rs:71:38`
   1: at `/home/ian/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cargo-leptos-0.2.41/src/compile/front.rs:151:10`
   2: 

      it looks like the Rust project used to create this Wasm file was linked against
      version of wasm-bindgen that uses a different bindgen format than this binary:

        rust Wasm file schema version: 0.2.101
           this binary schema version: 0.2.100

      Currently the bindgen format is unstable enough that these two schema versions
      must exactly match. You can accomplish this by either updating this binary or
      the wasm-bindgen dependency in the Rust project.

      You should be able to update the wasm-bindgen dependency with:

          cargo update -p wasm-bindgen --precise 0.2.100

      don't forget to recompile your Wasm file! Alternatively, you can update the
      binary with:

          cargo install -f wasm-bindgen-cli --version 0.2.101

      if this warning fails to go away though and you're not sure what to do feel free
      to open an issue at https://github.com/rustwasm/wasm-bindgen/issues!
   2: 
 ```

 This will generally happen after upgrading the project to a new version of
 wasm-bindgen or when installing project for the first time.

 The solution here is to get the versions to align 'duh' but the suggested fix
 in the error doesn't really work because  cargo leptos uses a static version
 itself instead of the locally install wasm-bindgen-cli 

 To get around this the fix is
 1. Clean the build to remove any outdated deps (not sure this is necissary)
    ``` sh 
    cargo clean
    ```

2. perform suggested fix of installing up-to-date wasm-bindgen-cli
    (again not certain this necissary) 
    ```sh
    cargo install -f wasm-bindgen-cli --version {{Target Version Number}}
    ```

3. Update cargo leptos
    ```sh
    cargo install -f cargo-leptos
    ```

4. Run the project
    ```sh
    fish ./watch.fish
    ```

## Getting browser console errors after rust toolchain changes
```
Uncaught (in promise) SyntaxError: redeclaration of function 
wasm_bindgen_b4561f3c7f71394c___convert__closures_____invoke______
mulib.js:229:10note: Previously declared at line 225, column 10mulib.js:225:10

The resource at “http://127.0.0.1:3000/pkg/mulib.wasm” 
preloaded with link preload was not used within a few seconds. 
Make sure all attributes of the preload tag are set correctly.
```
This is another type of error that can happen due to wasm bindgen version
mismatches between what is expected by Mulib and what is installed in
the rust tool-chain. That said this one is much less consistent on
how to resolve it, and it involves some permutation of the following steps


to fix it we just need to get the versions to line up
1. Clean the project
```sh
cargo clean
```

2. Check version of wasm-bindgen in cargo.toml:
```sh 
cargo pkgid wasm-bindgen
```

3. install that version of the wasm-bindgen cli
```sh 
cargo install -f wasm-bindgen-cli --version {{Target Version Number}} 
```

4. update the cargo-leptos installation to use freshly
installed version of wasm-bindgen cli
```sh
cargo install -f cargo-leptos
```

5. Full reboot, the host machine (I believe this cleans up the 
old versions of wasm-bindgen that were causing the issue but this
really makes no sense to me, every time after I reboot thats when 
the issue seems to resolve)

6. Build and run the project as normal
```sh
fish ./watch.fish
```
