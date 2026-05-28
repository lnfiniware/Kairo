#!/bin/bash
# KairoDB Standalone Installer for macOS and Linux
# Installs KairoDB globally without requiring Rust or Cargo.

set -e

# 1. Determine local source executable
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SOURCE_EXE="$SCRIPT_DIR/target/release/kairo"

if [ ! -f "$SOURCE_EXE" ]; then
    SOURCE_EXE="$SCRIPT_DIR/target/debug/kairo"
fi

if [ ! -f "$SOURCE_EXE" ]; then
    SOURCE_EXE="$SCRIPT_DIR/kairo"
fi

if [ ! -f "$SOURCE_EXE" ]; then
    echo "ERROR: Could not find kairo executable in target/release, target/debug, or project root."
    echo "Please compile the project first using 'cargo build --release'."
    exit 1
fi

# 2. Define global installation directory
INSTALL_DIR="$HOME/.kairo/bin"

# 3. Create install directory if missing
if [ ! -d "$INSTALL_DIR" ]; then
    echo "Creating installation directory: $INSTALL_DIR"
    mkdir -p "$INSTALL_DIR"
fi

# 4. Copy the executable and make it runnable
TARGET_EXE="$INSTALL_DIR/kairo"
echo "Installing KairoDB executable..."
cp "$SOURCE_EXE" "$TARGET_EXE"
chmod +x "$TARGET_EXE"

# 5. Also copy to ~/.cargo/bin if it exists (highly likely for user path compatibility)
CARGO_BIN_DIR="$HOME/.cargo/bin"
if [ -d "$CARGO_BIN_DIR" ]; then
    echo "Updating global cargo-bin copy..."
    cp "$SOURCE_EXE" "$CARGO_BIN_DIR/kairo"
    chmod +x "$CARGO_BIN_DIR/kairo"
fi

# 6. Add installation directory to User PATH if not already present
SHELL_PROFILE=""
if [ -n "$SHELL" ]; then
    SHELL_NAME=$(basename "$SHELL")
    if [ "$SHELL_NAME" = "zsh" ]; then
        SHELL_PROFILE="$HOME/.zshrc"
    elif [ "$SHELL_NAME" = "bash" ]; then
        SHELL_PROFILE="$HOME/.bashrc"
    fi
fi

if [ -z "$SHELL_PROFILE" ]; then
    if [ -f "$HOME/.zshrc" ]; then
        SHELL_PROFILE="$HOME/.zshrc"
    elif [ -f "$HOME/.bashrc" ]; then
        SHELL_PROFILE="$HOME/.bashrc"
    else
        SHELL_PROFILE="$HOME/.profile"
    fi
fi

# Append to PATH if not already there
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "Adding $INSTALL_DIR to your PATH in $SHELL_PROFILE..."
    echo "" >> "$SHELL_PROFILE"
    echo "# KairoDB PATH configuration" >> "$SHELL_PROFILE"
    echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_PROFILE"
    
    echo "SUCCESS: KairoDB has been successfully installed globally!"
    echo "Please source your profile or restart your terminal: source $SHELL_PROFILE"
else
    echo "SUCCESS: KairoDB has been successfully installed globally!"
    echo "Path is already configured in environment."
fi

echo "Installation path: $TARGET_EXE"
