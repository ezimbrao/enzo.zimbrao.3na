// models/workout.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Workout {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub muscle_group: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWorkout {
    pub name: String,
    pub description: String,
    pub muscle_group: String,
}
