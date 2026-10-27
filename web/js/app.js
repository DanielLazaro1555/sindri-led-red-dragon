// Lógica principal de la interfaz web: estado, eventos del teclado, edición de colores y API REST.

let encendidas = new Set();
let todasLasPosiciones = [];
let elementosTecla = []; // [{ el, pos, nombre, fila, col }]
let rutaSerpiente = [];
let zonasPorPosicion = {};
let paletaActual = { ...PALETA_POR_DEFECTO };

// Estado vivo para el motor de animación a 60 FPS
let efectoActualId = 1;
let efectoActualNombre = "fijo";
let brilloActual = 3;      // 1..5
let velocidadActual = 3;   // 1..5
let escalaAnimacion = 1.0;
let tInicioEfecto = performance.now();
let destellosReaccion = {}; // pos -> timestamp
let ondasActivas = [];      // [{ fila, col, t0 }]

let modoInteraccion = "control"; // "control" | "color"
let pincelZona = "yellow";
let arrastrando = false;
let modoArrastreEncender = true;
let timerEnvio = null;
let timerGuardarColores = null;
let toastTimer = null;

function crearElementoTecla([nombre, label, pos, extraClass]) {
  const zona = zonaPorDefecto(pos, nombre);
  zonasPorPosicion[pos] = zona;
  const fila = (pos - 1) % 6;
  const col = Math.floor((pos - 1) / 6);

  const el = document.createElement("div");
  el.className = "key" + (extraClass ? " " + extraClass : "");
  el.dataset.pos = pos;
  el.dataset.name = nombre;
  el.dataset.zone = zona;
  el.title = `${nombre} (posición ${pos}, fila ${fila}, col ${col})`;
  el.innerHTML = `<span>${label}</span><span class="sub">p${pos}</span>`;

  el.addEventListener("mousedown", (e) => {
    e.preventDefault();
    arrastrando = true;
    registrarPulsoInteractivo(pos, fila, col);
    if (modoInteraccion === "color") {
      pintarColorTecla(pos, el);
    } else {
      modoArrastreEncender = !encendidas.has(pos);
      aplicarTeclaLocal(pos, modoArrastreEncender);
      programarEnvioTeclas();
    }
  });

  el.addEventListener("mouseenter", () => {
    if (!arrastrando) return;
    registrarPulsoInteractivo(pos, fila, col);
    if (modoInteraccion === "color") {
      pintarColorTecla(pos, el);
    } else {
      aplicarTeclaLocal(pos, modoArrastreEncender);
      programarEnvioTeclas();
    }
  });

  todasLasPosiciones.push(pos);
  encendidas.add(pos);
  elementosTecla.push({ el, pos, nombre, fila, col });
  return el;
}

window.addEventListener("mouseup", () => {
  arrastrando = false;
});

// Al presionar teclas en el teclado físico, dispara destello en modo Reacción (3) y Ondas (9)
window.addEventListener("keydown", (e) => {
  const k = e.key.toLowerCase();
  const encontrada = elementosTecla.find(
    (item) =>
      item.nombre === k ||
      (k === " " && item.nombre === "espacio") ||
      (k === "enter" && item.nombre === "enter") ||
      (k === "escape" && item.nombre === "esc")
  );
  if (encontrada) {
    registrarPulsoInteractivo(encontrada.pos, encontrada.fila, encontrada.col);
  }
});

function cambiarModoInteraccion(modo) {
  modoInteraccion = modo;
  document.getElementById("tabControl").classList.toggle("active", modo === "control");
  document.getElementById("tabEditarColor").classList.toggle("active", modo === "color");
  document.getElementById("paintToolbar").classList.toggle("visible", modo === "color");
  document.getElementById("keyboardCase").classList.toggle("edit-mode", modo === "color");
  document.getElementById("subHeaderMsg").textContent =
    modo === "color"
      ? "Modo Editar Colores: elige un color arriba y haz clic o arrastra sobre las teclas para registrar su color real"
      : "Modo Control: haz clic o arrastra sobre las teclas para encenderlas o apagarlas";
}

