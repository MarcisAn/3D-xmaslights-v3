import math
import time

import socketio
import json
import time

# standard Python
sio = socketio.Client()

anim_frame_delay_time = 1.0
anim_frame_index = 0
anim_frames = []

@sio.on('reciveAnimation')
def on_message(data):
    global anim_frame_delay_time
    anim_frame_delay_time = math.pow(int(data['speed']), -1)
    global anim_frames
    anim_frames = data['animation']



sio.connect("wss://lampinas-server.cvgmerch.lv", transports=['websocket'])
print('connection ready, sio: ', sio.sid)

while True:
    if len(anim_frames) == 0:
        continue
    print(ord(anim_frames[anim_frame_index][1]))
    print(anim_frame_index,anim_frames[anim_frame_index])
    if anim_frame_index == len(anim_frames)-1:
        anim_frame_index = 0
    else:
        anim_frame_index += 1
    sio.sleep(anim_frame_delay_time * 400 / 1000)
