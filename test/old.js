import { createSimulation } from "behaviours-rs";

const [width, height] = [600, 600];

const randomPointOnCircle = r => {
  const angle = Math.random() * Math.PI * 2;
  return [Math.cos(angle) * r, Math.sin(angle) * r];
};

const add = (a, b) => [a[0] + b[0], a[1] + b[1]];

const ITERS = 1;
const SPAWN_ON_CIRCLE = true;

const points = Array.from({ length: 10000 }).map((_, i) =>
  i === 0
    ? [300, 300]
    : SPAWN_ON_CIRCLE
    ? add(randomPointOnCircle(Math.random() * 600 + 200), [300, 300])
    : [
        Math.random() * 100 - 50 + width / 2,
        Math.random() * 100 - 50 + height / 2
      ]
);

const pointsFloatArray = new Float32Array(points.length * 2);

points.forEach((p, i) => {
  pointsFloatArray[i * 2] = p[0];
  pointsFloatArray[i * 2 + 1] = p[1];
});

const bCorners = [
  ["attract", { p: [600, 300], f: 0.1, r: 300 * 300 }],
  ["attract", { p: [0, 300], f: 0.1, r: 300 * 300 }],
  ["attract", { p: [300, 0], f: 0.1, r: 300 * 300 }],
  ["attract", { p: [300, 600], f: 0.1, r: 300 * 300 }],
  ["repel", { f: 0.3, r: 50.0 }],
  ["dampen", { f: 0.1 }]
];

const bSimple = [
  [
    "if",
    { test: ["!=", "static", "true"] },
    [["attract", { p: [300, 300], f: 0.01 }], ["dampen", { f: 0.02 }]]
  ],
  ["if", { test: ["==", "static", "true"] }, [["dampen", { f: 1.0 }]]]
];

const bDLA = [
  [
    "if",
    { test: ["!=", "static", "true"] },
    [
      ["attract", { p: [300, 300], f: 0.2 }],
      // ["repel", { p: [300, 400], r: 200, f: 0.1 }],
      ["dampen", { f: 0.1 }],
      [
        "collide",
        { r: 10.0, test: ["==", "static", "true"] },
        [["set", { key: "static", value: "true" }], ["stop"]]
      ]
    ]
  ]
];

const bTest = [
  // ["repel", { p: [300, 300], r: 100, f: 1.0 }],
  // ["attract", { p: [300, 300], f: 0.1 }],
  ["attract", { r: 400, f: 0.2 }],
  ["repel", { r: 300, f: 0.5 }],
  // ["attract", { p: [300, 0], f: 0.5 }],
  // ["repel", { r: 50, f: 0.5 }],
  ["dampen", { f: 0.9 }]
];

const simulation = createSimulation(pointsFloatArray, bDLA);
simulation.setMeta(0, "static", "true");

// simulation.replaceBehaviours(bCorners);

const canvas = document.createElement("canvas");
canvas.width = 600;
canvas.height = 600;
document.body.appendChild(canvas);
const ctx = canvas.getContext("2d");

const loop = () => {
  ctx.clearRect(0, 0, 600, 600);

  for (let j = 0; j < ITERS; j++) {
    simulation.step();
  }

  const positions = simulation.getIf(["==", "static", "true"]);
  // const positions = simulation.get();

  for (let i = 0; i < positions.length; i += 2) {
    const x = positions[i];
    const y = positions[i + 1];

    ctx.fillRect(x, y, 1, 1);
  }

  requestAnimationFrame(loop);
};

loop();

