use crate::generate::{AnimationBuilder, LightCords};
use crate::util::anim_util::apply_color_to_range;

pub fn flicker(mut builder: &mut AnimationBuilder, colors: Vec<String>, cords: Vec<LightCords>) {
    let ref mut _builder = builder;

    let ref color1: String = colors[0];
    let ref color2: String = colors[1];

    apply_color_to_range(_builder, 0, 199, color1.to_string(), cords.clone());
    _builder.add_frame();

    apply_color_to_range(_builder, 0, 199, color2.to_string(), cords);
    _builder.add_frame();
}
