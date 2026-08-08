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

let isPaused = false;

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
};

CanvasRenderingContext2D.prototype.drawTriangle = function (x, y, size, rotation) {
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
    this.fillStyle = 'rgb(255, 255, 255)';
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
        ctxt.clearRect(0, 0, viewportWidth, viewportHeight);

        const stats = simulation.step();

        if (stats) {
            console.log(stats);
        }

        recordStats(stats);

        const world = simulation.world();

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
