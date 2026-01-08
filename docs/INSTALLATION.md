# Installation

## Universal Installer (Recommended)

The easiest way to install tmpltool on any supported platform:

```bash
curl -fsSL https://raw.githubusercontent.com/bordeux/repo/master/install.sh | sh -s -- tmpltool
```

This script automatically detects your OS and installs using the appropriate package manager.

## macOS (Homebrew)

```bash
brew tap bordeux/tap
brew install tmpltool
```

## Debian/Ubuntu (APT)

```bash
# Add repository and install
curl -fsSL https://raw.githubusercontent.com/bordeux/repo/master/install.sh | sh
sudo apt update
sudo apt install tmpltool
```

Or manually:

```bash
# Add GPG key
curl -fsSL https://bordeux.github.io/apt-repo/gpg.key | sudo gpg --dearmor -o /usr/share/keyrings/bordeux-archive-keyring.gpg

# Add repository
echo "deb [signed-by=/usr/share/keyrings/bordeux-archive-keyring.gpg] https://bordeux.github.io/apt-repo stable main" | sudo tee /etc/apt/sources.list.d/bordeux.list

# Install
sudo apt update
sudo apt install tmpltool
```

## Fedora/RHEL/CentOS (RPM)

```bash
# Add repository and install
curl -fsSL https://raw.githubusercontent.com/bordeux/repo/master/install.sh | sh
sudo dnf install tmpltool  # or yum on older systems
```

Or manually:

```bash
sudo curl -fsSL https://bordeux.github.io/rpm-repo/bordeux.repo -o /etc/yum.repos.d/bordeux.repo
sudo dnf install tmpltool
```

## From GitHub Releases

Download pre-built binaries for your platform from the [releases page](https://github.com/bordeux/tmpltool/releases):

- **Linux**: `tmpltool-linux-x86_64`, `tmpltool-linux-x86_64-musl` (static), `tmpltool-linux-aarch64` (ARM64)
- **macOS**: `tmpltool-macos-x86_64` (Intel), `tmpltool-macos-aarch64` (Apple Silicon)
- **Windows**: `tmpltool-windows-x86_64.exe`

Extract and place in your PATH:

```bash
# Linux/macOS example
tar -xzf tmpltool-linux-x86_64.tar.gz
sudo mv tmpltool /usr/local/bin/
chmod +x /usr/local/bin/tmpltool
```

## Using Docker

Docker images are available for extracting the binary into your own images (similar to gomplate pattern):

**In Your Dockerfile:**
```dockerfile
# Multi-stage build to copy tmpltool binary
FROM ghcr.io/bordeux/tmpltool:latest AS tmpltool

FROM alpine:latest
# Copy the binary from the tmpltool image
COPY --from=tmpltool /tmpltool /usr/local/bin/tmpltool

# Now use tmpltool in your build process
COPY config.tmpl /app/
RUN tmpltool /app/config.tmpl -o /app/config.json --validate json
```

**Available Tags:**
- `latest` - Latest stable release
- `v1.x.x` - Specific version tags
- Multi-arch support: `linux/amd64`, `linux/arm64`

**For Local Testing:**
```bash
# Extract binary to local system
docker create --name tmpltool-tmp ghcr.io/bordeux/tmpltool:latest
docker cp tmpltool-tmp:/tmpltool ./tmpltool
docker rm tmpltool-tmp
chmod +x ./tmpltool
./tmpltool --version
```

## From Source

```bash
cargo install --path .
```

Or build manually:

```bash
cargo build --release
# Binary will be at: ./target/release/tmpltool
```
