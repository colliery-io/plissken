#!/bin/bash
# plissken installer script
# Usage: curl -fsSL https://raw.githubusercontent.com/colliery-io/plissken/main/install.sh | bash

set -euo pipefail

REPO="colliery-io/plissken"
BINARY_NAME="plissken"
INSTALL_DIR="${PLISSKEN_INSTALL_DIR:-$HOME/.local/bin}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

info() {
    echo -e "${BLUE}info:${NC} $1"
}

success() {
    echo -e "${GREEN}success:${NC} $1"
}

warn() {
    echo -e "${YELLOW}warning:${NC} $1"
}

error() {
    echo -e "${RED}error:${NC} $1" >&2
    exit 1
}

# Detect OS
detect_os() {
    local os
    os="$(uname -s)"
    case "$os" in
        Linux*)  echo "linux" ;;
        Darwin*) echo "macos" ;;
        MINGW*|MSYS*|CYGWIN*) echo "windows" ;;
        *)       error "Unsupported operating system: $os" ;;
    esac
}

# Detect architecture
detect_arch() {
    local arch
    arch="$(uname -m)"
    case "$arch" in
        x86_64|amd64)  echo "x86_64" ;;
        arm64|aarch64) echo "aarch64" ;;
        *)             error "Unsupported architecture: $arch" ;;
    esac
}

# Get the download URL for the latest release
get_download_url() {
    local os="$1"
    local arch="$2"
    local target

    case "$os" in
        linux)
            target="${arch}-unknown-linux-gnu"
            ;;
        macos)
            target="${arch}-apple-darwin"
            ;;
        windows)
            target="${arch}-pc-windows-msvc"
            ;;
    esac

    # Get latest release info from GitHub API
    local release_url="https://api.github.com/repos/${REPO}/releases/latest"
    local release_info

    if command -v curl &> /dev/null; then
        release_info=$(curl -fsSL "$release_url")
    elif command -v wget &> /dev/null; then
        release_info=$(wget -qO- "$release_url")
    else
        error "Neither curl nor wget found. Please install one of them."
    fi

    # Extract the download URL for our target
    local download_url
    if [ "$os" = "windows" ]; then
        download_url=$(echo "$release_info" | grep -o "https://[^\"]*${target}[^\"]*\.zip" | head -1)
    else
        download_url=$(echo "$release_info" | grep -o "https://[^\"]*${target}[^\"]*\.tar\.gz" | head -1)
    fi

    if [ -z "$download_url" ]; then
        error "Could not find download URL for target: $target"
    fi

    echo "$download_url"
}

# Download and extract the binary
download_and_install() {
    local url="$1"
    local os="$2"
    local tmpdir

    tmpdir=$(mktemp -d)
    trap 'rm -rf "$tmpdir"' EXIT

    info "Downloading from $url"

    local archive="$tmpdir/plissken-archive"

    if command -v curl &> /dev/null; then
        curl -fsSL "$url" -o "$archive"
    elif command -v wget &> /dev/null; then
        wget -q "$url" -O "$archive"
    fi

    info "Extracting archive"

    if [ "$os" = "windows" ]; then
        unzip -q "$archive" -d "$tmpdir"
    else
        tar -xzf "$archive" -C "$tmpdir"
    fi

    # Create install directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"

    # Find and install the binary
    local binary
    if [ "$os" = "windows" ]; then
        binary=$(find "$tmpdir" -name "${BINARY_NAME}.exe" -type f | head -1)
        cp "$binary" "$INSTALL_DIR/${BINARY_NAME}.exe"
    else
        binary=$(find "$tmpdir" -name "$BINARY_NAME" -type f | head -1)
        cp "$binary" "$INSTALL_DIR/$BINARY_NAME"
        chmod +x "$INSTALL_DIR/$BINARY_NAME"
    fi

    success "Installed $BINARY_NAME to $INSTALL_DIR"
}

# Check if install directory is in PATH
check_path() {
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        warn "$INSTALL_DIR is not in your PATH"
        echo ""
        echo "Add it to your PATH by adding this line to your shell profile:"
        echo ""

        local shell_name
        shell_name=$(basename "$SHELL")
        case "$shell_name" in
            bash)
                echo "  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
                ;;
            zsh)
                echo "  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.zshrc"
                ;;
            fish)
                echo "  fish_add_path $INSTALL_DIR"
                ;;
            *)
                echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
                ;;
        esac
        echo ""
    fi
}

main() {
    echo ""
    echo "  ╔═══════════════════════════════════════╗"
    echo "  ║          plissken installer           ║"
    echo "  ╚═══════════════════════════════════════╝"
    echo ""

    local os arch url

    info "Detecting platform..."
    os=$(detect_os)
    arch=$(detect_arch)
    info "Detected: $os ($arch)"

    info "Finding latest release..."
    url=$(get_download_url "$os" "$arch")

    download_and_install "$url" "$os"

    check_path

    echo ""
    success "Installation complete!"
    echo ""
    echo "  Run 'plissken --help' to get started"
    echo ""
}

main "$@"
