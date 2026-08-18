#!/usr/bin/env bash
set -euo pipefail

readonly release_version="0.0.2"

grep -Fq -- "version = \"$release_version\"" Cargo.toml
grep -Fq -- "version = \"$release_version\"" src-tauri/Cargo.toml
grep -Fq -- "\"version\": \"$release_version\"" src-tauri/tauri.conf.json
grep -Fq -- "\"version\": \"$release_version\"" ui/package.json
grep -Fq -- "\"version\": \"$release_version\"" ui/package-lock.json
grep -Fxq -- "/artifacts/" .gitignore
