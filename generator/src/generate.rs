use angle::Deg;
use meval::{Context, Expr};
extern crate angular_units as angle;
extern crate meval;

use crate::{animations, cords_helper::map_min_max_cords};

#[derive(Debug)]
pub struct LightState {
    pub index: u8,
    pub color: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct LightCords {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

pub struct AnimationBuilder {
    temp_lights: Vec<LightState>,
    frames: Vec<String>,
    pub cords: Vec<LightCords>,
}

impl AnimationBuilder {
    pub fn new(cords: Vec<LightCords>) -> AnimationBuilder {
        return AnimationBuilder {
            frames: vec![],

            cords: cords,
            temp_lights: vec![],
        };
    }
    pub fn set_light_state(&mut self, index: usize, color: Color) {
        for state in &mut self.temp_lights {
            if state.index as usize == index {
                if state.color != color {
                    state.color = color;
                    return;
                }
            }
        }
        self.temp_lights.push(LightState {
            index: index as u8,
            color: color,
        });
    }
    pub fn add_frame(&mut self) {
        let mut frame_string = String::new();
        for light in &self.temp_lights {
            frame_string.push(light.index as char);
            frame_string.push(light.color.r as char);
            frame_string.push(light.color.g as char);
            frame_string.push(light.color.b as char);
        }
        self.frames.push(frame_string);
        self.temp_lights = vec![];
    }

    fn map(x: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
        let mut mapped: i32 = (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
        if mapped > out_max {
            mapped = out_max;
        }
        if mapped < out_min {
            mapped = out_min;
        }
        return mapped;
    }
}
/*
Katrai animācija ir vairākas krāsas, ko nosaka lietotājs. Krāsas var būt vienkāršas(izteiktas RGB sistēmā), vai
krāsu funkcijas.

Vienkāršai krāsai ir trīs vērtības, atdalītas ar semikoliem   piem. - "255;255;255"

Krāsu izteiksmei priekšā ir @RGB@ vai @HSV@, norādot kādā sistēmā izteiksmes ir rakstītas.
Pēc tam seko trīs izteiksmes - atdalītas ar semikoliem.  piem. - "@RGB@x;y;z"
*/
pub fn get_color(
    color_string: String,
    current_light_index: usize,
    current_light_cords: LightCords,
) -> Color {
    if !color_string.starts_with('@') {
        //ja nav krāsas funkcija
        let mut split = color_string.split(';');
        return Color {
            r: split.next().unwrap().parse::<u8>().unwrap(),
            g: split.next().unwrap().parse::<u8>().unwrap(),
            b: split.next().unwrap().parse::<u8>().unwrap(),
        };
    }

    let mut values: Vec<f32> = vec![]; //vektors, kurā ieliekam RGB vērtības pēc parsošanas

    for expression in color_string[5..color_string.len()].split(";") {
        let expr: Expr = expression.parse().unwrap();

        // create a context with function definitions and variables
        let mut ctx = Context::new(); // built-ins
        ctx.var("x", current_light_cords.x as f64)
            .var("y", current_light_cords.y as f64)
            .var("z", current_light_cords.z as f64);
        // bind function with a custom context
        let func = expr.bind_with_context(ctx, "t").unwrap();
        values.push(func(1.0) as f32);
    }

    if color_string.starts_with("@RGB@") {
        return Color {
            r: values[0] as u8,
            g: values[1] as u8,
            b: values[2] as u8,
        };
    } else {
        println!("{:?}", values);
        let hsv = prisma::Hsv::new(
            Deg(AnimationBuilder::map(values[0] as i32, 0, 100, 0, 359) as f32),
            values[1] / 100.0,
            values[2],
        );
        let rgb = prisma::Rgb::from(hsv);
        return Color {
            r: rgb.red() as u8,
            g: rgb.green() as u8,
            b: rgb.blue() as u8,
        };
    }
}
pub fn run_anim(anim_name: String, colors: Vec<String>, cords: Vec<LightCords>) -> Vec<String> {
    let mapped_cords = map_min_max_cords(cords.clone());
    let mut builder = AnimationBuilder::new(mapped_cords.clone());

    if (anim_name == "axis") {
        animations::climbing_axis::climbing_axis(&mut builder, colors, mapped_cords);
    } else if (anim_name == "flicker") {
        animations::flicker::flicker(&mut builder, colors, mapped_cords);
    } else if (anim_name == "expanding_spheres") {
        animations::expanding_spheres::expanding_spheres(&mut builder, colors, mapped_cords);
    }
    else if (anim_name == "climbing") {
            animations::climbing::climbing(&mut builder, colors, mapped_cords);
        }
    else if (anim_name == "rotating_box") {
        animations::rotating_box::rotating_box(&mut builder, colors, mapped_cords);
    }
    else if (anim_name == "rotating_box_vertical") {
        animations::rotating_box_vertical::rotating_box_vertical(&mut builder, colors, mapped_cords);
    }
    else if (anim_name == "stripes") {
            animations::stripes::stripes(&mut builder, colors, mapped_cords);
        }
    //
    //println!("{:?}", builder.frames);
    //println!("{:?}", builder.get_color(0, 0));
    println!("{}", builder.frames.len());
    return builder.frames;
}
