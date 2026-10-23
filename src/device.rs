//! Acceso al teclado por `/dev/hidraw*` con los `ioctl` HIDIOCSFEATURE / HIDIOCGFEATURE.

use std::fs::{self, File, OpenOptions};
use std::io;
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};

use crate::protocol::{LedReport, KEYS_REPORT_LEN, LED_REPORT_LEN, REPORT_LED};

/// `HID_ID` del teclado en el `uevent` de sysfs: bus 0003 (USB), vendor 258A, producto 013B.
const HID_ID: &str = "0000258A:0000013B";

const HIDIOCSFEATURE: u8 = 0x06;
const HIDIOCGFEATURE: u8 = 0x07;

/// Calcula el número de `ioctl` HIDIOC{S,G}FEATURE(len), igual que las macros de `linux/hidraw.h`.
fn hid_request(nr: u8, len: usize) -> libc::c_ulong {
    const IOC_READ_WRITE: libc::c_ulong = 3;
    (IOC_READ_WRITE << 30) | ((len as libc::c_ulong) << 16) | (0x48 << 8) | nr as libc::c_ulong
}

pub struct Keyboard {
    file: File,
    pub path: PathBuf,
}

impl Keyboard {
    /// Abre el teclado. Sin ruta, busca entre los `hidraw` del dispositivo `258a:013b`
    /// el que responde al reporte `0x0A` (el teclado tiene dos interfaces y solo una lo hace).
    pub fn open(path: Option<&Path>) -> Result<Keyboard, String> {
        if let Some(p) = path {
            return Keyboard::open_path(p).map_err(|e| describir(p, &e));
        }
        let candidatos = candidatos();
        if candidatos.is_empty() {
            return Err("no se encontró el teclado (USB 258a:013b). ¿Está conectado?".into());
        }
        let mut sin_permiso = None;
        for p in &candidatos {
            match Keyboard::open_path(p) {
                Ok(kb) => {
                    if kb.get_led().is_ok() {
                        return Ok(kb);
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::PermissionDenied => sin_permiso = Some(p.clone()),
                Err(_) => {}
            }
        }
        match sin_permiso {
            Some(p) => Err(describir(&p, &io::Error::from(io::ErrorKind::PermissionDenied))),
            None => Err("se encontró el teclado pero ninguna interfaz respondió al reporte 0x0A".into()),
        }
    }

    fn open_path(path: &Path) -> io::Result<Keyboard> {
        let file = OpenOptions::new().read(true).write(true).open(path)?;
        Ok(Keyboard { file, path: path.to_path_buf() })
    }

    fn get_feature(&self, buf: &mut [u8]) -> io::Result<()> {
        let req = hid_request(HIDIOCGFEATURE, buf.len());
        // SAFETY: `buf` es un búfer válido de `buf.len()` bytes y el ioctl no escribe más que eso.
        let r = unsafe { libc::ioctl(self.file.as_raw_fd(), req as _, buf.as_mut_ptr()) };
        if r < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    fn set_feature(&self, buf: &[u8]) -> io::Result<()> {
        let req = hid_request(HIDIOCSFEATURE, buf.len());
        // SAFETY: `buf` es un búfer válido de `buf.len()` bytes y el ioctl solo lo lee.
        let r = unsafe { libc::ioctl(self.file.as_raw_fd(), req as _, buf.as_ptr()) };
        if r < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub fn get_led(&self) -> io::Result<LedReport> {
        let mut buf = [0u8; LED_REPORT_LEN];
        buf[0] = REPORT_LED;
        self.get_feature(&mut buf)?;
        LedReport::from_bytes(&buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn set_led(&self, report: &LedReport) -> io::Result<()> {
        self.set_feature(report.as_bytes())
    }

    pub fn set_keys(&self, report: &[u8; KEYS_REPORT_LEN]) -> io::Result<()> {
        self.set_feature(report)
    }
}

/// Rutas `/dev/hidrawN` cuyo `HID_ID` coincide con el teclado.
fn candidatos() -> Vec<PathBuf> {
    let mut v = Vec::new();
    let Ok(dir) = fs::read_dir("/sys/class/hidraw") else {
        return v;
    };
    for entrada in dir.flatten() {
        let uevent = entrada.path().join("device/uevent");
        let Ok(texto) = fs::read_to_string(uevent) else {
            continue;
        };
        let coincide = texto
            .lines()
            .any(|l| l.starts_with("HID_ID=") && l.to_uppercase().ends_with(HID_ID));
        if coincide {
            v.push(Path::new("/dev").join(entrada.file_name()));
        }
    }
    v.sort();
    v
}

fn describir(path: &Path, e: &io::Error) -> String {
    if e.kind() == io::ErrorKind::PermissionDenied {
        format!(
            "sin permiso para abrir {}. Instala la regla udev (scripts/arreglar-permisos-teclado.sh) \
             y vuelve a conectar el teclado",
            path.display()
        )
    } else {
        format!("no se pudo abrir {}: {e}", path.display())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeros_de_ioctl_como_en_linux_hidraw_h() {
        // Valores calculados con las macros _IOC de linux/hidraw.h y comprobados en el prototipo.
        assert_eq!(hid_request(HIDIOCGFEATURE, 42), 0xC02A_4807);
        assert_eq!(hid_request(HIDIOCSFEATURE, 42), 0xC02A_4806);
        assert_eq!(hid_request(HIDIOCSFEATURE, 127), 0xC07F_4806);
    }
}
