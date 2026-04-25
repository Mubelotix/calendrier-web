#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")"

tmp_dir=$(mktemp -d)
trap 'rm -rf "$tmp_dir"' EXIT

rm -rf pkg pkg-solar

wasm-pack build --target web
cp -a pkg "$tmp_dir/pkg-default"

wasm-pack build --target web --features solar
cp -a pkg pkg-solar

rm -rf pkg
mv "$tmp_dir/pkg-default" pkg

ln -sfn ../wrapper/pkg/calendrier_web.js ../static/calendrier_web.js
ln -sfn ../wrapper/pkg/calendrier_web_bg.wasm ../static/calendrier_web_bg.wasm
ln -sfn ../wrapper/pkg-solar/calendrier_web.js ../static/calendrier_web_solar.js
ln -sfn ../wrapper/pkg-solar/calendrier_web_bg.wasm ../static/calendrier_web_solar_bg.wasm
