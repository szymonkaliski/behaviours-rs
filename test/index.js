import { Simulation } from "behaviours-rs";

const [width, height] = [600, 600];

const points = Array.from({ length: 1000 }).map(() => [
  Math.random() * 100 + width / 2,
  Math.random() * 100 + height / 2
]);

const simulation = Simulation.create(points, [
  ["repel", { f: 0.2, r: 1.0 }],
  ["dampen", { f: 0.01, r: 0.0 }]
]);

const canvas = document.createElement("canvas");
canvas.width = 600;
canvas.height = 600;
document.body.appendChild(canvas);
const ctx = canvas.getContext("2d");

const loop = () => {
  ctx.clearRect(0, 0, 600, 600);

  simulation.step();

  const positions = simulation.get();

  for (let i = 0; i < positions.length; i += 2) {
    const x = positions[i];
    const y = positions[i + 1];

    ctx.fillRect(x, y, 1, 1);
  }

  requestAnimationFrame(loop);
};

loop();
