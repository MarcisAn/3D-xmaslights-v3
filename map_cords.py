import json
f = open('cords.json')
data = json.load(f)

string = ""

def map_range(x, in_min, in_max, out_min, out_max):
  return (x - in_min) * (out_max - out_min) // (in_max - in_min) + out_min

for i in data:
    string = string + str(map_range(i["x"], 60, 170, -170, 170))+ ','+str(map_range(i["y"], 60, 170, -170, 170))+','+str(map_range(i["z"], 9, 184, -179, 450)) + '\n'

print(string)

f.close()