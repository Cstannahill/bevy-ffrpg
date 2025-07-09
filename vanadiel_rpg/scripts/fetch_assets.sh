#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ASSET_DIR="$SCRIPT_DIR/../game/assets"
mkdir -p "$ASSET_DIR/textures" "$ASSET_DIR/audio" "$ASSET_DIR/fonts"

# URL -> destination table
ASSETS=(
  "https://raw.githubusercontent.com/kenneyNL/roguelike-redux/master/Assets/Sprites/roguelikeSheet_transparent.png $ASSET_DIR/textures/tileset.png"
  "https://raw.githubusercontent.com/DeadlyDan/rustyjam2023/main/assets/characters.png $ASSET_DIR/textures/characters.png"
  "https://opengameart.org/sites/default/files/short_loop.ogg $ASSET_DIR/audio/overworld_theme.ogg"
  "https://github.com/bevyengine/bevy/raw/main/examples/assets/FiraSans-Bold.ttf $ASSET_DIR/fonts/FiraSans-Bold.ttf"
)

for entry in "${ASSETS[@]}"; do
  URL="$(echo "$entry" | awk '{print $1}')"
  DEST="$(echo "$entry" | awk '{print $2}')"
  if [ ! -f "$DEST" ]; then
    echo "Downloading $DEST..."
    curl -L "$URL" -o "$DEST" || exit 1
  fi
done

echo "Assets downloaded to $ASSET_DIR"
