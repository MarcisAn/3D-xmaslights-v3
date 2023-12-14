
use crate::generate::*;
use crate::util::anim_util::apply_color_to_range;

struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

fn is_point_inside_rotated_3d_box(
    point: &Point3D,
    box_center: &Point3D,
    box_width: f64,
    box_height: f64,
    box_depth: f64,
    rotation_angle_deg_x: f64,
    rotation_angle_deg_y: f64,
    rotation_angle_deg_z: f64,
) -> bool {
    // Convert rotation angles from degrees to radians
    let rotation_angle_rad_x = rotation_angle_deg_x.to_radians();
    let rotation_angle_rad_y = rotation_angle_deg_y.to_radians();
    let rotation_angle_rad_z = rotation_angle_deg_z.to_radians();

    // Translate the point and box center to the box's local coordinate system
    let translated_point = Point3D {
        x: point.x - box_center.x,
        y: point.y - box_center.y,
        z: point.z - box_center.z,
    };

    // Rotate the translated point around the origin (0, 0, 0)
    let rotated_point = Point3D {
        x: translated_point.x * rotation_angle_rad_x.cos() - translated_point.y * rotation_angle_rad_x.sin(),
        y: translated_point.x * rotation_angle_rad_x.sin() + translated_point.y * rotation_angle_rad_x.cos(),
        z: translated_point.z,
    };

    let rotated_point = Point3D {
        x: rotated_point.x * rotation_angle_rad_y.cos() - rotated_point.z * rotation_angle_rad_y.sin(),
        y: rotated_point.y,
        z: rotated_point.x * rotation_angle_rad_y.sin() + rotated_point.z * rotation_angle_rad_y.cos(),
    };

    let rotated_point = Point3D {
        x: rotated_point.x,
        y: rotated_point.y * rotation_angle_rad_z.cos() - rotated_point.z * rotation_angle_rad_z.sin(),
        z: rotated_point.y * rotation_angle_rad_z.sin() + rotated_point.z * rotation_angle_rad_z.cos(),
    };

    // Check if the rotated point is inside the axis-aligned bounding box
    let half_width = box_width / 2.0;
    let half_height = box_height / 2.0;
    let half_depth = box_depth / 2.0;

    rotated_point.x.abs() <= half_width && rotated_point.y.abs() <= half_height && rotated_point.z.abs() <= half_depth
}

pub fn rotating_box_vertical(
    mut builder: &mut AnimationBuilder,
    colors: Vec<String>,
    cords: Vec<LightCords>,
) {
    let ref mut _builder = builder;

    let ref iekspuse: String = colors[0];
    let ref arpuse: String = colors[1];

    let box_center = Point3D { x: 50.0, y: 50.0, z: 0.0 };


    for step in 0..180 {
        for light in 0..199{
            let point = Point3D { x: cords[light].x as f64, y: cords[light].y as f64, z: cords[light].z as f64};
            let inside = get_color(iekspuse.to_string(), light, cords[light]);
            let outside = get_color(arpuse.to_string(), light, cords[light]);

            let result = is_point_inside_rotated_3d_box(
                &point,
                &box_center,
                200.0,
                20.0,
                200.0,
                step as f64 * 2.0 as f64,
                0.0,
                0.0,
            );
            println!("{}",result);
            _builder.set_light_state(light, if (result) {inside} else {outside});
        }
        _builder.add_frame();
    }

    //let color1 = get_color(color1.to_string(), i, cords[i]);
    //_builder.set_light_state(i, color3);

    //_builder.add_frame();
}
