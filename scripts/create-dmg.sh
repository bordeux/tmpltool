#!/bin/bash
#
# Create a macOS DMG installer for tmpltool
#
# Usage: ./scripts/create-dmg.sh [binary_path] [output_path]
#
# Arguments:
#   binary_path  - Path to the tmpltool binary (default: target/release/tmpltool)
#   output_path  - Output DMG path (default: tmpltool-VERSION-ARCH.dmg)
#
# This script creates a drag-to-install DMG with:
#   - tmpltool binary
#   - Applications folder symlink
#   - README.md
#   - LICENSE
#

set -euo pipefail

# Configuration
APP_NAME="tmpltool"
VOLUME_NAME="tmpltool Installer"

# Get version from Cargo.toml
VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

# Detect architecture
ARCH=$(uname -m)
case "$ARCH" in
    x86_64)
        ARCH_NAME="x86_64"
        ;;
    arm64|aarch64)
        ARCH_NAME="aarch64"
        ;;
    *)
        ARCH_NAME="$ARCH"
        ;;
esac

# Arguments with defaults
BINARY_PATH="${1:-target/release/tmpltool}"
OUTPUT_PATH="${2:-${APP_NAME}-${VERSION}-darwin-${ARCH_NAME}.dmg}"

# Temporary staging directory
STAGING_DIR=$(mktemp -d)
trap "rm -rf '$STAGING_DIR'" EXIT

echo "Creating DMG for ${APP_NAME} v${VERSION} (${ARCH_NAME})..."
echo "  Binary: ${BINARY_PATH}"
echo "  Output: ${OUTPUT_PATH}"

# Verify binary exists
if [[ ! -f "$BINARY_PATH" ]]; then
    echo "Error: Binary not found at ${BINARY_PATH}"
    echo "Run 'cargo build --release' first."
    exit 1
fi

# Create staging directory structure
echo "Staging files..."
cp "$BINARY_PATH" "${STAGING_DIR}/${APP_NAME}"
chmod +x "${STAGING_DIR}/${APP_NAME}"

# Copy documentation
if [[ -f "README.md" ]]; then
    cp "README.md" "${STAGING_DIR}/"
fi
if [[ -f "LICENSE" ]]; then
    cp "LICENSE" "${STAGING_DIR}/"
fi

# Create Applications symlink for drag-to-install
# Note: For CLI tools, this symlink is to /usr/local/bin conceptually,
# but users typically copy the binary manually. The symlink provides
# a familiar drag-to-install experience.
ln -s /Applications "${STAGING_DIR}/Applications"

# Create the DMG
echo "Creating DMG..."

# Remove existing DMG if present
rm -f "$OUTPUT_PATH"

# Create DMG using hdiutil
hdiutil create \
    -volname "$VOLUME_NAME" \
    -srcfolder "$STAGING_DIR" \
    -ov \
    -format UDZO \
    "$OUTPUT_PATH"

echo "DMG created successfully: ${OUTPUT_PATH}"

# Show DMG info
echo ""
echo "DMG contents:"
hdiutil attach -nobrowse "$OUTPUT_PATH" -mountpoint /tmp/tmpltool-dmg-verify 2>/dev/null || true
ls -la /tmp/tmpltool-dmg-verify 2>/dev/null || true
hdiutil detach /tmp/tmpltool-dmg-verify 2>/dev/null || true

echo ""
echo "To install, users should:"
echo "  1. Open the DMG"
echo "  2. Copy 'tmpltool' to /usr/local/bin or add to PATH"
echo "  3. Run: chmod +x /usr/local/bin/tmpltool"
