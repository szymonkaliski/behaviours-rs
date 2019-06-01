import { createSimulation } from "behaviours-rs";

export default () => {
  const [width, height] = [600, 600];

  const numPoints = 10000;

  const points = new Float32Array(numPoints * 2);

  const randomPointOnCircle = r => {
    const angle = Math.random() * Math.PI * 2;
    return [Math.cos(angle) * r, Math.sin(angle) * r];
  };

  for (let i = 0; i < numPoints; i++) {
    if (i === 0) {
      points[i * 2] = 300;
      points[i * 2 + 1] = 300;
    } else {
      const [x, y] = randomPointOnCircle(Math.random() * 600 + 200);

      points[i * 2] = x + 300;
      points[i * 2 + 1] = y + 300;
    }
  }

  const behaviours = [
    [
      "if",
      { test: ["!=", "static", "true"] },
      [
        ["attract", { p: [300, 300], f: 0.2 }],
        ["dampen", { f: 0.1 }],
        [
          "collide",
          { r: 10.0, test: ["==", "static", "true"] },
          [
            ["set", { key: "static", value: "true" }],
            ["stop"]
          ]
        ]
      ]
    ]
  ];

  const simulation = createSimulation(points, behaviours);

  simulation.setMeta(0, "static", "true");

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
