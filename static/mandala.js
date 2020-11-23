import init, { generate_mandala } from './mandala_wasm.js';
const seedInput = document.getElementById('seed');
const downloadButton = document.getElementById('download');
const targetElement = document.getElementById('mandala');
const colorInput = document.getElementById('color');

// set form values from url:
new URL(document.URL).searchParams.forEach(
  (x, y) => (document.getElementById(y).value = x)
);

export function downloadSVG() {
  const svg = document.getElementById('mandala').innerHTML;
  const blob = new Blob([svg.toString()]);
  const element = document.createElement('a');
  element.download = `manadala_${seedInput.value}.svg`;
  element.href = window.URL.createObjectURL(blob);
  element.click();
  element.remove();
}

function generate(color) {
  if (seedInput.checkValidity()) {
    let mandala = generate_mandala(seedInput.value, 12, 12, 240, color);
    targetElement.innerHTML = mandala;
  }
}
async function run() {
  await init();
  generate(colorInput.value);
  seedInput.addEventListener('input', function () {
    generate(colorInput.value);
  });
  colorInput.addEventListener('input', function () {
    generate(colorInput.value);
  });
  downloadButton.addEventListener('click', function () {
    downloadSVG();
  });
}

run();
