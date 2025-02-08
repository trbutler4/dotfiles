1. Build the project:
```bash
cargo build --release
```

2. Create your dotfiles structure:
```bash
~/dotfiles/
├── nvim/
│   └── .vimrc
├── zsh/
│   └── .zshrc
```

3. Run the program:
```bash
# Install configs
cargo run -- install

# List available configs
cargo run -- list

# Check configuration
cargo run -- check

# Run with verbose output
cargo run -- install -v
