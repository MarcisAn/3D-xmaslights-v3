import requests
import time

f = open("cords.csv")

index = 0
for i in f.read().split('\n'):
    requests.post("https://lampinas-server.cvgmerch.lv/updateCords", json= {'index': index, 'cords': i})
    time.sleep(0.4)# pieprasījumi atjaunināt koordinātas jāsūta lēnām, lai 'prisma' tur līdzi
    print(i)
    index += 1
