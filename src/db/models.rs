use serde::{Deserialize, Serialize};

use diesel::{Insertable, Queryable};

use crate::db::schema::{lists, tasks};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Lists {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "lists"]
pub struct CreateList {
    pub name: String,
}

#[derive(Deserialize, AsChangeset, Serialize)]
#[table_name = "lists"]
pub struct UpdateListName {
    name: String,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tasks {
    pub id: Option<i32>,
    pub title: String,
    pub description: String,
    pub completed: i32,
    pub date: Option<String>,
    pub list_id: i32,
}

#[derive(Deserialize, AsChangeset, Serialize)]
#[table_name = "tasks"]
pub struct UpdateTask {
    title: String,
    description: String,
    completed: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[table_name = "tasks"]
pub struct CreateTask<> {
    pub title: String,
    pub description: String,
    pub completed: i32,
    pub list_id: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub deleted_list: Lists,
    pub deleted_tasks: Vec<Tasks>,
}
