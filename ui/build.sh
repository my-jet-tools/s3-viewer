#!/usr/bin/env bash
# Release build of the Dioxus UI, copied into ../wwwroot where the API server serves it from
# (my-http-server StaticFilesMiddleware). Run it after changing anything under ui/.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WWWROOT="${SCRIPT_DIR}/../wwwroot"
DX_OUT="${SCRIPT_DIR}/target/dx/s3-viewer-ui/release/web/public"

cd "${SCRIPT_DIR}"

# dx never cleans its output dir, so every rebuild would leave the previous hashed js/wasm next to
# the new ones and they would all end up in wwwroot. Start from an empty dir (the cargo cache stays).
echo ">> cleaning ${DX_OUT}"
rm -rf "${DX_OUT}"

echo ">> dx build --release --web"
dx build --release --web

if [ ! -d "${DX_OUT}" ]; then
    echo "ERROR: build output not found at ${DX_OUT}"
    exit 1
fi

echo ">> cleaning ${WWWROOT}"
rm -rf "${WWWROOT}"
mkdir -p "${WWWROOT}"

echo ">> copying ${DX_OUT}/. -> ${WWWROOT}/"
cp -R "${DX_OUT}/." "${WWWROOT}/"

echo ">> done."
