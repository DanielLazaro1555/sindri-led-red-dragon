#!/usr/bin/env bash
# Enciende todas las teclas del K671 (efecto Custom, arcoiris por zonas).
exec python3 -I "$(dirname "$(readlink -f "$0")")/todas.py"
