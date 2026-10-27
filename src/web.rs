//! Servidor HTTP local para controlar el teclado K671 Sindri desde el navegador web.
//! No requiere dependencias externas: usa `std::net::TcpListener` y sirve la carpeta `web/`.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use crate::device::Keyboard;
use crate::perfil_por_defecto;
use crate::protocol::{
    keys_report, Effect, LedReport, DEFAULT_BRIGHTNESS, DEFAULT_EFFECT, DEFAULT_GROUP,
    DEFAULT_SPEED, MAX_POSITION,
};
use crate::teclas;

const HTML_INDEX: &str = include_str!("../web/index.html");
const CSS_ESTILOS: &str = include_str!("../web/css/estilos.css");
const JS_DISPOSICION: &str = include_str!("../web/js/disposicion.js");
const JS_ANIMACIONES: &str = include_str!("../web/js/animaciones.js");
const JS_APP: &str = include_str!("../web/js/app.js");
const COLORES_POR_DEFECTO: &str = include_str!("../colores.json");

struct EstadoWeb {
    dispositivo_fijo: Option<PathBuf>,
    posiciones_activas: Vec<u8>,
    archivo_colores: PathBuf,
}

pub fn servir(dispositivo: Option<&Path>, puerto: u16, abrir_navegador: bool) -> Result<(), String> {
    let addr = format!("127.0.0.1:{puerto}");
    let listener = TcpListener::bind(&addr)
        .map_err(|e| format!("no se pudo abrir http://{addr}: {e}"))?;

    let url = format!("http://{addr}");
    println!("Panel web de sindri-led activo en: {url}");
    println!("Presiona Ctrl+C para detener el servidor.");

    if abrir_navegador {
        let _ = Command::new("xdg-open").arg(&url).spawn();
    }

    let mut estado = EstadoWeb {
        dispositivo_fijo: dispositivo.map(Path::to_path_buf),
        posiciones_activas: teclas::posiciones_mapeadas(),
        archivo_colores: ruta_archivo_colores(),
    };

    for stream in listener.incoming().flatten() {
        let _ = atender_conexion(stream, &mut estado);
    }
    Ok(())
}

fn ruta_archivo_colores() -> PathBuf {
    let local = PathBuf::from("colores.json");
    if local.exists() || Path::new("Cargo.toml").exists() {
        return local;
    }
    perfil_por_defecto().with_file_name("colores.json")
}

fn atender_conexion(mut stream: TcpStream, estado: &mut EstadoWeb) -> std::io::Result<()> {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));
    let mut buf = [0u8; 32768];
    let n = stream.read(&mut buf)?;
    if n == 0 {
        return Ok(());
    }
    let peticion = String::from_utf8_lossy(&buf[..n]);
    let primera_linea = peticion.lines().next().unwrap_or("");
    let mut partes = primera_linea.split_whitespace();
    let metodo = partes.next().unwrap_or("");
    let ruta = partes.next().unwrap_or("");
    let cuerpo = peticion
        .split_once("\r\n\r\n")
        .map(|(_, b)| b)
        .or_else(|| peticion.split_once("\n\n").map(|(_, b)| b))
        .unwrap_or("");

    match (metodo, ruta) {
        ("GET", "/") | ("GET", "/index.html") => {
            let body = leer_recurso_web("web/index.html", HTML_INDEX);
            responder(&mut stream, 200, "text/html; charset=utf-8", &body)
        }
        ("GET", "/css/estilos.css") => {
            let body = leer_recurso_web("web/css/estilos.css", CSS_ESTILOS);
            responder(&mut stream, 200, "text/css; charset=utf-8", &body)
        }
        ("GET", "/js/disposicion.js") => {
            let body = leer_recurso_web("web/js/disposicion.js", JS_DISPOSICION);
            responder(&mut stream, 200, "application/javascript; charset=utf-8", &body)
        }
        ("GET", "/js/animaciones.js") => {
            let body = leer_recurso_web("web/js/animaciones.js", JS_ANIMACIONES);
            responder(&mut stream, 200, "application/javascript; charset=utf-8", &body)
        }
        ("GET", "/js/app.js") => {
            let body = leer_recurso_web("web/js/app.js", JS_APP);
            responder(&mut stream, 200, "application/javascript; charset=utf-8", &body)
        }
        ("GET", "/api/estado") => {
            let (code, json) = api_estado(estado);
            responder(&mut stream, code, "application/json; charset=utf-8", &json)
        }
        ("GET", "/api/colores") => {
            let (code, json) = api_leer_colores(estado);
            responder(&mut stream, code, "application/json; charset=utf-8", &json)
        }
        ("POST", "/api/colores") => {
            let (code, json) = api_guardar_colores(estado, cuerpo);
            responder(&mut stream, code, "application/json; charset=utf-8", &json)
        }
        ("POST", "/api/efecto") => {
            let (code, json) = api_efecto(estado, cuerpo);
            responder(&mut stream, code, "application/json; charset=utf-8", &json)
        }
        ("POST", "/api/brillo") => {
            let (code, json) = api_brillo(estado, cuerpo);
            responder(&mut stream, code, "application/json; charset=utf-8", &json)
        }
        ("POST", "/api/velocidad") => {
            let (code, json) = api_velocidad(estado, cuerpo);
            responder(&mut stream, code, "application/json; charset=utf-8", &json)
        }
        ("POST", "/api/teclas") => {
            let (code, json) = api_teclas(estado, cuerpo);
            responder(&mut stream, code, "application/json; charset=utf-8", &json)
        }
        ("POST", "/api/restablecer") => {
            let (code, json) = api_restablecer(estado);
            responder(&mut stream, code, "application/json; charset=utf-8", &json)
        }
        ("POST", "/api/guardar") => {
            let (code, json) = api_guardar(estado);
            responder(&mut stream, code, "application/json; charset=utf-8", &json)
        }
        ("POST", "/api/cargar") => {
            let (code, json) = api_cargar(estado);
            responder(&mut stream, code, "application/json; charset=utf-8", &json)
        }
        _ => responder(
            &mut stream,
            404,
            "application/json; charset=utf-8",
            r#"{"error":"ruta no encontrada"}"#,
        ),
    }
}

