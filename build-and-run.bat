REM for development testing; use `cargo run` for final tests
cargo rustc --release --lib -- -C prefer-dynamic
cargo rustc --release --bin -- -C prefer-dynamic
target\release\jas497-p1.exe
