use super::super::models::*;
use super::super::schema::lists;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn create_list<'a>(conn: &PgConnection, title: &'a str) -> List {
    let new_list = NewList { title: title };
    diesel::insert_into(lists::table)
        .values(&new_list)
        .get_result(conn)
        .expect("Error saving new list")
}

pub fn delete_list<'a>(connection: &PgConnection, list_id: Uuid) {
    use super::super::schema::lists::dsl::*;
    diesel::delete(lists.filter(id.eq(&list_id)))
        .execute(connection)
        .expect("Error deleting list");

    super::item::delete_all_items_in_list(&connection, list_id);
}

#[derive(Serialize, Deserialize)]
pub struct AllLists {
    pub lists: Vec<List>,
}

pub fn get_all_lists<'a>(connection: &PgConnection) -> AllLists {
    use super::super::schema::lists::dsl::*;
    let all_lists = lists.load::<List>(connection).expect("Error loading lists");
    let mut all_list = Vec::new();
    for list in all_lists {
        all_list.push(List {
            title: list.title,
            id: list.id,
        })
    }

    AllLists { lists: all_list }
}

#[derive(Serialize, Deserialize)]
pub struct CompleteList {
    pub title: String,
    pub items: Vec<Item>,
}

pub fn get_complete_list<'a>(connection: &PgConnection, show_id: Uuid) -> CompleteList {
    use super::super::schema::lists::dsl::*;

    let list = lists
        .filter(id.eq(&show_id))
        .load::<List>(connection)
        .expect("Error loading list");

    let items = super::item::get_all_items_in_list(&connection, show_id);
    let mut all_items = Vec::new();
    for item in items {
        all_items.push(Item {
            title: item.title,
            id: item.id,
            list_id: item.list_id,
        });
    }

    CompleteList {
        title: list[0].title.to_string(),
        items: all_items,
    }
}
