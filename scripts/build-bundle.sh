#!/usr/bin/env sh
# Pure POSIX shell release bundle builder for kennedy-mosoti-dev (zero Python).
set -eu

ROOT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
cd "$ROOT_DIR"

VERSION="${1:-0.1.0-alpha.1}"
OUT_DIR="${OUT_DIR:-$ROOT_DIR/dist}"
BUNDLE_NAME="kennedy-mosoti-dev-${VERSION}-linux-x86_64"
STAGE_DIR="$OUT_DIR/$BUNDLE_NAME"

rm -rf "$STAGE_DIR"
mkdir -p "$STAGE_DIR/bin" "$STAGE_DIR/site/pkg" "$STAGE_DIR/deploy"

cargo build --release --locked -p site-server --bin site-server
cargo build --profile wasm-release --locked -p site-app \
  --no-default-features --features hydrate --target wasm32-unknown-unknown

TARGET_DIR="${CARGO_TARGET_DIR:-${CARGO_BUILD_TARGET_DIR:-$ROOT_DIR/target}}"
if [ ! -f "$TARGET_DIR/release/site-server" ] && [ -f "$ROOT_DIR/target/release/site-server" ]; then
  TARGET_DIR="$ROOT_DIR/target"
fi

SERVER_BIN="$TARGET_DIR/release/site-server"
WASM_RAW="$TARGET_DIR/wasm32-unknown-unknown/wasm-release/site_app.wasm"

cp "$SERVER_BIN" "$STAGE_DIR/bin/site-server"
cp -R content "$STAGE_DIR/content"
cp -R public "$STAGE_DIR/public"
cp deploy/Caddyfile deploy/kennedy-mosoti-dev.service deploy/kennedy-mosoti-dev.env.example "$STAGE_DIR/deploy/"

if command -v wasm-bindgen >/dev/null 2>&1; then
  wasm-bindgen --target web --no-typescript --out-dir "$STAGE_DIR/site/pkg" "$WASM_RAW"
  if command -v wasm-opt >/dev/null 2>&1; then
    wasm-opt -Oz --all-features -o "$STAGE_DIR/site/pkg/site_app_bg.wasm.opt" "$STAGE_DIR/site/pkg/site_app_bg.wasm"
    mv -f "$STAGE_DIR/site/pkg/site_app_bg.wasm.opt" "$STAGE_DIR/site/pkg/site_app_bg.wasm"
  fi
else
  cp "$WASM_RAW" "$STAGE_DIR/site/pkg/site_app_bg.wasm"
fi

if command -v brotli >/dev/null 2>&1; then
  for f in "$STAGE_DIR"/site/pkg/*.wasm "$STAGE_DIR"/site/pkg/*.js; do
    [ -f "$f" ] && brotli -f -q 11 "$f"
  done
fi

if command -v zstd >/dev/null 2>&1; then
  for f in "$STAGE_DIR"/site/pkg/*.wasm "$STAGE_DIR"/site/pkg/*.js; do
    [ -f "$f" ] && zstd -q -19 -k -f "$f"
  done
fi

if command -v zstd >/dev/null 2>&1; then
  tar -C "$OUT_DIR" -cf - "$BUNDLE_NAME" | zstd -q -19 -o "$OUT_DIR/$BUNDLE_NAME.tar.zst"
fi
tar -C "$OUT_DIR" -czf "$OUT_DIR/$BUNDLE_NAME.tar.gz" "$BUNDLE_NAME"

(
  cd "$OUT_DIR"
  sha256sum "$BUNDLE_NAME".tar.* > SHA256SUMS
)

COMMIT_SHA="$(git rev-parse HEAD 2>/dev/null || echo unknown)"
cat > "$OUT_DIR/RELEASE-EVIDENCE.json" <<EOF
{
  "project": "kennedy-mosoti-dev",
  "version": "${VERSION}",
  "commit": "${COMMIT_SHA}",
  "rust_toolchain": "1.100.0",
  "renderer": "iso-diagram + leptos-0.8-islands",
  "threejs_present": false
}
EOF

echo "Built release bundle in $OUT_DIR"
