# Mapa de teclas

Cómo se corresponde cada tecla física con una posición del reporte `0x0B` (ver
[`protocolo.md`](protocolo.md)). Se midió encendiendo posiciones y anotando qué tecla se encendía,
en un teclado **ISO con distribución latina**.

## La matriz

El teclado numera las teclas como una matriz de **6 filas por 21 columnas**:

```
posición = 6 × columna + fila + 1        (fila 0..5, columna 0..20, posición 1..125)
```

La fila no es la fila física del teclado en todos los casos, pero sí agrupa las teclas de cada
franja: la fila 0 es la superior (Esc y F1 a F12), la 1 la de los números, la 2 la de la `Q`, la 3
la de la `A`, la 4 la del Shift y la 5 la de la barra espaciadora.

De las 126 casillas de la matriz, **105 tienen tecla**. Las otras 21 no corresponden a ninguna
tecla conocida; `sindri-led teclas huecos` las enciende, para quien quiera comprobarlo en su teclado.

## Tabla por filas

Cada celda es `columna: tecla`. Las teclas se nombran como en `sindri-led mapa`; los signos se
nombran por su símbolo en la distribución latina (`enie` es la `Ñ`, `llave-izq` es `{`, y así).

**Fila 0** (posición = 6 × columna + 1)
`1: esc` · `2-13: f1 … f12` · `14: impr` · `15: bloq-despl` · `16: pausa`

**Fila 1** (posición = 6 × columna + 2)
`0: barra-vertical` · `1-10: 1 2 3 4 5 6 7 8 9 0` · `11: apostrofo` · `12: interrogacion` ·
`13: retroceso` · `14: insert` · `15: inicio` · `16: repag` · `17: bloq-num` · `18: num-div` ·
`19: num-mul` · `20: num-menos`

**Fila 2** (posición = 6 × columna + 3)
`0: tab` · `1-10: q w e r t y u i o p` · `11: acento` · `12: mas` · *(13 sin tecla)* · `14: supr` ·
`15: fin` · `16: avpag` · `17: num-7` · `18: num-8` · `19: num-9` · `20: num-mas`

**Fila 3** (posición = 6 × columna + 4)
`0: mayus` · `1-9: a s d f g h j k l` · `10: enie` · `11: llave-izq` · `12: llave-der` ·
`13: enter` · *(14 a 16 sin tecla)* · `17: num-4` · `18: num-5` · `19: num-6`

**Fila 4** (posición = 6 × columna + 5)
`0: shift-izq` · `1-7: z x c v b n m` · `8: coma` · `9: punto` · `10: guion` · *(11 sin tecla)* ·
`12: shift-der` · `13: menor` · *(14 sin tecla)* · `15: arriba` · *(16 sin tecla)* ·
`17: num-1` · `18: num-2` · `19: num-3` · `20: num-enter`

**Fila 5** (posición = 6 × columna + 6)
`0: ctrl-izq` · `1: win-izq` · `2: alt-izq` · `5: espacio` · `8: alt-gr` · `9: fn` · `10: menu` ·
*(11 sin tecla)* · `12: ctrl-der` · `14: izquierda` · `15: abajo` · `16: derecha` · `17: num-0` ·
`19: num-punto`

Algunos ejemplos de cálculo: `esc` es columna 1 y fila 0, así que su posición es 6 × 1 + 0 + 1 = **7**.
`espacio` es columna 5 y fila 5: 6 × 5 + 5 + 1 = **36**. `num-enter` es columna 20 y fila 4:
6 × 20 + 4 + 1 = **125**, la posición más alta del reporte.

## Cómo se comprobó

1. **Por filas.** Se encendieron las posiciones `1, 7, 13…` (fila 0), luego `2, 8, 14…` (fila 1), y así
   hasta la fila 5. Cada grupo encendió exactamente una fila del teclado, de izquierda a derecha.
2. **Por muestreo.** En cada fila se encendió una de cada tres teclas, más posiciones que debían estar
   vacías. Salió justo lo previsto en las filas 0 a 3.
3. **Por tramos.** En las filas 4 y 5 el mapa inicial fallaba (la `Z` está en la columna 1, y el `<` en
   la 13), así que se midió por tramos hasta dar con cada tecla. La barra espaciadora ocupa **una sola
   posición** (la 36) aunque sea ancha.
4. **Todas a la vez.** Al encender las 105 posiciones del mapa, **todas las teclas del teclado se
   encienden**: no falta ninguna.

## Qué es una suposición

- Las teclas intermedias de cada fila (por ejemplo `f1`, `f2`, `w`, `r`) se dedujeron de que las
  columnas son contiguas: las que se midieron a ambos lados lo confirman, pero no se encendió cada
  una por separado.
- En el bloque numérico, `num-7`, `num-9`, `num-4`, `num-6`, `num-1` y `num-3` se anotaron por el orden
  en que se vieron encendidas, y `num-8`, `num-5`, `num-2` se comprobaron una por una.
- Es un teclado **ISO latino**. En otra distribución las teclas que cambian de símbolo conservan
  la **posición física**, pero el nombre de este mapa puede no coincidir con lo que ves impreso.

## Ayudar a mejorarlo

Con `sindri-led teclas <nombre>` puedes encender una tecla y comprobarla. Si en tu teclado alguna no
coincide, abre un issue con la tecla, su posición y tu distribución.
