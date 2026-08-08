import * as sim from "lib-simulation-wasm"

let simulation = new sim.Simulation();

const viewport = document.getElementById('viewport');
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;
const ctxt = viewport.getContext('2d');

const chart = document.getElementById('chart');
const chartWidth = chart.width;
const chartHeight = chart.height;
const chartCtxt = chart.getContext('2d');

const pauseBtn = document.getElementById('pause');
const resetBtn = document.getElementById('reset');
const trainBtn = document.getElementById('train');

const statGeneration = document.getElementById('stat-generation');
const statMin = document.getElementById('stat-min');
const statMax = document.getElementById('stat-max');
const statAvg = document.getElementById('stat-avg');

const cfgNumAnimals = document.getElementById('cfg-num-animals');
const cfgNumFoods = document.getElementById('cfg-num-foods');
const cfgMutationChance = document.getElementById('cfg-mutation-chance');
const cfgMutationCoeff = document.getElementById('cfg-mutation-coeff');
const cfgMaxSpeed = document.getElementById('cfg-max-speed');
const configApplyBtn = document.getElementById('config-apply');

const statAnimals = document.getElementById('stat-animals');
const statFoods = document.getElementById('stat-foods');

const trailsBtn = document.getElementById('trails');
const themeBtn = document.getElementById('theme');
const speedSelect = document.getElementById('speed');

let isPaused = false;
let trailsEnabled = true;
let isLightTheme = false;
let simSpeed = 1;

// History of per-generation fitness stats, used to draw the chart.
// Capped so the chart stays readable and memory bounded over long runs.
const MAX_HISTORY = 200;
let statsHistory = [];

function recordStats(stats) {
    statGeneration.textContent = simulation.generation();

    if (!stats) {
        return;
    }

    statMin.textContent = stats.min_fitness.toFixed(2);
    statMax.textContent = stats.max_fitness.toFixed(2);
    statAvg.textContent = stats.avg_fitness.toFixed(2);

    statsHistory.push(stats);

    if (statsHistory.length > MAX_HISTORY) {
        statsHistory.shift();
    }

    drawChart();
}

function resetStats() {
    statMin.textContent = '-';
    statMax.textContent = '-';
    statAvg.textContent = '-';
    statsHistory = [];
    drawChart();
}

function drawChart() {
    chartCtxt.clearRect(0, 0, chartWidth, chartHeight);

    if (statsHistory.length === 0) {
        return;
    }

    const allFitness = statsHistory.flatMap((s) => [s.min_fitness, s.max_fitness]);
    const loFitness = Math.min(...allFitness);
    const hiFitness = Math.max(...allFitness, loFitness + 0.001);

    const xStep = statsHistory.length > 1
        ? chartWidth / (statsHistory.length - 1)
        : 0;

    function toY(fitness) {
        const t = (fitness - loFitness) / (hiFitness - loFitness);
        return chartHeight - t * chartHeight;
    }

    function drawLine(color, valueOf) {
        chartCtxt.beginPath();
        chartCtxt.strokeStyle = color;
        chartCtxt.lineWidth = 1.5;

        statsHistory.forEach((s, i) => {
            const x = i * xStep;
            const y = toY(valueOf(s));

            if (i === 0) {
                chartCtxt.moveTo(x, y);
            } else {
                chartCtxt.lineTo(x, y);
            }
        });

        chartCtxt.stroke();
    }

    drawLine('rgb(255, 99, 99)', (s) => s.max_fitness);
    drawLine('rgb(255, 220, 99)', (s) => s.avg_fitness);
    drawLine('rgb(99, 170, 255)', (s) => s.min_fitness);
}

trainBtn.onclick = function () {
    const stats = simulation.train();
    console.log(stats);
    recordStats(stats);
};

pauseBtn.onclick = function () {
    isPaused = !isPaused;
    pauseBtn.textContent = isPaused ? 'resume' : 'pause';
};

resetBtn.onclick = function () {
    simulation = new sim.Simulation();
    resetStats();
    ctxt.clearRect(0, 0, viewportWidth, viewportHeight);
};

function readConfigFromInputs() {
    return [
        parseInt(cfgNumAnimals.value, 10),
        parseInt(cfgNumFoods.value, 10),
        parseFloat(cfgMutationChance.value),
        parseFloat(cfgMutationCoeff.value),
        parseFloat(cfgMaxSpeed.value),
    ];
}

