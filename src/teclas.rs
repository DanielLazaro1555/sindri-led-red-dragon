//! Mapa de teclas físicas a posiciones del reporte `0x0B`.
//!
//! El teclado numera las teclas como una matriz de 6 filas por 21 columnas:
//! `posición = 6 * columna + fila + 1`. Los nombres corresponden a la distribución ISO latina
//! del teclado de pruebas (los signos se nombran por su símbolo en esa distribución).
//! Detalle y tabla completa en `docs/mapa-de-teclas.md`.

use crate::protocol::{parse_positions, MAX_POSITION};

pub struct Tecla {
    pub nombre: &'static str,
    pub fila: u8,
    pub columna: u8,
}

impl Tecla {
    pub const fn posicion(&self) -> u8 {
        6 * self.columna + self.fila + 1
    }
}

const fn t(nombre: &'static str, fila: u8, columna: u8) -> Tecla {
    Tecla { nombre, fila, columna }
}

/// Las 105 teclas del teclado, de la fila superior a la inferior.
pub const TECLAS: [Tecla; 105] = [
    // Fila 0: Esc, F1-F12 y Impr / Bloq Despl / Pausa
    t("esc", 0, 1),
    t("f1", 0, 2),
    t("f2", 0, 3),
    t("f3", 0, 4),
    t("f4", 0, 5),
    t("f5", 0, 6),
    t("f6", 0, 7),
    t("f7", 0, 8),
    t("f8", 0, 9),
    t("f9", 0, 10),
    t("f10", 0, 11),
    t("f11", 0, 12),
    t("f12", 0, 13),
    t("impr", 0, 14),
    t("bloq-despl", 0, 15),
    t("pausa", 0, 16),
    // Fila 1: números, Retroceso, Insert/Inicio/RePág y la fila superior del bloque numérico
    t("barra-vertical", 1, 0),
    t("1", 1, 1),
    t("2", 1, 2),
    t("3", 1, 3),
    t("4", 1, 4),
    t("5", 1, 5),
    t("6", 1, 6),
    t("7", 1, 7),
    t("8", 1, 8),
    t("9", 1, 9),
    t("0", 1, 10),
    t("apostrofo", 1, 11),
    t("interrogacion", 1, 12),
    t("retroceso", 1, 13),
    t("insert", 1, 14),
    t("inicio", 1, 15),
    t("repag", 1, 16),
    t("bloq-num", 1, 17),
    t("num-div", 1, 18),
    t("num-mul", 1, 19),
    t("num-menos", 1, 20),
    // Fila 2: Tab, QWERTY, Supr/Fin/AvPág y num 7-8-9 y +
    t("tab", 2, 0),
    t("q", 2, 1),
    t("w", 2, 2),
    t("e", 2, 3),
    t("r", 2, 4),
    t("t", 2, 5),
    t("y", 2, 6),
    t("u", 2, 7),
    t("i", 2, 8),
    t("o", 2, 9),
    t("p", 2, 10),
    t("acento", 2, 11),
    t("mas", 2, 12),
    t("supr", 2, 14),
    t("fin", 2, 15),
    t("avpag", 2, 16),
    t("num-7", 2, 17),
    t("num-8", 2, 18),
    t("num-9", 2, 19),
    t("num-mas", 2, 20),
    // Fila 3: Bloq Mayús, ASDF, Enter y num 4-5-6
    t("mayus", 3, 0),
    t("a", 3, 1),
    t("s", 3, 2),
    t("d", 3, 3),
    t("f", 3, 4),
    t("g", 3, 5),
    t("h", 3, 6),
    t("j", 3, 7),
    t("k", 3, 8),
    t("l", 3, 9),
    t("enie", 3, 10),
    t("llave-izq", 3, 11),
    t("llave-der", 3, 12),
    t("enter", 3, 13),
    t("num-4", 3, 17),
    t("num-5", 3, 18),
    t("num-6", 3, 19),
    // Fila 4: Shift, ZXCV, flecha arriba y num 1-2-3 y Enter numérico
    t("shift-izq", 4, 0),
    t("z", 4, 1),
    t("x", 4, 2),
    t("c", 4, 3),
    t("v", 4, 4),
    t("b", 4, 5),
    t("n", 4, 6),
    t("m", 4, 7),
    t("coma", 4, 8),
    t("punto", 4, 9),
    t("guion", 4, 10),
    t("shift-der", 4, 12),
    t("menor", 4, 13),
    t("arriba", 4, 15),
    t("num-1", 4, 17),
    t("num-2", 4, 18),
    t("num-3", 4, 19),
    t("num-enter", 4, 20),
    // Fila 5: Ctrl, Win, Alt, Espacio, flechas y num 0 y punto
    t("ctrl-izq", 5, 0),
    t("win-izq", 5, 1),
    t("alt-izq", 5, 2),
    t("espacio", 5, 5),
    t("alt-gr", 5, 8),
    t("fn", 5, 9),
    t("menu", 5, 10),
    t("ctrl-der", 5, 12),
    t("izquierda", 5, 14),
    t("abajo", 5, 15),
    t("derecha", 5, 16),
    t("num-0", 5, 17),
    t("num-punto", 5, 19),
];

