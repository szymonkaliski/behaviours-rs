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

    const radius = 100;
    const numPoints = 10000;

    const points = new Float32Array(numPoints * 3);

    for (let i = 0; i < numPoints; i++) {
      points[i * 3 + 0] = (Math.random() * 2 - 1) * radius;
      points[i * 3 + 1] = (Math.random() * 2 - 1) * radius;
      points[i * 3 + 2] = (Math.random() * 2 - 1) * radius;
    }

    const behaviours = [
      ["repel", { f: 0.8, r: 50.0 }],
      ["attract", { f: 0.2, p: [500, 0, 0], r: 520 * 520 }],
      ["attract", { f: 0.2, p: [-500, 0, 0], r: 520 * 520 }],
      ["dampen", { f: 0.05 }]
    ];

    simulation = createSimulation(points, 3, behaviours);

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
