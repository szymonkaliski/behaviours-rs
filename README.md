# behaviours-rs

**Work in progress!**

Rust + wasm library for declarative modeling of simple particle behaviours.

## Installation

`npm install behaviours-rs`

## Example

10000 points repelling each other:

```js
import { createSimulation } from "behaviours-rs";

const [width, height] = [600, 600];

const numPoints = 10000;

const points = new Float32Array(numPoints * 2);

for (let i = 0; i < numPoints; i++) {
  const x = Math.random() * width;
  const y = Math.random() * height;

  points[i * 2] = x;
  points[i * 2 + 1] = y;
}

// main behaviour modeling
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
```

Running examples:

```bash
cd ./examples/
npm install
npm start
```

## API

### Constructor

#### `const simulation = createSimulation(points, dimensions, behaviours)` - creates new simulation

- `points` - `Float32Array` of flat `x`, `y`, `z` (if in 3d) positions: `[ x1,y1,z1, x2,y2,z2, ... ]`
- `dimensions` - `2` or `3`, can be omitted, defaults to `2`
- `behaviours` - tree of behaviours

### Functions

- `simulation.step()` - single step of simulation
- `simulation.get()` - returns all positions (same format as `points` when creating simulation)
- `simulation.getIf(test)` - returns all points matching provided test (look at `"if"` behaviour)
- `simulation.setMeta(idx, key, value)` - sets additional `key`/`value` for specified point
- `simulation.getMeta(idx, key)` - returns value for provided `key` or empty string

### Behaviours

- `["repel", { f, r, p }]`
  - `f` - force, ideally between `0.0` and `1.0`
  - `r` - impact radius
  - `p` - position when the repelling happens, if ommited the points repel each other
- `["attract", { f, r, p }]`
  - `f` - force, ideally between `0.0` and `1.0`
  - `r` - impact radius
  - `p` - position when the attraction happens, if ommited the points attract each other
- `["dampen", { f }]"` - dampens velocity
  - `f` - force, ideally between `0.0` and `1.0`
- `["if", { test }, children]` - executes `children` when `test` passes
  - `test`: [`op`, `key`, `value`], where `op` is either `"==`" or `"!="`, and `key`/`value` are this point's metadata
- `["collide", { test, r }, children]` - executes `children` when points collide in given `r`, optionally passing a `test`
  - `test` - same as in `"if"`, optional
  - `r` - radius of collision
- `["set", { ke, value }]"` - sets `key`/`value` metadata on current point

For `collide` and `if` usage look into [`examples/03.js`](./examples/03.js).

