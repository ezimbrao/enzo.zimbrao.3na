-- Add migration script here
CREATE TABLE workout_sets (
    id SERIAL PRIMARY KEY,
    scheduled_workout_id INTEGER REFERENCES scheduled_workouts(id) ON DELETE CASCADE,
    load FLOAT NOT NULL,
    reps INTEGER NOT NULL,
    sets INTEGER NOT NULL,
    duration INTEGER NOT NULL
);
