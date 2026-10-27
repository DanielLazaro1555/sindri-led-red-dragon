// Disposición física ISO Español / Latinoamérica (105 teclas) del Redragon K671 Sindri
// y metadatos de los 13 modos de iluminación (0..12) según Cfg.ini (LedOpt1..13).

const EFECTOS = [
  { id: 1,  name: "fijo",          label: "Fijo",                   speed: false, light: true  },
  { id: 2,  name: "respira",       label: "Respira",                speed: true,  light: true  },
  { id: 12, name: "personalizado", label: "Personalizado (Teclas)", speed: false, light: true  },
  { id: 0,  name: "apagado",       label: "Apagado (OFF)",          speed: false, light: false },
  { id: 3,  name: "reaccion",      label: "Reacción",               speed: true,  light: true  },
  { id: 4,  name: "neon",          label: "Neón",                   speed: true,  light: true  },
  { id: 5,  name: "caballo",       label: "Caballo",                speed: true,  light: false },
  { id: 6,  name: "onda",          label: "Onda senoidal",          speed: true,  light: false },
  { id: 7,  name: "serpiente",     label: "Serpiente",              speed: true,  light: false },
  { id: 8,  name: "cascada",       label: "Cascada",                speed: true,  light: false },
  { id: 9,  name: "ondas",         label: "Ondas",                  speed: true,  light: true  },
  { id: 10, name: "lluvia",        label: "Lluvia",                 speed: true,  light: true  },
  { id: 11, name: "molino",        label: "Molino",                 speed: true,  light: true  },
];

const PALETA_POR_DEFECTO = {
  yellow: "#ffdd33",
  green:  "#22ee66",
  cyan:   "#22ddff",
  blue:   "#3377ff",
  purple: "#bb44ff",
  red:    "#ff3344"
};

const MAIN_ROWS = [
  {
    rowClass: "kb-row fn-row",
    keys: [
      ["esc", "Esc", 7, ""],
      ["f1", "F1", 13, "w-gap-f"], ["f2", "F2", 19, ""], ["f3", "F3", 25, ""], ["f4", "F4", 31, ""],
      ["f5", "F5", 37, "w-gap-f"], ["f6", "F6", 43, ""], ["f7", "F7", 49, ""], ["f8", "F8", 55, ""],
      ["f9", "F9", 61, "w-gap-f"], ["f10", "F10", 67, ""], ["f11", "F11", 73, ""], ["f12", "F12", 79, ""]
    ]
  },
  {
    rowClass: "kb-row",
    keys: [
      ["barra-vertical", "| °", 2, ""],
      ["1", "1 !", 8, ""], ["2", "2 \"", 14, ""], ["3", "3 #", 20, ""], ["4", "4 $", 26, ""],
      ["5", "5 %", 32, ""], ["6", "6 &", 38, ""], ["7", "7 /", 44, ""], ["8", "8 (", 50, ""],
      ["9", "9 )", 56, ""], ["0", "0 =", 62, ""], ["apostrofo", "' ?", 68, ""], ["interrogacion", "¿ ¡", 74, ""],
      ["retroceso", "Retroceso", 80, "w-backspace"]
    ]
  },
  {
    rowClass: "kb-row",
    keys: [
      ["tab", "Tab", 3, "w-tab"],
      ["q", "Q", 9, ""], ["w", "W", 15, ""], ["e", "E", 21, ""], ["r", "R", 27, ""],
      ["t", "T", 33, ""], ["y", "Y", 39, ""], ["u", "U", 45, ""], ["i", "I", 51, ""],
      ["o", "O", 57, ""], ["p", "P", 63, ""], ["acento", "´ ¨", 69, ""], ["mas", "+ *", 75, ""],
      ["enter", "Intro", 82, "w-enter-iso"]
    ]
  },
  {
    rowClass: "kb-row",
    keys: [
      ["mayus", "Bloq Mayús", 4, "w-caps"],
      ["a", "A", 10, ""], ["s", "S", 16, ""], ["d", "D", 22, ""], ["f", "F", 28, ""],
      ["g", "G", 34, ""], ["h", "H", 40, ""], ["j", "J", 46, ""], ["k", "K", 52, ""],
      ["l", "L", 58, ""], ["enie", "Ñ", 64, ""], ["llave-izq", "{ [", 70, ""], ["llave-der", "} ]", 76, ""]
    ]
  },
  {
    rowClass: "kb-row",
    keys: [
      ["shift-izq", "Shift", 5, "w-shift-l"],
      ["menor", "< >", 83, ""],
      ["z", "Z", 11, ""], ["x", "X", 17, ""], ["c", "C", 23, ""], ["v", "V", 29, ""],
      ["b", "B", 35, ""], ["n", "N", 41, ""], ["m", "M", 47, ""], ["coma", ", ;", 53, ""],
      ["punto", ". :", 59, ""], ["guion", "- _", 65, ""],
      ["shift-der", "Shift", 77, "w-shift-r"]
    ]
  },
  {
    rowClass: "kb-row",
    keys: [
      ["ctrl-izq", "Ctrl", 6, "w-ctrl"],
      ["win-izq", "Win", 12, "w-ctrl"],
      ["alt-izq", "Alt", 18, "w-ctrl"],
      ["espacio", "Espacio", 36, "w-space"],
      ["alt-gr", "AltGr", 54, "w-ctrl"],
      ["fn", "Fn", 60, "w-ctrl"],
      ["menu", "Menú", 66, "w-ctrl"],
      ["ctrl-der", "Ctrl", 78, "w-ctrl"]
    ]
  }
];

