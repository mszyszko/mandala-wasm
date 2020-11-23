import init, { generate_mandala } from './mandala_wasm.js';

/**
 * Pseudo-random string generator
 * http://stackoverflow.com/a/27872144/383904
 * Default: return a random alpha-numeric string
 *
 * @param {Integer} len Desired length
 * @param {String} an Optional (alphanumeric), "a" (alpha), "n" (numeric)
 * @return {String}
 */
function randomString(len, an) {
  an = an && an.toLowerCase();
  var str = '',
    i = 0,
    min = an == 'a' ? 10 : 0,
    max = an == 'n' ? 10 : 62;
  for (; i++ < len; ) {
    var r = (Math.random() * (max - min) + min) << 0;
    str += String.fromCharCode((r += r > 9 ? (r < 36 ? 55 : 61) : 48));
  }
  return str;
}

const target_dom_element = document.getElementById('mandalas');

function generate_seeds(num) {
  let seeds = [];
  for (let i = 0; i < num; i++) {
    let seed = randomString(16);
    seeds.push(seed);
  }
  return seeds;
}

async function run() {
  await init();

  let seeds = generate_seeds(28);

  Promise.all(
    seeds.map((seed) => {
      return {
        seed: seed,
        mandala: generate_mandala(seed, 12, 12, 120, '#000000'),
      };
    })
  ).then((output) => {
    output.map((out) => {
      let a = document.createElement('a');
      a.setAttribute('href', './mandala.html?seed=' + out.seed);
      a.innerHTML = out.mandala;
      target_dom_element.appendChild(a);
    });
  });
}

run();
