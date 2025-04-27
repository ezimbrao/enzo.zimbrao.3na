use axum::Router;
sqlx::migrate!().run(&pool).await.expect("Falha nas migrações");
use sqlx::postgres::PgPoolOptions;

mod models;
mod services;
mod repositories;
mod routes;
mod db;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:root@localhost:5432/workouts")
        .await
        .unwrap();

    // Inicialização das dependências
    let workout_repo = repositories::workout_repo::WorkoutRepository::new(pool.clone());
    let workout_service = services::workout_service::WorkoutService::new(workout_repo);
    let workout_router = routes::workouts::WorkoutRouter::new(workout_service);

    let app = Router::new()
        .nest("/api", workout_router.router());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
