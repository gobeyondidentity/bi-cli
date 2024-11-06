#!/bin/sh

set -e

# Detect OS
OS="$(uname -s)"
ARCH="$(uname -m)"

# Normalize OS names
if [ "$OS" = "Linux" ]; then
    OS="Linux"
elif [ "$OS" = "Darwin" ]; then
    OS="Darwin"
else
    echo "Unsupported OS: $OS"
    exit 1
fi

# Normalize architecture names
if [ "$ARCH" = "x86_64" ] || [ "$ARCH" = "amd64" ]; then
    ARCH="x86_64"
elif [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
    ARCH="arm64"
else
    echo "Unsupported architecture: $ARCH"
    exit 1
fi

# Fetch the latest version tag from GitHub API
VERSION=$(curl -s https://api.github.com/repos/gobeyondidentity/bi-cli/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
BINARY_NAME="bi_${VERSION}_${OS}_${ARCH}.tar.gz"
DOWNLOAD_URL="https://github.com/gobeyondidentity/bi-cli/releases/download/${VERSION}/${BINARY_NAME}"

echo "Downloading ${BINARY_NAME} from ${DOWNLOAD_URL}..."
curl -L -o "${BINARY_NAME}" "${DOWNLOAD_URL}"

# Verify download success
if [ ! -f "${BINARY_NAME}" ]; then
    echo "Download failed or file '${BINARY_NAME}' not found."
    exit 1
fi

# Extract the tarball
echo "Extracting ${BINARY_NAME}..."
tar -xzf "${BINARY_NAME}"
rm "${BINARY_NAME}"

chmod +x bi

# Try to install in /usr/local/bin if writable
if [ -w "/usr/local/bin" ]; then
    DESTINATION="/usr/local/bin"
    mv bi "$DESTINATION/bi"
    echo "bi installed to $DESTINATION/bi"
    exit 0
fi

# Try to install with sudo to /usr/local/bin
if command -v sudo >/dev/null 2>&1; then
    echo "Attempting to install to /usr/local/bin with sudo..."
    sudo mkdir -p /usr/local/bin
    sudo mv bi /usr/local/bin/bi
    echo "bi installed to /usr/local/bin/bi"
    exit 0
fi

# Fall back to $HOME/.local/bin
DESTINATION="$HOME/.local/bin"
mkdir -p "$DESTINATION"
mv bi "$DESTINATION/bi"
echo "bi installed to $DESTINATION/bi"

# Provide PATH update instructions if necessary
if ! echo "$PATH" | grep -q "$DESTINATION"; then
    echo "Note: $DESTINATION may not be in your PATH."
    echo "To add it to your PATH, add the following line to your shell profile:"
    echo ""
    echo 'export PATH="$HOME/.local/bin:$PATH"'
    echo ""
    echo "For Bash, add it to ~/.bashrc:"
    echo "    echo 'export PATH=\"$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
    echo ""
    echo "For Zsh, add it to ~/.zshrc:"
    echo "    echo 'export PATH=\"$HOME/.local/bin:\$PATH\"' >> ~/.zshrc"
    echo ""
    echo "Then reload your shell or source the file, e.g., 'source ~/.bashrc' or 'source ~/.zshrc'."
fi
