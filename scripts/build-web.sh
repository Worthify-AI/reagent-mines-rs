#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
repo_path="$(pwd -P)"
task_cargo_home="${CARGO_HOME:-${HOME}/.cargo}"
task_rustup_home="${RUSTUP_HOME:-${HOME}/.rustup}"
export CARGO_ENCODED_RUSTFLAGS
CARGO_ENCODED_RUSTFLAGS="$(printf '%s\037%s\037%s\037%s\037%s' '-C' 'link-arg=--import-undefined' "--remap-path-prefix=${repo_path}=." "--remap-path-prefix=${task_cargo_home}=cargo" "--remap-path-prefix=${task_rustup_home}=rust")"
cargo build --locked --release --target wasm32-unknown-unknown
cp "${CARGO_TARGET_DIR:-target}/wasm32-unknown-unknown/release/reagent-mines-rs.wasm" web/reagent_mines_rs.wasm
python3 - <<'HASH'
from pathlib import Path
import json,hashlib
w=Path('web')
(w/'loader-manifest.json').write_text(json.dumps({'source':'miniquad 0.4.8 crate js/gl.js','loader_sha256':hashlib.sha256((w/'gl.js').read_bytes()).hexdigest(),'wasm_sha256':hashlib.sha256((w/'reagent_mines_rs.wasm').read_bytes()).hexdigest()},indent=2)+'\n')
HASH
