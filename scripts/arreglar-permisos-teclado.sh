#!/usr/bin/env bash
# Instala la regla udev que da a tu usuario permiso sobre el teclado K671 (258a:013b).
# Ejecútalo en una terminal normal: pide tu contraseña (sudo).
set -euo pipefail

AQUI="$(dirname "$(readlink -f "$0")")"
REGLA_ORIGEN="$AQUI/../udev/70-sindri-k671.rules"
REGLA_DESTINO=/etc/udev/rules.d/70-sindri-k671.rules

# La regla va con prefijo 70-: la etiqueta uaccess debe procesarse antes de 73-seat-late.rules.
# Con un prefijo mayor (por ejemplo 99-) solo funciona si se recargan las reglas a mano.
sudo install -m 644 "$REGLA_ORIGEN" "$REGLA_DESTINO"
sudo udevadm control --reload
sudo udevadm trigger --subsystem-match=hidraw --action=change
sleep 1

echo
echo "Regla instalada en $REGLA_DESTINO"
getfacl -p /dev/hidraw* 2>/dev/null | awk '/^# file:/{f=$3} /^user:'"$USER"':/{print f ": tienes acceso"}' || true
echo
echo "Si no aparece ninguna línea 'tienes acceso', desconecta y vuelve a conectar el teclado."
