## fish script to start dev watch and serve 

## Environment variables
# path to sqlite database 
# (note for development if location changes also 
# change DATABSE_URL in .env for accurate suggestions in code editor)
set -x DATABASE_NAME                "music.db"
set -x DATABASE_URL                 "sqlite://sqlite/$DATABASE_NAME"
set -x DATABASE_CREATE_PATH         "./sqlite/$DATABASE_NAME"
set -x DATABASE_MIRGRATIONS_PATH    "./src/database/migrations"

# set backend fo getrandom needed to generate UUIDs in frontend
set RUSTFLAGS                   '--cfg getrandom_backend="wasm_js"' 

## Startup
# create database file if not exists
touch $DATABASE_CREATE_PATH
# pre-run sqlx migrations to allow use of query!() macro compile time validation
fish -c "cargo sqlx migrate run --source $DATABSE_MIGRATIONS_PATH"

# run stylance with watch to preprocess CSS
fish -c "~/.cargo/bin/stylance --watch . --output-dir ./" &

# run leptos server with watch to serve app. 
fish -c "cargo leptos watch" &
wait