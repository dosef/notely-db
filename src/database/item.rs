use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

use super::super::models::*;

pub fn add_item_to_list<'a>(conn: &PgConnection, list_id: Uuid, title: &'a str) -> Item {
    use super::super::schema::items;

    let new_item = NewItem {
        title: title,
        list_id: list_id,
    };

    diesel::insert_into(items::table)
        .values(&new_item)
        .get_result(conn)
        .expect("Error creating new item")
}

pub fn delete_item<'a>(connection: &PgConnection, item_id: Uuid) {
    use super::super::schema::items::dsl::*;
    diesel::delete(items.filter(id.eq(&item_id)))
        .execute(connection)
        .expect("Error deleting posts");
}

pub fn delete_all_items_in_list<'a>(connection: &PgConnection, remove_id: Uuid) {
    use super::super::schema::items::dsl::*;
    diesel::delete(items.filter(list_id.eq(remove_id)))
        .execute(connection)
        .expect("Error deleting posts");
}

pub fn get_all_items_in_list<'a>(connection: &PgConnection, show_id: Uuid) -> Vec<Item> {
    use super::super::schema::items::dsl::*;

    items
        .filter(list_id.eq(&show_id))
        .load::<Item>(connection)
        .expect("Error loading items")
}
