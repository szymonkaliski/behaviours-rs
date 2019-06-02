import { createSimulation } from "behaviours-rs";

export default () => {
  const [width, height] = [600, 600];

  const numPoints = 10000;
  // const numPoints = 10;

  const points = new Float32Array(numPoints * 2);

  for (let i = 0; i < numPoints; i++) {
    const x = Math.random() * width;
    const y = Math.random() * height;

    points[i * 2] = x;
    points[i * 2 + 1] = y;
  }

  const behaviours = [
    ["repel", { f: 0.3, r: 50.0 }],
    ["dampen", { f: 0.1 }]
  ];

  const simulation = createSimulation(points, 2, behaviours);

  const canvas = document.createElement("canvas");
  canvas.width = width;
  canvas.height = height;
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
};
