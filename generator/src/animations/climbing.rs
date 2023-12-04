use crate::generate::*;
use crate::util::anim_util::apply_color_to_range;

fn distance_between_points_3d(x1: i16, y1: i16, z1: i16, x2: i16, y2: i16, z2: i16) -> u16 {
    ((x2 - x1).pow(2) + (y2 - y1).pow(2) + (z2 - z1).pow(2))
        .try_into()
        .unwrap()
}

pub fn climbing(
    mut builder: &mut AnimationBuilder,
    colors: Vec<String>,
    cords: Vec<LightCords>,
) {
    let ref mut _builder = builder;

    let ref col1: String = colors[0];
    let ref col2: String = colors[1];


    for i in 1..196 {
        _builder.set_light_state(i, get_color(col1.to_string(), i, cords[i]));
        _builder.set_light_state(i+1, get_color(col1.to_string(), i, cords[i]));
        _builder.set_light_state(i+2, get_color(col1.to_string(), i, cords[i]));
        _builder.set_light_state(i+3, get_color(col1.to_string(), i, cords[i]));
        _builder.set_light_state(i-1, get_color(col2.to_string(), i, cords[i]));
        _builder.add_frame();
    }


}
