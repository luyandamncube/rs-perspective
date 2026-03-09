#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
P_DIR="${ROOT_DIR}/server-rs/static/perspective"

required=(
  "${P_DIR}/viewer/cdn/perspective-viewer.js"
  "${P_DIR}/viewer/wasm/perspective-viewer.wasm"
  "${P_DIR}/viewer/wasm/perspective-viewer.js"
  "${P_DIR}/client/esm/perspective.js"
  "${P_DIR}/client/wasm/perspective-js.wasm"
  "${P_DIR}/client/wasm/perspective-client.wasm"
  "${P_DIR}/viewer-datagrid/cdn/perspective-viewer-datagrid.js"
)

for file in "${required[@]}"; do
  if [[ ! -f "${file}" ]]; then
    echo "Missing required asset: ${file}" >&2
    exit 1
  fi

  size=$(wc -c < "${file}")
  if [[ "${size}" -le 0 ]]; then
    echo "Asset is empty: ${file}" >&2
    exit 1
  fi

  echo "${size} bytes  ${file}"
done

if [[ -d "${P_DIR}/viewer/wasm/snippets" ]]; then
  snippet_count=$(find "${P_DIR}/viewer/wasm/snippets" -type f | wc -l | tr -d ' ')
  echo "${snippet_count} snippet files  ${P_DIR}/viewer/wasm/snippets"
fi

echo "Perspective assets verified."