fn leer_recurso_web(ruta_relativa: &str, incrustado: &str) -> String {
    std::fs::read_to_string(ruta_relativa).unwrap_or_else(|_| incrustado.to_string())
}

fn responder(stream: &mut TcpStream, code: u16, content_type: &str, body: &str) -> std::io::Result<()> {
    let status = match code {
        200 => "200 OK",
        400 => "400 Bad Request",
        404 => "404 Not Found",
        _ => "500 Internal Server Error",
    };
    let header = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\nCache-Control: no-store\r\n\r\n",
        body.len()
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body.as_bytes())?;
    stream.flush()
}

fn abrir(estado: &EstadoWeb) -> Result<Keyboard, String> {
    Keyboard::open(estado.dispositivo_fijo.as_deref())
}

fn api_leer_colores(estado: &EstadoWeb) -> (u16, String) {
    if let Ok(texto) = std::fs::read_to_string(&estado.archivo_colores) {
        let t = texto.trim();
        if t.starts_with('{') && t.ends_with('}') {
            return (200, t.to_string());
        }
    }
    (200, COLORES_POR_DEFECTO.trim().to_string())
}

fn api_guardar_colores(estado: &EstadoWeb, cuerpo: &str) -> (u16, String) {
    let limpio = cuerpo.trim();
    if !limpio.starts_with('{') || !limpio.ends_with('}') {
        return (400, json_error("JSON de colores inválido"));
    }
    if let Some(dir) = estado.archivo_colores.parent() {
        if !dir.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(dir);
        }
    }
    match std::fs::write(&estado.archivo_colores, format!("{limpio}\n")) {
        Ok(()) => (
            200,
            format!(
                r#"{{"ok":true,"archivo":"{}"}}"#,
                escapar_json(&estado.archivo_colores.display().to_string())
            ),
        ),
        Err(e) => (
            500,
            json_error(&format!(
                "no se pudo guardar {}: {e}",
                estado.archivo_colores.display()
            )),
        ),
    }
}

