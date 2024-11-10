# `bi-cli`

$${\color{red}\textbf{WARNING: This tool is in alpha. Expect breaking changes.}}$$

Official Beyond Identity command-line interface.

## Table of Contents

1. [Installation](#installation)
   - [macOS and Linux](#macos-and-linux)
   - [Windows](#windows)
2. [Usage](#usage)

## Installation

### macOS and Linux

You can install bi using the provided install.sh script.

#### Using cURL

Open your terminal and run:

```bash
curl -fsSL https://raw.githubusercontent.com/gobeyondidentity/bi-cli/main/install.sh | sh
```

#### Using wget

If you prefer wget, run:

```bash
wget -qO- https://raw.githubusercontent.com/gobeyondidentity/bi-cli/main/install.sh | sh
```

### Windows

For Windows users, follow these steps:

1. Download the executable:

- Go to the [Releases](https://github.com/gobeyondidentity/bi-cli/releases) page.
- Download the latest release for your architecture (`arm64` or `x86_64`).
- Unzip the downloaded file and move the `bi` binary to a preferred directory, e.g., C:\Program Files\bi.

2. Run bi:

- Open Command Prompt or PowerShell.
- Change directory to where the executable is located.
- Type `.\bi.exe --version` to verify the installation.

3. Add to PATH (Optional):

- To add the directory where the `bi` executable is located to your system's PATH:
  - Press Win + X and select System.
  - Click on Advanced system settings.
  - Click Environment Variables.
  - Under System variables, scroll to Path and click Edit.
  - Click New and add the path to your bi executable.
  - Click OK to close all dialogs.
  - Restart Command Prompt or PowerShell to ensure changes take effect.

4. Verify PATH addition (Optional):
  - Open Command Prompt or PowerShell and type `bi --version` to confirm `bi` runs globally.

## Usage

See [Command Line Help](docs/CommandLineHelp.md)
