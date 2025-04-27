// routes/workouts.rs
use axum::{
    extract::{State, Json}, 
    routing::{post, get},
    Router
};
use crate::services::workout_service::WorkoutService;
use crate::models::workout::{Workout, CreateWorkout};

pub struct WorkoutRouter {
    service: WorkoutService,
}

impl WorkoutRouter {
    pub fn new(service: WorkoutService) -> Self {
        Self { service }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/workouts", post(create_workout))
            .with_state(self.service)
    }
}

async fn create_workout(
    State(service): State<WorkoutService>,
    Json(workout): Json<CreateWorkout>,
) -> Result<Json<Workout>, String> {
    let workout = service.create_workout(workout).await?;
    Ok(Json(workout))
}
