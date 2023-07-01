use crate::db::db_conn::SqlitePool;
use crate::db::models::{CreateList, Lists, UpdateListName};
use crate::diesel::ExpressionMethods;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use diesel::{QueryDsl, RunQueryDsl};

#[get("/lists")]
async fn get_lists(pool: web::Data<SqlitePool>) -> impl Responder {
    use crate::db::schema::lists::dsl::lists;

    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = lists.load::<Lists>(&conn).expect("couldn't load lists");

    HttpResponse::Ok().json(result)
}

#[post("/lists")]
async fn create_list(pool: web::Data<SqlitePool>, new_list: web::Json<CreateList>) -> HttpResponse {
    use crate::db::schema::lists::dsl::lists;

    let conn = pool.get().expect("Failed to get DB connection from pool");

    diesel::insert_into(lists)
        .values(new_list.0)
        .execute(&conn)
        .expect("Error inserting new post");

    let created_list = lists
        .order(crate::db::schema::lists::dsl::id.desc())
        .first::<Lists>(&conn)
        .expect("Error loading created list");

    HttpResponse::Ok().json(created_list)
}

#[patch("/lists/{list_id}")]
async fn update_list_name(
    pool: web::Data<SqlitePool>,
    list_id: web::Path<i32>,
    name: web::Json<UpdateListName>,
) -> HttpResponse {
    use crate::db::schema::lists::dsl::{id as list_id_field, lists};

    let id = list_id.into_inner();
    let conn = pool.get().expect("Failed to get DB connection from pool");

    diesel::update(lists.filter(list_id_field.eq(id)))
        .set(name.0)
        .execute(&conn)
        .expect("Error updating list");

    let updated_list = lists
        .find(id)
        .first::<Lists>(&conn)
        .expect("Error loading updated list");

    HttpResponse::Ok().json(updated_list)
}

#[delete("/lists/{list_id}")]
async fn delete_list_and_tasks(
    pool: web::Data<SqlitePool>,
    list_id: web::Path<i32>,
) -> HttpResponse {
    use crate::db::schema::lists::dsl::{id as list_id_field, lists};

    let id = list_id.into_inner();
    let conn = pool.get().expect("Failed to get DB connection from pool");

    let deleted_list: Lists = lists.find(id).get_result(&conn)
        .expect("Error loading list");

    diesel::delete(lists.filter(list_id_field.eq(id)))
        .execute(&conn)
        .expect("Error deleting list");

    HttpResponse::Ok().json(deleted_list)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_lists);
    cfg.service(create_list);
    cfg.service(update_list_name);
    cfg.service(delete_list_and_tasks);
}
