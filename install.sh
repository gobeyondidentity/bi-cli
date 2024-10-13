#!/bin/sh

set -e

# Detect OS
OS="$(uname -s)"
ARCH="$(uname -m)"

# Normalize OS names
if [ "$OS" = "Linux" ]; then
    OS="linux"
elif [ "$OS" = "Darwin" ]; then
    OS="macos"
else
    echo "Unsupported OS: $OS"
    exit 1
fi

# Normalize architecture names
if [ "$ARCH" = "x86_64" ] || [ "$ARCH" = "amd64" ]; then
    ARCH="amd64"
elif [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
    ARCH="arm64"
else
    echo "Unsupported architecture: $ARCH"
    exit 1
fi

# Fetch the latest version tag from GitHub API
VERSION=$(curl -s https://api.github.com/repos/gobeyondidentity/bi-cli/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

BINARY_NAME="bi-${VERSION}-${OS}-${ARCH}"

DOWNLOAD_URL="https://github.com/gobeyondidentity/bi-cli/releases/download/${VERSION}/${BINARY_NAME}"

echo "Downloading ${BINARY_NAME} from ${DOWNLOAD_URL}..."
curl -L -o bi "${DOWNLOAD_URL}"

chmod +x bi

# Determine installation directory
if [ -w "/usr/local/bin" ]; then
    DESTINATION="/usr/local/bin"
elif [ -w "$HOME/.local/bin" ]; then
    DESTINATION="$HOME/.local/bin"
    mkdir -p "$DESTINATION"
else
    echo "Cannot write to /usr/local/bin or ~/.local/bin"
    echo "Attempting to use sudo to install to /usr/local/bin"
    if command -v sudo >/dev/null 2>&1; then
        sudo mv bi /usr/local/bin/bi
        echo "bi installed to /usr/local/bin/bi"
    else
        echo "sudo is not installed. Please move the 'bi' binary to a directory in your PATH manually."
        exit 1
    fi
    exit 0
fi

mv bi "$DESTINATION/bi"
echo "bi installed to $DESTINATION/bi"

