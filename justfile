deploy:
    @cargo build --release
    @cp target/release/diary ~/.cargo/bin


clean:
    @rm diary -rf
    @rm ~/.config/diary/diary.toml


coverage:
    @cargo tarpaulin -v --follow-exec --skip-clean

test:
    @cargo nextest run
