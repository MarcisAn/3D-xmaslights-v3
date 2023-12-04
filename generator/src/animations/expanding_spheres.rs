use crate::generate::*;
use crate::util::anim_util::apply_color_to_range;

fn distance_between_points_3d(x1: i16, y1: i16, z1: i16, x2: i16, y2: i16, z2: i16) -> u16 {
    ((x2 - x1).pow(2) + (y2 - y1).pow(2) + (z2 - z1).pow(2))
        .try_into()
        .unwrap()
}

pub fn expanding_spheres(
    mut builder: &mut AnimationBuilder,
    colors: Vec<String>,
    cords: Vec<LightCords>,
) {
    let ref mut _builder = builder;

    let ref lode1: String = colors[0];
    let ref lode2: String = colors[1];
    let ref lode3: String = colors[2];

    for step in 0..80 {
        for i in 0..199 {
            if distance_between_points_3d(50, 50, 40, cords[i].x, cords[i].y, cords[i].z)
                < step * step
            {
                let color1 = get_color(lode1.to_string(), i, cords[i]);
                _builder.set_light_state(i, color1);
            } /*else {
                _builder.set_light_state(i, Color { r: 0, g: 0, b: 0 });
            }*/
        }
        _builder.add_frame();
    }
    for step in 0..80 {
        for i in 0..199 {
            if distance_between_points_3d(50, 50, 40, cords[i].x, cords[i].y, cords[i].z)
                < step * step
            {
                let color2 = get_color(lode2.to_string(), i, cords[i]);
                _builder.set_light_state(i, color2);
            }
        }
        _builder.add_frame();
    }
    for step in 0..80 {
        for i in 0..199 {
            if distance_between_points_3d(50, 50, 40, cords[i].x, cords[i].y, cords[i].z)
                < step * step
            {
                let color3 = get_color(lode3.to_string(), i, cords[i]);
                _builder.set_light_state(i, color3);
            }
        }
        _builder.add_frame();
    }

    //let color1 = get_color(color1.to_string(), i, cords[i]);
    //_builder.set_light_state(i, color3);

    //_builder.add_frame();
}
