# `bi-cli`

$${\color{red}\textbf{WARNING: This tool is in alpha. Expect breaking changes.}}$$

Official Beyond Identity command-line interface.

## Table of Contents

1. [Installation](#installation)

- [macOS and Linux](#macos-and-linux)
  - [Install Script](#install-script)
  - [Manual Installation](#manual-installation-macos-and-linux)
  - [nix](#nix)
- [Windows](#windows)
  - [Using Scoop](#using-scoop)
  - [Manual Installation](#manual-installation-windows)

2. [Usage](#usage)

## Installation

### macOS and Linux

#### Install Script

You can install `bi` using the provided `install.sh` script.

**Using cURL**

Open your terminal and run:

```bash
curl -fsSL https://raw.githubusercontent.com/gobeyondidentity/bi-cli/main/install.sh | sh
```

**Using wget**

If you prefer `wget`, run:

```bash
wget -qO- https://raw.githubusercontent.com/gobeyondidentity/bi-cli/main/install.sh | sh
```

#### Manual Installation (macOS and Linux)

<details><summary><strong>Step-by-step Instructions</strong></summary>

<br>

If you prefer to install `bi` manually:

1. **Download the Binary:**

- Visit the [Releases](https://github.com/gobeyondidentity/bi-cli/releases) page.
- Download the latest release for your operating system and architecture (`arm64` or `x86_64`).

2. **Extract the Binary:**

- Extract the contents of the tarball to a directory of your choice.

3. **Make the Binary Executable:**

- Open your terminal and navigate to the download directory.
- Run:

  ```bash
  chmod +x bi
  ```

4. **Move the Binary to a Directory in your PATH:**

- For example:

  ```bash
  sudo mv bi /usr/local/bin/
  ```

5. **Verify the Installation:**

- Run:

  ```bash
  bi --version
  ```

  to confirm that `bi` is installed and accessible from your terminal.

</details>

#### Nix

If you are a Nix user with flakes enabled, the CLI is exposed as an output of the flake here.
For Linux or Darwin systems with aarch64 or x86-64 architectures, you can try out the CLI with
`nix run`:

```bash
nix run github:gobeyondidentity/bi-cli -- --help
```

Or drop into a bash shell containing it:

```bash
nix develop 'github:gobeyondidentity/bi-cli#bi'
```

Finally, you can install it for your profile to always have it on your path with:

```bash
nix profile install github:gobeyondidentity/bi-cli
```

### Windows

#### Using Scoop

You can install `bi` on Windows using [Scoop](https://scoop.sh/), a command-line installer for Windows.

1. **Add the `bi` Bucket:**

```powershell
scoop bucket add bi https://github.com/gobeyondidentity/bi-cli.git
```

2. **Install `bi`:**

```powershell
scoop install bi
```

3. **Verify the Installation:**

```powershell
bi --version
```

#### Manual Installation (Windows)

<details><summary><strong>Step-by-step Instructions</strong></summary>

<br>

1. **Download the Executable:**

- Go to the [Releases](https://github.com/gobeyondidentity/bi-cli/releases) page.
- Download the latest Windows release for your architecture (`arm64` or `x86_64`).

2. **Extract the Binary:**

- Unzip the downloaded file to a directory of your choice (e.g., `C:\Program Files\bi`).

3. **Add to PATH (Optional but Recommended):**

- Press `Win + X` and select **System**.
- Click on **Advanced system settings**.
- Click **Environment Variables**.
- Under **System variables**, scroll to `Path` and click **Edit**.
- Click **New** and add the path to your `bi` executable (e.g., `C:\Program Files\bi`).
- Click **OK** to close all dialogs.
- Restart Command Prompt or PowerShell to ensure changes take effect.

4. **Run `bi`:**

- Open Command Prompt or PowerShell.
- If you didn't add `bi` to your PATH, navigate to the directory containing `bi.exe`.
- Run:

  ```powershell
  .\bi.exe --version
  ```

- If you added `bi` to your PATH, simply run:

  ```powershell
  bi --version
  ```

</details>

## Usage

For detailed usage instructions, see [Command Line Help](docs/CommandLineHelp.md).


## Local Database Management
The following are the locations of the local database

`MacOs`

```sh
/Users/<localuser>/Library/Application Support/com.BeyondIdentity.bi
```

`Linux`

```sh
/home/<localuser>/.config/bi
```

`Windows`

```sh
C:\Users\<localuser>\AppData\Roaming\BeyondIdentity\bi
```
