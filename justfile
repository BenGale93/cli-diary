deploy:
    @cargo build --release
    @cp target/release/diary ~/.cargo/bin


clean:
    @rm diary -rf
    @rm ~/.config/diary/diary.toml
