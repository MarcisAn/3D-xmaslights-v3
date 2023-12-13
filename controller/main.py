import math
import random
import time

import requests
import socketio
import json
import time

try:
    import board
    import neopixel
    pixels = neopixel.NeoPixel(board.D18, 200, brightness=1)
except:
    print("neopixel error")
# standard Python
sio = socketio.Client()

anim_frame_delay_time = 1.0
anim_frame_index = 0
anim_frames = []

time_anim_started = time.time()

@sio.on('reciveAnimation')
def on_message(data):
    global anim_frame_delay_time
    anim_frame_delay_time = math.pow(int(data['speed']), -1)
    global anim_frames
    anim_frames = []
    for frame in data['animation']:
        light = ''
        loop_index = 1
        for i in frame:
            light = light + i
            if loop_index % 4 == 0:
                loop_index = 1
                anim_frames.append([ord(light[0]), ord(light[1], ord(light[2])), ord(light[3])])
                #print(ord(light[0]),ord(light[1]),ord(light[2]),ord(light[3]))
            loop_index += 1
    global anim_frame_index
    anim_frame_index = 0

    for i in range(0,199):
        pixels[i] = (0,0,0)
        pixels.show()
        print(anim_frames)

def run_new_anim_from_cache():
    cache = requests.get("https://lampinas-server.cvgmerch.lv/getAnimCache")
    len_cache = len(cache.json())
    selected_anim = cache.json()[random.randint(0, len_cache-1)]
    global anim_frames
    anim_frames = []
    for frame in selected_anim['data']:
        light = ''
        loop_index = 1
        for i in frame:
            light = light + i
            if loop_index % 4 == 0:
                loop_index = 1
                anim_frames.append([ord(light[0]), ord(light[1]), ord(light[2]), ord(light[3])])
                #print(ord(light[0]),ord(light[1]),ord(light[2]),ord(light[3]))
            loop_index += 1
    global anim_frame_index
    anim_frame_index = 0
    global anim_frame_delay_time
    anim_frame_delay_time = math.pow(int(selected_anim['speed']), -1)
    global time_anim_started
    time_anim_started = time.time()

    for i in range(0,199):
        pixels[i] = (0,0,0)
        pixels.show()


sio.connect("wss://lampinas-server.cvgmerch.lv", transports=['websocket'])
print('connection ready, sio: ', sio.sid)

run_new_anim_from_cache()

while True:
    if time.time() - time_anim_started > 12:
        run_new_anim_from_cache()
    if len(anim_frames) == 0:
        continue

    frame = anim_frames[anim_frame_index]
    pixels[frame[0]] = (frame[1], frame[2], pixels[3])
    

    pixels.show()
    #print(anim_frame_index,anim_frames[anim_frame_index])
    if anim_frame_index == len(anim_frames)-1:
        anim_frame_index = 0
    else:
        anim_frame_index += 1
    sio.sleep(anim_frame_delay_time * 400 / 1000)
