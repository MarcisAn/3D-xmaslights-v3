use crate::generate::LightCords;
use sqlite::Connection;

pub fn load_cords(connection: &Connection) -> Vec<LightCords> {
    let get_cords_query = "
    SELECT * FROM Light;
";

    let mut cords: Vec<LightCords> = vec![]; //vektors, kurā no datubāzes tiks ielādētas visas lampiņu koordinātas
    connection
        .iterate(get_cords_query, |pairs| {
            let mut light_cords = LightCords { x: 0, y: 0, z: 0 };
            for &(name, value) in pairs.iter() {
                let parsed_value: i16 = value.unwrap().parse::<i16>().unwrap();
                if name == "x" {
                    light_cords.x = parsed_value;
                } else if name == "y" {
                    light_cords.y = parsed_value;
                } else if name == "z" {
                    light_cords.z = parsed_value;
                }
            }
            cords.push(light_cords);

            true
        })
        .unwrap();
    cords
}