const NAV_ROWS = [
  {
    rowClass: "kb-row fn-row",
    keys: [["impr", "Impr", 85, ""], ["bloq-despl", "Bloq", 91, ""], ["pausa", "Pausa", 97, ""]]
  },
  {
    rowClass: "kb-row",
    keys: [["insert", "Ins", 86, ""], ["inicio", "Inicio", 92, ""], ["repag", "RePág", 98, ""]]
  },
  {
    rowClass: "kb-row",
    keys: [["supr", "Supr", 87, ""], ["fin", "Fin", 93, ""], ["avpag", "AvPág", 99, ""]]
  },
  {
    rowClass: "kb-row",
    keys: [null, null, null]
  },
  {
    rowClass: "kb-row",
    keys: [null, ["arriba", "↑", 95, ""], null]
  },
  {
    rowClass: "kb-row",
    keys: [["izquierda", "←", 90, ""], ["abajo", "↓", 96, ""], ["derecha", "→", 102, ""]]
  }
];

const NUMPAD_KEYS = [
  ["bloq-num", "Num", 104, ""], ["num-div", "/", 110, ""], ["num-mul", "*", 116, ""], ["num-menos", "-", 122, ""],
  ["num-7", "7", 105, ""],      ["num-8", "8", 111, ""],   ["num-9", "9", 117, ""],   ["num-mas", "+", 123, "num-tall"],
  ["num-4", "4", 106, ""],      ["num-5", "5", 112, ""],   ["num-6", "6", 118, ""],
  ["num-1", "1", 107, ""],      ["num-2", "2", 113, ""],   ["num-3", "3", 119, ""],   ["num-enter", "Intro", 125, "num-tall"],
  ["num-0", "0", 108, "num-wide"],                         ["num-punto", ".", 120, ""]
];

const AZUL_PURO = new Set([66, 73, 74, 76, 77, 78, 79, 80]);

function zonaPorDefecto(pos, nombre) {
  if (nombre === "menor") return "yellow";
  if (AZUL_PURO.has(pos) || pos === 75) return "blue";
  const col = Math.floor((pos - 1) / 6);
  if (col <= 3) return "yellow";
  if (col <= 6) return "green";
  if (col <= 11) return "cyan";
  if (col <= 16) return "purple";
  return "red";
}
