import { all_to_black, createScene, renderVis } from "./counter";
import "./style.css";
import { io } from "socket.io-client";
import { CanvasRecorder } from "./CanvasRecorder.ts";

let server_url = "";
if (import.meta.env.PROD) {
  server_url = "ws://lampinas-server.cvgmerch.lv:81";
} else {
  server_url = "ws://localhost:81";
}
const socket = io(server_url, {
  autoConnect: true,
  reconnection: true,
  reconnectionAttempts: 1000,
  reconnectionDelay: 30,
  transports: ["websocket"],
});
let anim_data: any[] = [];
let anim_speed = 10;
let record = false;
let recording = false;
let frame = 0;
let intervalId: number;

socket.on("reciveAnimation", (data) => {
  clearInterval(intervalId);
  if (record) {
    alert("RECORD START");
    recording = true;
    recorder.start();
  }
  anim_speed = data.speed;
  frame = 0;
  all_to_black();
  anim_data = [];
  data.animation.forEach((frame: string) => {
    let tree_data: number[][] = [];
    let light_data: number[] = [];
    let char_index = 0; // indekss baitam, kuru pašlaik apskatām: 0-lampiņas idekss, 1-red, 2-green, 3-blue
    frame.split("").forEach(function (c: string) {
      light_data.push(c.charCodeAt(0));
      char_index++;
      if (char_index == 4) {
        char_index = 0;
        tree_data.push(light_data);
        light_data = [];
      }
    });
    anim_data.push(tree_data);
  });
  intervalId = window.setInterval(function () {
    if (anim_data.length > 0) {
      renderVis(anim_data[frame]);
    }
    frame++;

    if (frame == anim_data.length) {
      if (record && recording) {
        recorder.stop();
        recorder.save();
        record = false;
      }
      frame = 0;
    }
    //console.log(Math.pow(anim_speed, -1)*400)
  }, Math.pow(anim_speed, -1) * 400);
});

let html = ``;
if (process.env.NODE_ENV == "development") {
  html = `

  <button id="savevideo">save video</button>
  <canvas id="canvas">
  </canvas>
`;
} else {
  html = `
  <canvas id="canvas">
  </canvas>
`;
}

document.querySelector<HTMLDivElement>("#app")!.innerHTML = html;
document.getElementById("savevideo")?.addEventListener("click", () => {
  record = true;
});
createScene(document.getElementById("canvas"));
//@ts-ignore
const recorder = new CanvasRecorder(
  //@ts-ignore
  document.getElementById("canvas"),
  4200000
);
