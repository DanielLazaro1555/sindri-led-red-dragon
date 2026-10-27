//! sindri-led: control de luces LED del K671 Sindri en Linux. No oficial.

mod device;
mod protocol;
mod teclas;
mod web;

use std::path::PathBuf;
use std::process::ExitCode;
use std::thread::sleep;
use std::time::Duration;

use clap::{Parser, Subcommand};

use device::Keyboard;
use protocol::{
    keys_report, Effect, LedReport, DEFAULT_BRIGHTNESS, DEFAULT_EFFECT, DEFAULT_GROUP,
    DEFAULT_SPEED, POSITIONS_ALL, POSITIONS_BLUE,
};

#[derive(Parser)]
#[command(
    name = "sindri-led",
    version,
    about = "Control de luces LED del teclado K671 Sindri en Linux. Herramienta no oficial.",
    after_help = "Probado solo en el teclado USB 258a:013b. Los LED son de color fijo por zona: \
                  las teclas se encienden o se apagan, pero no se les puede elegir el color."
)]
struct Cli {
    /// Ruta del hidraw a usar (por defecto se detecta sola), por ejemplo /dev/hidraw6.
    #[arg(long, global = true, value_name = "RUTA")]
    dispositivo: Option<PathBuf>,

    #[command(subcommand)]
    orden: Orden,
}

#[derive(Subcommand)]
enum Orden {
    /// Muestra el efecto, grupo, velocidad y brillo actuales.
    Estado,
    /// Cambia el efecto de luz: un número 0-12 o un nombre (fijo, respira, personalizado, apagado...).
    Efecto { efecto: String },
    /// Cambia el brillo (1 a 5).
    Brillo {
        #[arg(value_parser = clap::value_parser!(u8).range(1..=5))]
        nivel: u8,
    },
    /// Cambia la velocidad del efecto (1 a 5). Sin probar en un teclado real.
    Velocidad {
        #[arg(value_parser = clap::value_parser!(u8).range(1..=5))]
        nivel: u8,
    },
    /// Enciende solo las teclas indicadas (efecto personalizado): `todas`, `ninguna`, `azul` o una
    /// lista de nombres como `esc,f1,a` (ver `mapa`). Una posición suelta se escribe `p66`.
    Teclas { seleccion: String },
    /// Muestra los nombres de tecla y su posición en el reporte.
    Mapa,
    /// Atajo: enciende solo la franja azul del teclado.
    Azul,
    /// Atajo: enciende todas las teclas.
    Todas,
    /// Guarda el perfil actual (reporte 0x0A) en un archivo, para poder volver a él.
    Guardar { archivo: Option<PathBuf> },
    /// Carga un perfil guardado con `guardar`.
    Cargar { archivo: Option<PathBuf> },
    /// Vuelve a los valores de fábrica observados: efecto fijo, grupo 1, velocidad 3, brillo 3.
    Restablecer,
    /// Abre el panel de control interactivo en el navegador web (por defecto http://127.0.0.1:8421).
    Web {
        /// Puerto local donde escuchar.
        #[arg(long, default_value_t = 8421)]
        puerto: u16,
        /// No abrir el navegador automáticamente al iniciar.
        #[arg(long)]
        sin_abrir: bool,
    },
}

