PREFIX := "usr"
BINARY := PREFIX / "bin"
DESTDIR := "/"


build:
    cargo build --release

install:
    install -Dm0755 target/release/verdi {{ DESTDIR }}{{ BINARY }}/verdi
