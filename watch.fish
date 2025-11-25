## fish script to start dev watch and serve 

## Environment variables
# path to sqlite database 
# (note for development if location changes also 
# change DATABASE_URL in .env for accurate suggestions in code editor)
set -x DATABASE_NAME                "music.db"
set -x DATABASE_URL                 "sqlite://sqlite/$DATABASE_NAME"
set -x DATABASE_DIR                 "sqlite"
set -x DATABASE_CREATE_PATH         "./$DATABASE_DIR/$DATABASE_NAME"
set -x DATABASE_MIGRATIONS_PATH     "./src/database/migrations"
set -x MUSIC_DIR                    "./music"

# set backend fo getrandom needed to generate UUIDs in frontend
set RUSTFLAGS                   '--cfg getrandom_backend="wasm_js"' 

## Startup
# create database file if not exists
echo "Making music dir: $MUSIC_DIR"
mkdir -p $MUSIC_DIR
echo "Making database dir: $DATABASE_DIR"
mkdir -p $DATABASE_DIR 
echo "Making database file: $DATABASE_CREATE_PATH"
touch -f $DATABASE_CREATE_PATH

# pre-run sqlx migrations to allow use of query!() macro compile time validation
echo "Databse migrations runing at: $DATABASE_URL"
fish -c "cargo sqlx migrate run --source $DATABASE_MIGRATIONS_PATH"

# run stylance with watch to preprocess CSS
fish -c "~/.cargo/bin/stylance . --watch" &

# run dart sass with watch
fish -c "dart-sass --watch stylance/_index.scss target/site/pkg/mulib.css"&

# run leptos server with watch to serve app. 
fish -c "cargo leptos watch" &
wait
