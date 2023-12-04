//use animationengine::{run_anim, LightCords};
mod util;
use serde::{Deserialize, Serialize};
use serde_json::json;
//use tiny_http::Method::Post;
use tiny_http::{Response, Server};
mod animations;
mod cords_helper;
mod generate;

use crate::util::load_cords::load_cords;
use generate::run_anim;

#[derive(Serialize, Deserialize, Debug)]
struct AnimationRequest {
    name: String,
    colors: Vec<String>,
    speed: u32,
}
fn main() {
    let connection = sqlite::open("../server/prisma/db.db").unwrap();

    let server = Server::http("0.0.0.0:8000").unwrap();

    for mut request in server.incoming_requests() {
        //println!(
        //    "received request! method: {:?}, url: {:?}, headers: {:?}",
        //    request.method(),
        //    request.url(),
        //    request.headers(),
        //);

        //no node servera saņemam pieprasījumu ģenerēt animāciju
        let mut content = String::new();
        request.as_reader().read_to_string(&mut content).unwrap();
        println!("{}", content);
        let anim_request: AnimationRequest = serde_json::from_str(&content).unwrap();
        println!("{:?}", anim_request);

        //izsaucam funkciju, kas ģenerē nepieciešamo animāciju un ielādējam vektorā tās animācijas kadrus
        let frames = run_anim(
            anim_request.name,
            anim_request.colors,
            load_cords(&connection),
        );

        //atgriežam node serverim animācijas kadrus
        let res_json = json!(frames);
        let response = Response::from_string(res_json.to_string());
        let _ = request.respond(response);
    }
}
