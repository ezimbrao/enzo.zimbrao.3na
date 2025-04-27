// repositories/workout_repo.rs
use sqlx::{PgPool, Result};
use super::models::workout::{Workout, CreateWorkout};

pub struct WorkoutRepository {
    pool: PgPool,
}

impl WorkoutRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, workout: CreateWorkout) -> Result<Workout> {
        sqlx::query_as!(
            Workout,
            r#"
            INSERT INTO workouts (name, description, muscle_group)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            workout.name,
            workout.description,
            workout.muscle_group
        )
        .fetch_one(&self.pool)
        .await
    }
}