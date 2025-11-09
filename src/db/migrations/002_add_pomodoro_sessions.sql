-- Pomodoro Sessions Table: Tracks completed focus sessions and breaks for statistics
CREATE TABLE IF NOT EXISTS pomodoro_sessions (
    session_id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    pomodoro_id UUID, -- Optional reference to the preset used
    session_type VARCHAR(20) NOT NULL, -- 'work', 'short_break', 'long_break'
    duration_seconds INTEGER NOT NULL, -- Actual duration of the session
    completed BOOLEAN NOT NULL DEFAULT TRUE, -- Whether the session was completed or skipped
    started_at TIMESTAMP NOT NULL,
    completed_at TIMESTAMP NOT NULL,
    notes TEXT,
    FOREIGN KEY (user_id) REFERENCES users(user_id),
    FOREIGN KEY (pomodoro_id) REFERENCES pomodoros(pomodoro_id)
);

-- Index for faster queries by user and date
CREATE INDEX IF NOT EXISTS idx_pomodoro_sessions_user_date
ON pomodoro_sessions(user_id, started_at DESC);

-- Index for statistics queries
CREATE INDEX IF NOT EXISTS idx_pomodoro_sessions_type
ON pomodoro_sessions(user_id, session_type, started_at DESC);
