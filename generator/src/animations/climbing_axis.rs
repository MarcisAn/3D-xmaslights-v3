use crate::generate::{get_color, AnimationBuilder, Color, LightCords};
use crate::util::anim_util::apply_color_to_range;

pub fn climbing_axis(
    mut builder: &mut AnimationBuilder,
    colors: Vec<String>,
    cords: Vec<LightCords>,
) {
    let ref mut _builder = builder;

    let ref color1: String = colors[0];
    let ref color2: String = colors[1];
    let ref color3: String = colors[2];

    for step in 0..100 {
        for i in 0..199 {
            if cords[i].x > step - 10 && cords[i].x < step + 10 {
                let color1 = get_color(color1.to_string(), i, cords[i]);
                _builder.set_light_state(i, color1);
            } else {
                //_builder.set_light_state(i, Color { r: 0, g: 0, b: 0 });
            }
        }
        _builder.add_frame();
    }
    for step in 0..100 {
        for i in 0..199 {
            if cords[i].y > step - 10 && cords[i].y < step + 10 {
                let color2 = get_color(color2.to_string(), i, cords[i]);
                _builder.set_light_state(i, color2);
            } else {
                //_builder.set_light_state(i, Color { r: 0, g: 0, b: 0 });
            }
        }
        _builder.add_frame();
    }
    for step in 0..100 {
        for i in 0..199 {
            if cords[i].z > step - 10 && cords[i].z < step + 10 {
                let color3 = get_color(color3.to_string(), i, cords[i]);
                _builder.set_light_state(i, color3);
            } else {
                //_builder.set_light_state(i, Color { r: 0, g: 0, b: 0 });
            }
        }
        _builder.add_frame();
    }
}
