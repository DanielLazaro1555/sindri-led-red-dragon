// Motor de animación en tiempo real (60 FPS) para los 13 efectos (0..12) del Redragon K671 Sindri.

function registrarPulsoInteractivo(pos, fila, col) {
  const ahora = performance.now();
  destellosReaccion[pos] = ahora;
  ondasActivas.push({ fila, col, t0: ahora });
  if (ondasActivas.length > 8) ondasActivas.shift();
}

function calcularIntensidadTecla(item, tSeg, velFactor, ahoraMs) {
  const { pos, fila, col } = item;

  switch (efectoActualId) {
    case 0: // Apagado
      return 0;

    case 1: // Fijo (Steady)
      return 1;

    case 2: { // Respira (Breathing): sube y baja entre 0.04 y 1.0
      const fase = (Math.cos(tSeg * velFactor * 1.45) + 1) * 0.5;
      return 0.04 + 0.96 * Math.pow(fase, 1.35);
    }

    case 3: { // Reacción (Reaction): destellos al pulsar teclas + pulsos aleatorios suaves
      let val = 0.06;
      const tPulso = destellosReaccion[pos];
      if (tPulso !== undefined) {
        const dt = (ahoraMs - tPulso) / 1000;
        const dur = 1.4 / velFactor;
        if (dt < dur) {
          val = Math.max(val, 1 - dt / dur);
        }
      }
      const semilla = Math.sin(pos * 12.9898 + Math.floor(tSeg * velFactor * 1.8) * 78.233) * 43758.5453;
      const frac = semilla - Math.floor(semilla);
      if (frac > 0.92) {
        const subFase = (tSeg * velFactor * 1.8) % 1;
        val = Math.max(val, 1 - subFase);
      }
      return val;
    }

    case 4: { // Neón (Neon stream): pulsos alternados por bloques horizontales
      const onda = Math.sin(tSeg * velFactor * 2.1 - col * 0.14);
      return Math.max(0.05, (onda + 1) * 0.5);
    }

    case 5: { // Caballo (Horizontal running horse): franja que corre de izq a der y rebota
      const ciclo = (tSeg * velFactor * 0.85) % 2;
      const posX = (ciclo < 1 ? ciclo : 2 - ciclo) * 20;
      const dist = Math.abs(col - posX);
      return Math.max(0.06, Math.exp(-(dist * dist) / 5.5));
    }

    case 6: { // Onda senoidal (Sine wave): ola recorriendo las 21 columnas
      const v = Math.sin(col * 0.45 - tSeg * velFactor * 3.0 + fila * 0.35);
      return Math.max(0.05, Math.pow((v + 1) * 0.5, 1.6));
    }

    case 7: { // Serpiente (Retro snake): recorre las 105 teclas en zig-zag
      const total = rutaSerpiente.length || 105;
      const cabeza = (tSeg * velFactor * 18) % total;
      const idx = rutaSerpiente.indexOf(pos);
      let d = cabeza - idx;
      if (d < 0) d += total;
      const largoCola = 16;
      if (d >= 0 && d <= largoCola) {
        return 1 - (d / largoCola) * 0.85;
      }
      return 0.05;
    }

    case 8: { // Cascada (Gaoshan waterfall): cae de la fila 0 a la 5
      const caida = fila - tSeg * velFactor * 2.6 + Math.sin(col * 0.6) * 0.7;
      const mod = ((caida % 6) + 6) % 6;
      return Math.max(0.05, Math.exp(-(mod * mod) / 2.2));
    }

    case 9: { // Ondas (Dripping ripples): anillos expansivos desde el centro o desde clics
      const radioAuto = (tSeg * velFactor * 7.5) % 24;
      const distCentro = Math.hypot(col - 9.5, (fila - 2.5) * 1.6);
      let v = Math.exp(-Math.pow(distCentro - radioAuto, 2) / 3.0);

      for (const o of ondasActivas) {
        const dt = (ahoraMs - o.t0) / 1000;
        const r = dt * velFactor * 12;
        if (r < 26) {
          const d = Math.hypot(col - o.col, (fila - o.fila) * 1.6);
          v = Math.max(v, Math.exp(-Math.pow(d - r, 2) / 2.4));
        }
      }
      return Math.max(0.05, Math.min(1, v));
    }

    case 10: { // Lluvia (Rain falling bead curtain): gotas verticales por columna
      const velCol = 1.8 + ((col * 7) % 5) * 0.35;
      const desfase = ((col * 13) % 11) * 0.6;
      const gotaY = ((tSeg * velFactor * velCol + desfase) % 7.5) - 1;
      const dist = fila - gotaY;
      if (dist <= 0 && dist > -2.6) {
        return Math.max(0.05, 1 + dist / 2.6);
      }
      return 0.05;
    }

    case 11: { // Molino (Rotating windmill): aspas girando alrededor del centro
      const angulo = Math.atan2((fila - 2.5) * 1.5, col - 9.5);
      const giro = Math.sin(angulo * 2 - tSeg * velFactor * 2.8);
      return Math.max(0.06, Math.pow((giro + 1) * 0.5, 1.5));
    }

    case 12: // Personalizado (Custom)
    default:
      return encendidas.has(pos) ? 1 : 0;
  }
}

function bucleAnimacion(ahoraMs) {
  const tSeg = (ahoraMs - tInicioEfecto) / 1000;
  // Velocidad 1..5 -> factor de velocidad base (1 = más lento, 5 = más rápido)
  const mapaVel = { 1: 0.55, 2: 0.78, 3: 1.0, 4: 1.35, 5: 1.75 };
  const velFactor = (mapaVel[velocidadActual] || 1.0) * escalaAnimacion;
  // Según LedOpt5..8 de Cfg.ini (analizado con REA), los efectos 5, 6, 7 y 8 ignoran el brillo y van siempre al 100%
  const ignoraBrillo = [5, 6, 7, 8].includes(efectoActualId);
  const escalaBrillo = [0.42, 0.56, 0.72, 0.86, 1.0][Math.max(0, Math.min(4, brilloActual - 1))] || 0.72;
  const maxBrillo = efectoActualId === 0 ? 0 : (ignoraBrillo ? 1.0 : escalaBrillo);

  for (const item of elementosTecla) {
    const raw = calcularIntensidadTecla(item, tSeg, velFactor, ahoraMs);
    const alpha = raw * maxBrillo;
    const el = item.el;
    el.style.setProperty("--k-alpha", alpha.toFixed(3));

    if (alpha > 0.10) {
      const pct = Math.round(alpha * 100);
      const glowPx = Math.round(alpha * 18);
      el.style.color = `color-mix(in srgb, var(--led) ${Math.max(45, pct)}%, #758096)`;
      el.style.borderColor = `color-mix(in srgb, var(--led) ${Math.round(alpha * 95)}%, #262e3d)`;
      el.style.boxShadow = `0 0 ${glowPx}px color-mix(in srgb, var(--led) ${Math.round(alpha * 70)}%, transparent)`;
      el.style.textShadow = `0 0 ${Math.round(alpha * 10)}px var(--led)`;
    } else {
      el.style.color = "#465061";
      el.style.borderColor = "#262e3d";
      el.style.boxShadow = "none";
      el.style.textShadow = "none";
    }
  }

  requestAnimationFrame(bucleAnimacion);
}

function ajustarEscalaAnimacion(val) {
  escalaAnimacion = Number(val);
  document.getElementById("animScaleLabel").textContent = `${escalaAnimacion.toFixed(2)}x`;
}

function reiniciarFaseEfecto() {
  tInicioEfecto = performance.now();
  cambiarEfecto(efectoActualNombre);
}
