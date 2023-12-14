import { io } from "socket.io-client";
import {SerialPort} from "serialport"
import ws from "rpi-ws281x-native";
const channel = ws(200);


let server_url = "wss://lampinas-server.cvgmerch.lv";
const socket = io(server_url, {
    autoConnect: true,
    reconnection: true,
    reconnectionAttempts: 1000,
    reconnectionDelay: 30,
    transports: ["websocket"],
});
let anim_data  = [];
let anim_speed = 10;
let record = false;
let recording = false;
let frame = 0;
let intervalId;

const port = new SerialPort({ path: '/dev/tty-usbserial1', baudRate: 57600 })

socket.on("reciveAnimation", (data) => {
    clearInterval(intervalId);
    if (record) {
        alert("RECORD START");
        recording = true;
        recorder.start();
    }
    anim_speed = data.speed;
    frame = 0;
    //all_to_black();
    anim_data = [];
    data.animation.forEach((frame) => {
        let tree_data  = [];
        let light_data  = [];
        let char_index = 0; // indekss baitam, kuru pašlaik apskatām: 0-lampiņas idekss, 1-red, 2-green, 3-blue
        frame.split("").forEach(function (c) {
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
    intervalId = setInterval(function () {
        if (anim_data.length > 0) {
            sendFrame(anim_data[frame]);
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
const colorArray = channel.array;

function sendFrame(data){
    data.forEach((light) => {
      colorArray[light[0]] =
        colorArray[light[0]] = parseInt(toHexString([data[1], data[2],data[3]]),16)
        console.log(light)
        //obj.changeColor(light[1], light[2], light[3]);
    });
}

function toHexString(bytes) {
  return Array.from(bytes, function(byte) {
    return ('0' + (byte & 0xFF).toString(16))
  }).join('')
}
