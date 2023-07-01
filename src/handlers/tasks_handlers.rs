use actix_web::{get, post, patch, web, HttpResponse, Responder, delete};
use diesel::{QueryDsl, RunQueryDsl};

use crate::db_conn::SqlitePool;
use crate::db::models::{Tasks, UpdateTask};
use crate::diesel::ExpressionMethods;

#[get("/tasks")]
async fn get_tasks(pool: web::Data<SqlitePool>) -> impl Responder {
    use crate::db::schema::tasks::dsl::tasks;

    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = tasks.load::<Tasks>(&conn).expect("couldn't load tasks");

    HttpResponse::Ok().json(result)
}

#[post("/tasks")]
async fn create_task(
    pool: web::Data<SqlitePool>,
    task: web::Json<crate::db::models::CreateTask>,
) -> HttpResponse {
    use crate::db::schema::tasks::dsl::tasks;

    let conn = pool.get().expect("Failed to get DB connection from pool");

    diesel::insert_into(tasks)
        .values(task.0)
        .execute(&conn)
        .expect("Error inserting new task");

    let posted_task = tasks.order(crate::db::schema::tasks::dsl::id.desc())
        .first::<Tasks>(&conn)
        .expect("Error loading posted task");

    HttpResponse::Ok().json(posted_task)
}

#[delete("/tasks/{task_id}")]
async fn delete_task(
    pool: web::Data<SqlitePool>,
    task_id: web::Path<i32>,
) -> HttpResponse {
    use crate::db::schema::tasks::dsl::{tasks, id as task_id_field};

    let id = task_id.into_inner();
    let conn = pool.get().expect("Failed to get DB connection from pool");

    let deleted_task: Tasks = tasks.find(id).get_result(&conn)
        .expect("Error loading task");

    diesel::delete(tasks.filter(task_id_field.eq(id)))
        .execute(&conn)
        .expect("Error deleting task");

    HttpResponse::Ok().json(deleted_task)
}

#[patch("/tasks/{task_id}")]
async fn update_task(
    pool: web::Data<SqlitePool>,
    task_id: web::Path<i32>,
    task: web::Json<UpdateTask>,
) -> HttpResponse {
    use crate::db::schema::tasks::dsl::{tasks, id as task_id_field};

    let id = task_id.into_inner();

    let conn = pool.get().expect("Failed to get DB connection from pool");

    diesel::update(tasks.filter(task_id_field.eq(id)))
        .set(task.0)
        .execute(&conn)
        .expect("Error updating task");

    let updated_task: Tasks = tasks.find(id).get_result(&conn)
        .expect("Error loading updated task");

    HttpResponse::Ok().json(updated_task)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_tasks);
    cfg.service(create_task);
    cfg.service(delete_task);
    cfg.service(update_task);
}
