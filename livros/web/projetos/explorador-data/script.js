function updateClock() {
    const now = new Date();
    document.getElementById('clock').textContent = now.toLocaleString('pt-BR');
}

function createResultItem(label, value) {
    const div = document.createElement('div');
    div.className = 'result-item';
    div.innerHTML = `<span class="result-label">${label}</span><span class="result-value">${value}</span>`;
    return div;
}

function analyzeDate() {
    const input = document.getElementById('dateInput').value;
    if (!input) return alert('Por favor, selecione uma data!');

    const date = new Date(input);
    const componentsList = document.getElementById('componentsList');
    const formatsList = document.getElementById('formatsList');
    
    componentsList.innerHTML = '';
    formatsList.innerHTML = '';

    const components = {
        'Ano': date.getFullYear(),
        'Mês (0-11)': date.getMonth(),
        'Dia do Mês': date.getDate(),
        'Dia da Semana': date.getDay(),
        'Hora': date.getHours(),
        'Minutos': date.getMinutes(),
        'Segundos': date.getSeconds(),
        'UTC Hora': date.getUTCHours()
    };

    const formats = {
        'toString()': date.toString(),
        'toUTCString()': date.toUTCString(),
        'toLocaleDateString()': date.toLocaleDateString(),
        'toLocaleTimeString()': date.toLocaleTimeString(),
        'toISOString()': date.toISOString()
    };

    for (let [label, value] of Object.entries(components)) {
        componentsList.appendChild(createResultItem(label, value));
    }

    for (let [label, value] of Object.entries(formats)) {
        formatsList.appendChild(createResultItem(label, value));
    }

    document.getElementById('analysisResult').classList.remove('hidden');
}


// Init
setInterval(updateClock, 1000);
updateClock();

document.getElementById('analyzeBtn').addEventListener('click', analyzeDate);
