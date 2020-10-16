use rocket::http::RawStr;
use rocket::request::Form;
use rocket_contrib::json::Json;
use std::collections::HashMap;
use uuid::Uuid;

use super::database;

#[post("/add_list", data = "<name>")]
pub fn add_list(name: String) -> Json<HashMap<String, String>> {
    let connection = database::establish_connection();
    let list = database::list::create_list(&connection, &name);
    let mut result = HashMap::new();
    result.insert("name".to_string(), list.title.to_string());
    result.insert("uuid".to_string(), list.id.to_string());
    Json(result)
}

#[post("/delete_list", data = "<id>")]
pub fn delete_list(id: String) -> Json<HashMap<String, String>> {
    let connection = database::establish_connection();
    let id = Uuid::parse_str(&id).unwrap();
    database::list::delete_list(&connection, id);
    let mut result = HashMap::new();
    result.insert("uuid".to_string(), id.to_string());
    Json(result)
}

#[derive(FromForm)]
pub struct NewItem {
    list_id: String,
    name: String,
}

#[post("/add_item", data = "<item>")]
pub fn add_item(item: Form<NewItem>) -> Json<HashMap<String, String>> {
    let connection = database::establish_connection();
    let list_id = Uuid::parse_str(&item.list_id).unwrap();
    let item = database::item::add_item_to_list(&connection, list_id, &item.name);
    let mut result = HashMap::new();
    result.insert("uuid".to_string(), item.id.to_string());
    Json(result)
}

#[post("/delete_item", data = "<id>")]
pub fn delete_item(id: String) -> Json<HashMap<String, String>> {
    let connection = database::establish_connection();
    let item_id = Uuid::parse_str(&id).unwrap();
    database::item::delete_item(&connection, item_id);
    let mut result = HashMap::new();
    result.insert("uuid".to_string(), item_id.to_string());
    Json(result)
}

#[get("/lists")]
pub fn get_lists() -> Json<database::list::AllLists> {
    let connection = database::establish_connection();
    let all_lists = database::list::get_all_lists(&connection);
    Json(all_lists)
}

#[get("/list?<id>")]
pub fn get_list(id: &RawStr) -> Json<database::list::CompleteList> {
    let connection = database::establish_connection();
    let list_id = Uuid::parse_str(id).unwrap();
    let list = database::list::get_complete_list(&connection, list_id);
    Json(list)
}
