#!/bin/bash
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ASSET_DIR="$SCRIPT_DIR/../game/assets"
mkdir -p "$ASSET_DIR/fonts"

curl -L https://github.com/bevyengine/bevy/raw/main/examples/assets/tiles.png -o "$ASSET_DIR/tiles.png"
curl -L https://github.com/bevyengine/bevy/raw/main/examples/assets/FiraSans-Bold.ttf -o "$ASSET_DIR/fonts/FiraSans-Bold.ttf"

echo "Assets downloaded to $ASSET_DIR"
