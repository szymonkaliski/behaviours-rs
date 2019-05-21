import { Simulation } from "behaviours-rs";

const [width, height] = [600, 600];

const points = Array.from({ length: 1000 }).map(() => [
  Math.random() * 100 - 50 + width / 2,
  Math.random() * 100 - 50 + height / 2
]);

const createSimulation = (points, behaviours) =>
  Simulation.create(
    points,
    behaviours.reduce(
      (memo, [behaviour, params]) => [...memo, { behaviour, params }],
      []
    )
  );

const simulation = createSimulation(points, [
  ["attract", { p: [600, 300], f: 0.1, r: 300 }],
  ["attract", { p: [0, 300], f: 0.1, r: 300 }],
  ["attract", { p: [300, 0], f: 0.1, r: 300 }],
  ["attract", { p: [300, 600], f: 0.1, r: 300 }],
  ["repel", { f: 0.3, r: 50.0 }],
  ["dampen", { f: 0.1 }]
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