function seleccionarPincel(zona) {
  pincelZona = zona;
  document.querySelectorAll(".brush-item").forEach((b) => {
    b.classList.toggle("active", b.dataset.zone === zona);
  });
}

function pintarColorTecla(pos, el) {
  zonasPorPosicion[pos] = pincelZona;
  el.dataset.zone = pincelZona;
  programarGuardarColores();

  if (document.getElementById("chkProbarEnTeclado").checked) {
    encendidas = new Set([pos]);
    actualizarContadorTeclas();
    programarEnvioTeclas();
  }
}

function cambiarTonoPaleta(zona, hex) {
  paletaActual[zona] = hex;
  document.documentElement.style.setProperty(`--c-${zona}`, hex);
  programarGuardarColores();
}

function aplicarPaletaADOM() {
  for (const [zona, hex] of Object.entries(paletaActual)) {
    document.documentElement.style.setProperty(`--c-${zona}`, hex);
    const picker = document.getElementById(`picker-${zona}`);
    if (picker) picker.value = hex;
  }
}

function aplicarZonasADOM() {
  for (const item of elementosTecla) {
    if (zonasPorPosicion[item.pos]) {
      item.el.dataset.zone = zonasPorPosicion[item.pos];
    }
  }
}

function programarGuardarColores() {
  if (timerGuardarColores) clearTimeout(timerGuardarColores);
  timerGuardarColores = setTimeout(guardarColoresEnServidor, 250);
}

async function guardarColoresEnServidor() {
  try {
    const payload = {
      zonas: zonasPorPosicion,
      paleta: paletaActual
    };
    const res = await fetch("/api/colores", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload)
    });
    if (res.ok) {
      mostrarToast("Colores registrados guardados en colores.json");
    }
  } catch (e) {
    mostrarToast("No se pudo guardar colores.json", true);
  }
}

async function cargarColoresRegistrados() {
  try {
    const res = await fetch("/api/colores");
    if (!res.ok) return;
    const data = await res.json();
    if (data.paleta && typeof data.paleta === "object") {
      paletaActual = { ...PALETA_POR_DEFECTO, ...data.paleta };
      aplicarPaletaADOM();
    }
    if (data.zonas && typeof data.zonas === "object") {
      for (const [posStr, zona] of Object.entries(data.zonas)) {
        zonasPorPosicion[Number(posStr)] = zona;
      }
      aplicarZonasADOM();
    }
  } catch (_) {}
}

function restaurarColoresPorDefecto() {
  paletaActual = { ...PALETA_POR_DEFECTO };
  aplicarPaletaADOM();
  for (const item of elementosTecla) {
    const z = zonaPorDefecto(item.pos, item.nombre);
    zonasPorPosicion[item.pos] = z;
    item.el.dataset.zone = z;
  }
  programarGuardarColores();
}

function construirTeclado() {
  const mainBlock = document.getElementById("mainBlock");
  for (const row of MAIN_ROWS) {
    const r = document.createElement("div");
    r.className = row.rowClass;
    for (const k of row.keys) {
      r.appendChild(crearElementoTecla(k));
    }
    mainBlock.appendChild(r);
  }

  const navBlock = document.getElementById("navBlock");
  for (const row of NAV_ROWS) {
    const r = document.createElement("div");
    r.className = row.rowClass;
    for (const k of row.keys) {
      if (!k) {
        const sp = document.createElement("div");
        sp.className = "spacer-key";
        r.appendChild(sp);
      } else {
        r.appendChild(crearElementoTecla(k));
      }
    }
    navBlock.appendChild(r);
  }

  const numBlock = document.getElementById("numBlock");
  for (const k of NUMPAD_KEYS) {
    numBlock.appendChild(crearElementoTecla(k));
  }

  // Construir orden en zig-zag por filas para el efecto Serpiente (7)
  for (let f = 0; f < 6; f++) {
    const deFila = elementosTecla.filter((it) => it.fila === f).sort((a, b) => a.col - b.col);
    if (f % 2 === 1) deFila.reverse();
    for (const it of deFila) rutaSerpiente.push(it.pos);
  }

  const effContainer = document.getElementById("effectsContainer");
  for (const ef of EFECTOS) {
    const b = document.createElement("button");
    b.className = "btn" + (ef.id === 0 ? " danger" : "");
    b.dataset.effectId = ef.id;
    b.textContent = `${ef.label}`;
    b.onclick = () => cambiarEfecto(ef.name);
    effContainer.appendChild(b);
  }

  actualizarContadorTeclas();
}