fn formatear_estado(kb: &Keyboard, r: &LedReport, activas: &[u8], msg: Option<&str>) -> String {
    let ef = Effect::from_id(r.effect_id());
    let nombre_efecto = ef.map(Effect::name).unwrap_or("desconocido");
    let admite_vel = ef.map(Effect::uses_speed).unwrap_or(true);
    let admite_bri = ef.map(Effect::uses_brightness).unwrap_or(true);
    let activas_vista: Vec<u8>;
    let lista_para_json = if r.effect_id() == Effect::Off.id() {
        &[][..]
    } else if r.effect_id() == Effect::Custom.id() {
        activas
    } else {
        activas_vista = teclas::posiciones_mapeadas();
        &activas_vista[..]
    };
    let lista_pos = lista_para_json
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(",");
    let extra_msg = match msg {
        Some(m) => format!(r#","mensaje":"{}""#, escapar_json(m)),
        None => String::new(),
    };
    format!(
        r#"{{"conectado":true,"dispositivo":"{}","efecto_id":{},"efecto_nombre":"{}","admite_velocidad":{},"admite_brillo":{},"grupo":{},"velocidad":{},"brillo":{},"posiciones_activas":[{}]{}}}"#,
        escapar_json(&kb.path.display().to_string()),
        r.effect_id(),
        nombre_efecto,
        admite_vel,
        admite_bri,
        r.group(),
        r.speed(),
        r.brightness(),
        lista_pos,
        extra_msg
    )
}

fn json_error(msg: &str) -> String {
    format!(r#"{{"conectado":false,"error":"{}"}}"#, escapar_json(msg))
}

fn api_estado(estado: &EstadoWeb) -> (u16, String) {
    let kb = match abrir(estado) {
        Ok(k) => k,
        Err(e) => return (200, json_error(&e)),
    };
    match kb.get_led() {
        Ok(r) => (200, formatear_estado(&kb, &r, &estado.posiciones_activas, None)),
        Err(e) => (500, json_error(&format!("no se pudo leer el reporte 0x0A: {e}"))),
    }
}

fn modificar_0x0a(
    estado: &EstadoWeb,
    f: impl FnOnce(&mut LedReport),
    msg: Option<&str>,
) -> (u16, String) {
    let kb = match abrir(estado) {
        Ok(k) => k,
        Err(e) => return (500, json_error(&e)),
    };
    let mut r = match kb.get_led() {
        Ok(r) => r,
        Err(e) => return (500, json_error(&format!("no se pudo leer 0x0A: {e}"))),
    };
    f(&mut r);
    if let Err(e) = kb.set_led(&r) {
        return (500, json_error(&format!("no se pudo escribir 0x0A: {e}")));
    }
    sleep(Duration::from_millis(120));
    let actual = kb.get_led().unwrap_or(r);
    (200, formatear_estado(&kb, &actual, &estado.posiciones_activas, msg))
}

fn api_efecto(estado: &mut EstadoWeb, cuerpo: &str) -> (u16, String) {
    let Some(valor) = extraer_string_json(cuerpo, "efecto") else {
        return (400, json_error("falta el campo 'efecto'"));
    };
    let efecto = match Effect::parse(&valor) {
        Ok(e) => e,
        Err(e) => return (400, json_error(&e)),
    };
    modificar_0x0a(estado, |r| r.set_effect(efecto), None)
}

fn api_brillo(estado: &EstadoWeb, cuerpo: &str) -> (u16, String) {
    let Some(nivel) = extraer_u8_json(cuerpo, "nivel") else {
        return (400, json_error("falta el campo 'nivel'"));
    };
    if !(1..=5).contains(&nivel) {
        return (400, json_error("el brillo debe estar entre 1 y 5"));
    }
    modificar_0x0a(estado, |r| r.set_brightness(nivel), None)
}

fn api_velocidad(estado: &EstadoWeb, cuerpo: &str) -> (u16, String) {
    let Some(nivel) = extraer_u8_json(cuerpo, "nivel") else {
        return (400, json_error("falta el campo 'nivel'"));
    };
    if !(1..=5).contains(&nivel) {
        return (400, json_error("la velocidad debe estar entre 1 y 5"));
    }
    modificar_0x0a(estado, |r| r.set_speed(nivel), None)
}

fn api_teclas(estado: &mut EstadoWeb, cuerpo: &str) -> (u16, String) {
    let posiciones = match extraer_array_u8_json(cuerpo, "posiciones") {
        Ok(p) => p,
        Err(e) => return (400, json_error(&e)),
    };
    let reporte = match keys_report(&posiciones, DEFAULT_GROUP) {
        Ok(r) => r,
        Err(e) => return (400, json_error(&e)),
    };

    let kb = match abrir(estado) {
        Ok(k) => k,
        Err(e) => return (500, json_error(&e)),
    };
    let mut r = match kb.get_led() {
        Ok(r) => r,
        Err(e) => return (500, json_error(&format!("no se pudo leer 0x0A: {e}"))),
    };

    if r.effect_id() != Effect::Custom.id() {
        r.set_effect(Effect::Custom);
        if let Err(e) = kb.set_led(&r) {
            return (500, json_error(&format!("no se pudo activar efecto personalizado: {e}")));
        }
        sleep(Duration::from_millis(250));
    }

    for i in 0..2 {
        if let Err(e) = kb.set_keys(&reporte) {
            return (500, json_error(&format!("no se pudo escribir 0x0B: {e}")));
        }
        if i == 0 {
            sleep(Duration::from_millis(200));
        }
    }

    estado.posiciones_activas = posiciones;
    (200, formatear_estado(&kb, &r, &estado.posiciones_activas, None))
}

fn api_restablecer(estado: &mut EstadoWeb) -> (u16, String) {
    estado.posiciones_activas = teclas::posiciones_mapeadas();
    modificar_0x0a(
        estado,
        |r| {
            r.set_effect(DEFAULT_EFFECT);
            r.set_group(DEFAULT_GROUP);
            r.set_speed(DEFAULT_SPEED);
            r.set_brightness(DEFAULT_BRIGHTNESS);
        },
        Some("Valores de fábrica restaurados"),
    )
}

fn api_guardar(estado: &EstadoWeb) -> (u16, String) {
    let kb = match abrir(estado) {
        Ok(k) => k,
        Err(e) => return (500, json_error(&e)),
    };
    let r = match kb.get_led() {
        Ok(r) => r,
        Err(e) => return (500, json_error(&format!("no se pudo leer 0x0A: {e}"))),
    };
    let ruta = perfil_por_defecto();
    if let Some(dir) = ruta.parent() {
        if let Err(e) = std::fs::create_dir_all(dir) {
            return (500, json_error(&format!("no se pudo crear {}: {e}", dir.display())));
        }
    }
    if let Err(e) = std::fs::write(&ruta, r.to_hex() + "\n") {
        return (500, json_error(&format!("no se pudo escribir {}: {e}", ruta.display())));
    }
    let msg = format!("Perfil guardado en {}", ruta.display());
    (200, formatear_estado(&kb, &r, &estado.posiciones_activas, Some(&msg)))
}

fn api_cargar(estado: &EstadoWeb) -> (u16, String) {
    let ruta = perfil_por_defecto();
    let texto = match std::fs::read_to_string(&ruta) {
        Ok(t) => t,
        Err(e) => return (400, json_error(&format!("no se pudo leer {}: {e}", ruta.display()))),
    };
    let r = match LedReport::from_hex(&texto) {
        Ok(r) => r,
        Err(e) => return (400, json_error(&e)),
    };
    let kb = match abrir(estado) {
        Ok(k) => k,
        Err(e) => return (500, json_error(&e)),
    };
    if let Err(e) = kb.set_led(&r) {
        return (500, json_error(&format!("no se pudo escribir 0x0A: {e}")));
    }
    sleep(Duration::from_millis(120));
    let actual = kb.get_led().unwrap_or(r);
    let msg = format!("Perfil cargado desde {}", ruta.display());
    (200, formatear_estado(&kb, &actual, &estado.posiciones_activas, Some(&msg)))
}

fn escapar_json(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn extraer_string_json(json: &str, clave: &str) -> Option<String> {
    let patron = format!("\"{clave}\"");
    let idx = json.find(&patron)?;
    let resto = &json[idx + patron.len()..];
    let dos_puntos = resto.find(':')?;
    let valor = resto[dos_puntos + 1..].trim_start();
    if let Some(sin_comilla) = valor.strip_prefix('"') {
        let fin = sin_comilla.find('"')?;
        Some(sin_comilla[..fin].to_string())
    } else {
        None
    }
}

fn extraer_u8_json(json: &str, clave: &str) -> Option<u8> {
    let patron = format!("\"{clave}\"");
    let idx = json.find(&patron)?;
    let resto = &json[idx + patron.len()..];
    let dos_puntos = resto.find(':')?;
    let valor = resto[dos_puntos + 1..].trim_start();
    let digitos: String = valor.chars().take_while(|c| c.is_ascii_digit()).collect();
    digitos.parse().ok()
}

fn extraer_array_u8_json(json: &str, clave: &str) -> Result<Vec<u8>, String> {
    let patron = format!("\"{clave}\"");
    let idx = json.find(&patron).ok_or("falta el campo de posiciones")?;
    let resto = &json[idx + patron.len()..];
    let ini = resto.find('[').ok_or("se esperaba una lista '['")?;
    let fin = resto[ini..].find(']').ok_or("lista sin cerrar ']'")?;
    let contenido = &resto[ini + 1..ini + fin];
    let mut v = Vec::new();
    for tok in contenido.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let p: u8 = tok.parse().map_err(|_| format!("posición inválida: '{tok}'"))?;
        if p == 0 || p > MAX_POSITION {
            return Err(format!("posición {p} fuera de rango (1 a {MAX_POSITION})"));
        }
        v.push(p);
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extrae_campos_json_simples() {
        assert_eq!(
            extraer_string_json(r#"{"efecto": "respira"}"#, "efecto").as_deref(),
            Some("respira")
        );
        assert_eq!(extraer_u8_json(r#"{"nivel": 4}"#, "nivel"), Some(4));
        assert_eq!(
            extraer_array_u8_json(r#"{"posiciones": [7, 13, 125]}"#, "posiciones").unwrap(),
            vec![7, 13, 125]
        );
        assert_eq!(
            extraer_array_u8_json(r#"{"posiciones": []}"#, "posiciones").unwrap(),
            Vec::<u8>::new()
        );
        assert!(extraer_array_u8_json(r#"{"posiciones": [200]}"#, "posiciones").is_err());
    }
}
