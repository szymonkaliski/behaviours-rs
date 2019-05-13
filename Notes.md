## 2019-05-13

- thinking through most basic simulation - repulsion system
  - all points start in a random small circle
  - all are repulsed by each other

```js
const points = range(10000).map(() => randomPos())

// velocity - is it part of simulation only?

const simulation = Simulation.create(
  points,
  [
    ["repulseOthers", { r: 0.2, f: 1.0 }]
  ]
);
```

- so first task would be to parse the tree in rust
