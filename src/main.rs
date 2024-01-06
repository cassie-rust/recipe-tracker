use axum::{routing::post, Router};
use infra::repository::DbRepo;
use infra::service::DbService;
use sqlx::PgPool;

mod infra;
mod models;
mod routes;

#[derive(Clone)]
struct AppState {
    svc: DbService,
}

impl AppState {
    fn new(pool: PgPool) -> Self {
        Self {
            svc: DbRepo::new(pool).into(),
        }
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://recipe-tracker:password@localhost:5432/recipe-tracker"
    )]
    pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!().run(&pool).await.unwrap();

    let router = Router::new()
        .route("/recipes", post(routes::create_recipe))
        .with_state(AppState::new(pool));

    Ok(router.into())
}