configApplyBtn.onclick = function () {
    simulation = new sim.Simulation(...readConfigFromInputs());
    resetStats();
    ctxt.clearRect(0, 0, viewportWidth, viewportHeight);
};

trailsBtn.onclick = function () {
    trailsEnabled = !trailsEnabled;
    trailsBtn.textContent = trailsEnabled ? 'trails: on' : 'trails: off';

    if (!trailsEnabled) {
        ctxt.clearRect(0, 0, viewportWidth, viewportHeight);
    }
};

themeBtn.onclick = function () {
    isLightTheme = !isLightTheme;
    document.body.classList.toggle('light', isLightTheme);
    themeBtn.textContent = isLightTheme ? '☀️ light' : '🌙 dark';
    updateThemeColors();
    ctxt.clearRect(0, 0, viewportWidth, viewportHeight);
};

speedSelect.onchange = function () {
    simSpeed = parseInt(speedSelect.value, 10) || 1;
};

// Colors used when drawing on the <canvas>; kept in sync with the CSS
// theme variables so trails/birds look right in both light and dark mode.
let viewportBgColor = 'rgb(31, 38, 57)';
let birdColor = 'rgb(255, 255, 255)';

function updateThemeColors() {
    viewportBgColor = isLightTheme ? 'rgb(255, 255, 255)' : 'rgb(31, 38, 57)';
    birdColor = isLightTheme ? 'rgb(31, 38, 57)' : 'rgb(255, 255, 255)';
}

function toRgba(rgb, alpha) {
    return rgb.replace('rgb(', 'rgba(').replace(')', `, ${alpha})`);
}

CanvasRenderingContext2D.prototype.drawTriangle = function (x, y, size, rotation, color) {
    this.beginPath();

    this.moveTo(
        x + Math.cos(rotation) * size * 1.5,
        y + Math.sin(rotation) * size * 1.5,
    );

    this.lineTo(
        x + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
        y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
    );


    this.lineTo(
        x + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size,
        y + Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size,
    );

    this.lineTo(
        x + Math.cos(rotation) * size * 1.5,
        y + Math.sin(rotation) * size * 1.5,
    );

    this.stroke();
    this.fillStyle = color;
    this.fill();
}

CanvasRenderingContext2D.prototype.drawCircle = function (x, y, radius) {
    this.beginPath();

    this.arc(x, y, radius, 0, 2.0 * Math.PI);

    this.fillStyle = 'rgb(0, 255, 128)';
    this.fill();
}

function redraw() {
    if (!isPaused) {
        if (trailsEnabled) {
            // Instead of a full clear, paint a translucent rectangle over
            // the previous frame so past positions fade out gradually,
            // leaving a short motion trail behind each bird.
            ctxt.fillStyle = toRgba(viewportBgColor, 0.15);
            ctxt.fillRect(0, 0, viewportWidth, viewportHeight);
        } else {
            ctxt.clearRect(0, 0, viewportWidth, viewportHeight);
        }

        const stats = simulation.step();

        if (stats) {
            console.log(stats);
        }

        recordStats(stats);

        // Fast-forward: run the remaining steps for this frame without
        // re-rendering in between, so higher speeds actually save wall
        // time instead of just rendering faster.
        for (let i = 1; i < simSpeed; i++) {
            const extraStats = simulation.step();

            if (extraStats) {
                recordStats(extraStats);
            }
        }

        const world = simulation.world();

        statAnimals.textContent = world.animals.length;
        statFoods.textContent = world.foods.length;

        for (const food of world.foods) {
            ctxt.drawCircle(
                food.x * viewportWidth,
                food.y * viewportHeight,
                (0.01 / 2.0) * viewportWidth,
            );
        }

        for (const animal of world.animals) {
            ctxt.drawTriangle(
                animal.x * viewportWidth,
                animal.y * viewportHeight,
                0.01 * viewportWidth,
                animal.rotation,
                birdColor,
            );
        }
    }

    // requestAnimationFrame() schedules code only for the next frame.
    // 
    // Because we want for our simulation to continue forever, we've
    // gitta keep re-sceduling our function:
    requestAnimationFrame(redraw);
}

redraw();
