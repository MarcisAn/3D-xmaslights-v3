
use crate::generate::*;
use crate::util::anim_util::apply_color_to_range;

pub fn stripes(
    mut builder: &mut AnimationBuilder,
    colors: Vec<String>,
    cords: Vec<LightCords>,
) {
    let ref mut _builder = builder;

    let ref krasa1: String = colors[0];
    let ref krasa2: String = colors[1];
    let ref krasa3: String = colors[2];
    let ref krasa4: String = colors[3];

    for offset in 0..160{
        for i in 0..199{
            if (cords[i].z < offset){
                let color1 = get_color(krasa1.to_string(), i, cords[i]);
                _builder.set_light_state(i, color1);
            }
            if (cords[i].z < offset -20 ){
                let color2 = get_color(krasa2.to_string(), i, cords[i]);
                _builder.set_light_state(i, color2);
            }
            if (cords[i].z < offset -40){
                let color3 = get_color(krasa3.to_string(), i, cords[i]);
                _builder.set_light_state(i, color3);
            }
            if (cords[i].z < offset -60){
                let color4 = get_color(krasa4.to_string(), i, cords[i]);
                _builder.set_light_state(i, color4);
            }
        }
        _builder.add_frame();
    }

    //let color1 = get_color(color1.to_string(), i, cords[i]);
    //_builder.set_light_state(i, color3);

    //_builder.add_frame();
}
