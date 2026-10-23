#!/usr/bin/env bash
# Deja encendidas solo las teclas azules del K671 (efecto Custom).
exec python3 -I "$(dirname "$(readlink -f "$0")")/azul.py"
