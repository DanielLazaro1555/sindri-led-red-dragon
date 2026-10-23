# sindri-led

Control de luces LED para el teclado **K671 Sindri** en Linux, **sin software de Windows**.
Herramienta no oficial, escrita en Rust.

> Proyecto independiente, sin relación con el fabricante del teclado. El protocolo se obtuvo
> analizando el programa oficial para que el teclado funcione en Linux. Este repositorio no incluye
> ningún archivo del fabricante.

## Qué hace

```console
$ sindri-led estado
efecto: fijo (1)
grupo de color: 1
velocidad: 3
brillo: 3
dispositivo: /dev/hidraw6

$ sindri-led brillo 5          # brillo de 1 a 5
$ sindri-led efecto respira    # cambia de efecto
$ sindri-led azul              # enciende solo las teclas azules
$ sindri-led todas             # enciende todas las teclas
$ sindri-led teclas 66,72,73   # enciende solo esas posiciones
$ sindri-led guardar           # guarda el perfil actual
$ sindri-led cargar            # vuelve a ese perfil
$ sindri-led restablecer       # fijo, grupo 1, velocidad 3, brillo 3
```

El perfil **queda guardado en la memoria del teclado**: no hace falta dejar nada corriendo.

## Un aviso sobre los colores

Este teclado es de tipo *Rainbow*: cada zona tiene un LED de **color fijo** (de izquierda a derecha:
amarillo, verde, celeste, azul, morado y rojo). Se puede **encender o apagar cada tecla**, pero no
elegirle el color. Por eso `azul` enciende solo las teclas que ya son azules, y no es posible un azul
en todo el teclado.

## Instalación

Necesitas [Rust](https://rustup.rs) (o `cargo` de tu distribución).

```bash
git clone <url-del-repositorio> sindri-led
cd sindri-led
cargo install --path .
```

Luego dale permiso a tu usuario sobre el teclado (pide `sudo`) y **vuelve a conectarlo**:

```bash
bash scripts/arreglar-permisos-teclado.sh
```

Esto instala `udev/70-sindri-k671.rules`. Sin esta regla, el teclado solo se puede abrir como root.

## Estado del proyecto

Probado **solo** en un K671 Sindri (USB `258a:013b`, distribución latina).

| Función | Estado |
|---------|--------|
| Leer estado, guardar y cargar perfil | probado |
| Brillo (`1`, `3`, `5`) | probado |
| Efectos `fijo`, `respira`, `personalizado` | probado |
| Encender o apagar teclas (reporte `0x0B`) | probado |
| El perfil sobrevive a desconectar el teclado | probado |
| Velocidad, otros efectos (`3` a `11`), `apagado` | **sin probar** |
| Mapa de nombres de tecla a posiciones | **pendiente** (por ahora se usan números) |
| Otros teclados con el mismo ID u otro firmware | **desconocido** |

## Precauciones

- La herramienta **escribe en la memoria del teclado**. Lee primero el reporte, cambia solo lo
  necesario y conserva los bytes que no entiende, igual que el programa oficial.
- El reporte de teclas (`0x0B`) **no se puede leer**, así que no se puede guardar su valor anterior.
  Solo afecta al efecto personalizado y se puede volver a escribir.
- Si usas el programa oficial del fabricante, **no aceptes** el aviso de actualizar el firmware si dice
  que no detecta el teclado.
- `sindri-led guardar` antes de experimentar te permite volver atrás con `sindri-led cargar`.

## Documentación

- [`docs/protocolo.md`](docs/protocolo.md): el protocolo HID, byte a byte, y lo que no se sabe.
- [`docs/como-se-hizo.md`](docs/como-se-hizo.md): cómo se descubrió, con los intentos que fallaron.
- [`prototipo-python/`](prototipo-python): el prototipo original en Python, sin dependencias.

## Desarrollo

```bash
cargo test    # tests del protocolo, no necesitan el teclado
cargo build --release
```

La lógica del protocolo (`src/protocol.rs`) está separada del acceso al hardware (`src/device.rs`).

## Cómo ayudar

- Probar en tu teclado y contar el resultado (indica `lsusb` y el firmware).
- Probar la velocidad y los efectos `3` a `11`.
- Hacer el mapa de posiciones a teclas físicas.
- Averiguar para qué sirven los bytes sin documentar de `docs/protocolo.md`.

## Licencia

[MIT](LICENSE)
