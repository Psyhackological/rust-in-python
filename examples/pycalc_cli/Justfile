init:
    uv run maturin init

build:
    cargo build --release
    cp target/release/*.so .

clean:
    rm -f *.so

run:
    uv run main.py
