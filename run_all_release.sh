cargo build --release;
find target/release/pe* -maxdepth 0 -type f -executable -exec printf "{}\n" \; -execdir {} \; -execdir printf "\n" \;
