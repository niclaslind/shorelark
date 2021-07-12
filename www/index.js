import * as sim from "lib-simulation-wasm"

let simulation = new sim.Simulation();

document.getElementById('train').onclick = function () {
    console.log(simulation.train())
}

const viewport = document.getElementById('viewport');
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;
const ctxt = viewport.getContext('2d');

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
    ctxt.clearRect(0, 0, viewportWidth, viewportHeight);

    simulation.step();

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

    // requestAnimationFrame() schedules code only for the next frame.
    // 
    // Because we want for our simulation to continue forever, we've
    // gitta keep re-sceduling our function:
    requestAnimationFrame(redraw);
}

redraw();
