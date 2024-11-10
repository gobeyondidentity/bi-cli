#!/bin/bash

# Variables
VERSION="$1"
VERSION="${VERSION#v}"
OUTPUT_DIR="./scoop"
REPO_URL="https://github.com/gobeyondidentity/bi-cli/releases/download/v$VERSION"

# Function to fetch SHA-256 hash of a file from GitHub
fetch_sha256() {
  local url="$1"
  curl -L "$url" | sha256sum | awk '{print $1}'
}

# Generate the Scoop manifest
generate_scoop_manifest() {
  cat > $OUTPUT_DIR/bi.json <<EOM
{
    "version": "$VERSION",
    "description": "A CLI tool for Beyond Identity",
    "homepage": "https://github.com/gobeyondidentity/bi-cli",
    "license": "Apache-2.0",
    "architecture": {
        "64bit": {
            "url": "$REPO_URL/bi_v${VERSION}_Windows_x86_64.zip",
            "hash": "$(fetch_sha256 "$REPO_URL"/bi_v"${VERSION}"_Windows_x86_64.zip)"
        },
        "arm64": {
            "url": "$REPO_URL/bi_v${VERSION}_Windows_arm64.zip",
            "hash": "$(fetch_sha256 "$REPO_URL"/bi_v"${VERSION}"_Windows_arm64.zip)"
        }
    },
    "bin": "bi.exe",
    "checkver": {
        "url": "https://github.com/gobeyondidentity/bi-cli/releases",
        "re": "/releases/tag/v([\\\\d.]+)"
    },
    "autoupdate": {
        "architecture": {
            "64bit": {
                "url": "https://github.com/gobeyondidentity/bi-cli/releases/download/v\$version/bi_v\$version_Windows_x86_64.zip"
            },
            "arm64": {
                "url": "https://github.com/gobeyondidentity/bi-cli/releases/download/v\$version/bi_v\$version_Windows_arm64.zip"
            }
        }
    }
}
EOM
}

# Run the function to generate the manifest
generate_scoop_manifest

echo "Scoop manifest generated at $OUTPUT_DIR/bi.json"
