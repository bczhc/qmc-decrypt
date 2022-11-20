#!/usr/bin/env bash

set -e
pushd "$(realpath "$(dirname "$0")")"

if [ "$1" = "--build" ]; then
  wasm-pack build \
    --target web \
    --release \
    --scope jixun \
    --out-name qmc2_crypto \
    --out-dir pkg/web
fi

cp pkg/web/qmc2_crypto_bg.wasm npm/
cp pkg/web/qmc2_crypto.js npm/qmc2_crypto.mjs
cp pkg/web/*.ts npm/

TEMPLATE="$(env \
  WASM_B64="$(base64 --wrap=0 <pkg/web/qmc2_crypto_bg.wasm)" \
  envsubst < support/loader_template.js
)"

awk -v template="$TEMPLATE" -f support/loader_generate.awk < pkg/web/qmc2_crypto.js > npm/qmc2_crypto_embed.js
awk -f support/type_filter.awk < pkg/web/qmc2_crypto.d.ts > npm/qmc2_crypto_embed.d.ts
cp npm/qmc2_crypto_embed.js ../public/

# Generate doc & copy them
(cd npm; npm run build:doc)
cp -R npm/docs ../public/
