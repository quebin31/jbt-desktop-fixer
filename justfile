bin := "jbt-desktop-fixer"

build-release:
    @cargo build --release
    @# Use UPX to compress the binary even more
    @[ -x "$(command -v upx)" ] && upx -q --best target/release/{{bin}} || true

install PATH: build-release
    install -Dm755 target/release/{{bin}} {{PATH}}