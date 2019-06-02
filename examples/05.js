import { createSimulation } from "behaviours-rs";

const THREE = require("three");
const OrbitControls = require("three-orbitcontrols");

export default () => {
  const vertexShader = `
    void main() {
      vec4 mvPosition = modelViewMatrix * vec4(position, 1.0);
      gl_PointSize = 2.0;
      gl_Position = projectionMatrix * mvPosition;
    }
  `;

  const fragmentShader = `
    void main() {
      gl_FragColor = vec4(1.0);
    }
  `;

  let camera, controls, scene, renderer, simulation, geometry;

  const randomSpherePoint = radius => {
    const u = Math.random();
    const v = Math.random();
    const theta = 2 * Math.PI * u;
    const phi = Math.acos(2 * v - 1);
    const x = radius * Math.sin(phi) * Math.cos(theta);
    const y = radius * Math.sin(phi) * Math.sin(theta);
    const z = radius * Math.cos(phi);

    return [x, y, z];
  };

  const onWindowResize = () => {
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(window.innerWidth, window.innerHeight);
  };

  const init = () => {
    camera = new THREE.PerspectiveCamera(
      80,
      window.innerWidth / window.innerHeight
    );

    camera.position.z = 300;

    scene = new THREE.Scene();

    const shaderMaterial = new THREE.ShaderMaterial({
      vertexShader,
      fragmentShader,
      blending: THREE.AdditiveBlending,
      depthTest: false,
      transparent: true,
      vertexColors: true
    });

    const radius = 500;
    const numPoints = 20000;

    const points = new Float32Array(numPoints * 3);

    for (let i = 0; i < numPoints; i++) {
      const [x, y, z] =
        i === 0 ? [0, 0, 0] : randomSpherePoint(Math.random() * radius + 500);

      points[i * 3 + 0] = x;
      points[i * 3 + 1] = y;

      // full sphere
      // points[i * 3 + 2] = z;

      // ring
      points[i * 3 + 2] = Math.random() * 60 - 30;
    }

    const behaviours = [
      [
        "if",
        { test: ["!=", "static", "true"] },
        [
          ["attract", { p: [0, 0, 0], f: 0.2 }],
          ["dampen", { f: 0.1 }],
          [
            "collide",
            { r: 5.0, test: ["==", "static", "true"] },
            [
              ["set", { key: "static", value: "true" }],
              ["stop"]
            ]
          ]
        ]
      ]
    ];

    simulation = createSimulation(points, 3, behaviours);
    simulation.setMeta(0, "static", "true");

    geometry = new THREE.BufferGeometry();

    geometry.addAttribute(
      "position",
      new THREE.Float32BufferAttribute(points, 3)
    );

    const particleSystem = new THREE.Points(geometry, shaderMaterial);
    scene.add(particleSystem);

    renderer = new THREE.WebGLRenderer();
    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.setSize(window.innerWidth, window.innerHeight);

    document.body.appendChild(renderer.domElement);
    document.body.style.margin = 0;

    controls = new OrbitControls(camera, renderer.domElement);

    window.addEventListener("resize", onWindowResize, false);
  };

  const loop = () => {
    simulation.step();

    const positions = simulation.get();

    for (let i = 0; i < positions.length; i += 1) {
      geometry.attributes.position.array[i] = positions[i];
    }

    geometry.attributes.position.needsUpdate = true;

    renderer.render(scene, camera);

    requestAnimationFrame(loop);
  };

  init();
  loop();
};
