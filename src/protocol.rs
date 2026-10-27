//! Protocolo HID del K671 Sindri (Sino Wealth `258a:013b`).
//!
//! Todo lo de este módulo es puro (no toca el hardware), para poder probarlo con tests.
//! La descripción completa del protocolo está en `docs/protocolo.md`.

use std::fmt;

/// Reporte feature con la configuración de luz: efecto, grupo, velocidad y brillo.
pub const REPORT_LED: u8 = 0x0A;
/// Longitud del reporte `0x0A` incluyendo el byte del ID.
pub const LED_REPORT_LEN: usize = 42;

/// Reporte feature (solo escritura) con las teclas encendidas del efecto Custom.
pub const REPORT_KEYS: u8 = 0x0B;
/// Longitud del reporte `0x0B` incluyendo el byte del ID.
pub const KEYS_REPORT_LEN: usize = 127;
/// Valor que enciende una tecla. `0` la apaga. El color lo decide el LED, no este valor.
pub const KEY_ON: u8 = 7;
/// Posición más alta que cabe en el reporte `0x0B`.
pub const MAX_POSITION: u8 = 125;

/// Todas las posiciones del reporte `0x0B` (`1..=125`, incluida `num-enter` en 125).
pub const POSITIONS_ALL: std::ops::RangeInclusive<u8> = 1..=MAX_POSITION;
/// Teclas de la franja azul del arcoíris comprobadas en el teclado real:
/// menu (66), f11 (73), interrogacion (74), llave-der (76), shift-der (77),
/// ctrl-der (78), f12 (79) y retroceso (80). Se deja fuera `guion` (posición 65),
/// que sale celeste.
pub const POSITIONS_BLUE: [u8; 8] = [66, 73, 74, 76, 77, 78, 79, 80];

/// Valores por defecto que traía el teclado de fábrica (los leídos antes de cambiar nada).
pub const DEFAULT_EFFECT: Effect = Effect::Steady;
pub const DEFAULT_GROUP: u8 = 1;
pub const DEFAULT_SPEED: u8 = 3;
pub const DEFAULT_BRIGHTNESS: u8 = 3;

/// Efectos de luz. Los ids salen del `Cfg.ini` del programa oficial.
/// Solo `Steady`, `Breathing` y `Custom` están probados en un teclado real.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effect {
    Off = 0,
    Steady = 1,
    Breathing = 2,
    Reaction = 3,
    NeonStream = 4,
    RunningHorse = 5,
    SineWave = 6,
    RetroSnake = 7,
    Waterfall = 8,
    Ripples = 9,
    RainCurtain = 10,
    Windmill = 11,
    Custom = 12,
}

impl Effect {
    pub const ALL: [Effect; 13] = [
        Effect::Off,
        Effect::Steady,
        Effect::Breathing,
        Effect::Reaction,
        Effect::NeonStream,
        Effect::RunningHorse,
        Effect::SineWave,
        Effect::RetroSnake,
        Effect::Waterfall,
        Effect::Ripples,
        Effect::RainCurtain,
        Effect::Windmill,
        Effect::Custom,
    ];

    pub fn id(self) -> u8 {
        self as u8
    }

    pub fn from_id(id: u8) -> Option<Effect> {
        Effect::ALL.into_iter().find(|e| e.id() == id)
    }

    /// Indica si el efecto admite control de velocidad según `LedOpt` en `Cfg.ini`.
    pub const fn uses_speed(self) -> bool {
        matches!(
            self,
            Effect::Breathing
                | Effect::Reaction
                | Effect::NeonStream
                | Effect::RunningHorse
                | Effect::SineWave
                | Effect::RetroSnake
                | Effect::Waterfall
                | Effect::Ripples
                | Effect::RainCurtain
                | Effect::Windmill
        )
    }

    /// Indica si el efecto admite control de brillo según `LedOpt` en `Cfg.ini`.
    /// Los efectos 5 a 8 (Caballo, Onda, Serpiente, Cascada) tienen `light=0` y brillan siempre al máximo.
    pub const fn uses_brightness(self) -> bool {
        matches!(
            self,
            Effect::Steady
                | Effect::Breathing
                | Effect::Reaction
                | Effect::NeonStream
                | Effect::Ripples
                | Effect::RainCurtain
                | Effect::Windmill
                | Effect::Custom
        )
    }

