## fish script to start dev watch and serve 

## Environment variables
# path to sqlite database
set DATABASE_URL "sqlite://music.db"
# set backend fo getrandom needed to generate UUIDs in frontend
set RUSTFLAGS '--cfg getrandom_backend="wasm_js"' 

## Startup
# pre-run sqlx migrations to allow use of query!() macro compile time validation
fish -c "cargo sqlx migrate run --source ./src/migrations"

# run stylance with watch to preprocess CSS
fish -c "~/.cargo/bin/stylance --watch . --output-dir ./" &

# run leptos server with watch to serve app. 
fish -c "cargo leptos watch" &
wait