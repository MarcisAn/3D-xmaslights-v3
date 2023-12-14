import json

template = {
    "name": "",
    "f_name": "",
    "speed": 10,
    "color_names": [],
    "colors": [],
    "params":[]
}

CEND = '\033[0m'

template["name"] = input("Animācijas vienkāršais nosaukums(bez atstarpēm, garumzīmēm...): \033[94m")
print(CEND)
template["f_name"] = input("Animācijas smukais nosaukums, kas parādīsies kontroles applikācijā: \033[94m")
print(CEND)
template["speed"] = int(input("Animācijas noklusējuma ātrums: \033[94m"))
print(CEND)
color_count = input("Krāsu skaits: \033[94m")
print(CEND)



for i in range(int(color_count)):
    template["color_names"].append(input("Krāsas nosaukums, kas parādīsies applikācijā: "))
    template["colors"].append(input("Krāsas noklusējuma vērtība, RGB atdalīta ar semikoliem, vai krāsu funkcija(sk. kodu): "))


anim_file = json.loads(open("animations.json", "r",encoding="utf-8").read())
anim_file.append(template)

json_object = json.dumps(anim_file, indent=4)
 
# Writing to sample.json
with open("animations.json", "w", encoding="utf-8") as outfile:
    outfile.write(json_object)


get_color = "    let ref {color_name}: String = colors[{color_index}];\n"
get_color_block = ""

def process_color_string(input_string):
    latvian_chars = {'ā': 'a', 'č': 'c', 'ē': 'e', 'ģ': 'g', 'ī': 'i', 'ķ': 'k', 'ļ': 'l', 'ņ': 'n', 'š': 's', 'ū': 'u', 'ž': 'z'}

    processed_string = input_string.lower()

    processed_string = processed_string.replace(' ', '-')

    for latvian_char, english_char in latvian_chars.items():
        processed_string = processed_string.replace(latvian_char, english_char)

    return processed_string



for index, i in enumerate(template["color_names"]):
    get_color_block += get_color.format(color_name = process_color_string(i), color_index = index)


template_file = """
use crate::generate::*;
use crate::util::anim_util::apply_color_to_range;

pub fn {anim_name}(
    mut builder: &mut AnimationBuilder,
    colors: Vec<String>,
    cords: Vec<LightCords>,
) {{
    let ref mut _builder = builder;

{get_color_block}

    //let color1 = get_color(color1.to_string(), i, cords[i]);
    //_builder.set_light_state(i, color3);

    //_builder.add_frame();
}}
""".format(anim_name = template["name"], get_color_block = get_color_block)

with open("generator/src/animations/{}.rs".format(template["name"]), "w") as outfile:
    outfile.write(template_file)
    
with open("generator/src/animations/mod.rs", "a") as outfile:
    outfile.write("pub mod {};".format(template["name"]))