    /// Nombre corto en español, el que usa la línea de comandos.
    pub fn name(self) -> &'static str {
        match self {
            Effect::Off => "apagado",
            Effect::Steady => "fijo",
            Effect::Breathing => "respira",
            Effect::Reaction => "reaccion",
            Effect::NeonStream => "neon",
            Effect::RunningHorse => "caballo",
            Effect::SineWave => "onda",
            Effect::RetroSnake => "serpiente",
            Effect::Waterfall => "cascada",
            Effect::Ripples => "ondas",
            Effect::RainCurtain => "lluvia",
            Effect::Windmill => "molino",
            Effect::Custom => "personalizado",
        }
    }

    /// Acepta el nombre en español, el nombre en inglés del programa oficial o el número.
    pub fn parse(s: &str) -> Result<Effect, String> {
        let s = s.trim().to_lowercase();
        if let Ok(n) = s.parse::<u8>() {
            return Effect::from_id(n).ok_or_else(|| format!("efecto {n} fuera de rango (0 a 12)"));
        }
        let by_name = match s.as_str() {
            "apagado" | "off" => Some(Effect::Off),
            "fijo" | "steady" => Some(Effect::Steady),
            "respira" | "respire" | "breathing" => Some(Effect::Breathing),
            "reaccion" | "reaction" => Some(Effect::Reaction),
            "neon" | "neon_stream" => Some(Effect::NeonStream),
            "caballo" => Some(Effect::RunningHorse),
            "onda" | "sine_wave" => Some(Effect::SineWave),
            "serpiente" | "retro_snake" => Some(Effect::RetroSnake),
            "cascada" => Some(Effect::Waterfall),
            "ondas" => Some(Effect::Ripples),
            "lluvia" => Some(Effect::RainCurtain),
            "molino" => Some(Effect::Windmill),
            "personalizado" | "custom" => Some(Effect::Custom),
            _ => None,
        };
        by_name.ok_or_else(|| {
            let names: Vec<_> = Effect::ALL.iter().map(|e| e.name()).collect();
            format!("efecto desconocido '{s}'. Usa un número 0-12 o: {}", names.join(", "))
        })
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name(), self.id())
    }
}

/// Reporte `0x0A` completo (42 bytes). Se lee del teclado, se cambian unos pocos bytes
/// y se escribe de vuelta, igual que hace el programa oficial. Los demás bytes se conservan.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LedReport([u8; LED_REPORT_LEN]);

impl LedReport {
    pub fn from_bytes(b: &[u8]) -> Result<LedReport, String> {
        let raw: [u8; LED_REPORT_LEN] = b
            .try_into()
            .map_err(|_| format!("el reporte 0x0A debe tener {LED_REPORT_LEN} bytes, tiene {}", b.len()))?;
        if raw[0] != REPORT_LED {
            return Err(format!("el primer byte debe ser 0x0A, es {:#04x}", raw[0]));
        }
        Ok(LedReport(raw))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn effect_id(&self) -> u8 {
        self.0[1]
    }
    pub fn group(&self) -> u8 {
        self.0[2]
    }
    /// Nivel de velocidad de `1` (más lento) a `5` (más rápido).
    /// En `OemDrv.exe` (`FUN_00403150`), el byte 3 del reporte `0x0A` se guarda invertido (`6 - nivel`):
    /// `5` en el reporte es la velocidad más lenta (nivel 1) y `1` es la más rápida (nivel 5).
    pub fn speed(&self) -> u8 {
        let raw = self.0[3];
        if (1..=5).contains(&raw) {
            6 - raw
        } else {
            raw
        }
    }
    pub fn brightness(&self) -> u8 {
        self.0[4]
    }

    pub fn set_effect(&mut self, e: Effect) {
        self.0[1] = e.id();
    }
    pub fn set_group(&mut self, v: u8) {
        self.0[2] = v;
    }
    /// Ajusta la velocidad (`1` = más lento, `5` = más rápido), guardándola invertida (`6 - v`)
    /// como hace `FUN_00403150` en `OemDrv.exe`.
    pub fn set_speed(&mut self, v: u8) {
        self.0[3] = if (1..=5).contains(&v) { 6 - v } else { v };
    }
    pub fn set_brightness(&mut self, v: u8) {
        self.0[4] = v;
    }

    /// Bytes en hexadecimal separados por espacios, para guardar el perfil en un archivo.
    pub fn to_hex(&self) -> String {
        self.0.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" ")
    }

    pub fn from_hex(s: &str) -> Result<LedReport, String> {
        let bytes = s
            .split_whitespace()
            .map(|t| u8::from_str_radix(t, 16).map_err(|_| format!("byte hexadecimal inválido: '{t}'")))
            .collect::<Result<Vec<u8>, String>>()?;
        LedReport::from_bytes(&bytes)
    }
}

impl fmt::Display for LedReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let efecto = match Effect::from_id(self.effect_id()) {
            Some(e) => e.to_string(),
            None => format!("desconocido ({})", self.effect_id()),
        };
        write!(
            f,
            "efecto: {efecto}\ngrupo de color: {}\nvelocidad: {}\nbrillo: {}",
            self.group(),
            self.speed(),
            self.brightness()
        )
    }
}

/// Construye el reporte `0x0B` con las posiciones indicadas encendidas y el resto apagado.
pub fn keys_report(positions: &[u8], group: u8) -> Result<[u8; KEYS_REPORT_LEN], String> {
    let mut r = [0u8; KEYS_REPORT_LEN];
    r[0] = REPORT_KEYS;
    r[1] = group;
    for &p in positions {
        if p == 0 || p > MAX_POSITION {
            return Err(format!("posición {p} fuera de rango (1 a {MAX_POSITION})"));
        }
        r[1 + p as usize] = KEY_ON;
    }
    Ok(r)
}

