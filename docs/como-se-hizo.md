# Cómo se hizo

Este documento cuenta, en orden, cómo se pasó de "el teclado tiene un perfil de luces raro y no hay
nada para Linux" a una herramienta que lo controla. Incluye los callejones sin salida, porque son
parte de lo aprendido.

## 1. El problema

Teclado K671 Sindri con luces tipo arcoíris en un perfil que no gustaba. En Windows se
cambia con un programa del fabricante. En Linux no hay nada oficial.

## 2. Identificar el teclado

```
lsusb
Bus 003 Device 005: ID 258a:013b Sino Wealth USB Keyboard
```

El chip es un **Sino Wealth**, muy común en teclados genéricos. Linux solo lo ve como un teclado
normal; los efectos de luz los guarda y ejecuta el propio teclado en su memoria.

## 3. Intentos que no funcionaron

- **Combinaciones de teclas `Fn`.** Funcionan (por ejemplo `Fn` + `Insert` cambia de efecto), pero no
  dejan elegir qué teclas se encienden y no se pueden automatizar.
- **OpenRGB.** Es el proyecto de referencia para RGB en Linux. Tiene controladores para otros
  teclados Sino Wealth, pero **no** para el `258a:013b`, y no lo lista.
- **Buscar un proyecto abierto** para este ID: no se encontró ninguno.

## 4. El programa oficial con Proton

El fabricante publica en su página de descargas un programa de Windows para el K671. La idea era correrlo con
Proton (Steam), configurar la luz una vez y que quedara guardada en el teclado.

- Se instaló bien con Proton 9.
- Al abrirlo salía *"The keyboard is not detected"* y se cerraba. Ese cuadro ofrece actualizar el
  firmware: **no se debe aceptar**.
- Primero faltaban permisos sobre `/dev/hidraw*` (solo root). Se resolvió con una regla udev. Detalle
  que costó: debe llamarse `70-...`, porque la etiqueta `uaccess` se procesa antes de
  `73-seat-late.rules`; con `99-...` solo funciona si se recarga a mano.
- Wine veía el teclado pero decía *"deferring to a different backend"*: ignoraba los dispositivos
  que no son mandos. `PROTON_ENABLE_HIDRAW=0x258a/0x013b` lo arregló.
- Aun así el programa no encontraba el teclado. La causa probable: en Windows la interfaz de
  configuración aparece como un dispositivo HID propio, y en Wine llega mezclada con otras. El
  programa elige la interfaz comparando `UsagePage` y `Usage`, y no la encuentra.

## 5. Ingeniería inversa del protocolo

En vez de pelear con Wine, se leyó el programa oficial para saber **qué le manda al teclado**. Se
usó [REA](https://github.com/morluto/rea) con **Ghidra 12.1.4** (modo headless) para decompilar los
binarios, sin ejecutarlos.

1. `OemDrv.exe` carga `Lowerdev.dll` con `LoadLibrary` y pide por nombre `OpenHidDevice`,
   `SetFeature` y `GetFeature`.
2. `Lowerdev.dll` recorre las interfaces HID cuyo nombre contiene `VID_258A&PID_013B`, lee su
   descriptor (`HidP_GetCaps`) y acepta la que tiene el `Usage` y `UsagePage` esperados. Esto explica
   el fallo en Wine.
3. En `OemDrv.exe` hay una función que aplica toda la configuración. Llama a una función
   `SendCommand(id, longitud, datos)` que pone el ID en el primer byte y llama a `SetFeature`. Los
   IDs y longitudes usados coinciden con los reportes feature del descriptor HID del teclado
   (`0x0A` con 42 bytes, `0x0B` con 127...).
4. El reporte `0x0A` se **lee, se modifica y se escribe de vuelta**. Los textos del programa
   (`Set led data`, `Get LED Failed`) lo confirman.
5. El `Cfg.ini` y el `text.xml` del programa dan los ids y nombres de los efectos.

## 6. Pruebas en el teclado real

Con permisos sobre `hidraw`, se probó paso a paso, de lo más seguro a lo menos:

1. **Leer** el reporte `0x0A`: `0a 01 01 03 03 00 ...` (efecto fijo, brillo 3). Se guardó una copia.
2. **Escribir el mismo reporte** y releerlo: idéntico. El envío funciona y no cambia nada.
3. Cambiar el **brillo** y el **efecto** (Respire): se ven al instante. Esto confirmó que el teclado
   hace caso a los comandos.
4. Cambiar el **grupo de color**: no cambió nada visible.
5. Reporte `0x0B` (solo escritura): con `0` casi todas las teclas se apagan, con otros valores
   se encienden. El valor **no cambia el color**.
6. Mirando qué colores salían, se vio que el color depende de la **posición** de la tecla (seis
   franjas), no del dato. Es un teclado Rainbow de LEDs de color fijo.
7. Se encendieron solo las teclas de la franja azul. El primer intento incluía una tecla celeste;
   se afinó la lista hasta dar azul puro.
8. Se desconectó y reconectó el teclado: **el perfil se conserva**.

Todas las escrituras se hicieron con una copia del reporte original a mano para poder volver atrás.
El reporte `0x0B` es la excepción: no se puede leer, así que no hay copia de su valor anterior.

## 7. Prototipo en Python y versión en Rust

El prototipo está en [`prototipo-python/`](../prototipo-python): unas 25 líneas con `fcntl.ioctl`,
sin dependencias. La herramienta final está en Rust (`src/`), con la lógica del protocolo separada
del acceso al hardware para poder probarla con tests sin teclado.

## Herramientas

- [REA](https://github.com/morluto/rea) y [Ghidra](https://ghidra-sre.org/): decompilar el programa
  oficial.
- Proton (Steam) y Wine: probar el programa oficial.
- Python (prototipo) y Rust (herramienta final).

## Sobre los archivos del fabricante

Este repositorio **no incluye** ningún archivo del programa del fabricante (ni `.exe`, ni `.dll`, ni
`Cfg.ini`). Solo contiene código propio y la descripción del protocolo, obtenida para lograr que el
teclado funcione en Linux. Es un proyecto independiente y **sin relación con el fabricante**.
