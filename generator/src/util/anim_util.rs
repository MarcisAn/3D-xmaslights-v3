use crate::generate::{get_color, AnimationBuilder, LightCords};

pub fn apply_color_to_range(
    builder: &mut AnimationBuilder,
    start: usize,
    end: usize,
    color: String,
    cords: Vec<LightCords>,
) {
    for i in start..end {
        let color = get_color(color.to_string(), i, cords[i]);

        builder.set_light_state(i, color)
    }
}
