document.getElementById('round-res').textContent = Math.round(0.6);
document.getElementById('ceil-res').textContent = Math.ceil(0.6);
document.getElementById('floor-res').textContent = Math.floor(0.6);

document.getElementById('abs-res').textContent = Math.abs(-5);
document.getElementById('max-res').textContent = Math.max(10, 20, 30);
document.getElementById('min-res').textContent = Math.min(10, 20, 30);
document.getElementById('random-res').textContent = Math.random().toFixed(4);

document.getElementById('pow-res').textContent = Math.pow(2, 53);
document.getElementById('sqrt-res').textContent = Math.sqrt(3).toFixed(4);
document.getElementById('cbrt-res').textContent = Math.pow(3, 1/3).toFixed(4);

document.getElementById('pi-res').textContent = Math.PI.toFixed(4);
document.getElementById('e-res').textContent = Math.E.toFixed(4);

document.getElementById('sin-res').textContent = Math.sin(0);

document.getElementById('log-res').textContent = Math.log(10).toFixed(4);
document.getElementById('log10-res').textContent = (Math.log(100) / Math.LN10).toFixed(4);
document.getElementById('log2-res').textContent = (Math.log(512) / Math.LN2).toFixed(4);
document.getElementById('exp-res').textContent = Math.exp(3).toFixed(4);
