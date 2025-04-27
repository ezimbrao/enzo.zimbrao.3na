use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkoutSet {
    pub load: f32,
    pub reps: i32,
    pub sets: i32,
    pub duration: i32, // em segundos
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduledWorkout {
    pub workout_id: i32,
    pub date: NaiveDate,
    pub sets: Vec<WorkoutSet>,
}