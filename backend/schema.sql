--agents table
CREATE TABLE IF NOT EXISTS agents(
    agent_id    TEXT PRIMARY KEY,
    hostname    TEXT NOT NULL,
    status      TEXT NOT NULL DEFAULT 'offline',
    last_seen   TEXT,
    watched_dirs    TEXT NOT NULL DEFAULT '[]',
    sensitivity     REAL NOT NULL DEFAULT 0.75,
    version     TEXT,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
);

--logs table 
CREATE TABLE IF NOT EXISTS logs (
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_id    TEXT NOT NULL,
    timestamp   TEXT,
    received_at TEXT NOT NULL DEFAULT (datetime('now')),
    level       TEXT NOT NULL DEFAULT 'UNKNOWN',
    message     TEXT NOT NULL,
    source_file TEXT NOT NULL,
    FOREIGN KEY (agent_id) REFERENCE agents(agent_id)
);

-- alerts table 
CREATE TABLE IF NOT EXISTS alerts (
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_id    TEXT NOT NULL,
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    score       REAL NOT NULL,
    threshold   REAL NOT NULL,
    window_start TEXT NOT NULL,
    window_end TEXT NOT NULL,
    FOREIGN KEY (agent_id)  REFERENCES agents(agent_id)
);

--indices for common queries
CREATE INDEX IF NOT EXISTS idx_logs_agent_id ON logs(agent_id);
CREATE INDEX IF NOT EXISTS idx_logs_level ON logs(level);
CREATE INDEX IF NOT EXISTS idx_logs_received_at ON logs(received_at);
CREATE INDEX IF NOT EXISTS idx_alerts_agent_id ON alerts(agent_id);