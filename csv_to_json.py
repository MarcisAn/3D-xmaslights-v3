import time
import json

f = open("cords.csv")

cords = []

index = 0
for i in f.read().split('\n'):
    cords.append({"x": int(i.split(",")[0]), "y": int(i.split(",")[1]),"z": int(i.split(",")[2])})
print(cords)
open("cords.json", "w").write(json.dumps(cords))
