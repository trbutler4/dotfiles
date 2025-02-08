1. Build the project:
```bash
cargo build --release
```

2. Create your dotfiles structure:
```bash
~/dotfiles/
├── nvim/
│   └── .init.lua
├── zsh/
│   └── .zshrc
```

3. Run the program:
```bash
# Install your project as a binary
cargo install --path .

# Now you can use it from anywhere
dotfiles install
dotfiles list
dotfiles status
dotfiles add vim ~/.vimrc

# Get help
dotfiles --help
dotfiles add --help
