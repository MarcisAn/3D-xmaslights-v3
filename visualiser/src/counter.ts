import * as THREE from "three";
import { OrbitControls } from "three/addons/controls/OrbitControls.js";
import chords from "../../cords.json";


const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(30, 1, 0.1, 1000);
const geometry = new THREE.BoxGeometry();
let renderer: THREE.WebGLRenderer;
let controls: OrbitControls;

camera.position.z = 3;

const animate = () => {
  requestAnimationFrame(animate);
  renderer.render(scene, camera);
  controls.update();
};

const resize = () => {
  renderer.setSize(300, 300);
  camera.aspect = 1;
  camera.updateProjectionMatrix();
};
export const createScene = (el: any) => {
  renderer = new THREE.WebGLRenderer({ antialias: true, canvas: el });
  controls = new OrbitControls(camera, renderer.domElement);
  controls.enableZoom = false;
  controls.enablePan = false;
  //controls.minPolarAngle = -90;
  let lightIndex = 0;
  for (const light of chords) {
    const material = new THREE.MeshBasicMaterial({ color: 0x000000 });

    const cube = new THREE.Mesh(geometry, material);
    cube.position.x = light.x / 900;
    cube.position.z = light.y / 900;
    cube.position.y = light.z / 900;
    cube.scale.x = 0.01;
    cube.scale.y = 0.01;
    cube.scale.z = 0.01;
    // @ts-ignore
    try {
      cube.material.color.setRGB(0, 0, 0);
      cube.name = lightIndex.toString();
      // @ts-ignore
      cube.changeColor = (r: number, g: number, b: number) => {
        cube.material.color.setRGB(r / 255, g / 255, b / 255);
      };
      scene.add(cube);
    } catch (e) {}
    lightIndex += 1;
  }

  resize();
  animate();
};
export async function all_to_black() {
  for (let i = 0; i < 200; i++) {
    const obj = scene.getObjectByName(i.toString());
    //@ts-ignore
    obj.changeColor(0, 0, 0);
  }
}
export async function renderVis(data: any) {
  //console.log(data);
  data.forEach((light: string[]) => {
    let index = light[0];
    const obj = scene.getObjectByName(index.toString());
    //console.log(light);
    //@ts-ignore
    obj.changeColor(light[1], light[2], light[3]);
  });
}