function aplicarTeclaLocal(pos, encender) {
  efectoActualId = 12;
  efectoActualNombre = "personalizado";
  if (encender) encendidas.add(pos);
  else encendidas.delete(pos);
  actualizarContadorTeclas();
}

function actualizarContadorTeclas() {
  document.getElementById("litCount").textContent = encendidas.size;
}

function programarEnvioTeclas() {
  if (timerEnvio) clearTimeout(timerEnvio);
  timerEnvio = setTimeout(() => {
    enviarTeclasAlTeclado(Array.from(encendidas));
  }, 180);
}

async function enviarTeclasAlTeclado(listaPosiciones) {
  try {
    const res = await fetch("/api/teclas", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ posiciones: listaPosiciones })
    });
    const data = await res.json();
    if (!res.ok) throw new Error(data.error || "Error al escribir teclas");
    actualizarEstadoUI(data, false);
    mostrarToast(`${encendidas.size} teclas aplicadas en el teclado`);
  } catch (e) {
    mostrarToast("Error: " + e.message, true);
  }
}

function seleccionarPreset(tipo) {
  efectoActualId = 12;
  efectoActualNombre = "personalizado";
  if (tipo === "todas") {
    encendidas = new Set(todasLasPosiciones);
  } else if (tipo === "ninguna") {
    encendidas.clear();
  } else if (tipo === "wasd") {
    encendidas = new Set([7, 15, 10, 16, 22, 36, 95, 90, 96, 102]);
  } else if (tipo === "invertir") {
    const nuevo = new Set();
    for (const p of todasLasPosiciones) {
      if (!encendidas.has(p)) nuevo.add(p);
    }
    encendidas = nuevo;
  }
  actualizarContadorTeclas();
  programarEnvioTeclas();
}

function filtrarPorColor(zona, ev) {
  efectoActualId = 12;
  efectoActualNombre = "personalizado";
  const posZona = [];
  for (const item of elementosTecla) {
    if (item.el.dataset.zone === zona) posZona.push(item.pos);
  }

  if (ev && ev.shiftKey) {
    const todasEncendidas = posZona.every((p) => encendidas.has(p));
    for (const p of posZona) {
      if (todasEncendidas) encendidas.delete(p);
      else encendidas.add(p);
    }
  } else {
    encendidas = new Set(posZona);
  }
  actualizarContadorTeclas();
  programarEnvioTeclas();
}

async function cambiarEfecto(nombre) {
  try {
    const res = await fetch("/api/efecto", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ efecto: nombre })
    });
    const data = await res.json();
    if (!res.ok) throw new Error(data.error || "No se pudo cambiar el efecto");
    tInicioEfecto = performance.now();
    actualizarEstadoUI(data, true);
    mostrarToast(`Efecto cambiado a: ${data.efecto_nombre}`);
  } catch (e) {
    mostrarToast("Error: " + e.message, true);
  }
}

async function cambiarBrillo(nivel) {
  brilloActual = nivel;
  try {
    const res = await fetch("/api/brillo", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ nivel })
    });
    const data = await res.json();
    if (!res.ok) throw new Error(data.error || "No se pudo cambiar el brillo");
    actualizarEstadoUI(data, false);
    mostrarToast(`Brillo cambiado a nivel ${nivel}`);
  } catch (e) {
    mostrarToast("Error: " + e.message, true);
  }
}

