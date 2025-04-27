// services/workout_service.rs
use crate::repositories::workout_repo::WorkoutRepository;
use crate::models::workout::{Workout, CreateWorkout};

pub struct WorkoutService {
    repo: WorkoutRepository,
}

impl WorkoutService {
    pub fn new(repo: WorkoutRepository) -> Self {
        Self { repo }
    }

    pub async fn create_workout(&self, workout: CreateWorkout) -> Result<Workout, String> {
        self.repo.create(workout)
            .await
            .map_err(|e| format!("Erro ao criar treino: {}", e))
    }
}

impl WorkoutService {
    pub async fn generate_workout(
        &self,
        preferences: UserPreferences,
        history: Vec<CompletedWorkout>
    ) -> Result<Vec<Workout>, String> {
        // Lógica de recomendação baseada em:
        // 1. Preferências do usuário (musculação, cardio, HIIT)
        // 2. Histórico recente (evitar repetições)
        // 3. Progressão de carga (aumento gradual)
        // 4. Frequência semanal desejada
        
        // Exemplo simplificado:
        let recommended = sqlx::query_as!(
            Workout,
            r#"
            SELECT * FROM workouts
            WHERE muscle_group = $1
            ORDER BY RANDOM()
            LIMIT 3
            "#,
            preferences.preferred_muscle_group
        )
        .fetch_all(&self.repo.pool)
        .await
        .map_err(|e| format!("Erro na recomendação: {}", e))?;

        Ok(recommended)
    }
}
