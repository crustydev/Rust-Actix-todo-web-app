use serde_json::value::Value;
use serde_json::Map;
use actix_web::HttpRequest;

use crate::to_do;
use crate::state::read_file;
use crate::processes::process_input;

pub async fn create(req: HttpRequest) -> String {
    // get state from json file
    let state: Map<String, Value> = read_file("./state/json");
    // get placeholder title from request and convert to String
    let title = req.match_info().get("title").unwrap();
    let title_reference = title.clone();

    let item = to_do::to_do_factory(&String::from("pending"),
                title).expect("create");
    process_input(item, "create".to_string(), &state);
    return format!("{} created", title_reference)
}