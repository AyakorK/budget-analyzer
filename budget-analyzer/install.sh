#!/bin/sh
# Budget Analyzer Universal Installer

set -e

REPO="your-org/budget-analyzer"
VERSION="latest"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Darwin)
        case "$ARCH" in
            arm64) PLATFORM="macos-arm64" ;;
            x86_64) PLATFORM="macos-x64" ;;
            *) echo "❌ Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    Linux)
        case "$ARCH" in
            x86_64) PLATFORM="linux-x64" ;;
            aarch64) PLATFORM="linux-arm64" ;;
            *) echo "❌ Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    *)
        echo "❌ Unsupported OS: $OS (use install.ps1 for Windows)"
        exit 1
        ;;
esac

echo "🎯 Budget Analyzer Installer"
echo "============================"
echo "Platform: $PLATFORM"
echo "Install directory: $INSTALL_DIR"
echo ""

# Create install directory
mkdir -p "$INSTALL_DIR"

# Download binary
BINARY_NAME="budget-${PLATFORM}"
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${BINARY_NAME}"

echo "📦 Downloading..."
if command -v curl > /dev/null 2>&1; then
    curl -sSL "$DOWNLOAD_URL" -o "$INSTALL_DIR/budget"
elif command -v wget > /dev/null 2>&1; then
    wget -q "$DOWNLOAD_URL" -O "$INSTALL_DIR/budget"
else
    echo "❌ curl or wget required"
    exit 1
fi

chmod +x "$INSTALL_DIR/budget"

echo "✅ Budget Analyzer installed!"
echo ""
echo "📍 Location: $INSTALL_DIR/budget"
echo ""

# Check if in PATH
case ":$PATH:" in
    *:"$INSTALL_DIR":*)
        echo "✅ Ready to use: budget --version"
        ;;
    *)
        echo "⚠️  Add to PATH:"
        echo "    export PATH=\"$INSTALL_DIR:\$PATH\""
        echo ""
        echo "Add to ~/.bashrc, ~/.zshrc, or ~/.profile"
        ;;
esac
