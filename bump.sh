#!/bin/bash
set -euo pipefail

CARGO="src-tauri/Cargo.toml"
PACKAGE="package.json"
TAURI="src-tauri/tauri.conf.json"

# --- read current version from Cargo.toml ---
CURRENT=$(grep -m1 '^version' "$CARGO" | sed 's/.*"\(.*\)"/\1/')
IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT"

echo "Current version: $CURRENT"

# --- compute new version ---
case "${1:-}" in
  small)
    PATCH=$((PATCH + 1))
    TAG=false
    ;;
  mid)
    MINOR=$((MINOR + 1))
    PATCH=0
    TAG=true
    ;;
  big)
    MAJOR=$((MAJOR + 1))
    MINOR=0
    PATCH=0
    TAG=true
    ;;
  custom)
    if [[ -z "${2:-}" ]]; then
      echo "Usage: ./bump.sh custom <version>"
      echo "Example: ./bump.sh custom 1.2.3"
      exit 1
    fi
    if [[ ! "$2" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
      echo "Error: version must match X.Y.Z (e.g. 1.2.3)"
      exit 1
    fi
    IFS='.' read -r MAJOR MINOR PATCH <<< "$2"
    TAG=false
    ;;
  *)
    echo "Usage: ./bump.sh <small|mid|big|custom> [version]"
    echo ""
    echo "  small          patch bump  (0.3.2 → 0.3.3)  — no git tag"
    echo "  mid            minor bump  (0.3.2 → 0.4.0)  — git tag"
    echo "  big            major bump  (0.3.2 → 1.0.0)  — git tag"
    echo "  custom 1.2.3   set exact version             — no git tag"
    exit 1
    ;;
esac

NEW="$MAJOR.$MINOR.$PATCH"

if [[ "$NEW" == "$CURRENT" ]]; then
  echo "Version is already $CURRENT, nothing to do."
  exit 0
fi

echo "Bumping: $CURRENT → $NEW"

# --- update files ---
sed -i "0,/^version = \"$CURRENT\"/s//version = \"$NEW\"/" "$CARGO"
sed -i "s/\"version\": \"$CURRENT\"/\"version\": \"$NEW\"/" "$PACKAGE"
sed -i "s/\"version\": \"$CURRENT\"/\"version\": \"$NEW\"/" "$TAURI"

# --- verify all three were updated ---
ERRORS=0
for FILE in "$CARGO" "$PACKAGE" "$TAURI"; do
  if ! grep -q "$NEW" "$FILE"; then
    echo "ERROR: failed to update $FILE"
    ERRORS=1
  fi
done
if [[ $ERRORS -ne 0 ]]; then
  echo "Version bump failed — check files manually."
  exit 1
fi

echo "Updated:"
echo "  ✓ $CARGO"
echo "  ✓ $PACKAGE"
echo "  ✓ $TAURI"

# --- next steps ---
echo ""
echo "Next steps:"
echo "  git add $CARGO $PACKAGE $TAURI"
echo "  git commit -m \"bump: v$NEW\""
if [[ "$TAG" == "true" ]]; then
  echo "  git tag -a v$NEW -m \"Release v$NEW\""
  echo "  git push && git push --tags"
else
  echo "  git push"
fi
echo ""
echo "Done ✓"
