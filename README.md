# sindri-led

Control de luces LED para el teclado **K671 Sindri** en Linux, **sin software de Windows**.
Herramienta no oficial, escrita en Rust.

> Proyecto independiente, sin relación con el fabricante del teclado. El protocolo se obtuvo
> analizando el programa oficial para que el teclado funcione en Linux. Este repositorio no incluye
> ningún archivo del fabricante.

## Qué hace

Cambia el efecto, el brillo y las teclas encendidas del teclado desde la terminal. El perfil **queda
guardado en la memoria del teclado**, así que no hace falta dejar ningún programa corriendo y se
conserva al desconectarlo.

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

Escribe `sindri-led --help` para ver todas las órdenes.

## Un aviso sobre los colores

Este teclado es de tipo *Rainbow*: cada zona tiene un LED de **color fijo**. De izquierda a derecha se
vieron amarillo, verde, celeste, azul, morado y rojo. Se puede **encender o apagar cada tecla**, pero
no elegirle el color. Por eso `azul` enciende solo las teclas que ya son azules, y no es posible un
azul en todo el teclado.

## ¿Es tu teclado?

Comprueba que aparece este identificador USB:

```bash
lsusb | grep 258a:013b
```

Debe salir una línea como `ID 258a:013b Sino Wealth USB Keyboard`. Si no aparece, esta herramienta no
es para tu teclado.

## Instalación

Necesitas Rust y `cargo` (con [rustup](https://rustup.rs) o desde tu distribución, por ejemplo
`sudo dnf install cargo` en Fedora o `sudo apt install cargo` en Debian y Ubuntu).

```bash
git clone https://github.com/DanielLazaro1555/sindri-led-red-dragon.git
cd sindri-led-red-dragon
cargo install --path .
```

`cargo install` deja el programa en `~/.cargo/bin`; comprueba que esa carpeta esté en tu `PATH`.

Después dale permiso a tu usuario sobre el teclado y **vuelve a conectarlo**:

```bash
bash scripts/arreglar-permisos-teclado.sh
```

El script pide `sudo` e instala `udev/70-sindri-k671.rules`. Sin esa regla, el teclado solo se puede
abrir como root.

### Desinstalar

```bash
cargo uninstall sindri-led
sudo rm /etc/udev/rules.d/70-sindri-k671.rules
```

## Estado del proyecto

Probado **solo** en un teclado K671 Sindri (USB `258a:013b`, distribución latina).

| Función | Estado |
|---------|--------|
| Leer estado, guardar y cargar perfil | probado |
| Brillo (`1`, `3`, `5`) | probado |
| Efectos `fijo`, `respira`, `personalizado` | probado |
| Encender o apagar teclas (reporte `0x0B`) | probado |
| El perfil se conserva al desconectar el teclado | probado |
| Velocidad, otros efectos (`3` a `11`), `apagado` | **sin probar** |
| Mapa de nombres de tecla a posiciones | **pendiente** (por ahora se usan números) |
| Otros teclados con el mismo ID u otro firmware | **desconocido** |

## Precauciones

- La herramienta **escribe en la memoria del teclado**. Lee primero el reporte, cambia solo lo
  necesario y conserva los bytes que no entiende, igual que el programa oficial.
- El reporte de teclas (`0x0B`) **no se puede leer**, así que no se puede guardar su valor anterior.
  Solo afecta al efecto personalizado y se puede volver a escribir.
- Ejecuta `sindri-led guardar` antes de experimentar: así puedes volver atrás con `sindri-led cargar`.
- Si usas el programa oficial del fabricante, **no aceptes** el aviso de actualizar el firmware si dice
  que no detecta el teclado.

## Problemas frecuentes

| Mensaje | Qué hacer |
|---------|-----------|
| `no se encontró el teclado (USB 258a:013b)` | Comprueba que está conectado y que `lsusb` lo muestra. |
| `sin permiso para abrir /dev/hidraw...` | Ejecuta `scripts/arreglar-permisos-teclado.sh` y vuelve a conectar el teclado. |
| `ninguna interfaz respondió al reporte 0x0A` | Puede ser otro firmware. Abre un issue con la salida de `lsusb -v -d 258a:013b`. |
| Las teclas no cambian al usar `teclas` o `azul` | Vuelve a ejecutar la orden: el teclado a veces ignora el primer envío. |

## Documentación

- [`docs/protocolo.md`](docs/protocolo.md): el protocolo HID, byte a byte, y lo que no se sabe.
- [`docs/como-se-hizo.md`](docs/como-se-hizo.md): cómo se descubrió, con los intentos que fallaron.
- [`prototipo-python/`](prototipo-python): el prototipo original en Python, sin dependencias.

## Desarrollo

```bash
cargo test            # tests del protocolo, no necesitan el teclado
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
