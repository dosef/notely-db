use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::schema::{items, lists};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Queryable)]
pub struct List {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
    pub title: String,
}

#[derive(Insertable)]
#[table_name = "lists"]
pub struct NewList<'a> {
    pub title: &'a str,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Queryable)]
pub struct Item {
    #[serde(with = "my_uuid")]
    pub id: Uuid,
    #[serde(with = "my_uuid")]
    pub list_id: Uuid,
    pub title: String,
}

#[derive(Insertable)]
#[table_name = "items"]
pub struct NewItem<'a> {
    pub title: &'a str,
    pub list_id: Uuid,
}

mod my_uuid {
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
    use std::str::FromStr;
    use uuid::Uuid;

    pub fn serialize<S>(val: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        val.to_string().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: &str = Deserialize::deserialize(deserializer)?;
        Uuid::from_str(val).map_err(D::Error::custom)
    }
}