async function cambiarVelocidad(nivel) {
  velocidadActual = nivel;
  tInicioEfecto = performance.now();
  try {
    const res = await fetch("/api/velocidad", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ nivel })
    });
    const data = await res.json();
    if (!res.ok) throw new Error(data.error || "No se pudo cambiar la velocidad");
    actualizarEstadoUI(data, false);
    mostrarToast(`Velocidad cambiada a nivel ${nivel}`);
  } catch (e) {
    mostrarToast("Error: " + e.message, true);
  }
}

async function accionPerfil(accion) {
  try {
    const res = await fetch(`/api/${accion}`, { method: "POST" });
    const data = await res.json();
    if (!res.ok) throw new Error(data.error || "Falló la operación");
    tInicioEfecto = performance.now();
    if (data.efecto_id !== undefined) actualizarEstadoUI(data, true);
    mostrarToast(data.mensaje || `Acción '${accion}' completada`);
  } catch (e) {
    mostrarToast("Error: " + e.message, true);
  }
}

function actualizarEstadoUI(data, sincronizarTeclas) {
  const dot = document.getElementById("statusDot");
  const txt = document.getElementById("statusText");

  if (data.conectado) {
    dot.classList.remove("err");
    txt.textContent = `Conectado en ${data.dispositivo}`;
  } else {
    dot.classList.add("err");
    txt.textContent = data.error || "Teclado no detectado";
    return;
  }

  efectoActualId = data.efecto_id;
  efectoActualNombre = data.efecto_nombre;
  brilloActual = data.brillo;
  velocidadActual = data.velocidad;

  document.getElementById("currentEffectHint").textContent =
    `Actual: ${data.efecto_nombre} (${data.efecto_id})`;

  document.querySelectorAll("#effectsContainer button").forEach((b) => {
    b.classList.toggle("active", Number(b.dataset.effectId) === data.efecto_id);
  });

  const metaEf = EFECTOS.find((e) => e.id === data.efecto_id) || { speed: true, light: true };

  const botonesBrillo = document.querySelectorAll("#brightnessBtns button");
  botonesBrillo.forEach((b, i) => {
    b.classList.toggle("active", metaEf.light && i + 1 === data.brillo);
    b.style.opacity = metaEf.light ? "1" : "0.4";
    b.title = metaEf.light ? `Brillo ${i + 1}` : "Este efecto va siempre al 100% de brillo en el teclado";
  });

  const botonesVel = document.querySelectorAll("#speedBtns button");
  botonesVel.forEach((b, i) => {
    b.classList.toggle("active", metaEf.speed && i + 1 === data.velocidad);
    b.style.opacity = metaEf.speed ? "1" : "0.4";
    b.title = metaEf.speed ? `Velocidad ${i + 1}` : "Este efecto es estático y no usa velocidad";
  });

  if (sincronizarTeclas) {
    if (data.efecto_id === 0) {
      encendidas.clear();
    } else if (Array.isArray(data.posiciones_activas)) {
      encendidas = new Set(data.posiciones_activas.filter((p) => todasLasPosiciones.includes(p)));
    }
    actualizarContadorTeclas();
  }
}

function mostrarToast(msg, esError = false) {
  const t = document.getElementById("toast");
  t.textContent = msg;
  t.style.borderColor = esError ? "#f85149" : "#388bfd";
  t.classList.add("show");
  if (toastTimer) clearTimeout(toastTimer);
  toastTimer = setTimeout(() => t.classList.remove("show"), 2200);
}

async function inicializar() {
  construirTeclado();
  requestAnimationFrame(bucleAnimacion);
  await cargarColoresRegistrados();
  try {
    const res = await fetch("/api/estado");
    const data = await res.json();
    actualizarEstadoUI(data, true);
  } catch (e) {
    mostrarToast("No se pudo conectar con sindri-led", true);
  }
}

inicializar();
