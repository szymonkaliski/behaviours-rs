## 2019-05-14

- cont. with the research
- got basic `repulse` and `dampen` working, I'm surprised but how quickly it started working, exciting!
- I need to read up on serde parsing, as for example `dampen` requires only `f`, but it breaks rust when no `r` is available

## 2019-05-13

- thinking through most basic simulation - repulsion system
  - all points start in a random small circle
  - all are repulsed by each other

```js
const points = range(10000).map(() => randomPos())

const simulation = Simulation.create(
  points,
  [
    ["repulseOthers", { r: 0.2, f: 1.0 }]
  ]
);
```

- so first task would be to parse the tree in rust, and pass values in
- both seem to work using serde