fn main() -> ExitCode {
    // Rust ignora SIGPIPE por defecto y `println!` entraría en pánico con `sindri-led mapa | head`.
    // SAFETY: restaurar la acción por defecto de SIGPIPE al inicio, antes de crear hilos, es seguro.
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    let cli = Cli::parse();
    match ejecutar(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn ejecutar(cli: Cli) -> Result<(), String> {
    match cli.orden {
        Orden::Mapa => {
            mapa();
            return Ok(());
        }
        Orden::Web { puerto, sin_abrir } => {
            return web::servir(cli.dispositivo.as_deref(), puerto, !sin_abrir);
        }
        _ => {}
    }
    let kb = Keyboard::open(cli.dispositivo.as_deref())?;
    match cli.orden {
        Orden::Estado => {
            println!("{}", leer(&kb)?);
            println!("dispositivo: {}", kb.path.display());
        }
        Orden::Efecto { efecto } => {
            let e = Effect::parse(&efecto)?;
            cambiar(&kb, |r| r.set_effect(e))?;
        }
        Orden::Brillo { nivel } => cambiar(&kb, |r| r.set_brightness(nivel))?,
        Orden::Velocidad { nivel } => cambiar(&kb, |r| r.set_speed(nivel))?,
        Orden::Teclas { seleccion } => teclas(&kb, &seleccion)?,
        Orden::Mapa | Orden::Web { .. } => unreachable!("se atiende antes de abrir el teclado"),
        Orden::Azul => teclas(&kb, "azul")?,
        Orden::Todas => teclas(&kb, "todas")?,
        Orden::Guardar { archivo } => {
            let ruta = archivo.unwrap_or_else(perfil_por_defecto);
            if let Some(dir) = ruta.parent() {
                std::fs::create_dir_all(dir).map_err(|e| format!("no se pudo crear {}: {e}", dir.display()))?;
            }
            let r = leer(&kb)?;
            std::fs::write(&ruta, r.to_hex() + "\n")
                .map_err(|e| format!("no se pudo escribir {}: {e}", ruta.display()))?;
            println!("perfil guardado en {}", ruta.display());
        }
        Orden::Cargar { archivo } => {
            let ruta = archivo.unwrap_or_else(perfil_por_defecto);
            let texto = std::fs::read_to_string(&ruta).map_err(|e| format!("no se pudo leer {}: {e}", ruta.display()))?;
            escribir(&kb, &LedReport::from_hex(&texto)?)?;
        }
        Orden::Restablecer => cambiar(&kb, |r| {
            r.set_effect(DEFAULT_EFFECT);
            r.set_group(DEFAULT_GROUP);
            r.set_speed(DEFAULT_SPEED);
            r.set_brightness(DEFAULT_BRIGHTNESS);
        })?,
    }
    Ok(())
}

fn leer(kb: &Keyboard) -> Result<LedReport, String> {
    kb.get_led().map_err(|e| format!("no se pudo leer el reporte 0x0A: {e}"))
}

/// Lee el reporte, aplica el cambio y lo escribe de vuelta (los demás bytes se conservan).
fn cambiar(kb: &Keyboard, f: impl FnOnce(&mut LedReport)) -> Result<(), String> {
    let mut r = leer(kb)?;
    f(&mut r);
    escribir(kb, &r)
}

/// Escribe el reporte, lo vuelve a leer y avisa si el teclado no guardó lo mismo.
fn escribir(kb: &Keyboard, r: &LedReport) -> Result<(), String> {
    kb.set_led(r).map_err(|e| format!("no se pudo escribir el reporte 0x0A: {e}"))?;
    sleep(Duration::from_millis(150));
    let ahora = leer(kb)?;
    if &ahora != r {
        eprintln!("aviso: el teclado guardó un reporte distinto al enviado");
    }
    println!("{ahora}");
    Ok(())
}

/// Pone el efecto personalizado y manda el reporte 0x0B. Este reporte es de solo escritura:
/// el teclado no permite leer los valores anteriores.
fn teclas(kb: &Keyboard, seleccion: &str) -> Result<(), String> {
    let posiciones: Vec<u8> = match seleccion.trim().to_lowercase().as_str() {
        "todas" | "all" => POSITIONS_ALL.collect(),
        "ninguna" | "none" => Vec::new(),
        "azul" | "blue" => POSITIONS_BLUE.to_vec(),
        "mapeadas" => teclas::posiciones_mapeadas(),
        "huecos" => teclas::posiciones_sin_tecla(),
        lista => teclas::parse_lista(lista)?,
    };
    let reporte = keys_report(&posiciones, DEFAULT_GROUP)?;

    let mut r = leer(kb)?;
    r.set_effect(Effect::Custom);
    kb.set_led(&r).map_err(|e| format!("no se pudo poner el efecto personalizado: {e}"))?;
    sleep(Duration::from_millis(300));
    // El teclado a veces ignora el primer envío, por eso se manda dos veces (como en las pruebas).
    for _ in 0..2 {
        kb.set_keys(&reporte).map_err(|e| format!("no se pudo escribir el reporte 0x0B: {e}"))?;
        sleep(Duration::from_millis(500));
    }
    println!("{} teclas encendidas (efecto personalizado)", posiciones.len());
    Ok(())
}

/// Imprime la tabla de teclas: nombre, fila, columna y posición en el reporte 0x0B.
fn mapa() {
    println!("{:<16} {:>4} {:>7} {:>8}", "tecla", "fila", "columna", "posición");
    for t in &teclas::TECLAS {
        println!("{:<16} {:>4} {:>7} {:>8}", t.nombre, t.fila, t.columna, t.posicion());
    }
}

pub(crate) fn perfil_por_defecto() -> PathBuf {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("sindri-led").join("perfil.hex")
}
