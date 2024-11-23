# `acr`

Acridotheres CLI

---

## Installation

### From Source

```bash
git clone https://github.com/acridotheres/cli.git acr
cd acr
cargo build --release
sudo cp target/release/acridotheres_cli /usr/local/bin/acr
```

## Usage

### Extract an archive

```bash
acr extract -i 001.zip -f zip
```

extracts `001.zip` to the current directory. If the file type is known,
you can add the `-f` flag to specify it for faster extraction as auto-detection
is slower.