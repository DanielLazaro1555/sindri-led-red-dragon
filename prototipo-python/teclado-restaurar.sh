#!/usr/bin/env bash
# Vuelve al perfil de luces original del K671 (Steady, brillo 3).
exec python3 -I "$(dirname "$(readlink -f "$0")")/restore.py"
