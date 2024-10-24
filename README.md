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

- Go to the Releases page.
- Download bi-vx.x.x-win-amd64.exe.

2. Run bi:

- Open Command Prompt or PowerShell.
- Change directory to where the executable is located.
- Type .\bi-vx.x.x-win-amd64.exe to verify the installation.

3. Add to PATH (Optional):

- Move the downloaded .exe file to a preferred directory, e.g., C:\Program Files\bi.
- Add this directory to your system's PATH:
  - Press Win + X and select System.
  - Click on Advanced system settings.
  - Click Environment Variables.
  - Under System variables, scroll to Path and click Edit.
  - Click New and add the path to your bi executable.
  - Click OK to close all dialogs.

## Usage

See [Command Line Help](docs/CommandLineHelp.md)
