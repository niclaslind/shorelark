import * as sim from "lib-simulation-wasm"

let simulation = new sim.Simulation();

const viewport = document.getElementById('viewport');
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;
const ctxt = viewport.getContext('2d');

const pauseBtn = document.getElementById('pause');
const resetBtn = document.getElementById('reset');
const trainBtn = document.getElementById('train');

const statGeneration = document.getElementById('stat-generation');
const statMin = document.getElementById('stat-min');
const statMax = document.getElementById('stat-max');
const statAvg = document.getElementById('stat-avg');

let isPaused = false;

function updateStats(statsString) {
    statGeneration.textContent = simulation.generation();

    if (!statsString) {
        return;
    }

    // statsString looks like: "min=0.12, max=3.45, avg=1.23"
    const matches = statsString.match(/min=([\d.-]+), max=([\d.-]+), avg=([\d.-]+)/);

    if (matches) {
        statMin.textContent = matches[1];
        statMax.textContent = matches[2];
        statAvg.textContent = matches[3];
    }
}

trainBtn.onclick = function () {
    const stats = simulation.train();
    console.log(stats);
    updateStats(stats);
};

pauseBtn.onclick = function () {
    isPaused = !isPaused;
    pauseBtn.textContent = isPaused ? 'resume' : 'pause';
};

resetBtn.onclick = function () {
    simulation = new sim.Simulation();
    statMin.textContent = '-';
    statMax.textContent = '-';
    statAvg.textContent = '-';
    updateStats(null);
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

        updateStats(stats);

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
