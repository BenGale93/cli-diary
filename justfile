deploy:
    @cargo build --release
    @cp target/release/diary ~/.cargo/bin
