use crate::generate::LightCords;

//ieliekam lampiņu koordinātās diapazonā 0-100
pub fn map_min_max_cords(cords: Vec<LightCords>) -> Vec<LightCords> {
    let mut mapped_out: Vec<LightCords> = vec![];

    let mut x_min: i16 = 0;
    let mut x_max: i16 = 0;
    let mut y_min: i16 = 0;
    let mut y_max: i16 = 0;
    let mut z_min: i16 = 0;
    let mut z_max: i16 = 0;
    for cord in &cords {
        if cord.x < x_min {
            x_min = cord.x;
        }
        if cord.x > x_max {
            x_max = cord.x;
        }
        if cord.y < y_min {
            y_min = cord.y;
        }
        if cord.y > y_max {
            y_max = cord.y;
        }
        if cord.z < z_min {
            z_min = cord.z;
        }
        if cord.z > z_max {
            z_max = cord.z;
        }
    }
    // padaram atšķirību pamanāmāku
    x_max -= 10;
    x_min += 10;
    y_max -= 10;
    y_min += 10;
    z_max -= 10;
    z_min += 10;
    for cord in cords {
        let x: i32 =
            ((cord.x as i32 - x_min as i32) * (100 - 0) / (x_max as i32 - x_min as i32)).into();
        let y: i32 =
            ((cord.y as i32 - y_min as i32) * (100 - 0) / (y_max as i32 - y_min as i32)).into();
        let z: i32 =
            ((cord.z as i32 - z_min as i32) * (100 - 0) / (z_max as i32 - z_min as i32)).into();

        mapped_out.push(LightCords {
            x: x as i16,
            y: y as i16,
            z: z as i16,
        })
    }
    return mapped_out;
}
