import time
import json

f = open("cords.csv")

cords = []

index = 0
for i in f.read().split('\n'):
    cords.append({"x": i.split(",")[0], "y": i.split(",")[1],"z": i.split(",")[2]})
    #requests.post("http://localhost:3000/updateCords", json= {'index': index, 'cords': i})
    #time.sleep(0.4)# pieprasījumi atjaunināt koordinātas jāsūta lēnām, lai 'prisma' tur līdzi
    #print(i)
    #index += 1
print(cords)
open("cords.json", "w").write(json.dumps(cords))
