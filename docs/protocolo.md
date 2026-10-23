# Protocolo HID del K671 Sindri

Documentación del protocolo de luces del teclado **Sino Wealth `258a:013b`** (K671 Sindri,
firmware `bcdDevice 3005`). Está sacado del programa oficial de Windows y comprobado con un teclado
real. Lo que no se pudo comprobar está marcado como **sin probar**.

## Cómo se habla con el teclado

El teclado se presenta como dos interfaces HID (`hidraw` en Linux):

| Interfaz | Contenido |
|----------|-----------|
| `MI_00`  | Teclado básico (boot keyboard). **No** responde a los reportes de luz. |
| `MI_01`  | Teclado completo + control del sistema + multimedia + **canal de configuración del fabricante** (usage page `0xFF00`). Aquí se manda la luz. |

La configuración va por **reportes feature** (`HIDIOCGFEATURE` para leer, `HIDIOCSFEATURE` para
escribir). El primer byte del búfer es el ID del reporte. Del descriptor HID de `MI_01`:

| ID | Tamaño (con el ID) | Uso | Estado |
|----|--------------------|-----|--------|
| `0x05` | 6 | Desconocido | sin probar |
| `0x09` | 505 | Probablemente la asignación de teclas (el programa oficial lo lee) | solo lectura probada: devuelve ceros |
| `0x0A` | 42 | **Efecto, grupo, velocidad y brillo** | leído y escrito |
| `0x0B` | 127 | **Teclas encendidas** del efecto personalizado | solo escritura (leerlo da `EPIPE`) |
| `0x0C` | 1921 | Probablemente macros (el programa oficial lo escribe) | sin probar |

## Reporte `0x0A`: efecto y brillo (42 bytes)

El programa oficial **lee** el reporte, cambia unos pocos bytes y lo **escribe de vuelta**. Esa es la
forma correcta de usarlo: los bytes que no se entienden hay que conservarlos.

| Byte | Campo | Notas |
|------|-------|-------|
| 0 | ID `0x0A` | |
| 1 | Efecto | ver tabla de efectos. Probado: `1`, `2`, `12` |
| 2 | Grupo de color | Escribir `2` a `5` no cambió nada visible en modo fijo |
| 3 | Velocidad | El programa oficial la guarda invertida (nivel `1..5` se guarda como `5..1`). **Sin probar** |
| 4 | Brillo | `1` a `5`. Probado: `1`, `3`, `5` |
| 5–8 | Sin uso aparente | llegaron en `00` |
| 9 | Desconocido | llegó `0x93`. **Se conserva tal cual** |
| 14 | Desconocido | llegó `0x0A`. **Se conserva tal cual** |
| resto | Ceros | |

Valores de fábrica leídos antes de cambiar nada:

```
0a 01 01 03 03 00 00 00 00 93 00 00 00 00 0a 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
```

es decir: efecto fijo, grupo 1, velocidad 3, brillo 3.

### Efectos

Los ids y nombres salen del `Cfg.ini` y `text.xml` del programa oficial. En ese archivo cada efecto
indica si admite velocidad y brillo (`;mode,speed,light`).

| Id | Nombre oficial | Velocidad | Brillo | Probado |
|----|----------------|-----------|--------|---------|
| 0 | Apagado (OFF) | no | no | no |
| 1 | Steady (fijo) | no | sí | **sí** |
| 2 | Respire | sí | sí | **sí** (la luz respira) |
| 3 | Reaction | sí | sí | no |
| 4 | Neon stream | sí | sí | no |
| 5 | Horizontal running horse | sí | no | no |
| 6 | Sine wave | sí | no | no |
| 7 | Retro snake | sí | no | no |
| 8 | Gaoshan waterfall | sí | no | no |
| 9 | Dripping ripples | sí | sí | no |
| 10 | Rain falling bead curtain | sí | sí | no |
| 11 | Rotating windmill | sí | sí | no |
| 12 | Custom (personalizado) | no | sí | **sí** |

## Reporte `0x0B`: teclas encendidas (127 bytes, solo escritura)

Sirve para el efecto **personalizado** (`0x0A` byte 1 = `12`). Primero hay que poner ese efecto y
luego mandar este reporte.

| Byte | Campo |
|------|-------|
| 0 | ID `0x0B` |
| 1 | Grupo de color (el mismo que el byte 2 de `0x0A`; se usó `1`) |
| 1 + *p* | Estado de la tecla en la posición *p* (`1..=125`) |

- El programa oficial manda `7` para una tecla encendida y `0` para una apagada.
- Se probó con `1`, `2` y `4`: las tres encienden la tecla con **el mismo color**. El valor **no
  elige el color**, solo enciende. Con `0` se apaga.
- En las pruebas el teclado ignoró a veces el primer envío, por eso la herramienta lo manda **dos
  veces**, con 0,5 s entre ambos.
- El teclado **conserva** el estado al desconectarlo y volver a conectarlo (comprobado).

### El color lo pone el LED, no el software

Este teclado es de tipo *Rainbow*: cada zona horizontal tiene un LED de **color fijo**. De izquierda
a derecha se vieron seis colores: amarillo, verde, celeste, azul, morado y rojo. Por eso:

- No existe un "azul en todo el teclado". Se puede **encender solo las teclas que ya son azules**.
- Para un color, hay que elegir qué teclas se encienden.

Las posiciones `66, 72, 73, 74, 76, 77, 78, 79, 80, 81` dieron **azul puro**. Según el mapa de abajo,
las teclas que se encienden son `menu`, `f11`, `interrogacion`, `llave-der`, `shift-der`, `ctrl-der`,
`f12` y `retroceso` (las posiciones 72 y 81 no tienen tecla). Incluir además la posición `65`
encendía la tecla `guion`, que sale celeste.

### Mapa de posiciones

Las posiciones forman una matriz de 6 filas por 21 columnas:

```
posición = 6 × columna + fila + 1
```

La tabla completa de las 105 teclas, cómo se comprobó y qué es suposición está en
[`mapa-de-teclas.md`](mapa-de-teclas.md). Una nota sobre el origen: la numeración del archivo de
configuración del programa oficial va un número por debajo (`posición − 1`), y por eso al principio
el mapa parecía desplazado en uno.

## Detalles de implementación

- Los `ioctl` son `HIDIOCSFEATURE(len) = 0xC0004806 | len << 16` y
  `HIDIOCGFEATURE(len) = 0xC0004807 | len << 16`.
- Si se intenta leer `0x0A` por la interfaz `MI_00` el teclado responde `EPIPE`. La herramienta
  prueba cada `hidraw` del dispositivo y se queda con el que responde.
- Cambios de efecto y brillo: se ven al instante.

## Lo que no se sabe

- Para qué sirven los bytes 5–8, 9 y 14 de `0x0A`.
- Qué hace el grupo de color en los demás efectos.
- Si la velocidad y los efectos 3 a 11 funcionan como indica el programa oficial.
- Si otros teclados con el mismo `258a:013b` (o distinto firmware) usan el mismo protocolo.
- Si el mapa de teclas es igual en teclados de otra distribución (el de aquí es ISO latino).
