# fish script to start watch and serve
# for leptos + stylance
fish -c "cargo leptos watch" &
fish -c "~/.cargo/bin/stylance --watch . --output-dir ./" &
wait