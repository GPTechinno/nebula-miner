
# Install requirments
install:
    rustup target add thumbv6m-none-eabi
    cargo install poststation-cli

run:
    poststation-cli proxy -s E6616408435A6429 -p nebula-hm/picoboot/reset -m {}
    sleep 3
    cd nebula-hm && cargo run