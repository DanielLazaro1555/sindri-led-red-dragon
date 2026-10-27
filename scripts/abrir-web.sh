#!/usr/bin/env bash
# Compila (si hace falta) y abre el panel interactivo de sindri-led en el navegador web.
set -euo pipefail

AQUI="$(dirname "$(readlink -f "$0")")"
PROYECTO="$(readlink -f "$AQUI/..")"

cd "$PROYECTO"
exec cargo run --release --quiet -- web "$@"
