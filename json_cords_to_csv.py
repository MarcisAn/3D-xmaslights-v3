import json
f = open('cords.json')
data = json.load(f)

string = r""

for i in data:
    string = string + str(i["x"])+ ','+str(i["y"])+','+str(i["z"]) + '\n'
 
print(repr(string))
# Closing file
f.close()