/// Lee una lista de posiciones separadas por comas, por ejemplo `66,72,73`.
pub fn parse_positions(s: &str) -> Result<Vec<u8>, String> {
    let mut v = Vec::new();
    for t in s.split(',').map(str::trim).filter(|t| !t.is_empty()) {
        let p: u8 = t.parse().map_err(|_| format!("posición inválida: '{t}'"))?;
        if p == 0 || p > MAX_POSITION {
            return Err(format!("posición {p} fuera de rango (1 a {MAX_POSITION})"));
        }
        v.push(p);
    }
    if v.is_empty() {
        return Err("lista de posiciones vacía".into());
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// El reporte real que devolvió el teclado antes de tocar nada.
    const ORIGINAL: &str = "0a 01 01 03 03 00 00 00 00 93 00 00 00 00 0a 00 00 00 00 00 00 00 00 00 00 \
                            00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00";

    #[test]
    fn lee_el_reporte_original() {
        let r = LedReport::from_hex(ORIGINAL).unwrap();
        assert_eq!(r.effect_id(), 1);
        assert_eq!(Effect::from_id(r.effect_id()), Some(Effect::Steady));
        assert_eq!((r.group(), r.speed(), r.brightness()), (1, 3, 3));
    }

    #[test]
    fn velocidad_se_guarda_invertida_como_oemdrv() {
        let mut r = LedReport::from_hex(ORIGINAL).unwrap();
        // Nivel 1 (más lento) -> byte 5 en el reporte 0x0A
        r.set_speed(1);
        assert_eq!(r.as_bytes()[3], 5);
        assert_eq!(r.speed(), 1);
        // Nivel 5 (más rápido) -> byte 1 en el reporte 0x0A
        r.set_speed(5);
        assert_eq!(r.as_bytes()[3], 1);
        assert_eq!(r.speed(), 5);
    }

    #[test]
    fn cambiar_un_campo_conserva_el_resto() {
        let original = LedReport::from_hex(ORIGINAL).unwrap();
        let mut r = original.clone();
        r.set_brightness(5);
        for (i, (a, b)) in original.as_bytes().iter().zip(r.as_bytes()).enumerate() {
            if i == 4 {
                assert_eq!(*b, 5);
            } else {
                assert_eq!(a, b, "el byte {i} cambió sin querer");
            }
        }
    }

    #[test]
    fn rechaza_reportes_mal_formados() {
        assert!(LedReport::from_bytes(&[0x0A, 1, 2]).is_err());
        let mut mal = vec![0u8; LED_REPORT_LEN];
        mal[0] = 0x0B;
        assert!(LedReport::from_bytes(&mal).is_err());
        assert!(LedReport::from_hex("zz").is_err());
    }

    #[test]
    fn efectos_por_nombre_y_numero() {
        assert_eq!(Effect::parse("fijo").unwrap(), Effect::Steady);
        assert_eq!(Effect::parse("Steady").unwrap(), Effect::Steady);
        assert_eq!(Effect::parse("12").unwrap(), Effect::Custom);
        assert_eq!(Effect::parse("0").unwrap(), Effect::Off);
        assert!(Effect::parse("13").is_err());
        assert!(Effect::parse("nada").is_err());
        for e in Effect::ALL {
            assert_eq!(Effect::parse(e.name()).unwrap(), e);
            assert_eq!(Effect::from_id(e.id()), Some(e));
        }
    }

    #[test]
    fn reporte_de_teclas_azul() {
        let r = keys_report(&POSITIONS_BLUE, 1).unwrap();
        assert_eq!(r.len(), KEYS_REPORT_LEN);
        assert_eq!((r[0], r[1]), (REPORT_KEYS, 1));
        let encendidas: Vec<usize> = (2..r.len()).filter(|&i| r[i] != 0).map(|i| i - 1).collect();
        assert_eq!(encendidas, POSITIONS_BLUE.iter().map(|&p| p as usize).collect::<Vec<_>>());
        assert!(r[2..].iter().all(|&b| b == 0 || b == KEY_ON));
    }

    #[test]
    fn reporte_de_todas_las_teclas() {
        let todas: Vec<u8> = POSITIONS_ALL.collect();
        let r = keys_report(&todas, 1).unwrap();
        assert_eq!(r[2..].iter().filter(|&&b| b == KEY_ON).count(), 125);
    }

    #[test]
    fn posiciones_fuera_de_rango() {
        assert!(keys_report(&[0], 1).is_err());
        assert!(keys_report(&[126], 1).is_err());
        assert!(keys_report(&[125], 1).is_ok());
        assert_eq!(parse_positions("66, 72,73").unwrap(), vec![66, 72, 73]);
        assert!(parse_positions("").is_err());
        assert!(parse_positions("1,x").is_err());
        assert!(parse_positions("200").is_err());
    }
}
