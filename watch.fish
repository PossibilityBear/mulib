# fish script to start watch and serve
# for leptos + stylance
set RUSTFLAGS '--cfg getrandom_backend="wasm_js"' 
fish -c "cargo leptos watch" &
fish -c "~/.cargo/bin/stylance --watch . --output-dir ./" &
wait