/// Posición del reporte `0x0B` de una tecla, por su nombre (sin distinguir mayúsculas).
pub fn posicion_de(nombre: &str) -> Option<u8> {
    let n = nombre.trim().to_lowercase();
    TECLAS.iter().find(|t| t.nombre == n).map(Tecla::posicion)
}

/// Las posiciones de las 105 teclas conocidas.
pub fn posiciones_mapeadas() -> Vec<u8> {
    TECLAS.iter().map(Tecla::posicion).collect()
}

/// Las posiciones válidas del reporte que no corresponden a ninguna tecla conocida.
pub fn posiciones_sin_tecla() -> Vec<u8> {
    let usadas = posiciones_mapeadas();
    (1..=MAX_POSITION).filter(|p| !usadas.contains(p)).collect()
}

/// Lista separada por comas de nombres de tecla (`esc,f1,a`) y/o posiciones con prefijo `p`
/// (`p66,p72`). Un número suelto es siempre la tecla con ese nombre: `1` es la tecla 1.
pub fn parse_lista(s: &str) -> Result<Vec<u8>, String> {
    let mut v = Vec::new();
    for tok in s.split(',').map(str::trim).filter(|t| !t.is_empty()) {
        if let Some(p) = posicion_de(tok) {
            v.push(p);
        } else if let Some(num) = tok
            .strip_prefix(['p', 'P'])
            .filter(|n| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()))
        {
            v.extend(parse_positions(num)?);
        } else {
            return Err(format!(
                "tecla desconocida: '{tok}'. Mira los nombres con `sindri-led mapa` (una posición se escribe p66)"
            ));
        }
    }
    if v.is_empty() {
        return Err("lista de teclas vacía".into());
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn cada_tecla_tiene_posicion_valida_y_unica() {
        let mut vistas = HashSet::new();
        for t in &TECLAS {
            let p = t.posicion();
            assert!((1..=MAX_POSITION).contains(&p), "{} fuera de rango: {p}", t.nombre);
            assert!(t.fila < 6 && t.columna < 21, "{} fuera de la matriz", t.nombre);
            assert!(vistas.insert(p), "posición repetida {p} en {}", t.nombre);
        }
        assert_eq!(vistas.len(), 105);
    }

    #[test]
    fn los_nombres_son_unicos() {
        let nombres: HashSet<_> = TECLAS.iter().map(|t| t.nombre).collect();
        assert_eq!(nombres.len(), TECLAS.len());
    }

    #[test]
    fn posiciones_comprobadas_en_el_teclado_real() {
        // Cada una de estas se encendió sola o en grupos pequeños y se vio qué tecla era.
        let casos = [
            ("esc", 7),
            ("f3", 25),
            ("pausa", 97),
            ("barra-vertical", 2),
            ("interrogacion", 74),
            ("tab", 3),
            ("mas", 75),
            ("mayus", 4),
            ("enter", 82),
            ("shift-izq", 5),
            ("z", 11),
            ("menor", 83),
            ("num-enter", 125),
            ("ctrl-izq", 6),
            ("espacio", 36),
            ("alt-gr", 54),
            ("fn", 60),
            ("menu", 66),
            ("ctrl-der", 78),
            ("derecha", 102),
            ("num-punto", 120),
        ];
        for (nombre, pos) in casos {
            assert_eq!(posicion_de(nombre), Some(pos), "{nombre}");
        }
    }

    #[test]
    fn mapeadas_y_huecos_suman_todas_las_posiciones() {
        assert_eq!(posiciones_mapeadas().len() + posiciones_sin_tecla().len(), MAX_POSITION as usize);
    }

    #[test]
    fn lista_mezcla_nombres_y_numeros() {
        assert_eq!(parse_lista("esc, f1,p66").unwrap(), vec![7, 13, 66]);
        assert_eq!(parse_lista("A").unwrap(), parse_lista("a").unwrap());
        assert_eq!(parse_lista("1").unwrap(), vec![8]); // la tecla "1", no la posición 1
        assert_eq!(parse_lista("p").unwrap(), parse_lista("p").unwrap()); // "p" es la tecla P
        assert_eq!(parse_lista("p1").unwrap(), vec![1]);
        assert!(parse_lista("66").is_err()); // un número suelto no es una posición
        assert!(parse_lista("nada").is_err());
        assert!(parse_lista("").is_err());
        assert!(parse_lista("p200").is_err());
    }
}
