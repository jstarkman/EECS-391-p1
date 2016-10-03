REM for development testing; use `cargo run` for final tests
cargo rustc --release -- -C prefer-dynamic
target\release\jas497-p1.exe
