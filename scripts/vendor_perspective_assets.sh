#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-4.2.0}"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VENDOR_DIR="${ROOT_DIR}/.vendor/perspective"
STATIC_DIR="${ROOT_DIR}/server-rs/static"
P_DIR="${STATIC_DIR}/perspective"

rm -rf "${VENDOR_DIR}"
mkdir -p "${VENDOR_DIR}"

rm -rf "${P_DIR}"
mkdir -p \
  "${P_DIR}/viewer/cdn" \
  "${P_DIR}/viewer/wasm" \
  "${P_DIR}/client/esm" \
  "${P_DIR}/client/wasm" \
  "${P_DIR}/viewer-datagrid/cdn"

pushd "${VENDOR_DIR}" >/dev/null

npm pack "@perspective-dev/viewer@${VERSION}"
npm pack "@perspective-dev/client@${VERSION}"
npm pack "@perspective-dev/viewer-datagrid@${VERSION}"

mkdir -p viewer client viewer-datagrid
tar -xzf "perspective-dev-viewer-${VERSION}.tgz" -C viewer
tar -xzf "perspective-dev-client-${VERSION}.tgz" -C client
tar -xzf "perspective-dev-viewer-datagrid-${VERSION}.tgz" -C viewer-datagrid

# Viewer core
cp viewer/package/dist/cdn/perspective-viewer.js "${P_DIR}/viewer/cdn/perspective-viewer.js"
cp viewer/package/dist/wasm/perspective-viewer.wasm "${P_DIR}/viewer/wasm/perspective-viewer.wasm"
cp viewer/package/dist/wasm/perspective-viewer.js "${P_DIR}/viewer/wasm/perspective-viewer.js"

if [[ -d viewer/package/dist/wasm/snippets ]]; then
  mkdir -p "${P_DIR}/viewer/wasm/snippets"
  cp -R viewer/package/dist/wasm/snippets/. "${P_DIR}/viewer/wasm/snippets/"
fi

# Client
cp client/package/dist/esm/perspective.js "${P_DIR}/client/esm/perspective.js"
cp client/package/dist/wasm/perspective-js.wasm "${P_DIR}/client/wasm/perspective-js.wasm"
cp client/package/dist/wasm/perspective-js.wasm "${P_DIR}/client/wasm/perspective-client.wasm"
cp client/package/dist/wasm/perspective-js.wasm "${P_DIR}/client/esm/perspective-client.wasm"
cp client/package/dist/cdn/perspective-server.worker.js "${P_DIR}/client/esm/perspective-server.worker.js"

# Datagrid plugin
cp viewer-datagrid/package/dist/cdn/perspective-viewer-datagrid.js \
  "${P_DIR}/viewer-datagrid/cdn/perspective-viewer-datagrid.js"

popd >/dev/null

echo "Vendored Perspective assets for version ${VERSION}:"
find "${P_DIR}" -maxdepth 5 -type f | sort