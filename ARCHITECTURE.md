Architecture Document
1. System Overview
The system consists of four processes that communicate over a local network. At any given time, one or more agents are running on machines being monitored, one backend server is running as the central hub, and one dashboard is open in the browser. The CLI talks to the backend over HTTP to control everything.
[ Agent (Rust) ] ──TCP──▶ [ Backend (FastAPI) ] ──WebSocket──▶ [ Dashboard (React) ]
                                    ▲
                                    │ HTTP
                                [ CLI ]

2. Component Interaction
2.1 Agent → Backend (TCP)

The agent establishes a persistent TCP connection to the backend on startup
It identifies itself immediately by sending a registration message
After registration it streams log lines as they arrive, one JSON object per line, newline delimited
It sends a heartbeat message every 30 seconds so the backend knows it's still alive
If the connection drops, the agent writes incoming logs to a local queue file on disk, and retries the connection using exponential backoff (1s, 2s, 4s, 8s... capped at 60s)
On reconnect, it sends queued messages first in order, then resumes live streaming

2.2 Backend → Dashboard (WebSocket)

The dashboard opens a WebSocket connection to the backend on load
The backend pushes every incoming log line to the dashboard in real time
Anomaly alerts are also pushed over this same WebSocket connection
The backend sends agent status changes (online, offline, reconnecting) over WebSocket

2.3 CLI → Backend (HTTP)

The CLI talks to the backend via REST API calls
project logs tail opens a streaming HTTP response from the backend
project logs search sends a GET request with query parameters
project alerts list sends a GET request
project model train sends a POST request that triggers retraining
project agent list sends a GET request and prints the response

2.4 Dashboard → Backend (HTTP)

Agent settings changes are sent as PUT requests to the backend
The backend forwards the new config to the relevant agent over the existing TCP connection


3. Message Formats
3.1 Agent Registration Message
Sent once immediately after TCP connection is established:
json{
  "type": "register",
  "agent_id": "a1b2c3d4",
  "hostname": "johandev",
  "watched_dirs": ["/var/log", "/home/johan/projects/app/logs"],
  "version": "0.1.0"
}
3.2 Log Message
Sent for every new log line detected:
json{
  "type": "log",
  "agent_id": "a1b2c3d4",
  "timestamp": "2026-05-17T10:42:01Z",
  "level": "ERROR",
  "message": "Database connection failed after 3 retries",
  "source_file": "/var/log/postgres.log"
}
3.3 Heartbeat Message
Sent every 30 seconds:
json{
  "type": "heartbeat",
  "agent_id": "a1b2c3d4",
  "timestamp": "2026-05-17T10:42:30Z"
}
3.4 Config Push (Backend → Agent)
Sent by backend when agent settings are updated from dashboard:
json{
  "type": "config",
  "watched_dirs": ["/var/log", "/tmp/logs"],
  "sensitivity": 0.75
}
3.5 Anomaly Alert (Backend → Dashboard via WebSocket)
json{
  "type": "alert",
  "agent_id": "a1b2c3d4",
  "timestamp": "2026-05-17T10:45:00Z",
  "score": 0.91,
  "threshold": 0.75,
  "window": "2026-05-17T10:44:00Z to 2026-05-17T10:45:00Z"
}

4. Data Flow
4.1 Normal log flow
Log file gets a new line
        ↓
Agent detects it via file watcher
        ↓
Agent parses the line (extracts level, timestamp if present, raw message)
        ↓
Agent wraps it in a JSON log message
        ↓
Agent writes it to TCP stream (newline delimited)
        ↓
Backend receives it, parses JSON
        ↓
Backend writes it to SQLite
        ↓
Backend pushes it to dashboard via WebSocket
        ↓
Backend feeds it into anomaly detection window
        ↓
If anomaly score exceeds threshold → push alert to dashboard
4.2 Disconnect and recovery flow
TCP connection drops
        ↓
Agent detects broken pipe error
        ↓
Agent opens local queue file (queue.jsonl) for writing
        ↓
All new log lines written to queue file instead of TCP
        ↓
Agent begins reconnect loop with exponential backoff
        ↓
Connection restored
        ↓
Agent sends registration message
        ↓
Agent reads queue file line by line and sends each message
        ↓
Agent deletes queue file
        ↓
Agent resumes live streaming

5. Directory Structure
/
├── agent/                  # Rust project
│   ├── src/
│   │   ├── main.rs         # entry point, CLI args
│   │   ├── watcher.rs      # directory and process log discovery
│   │   ├── tailer.rs       # file tailing logic
│   │   ├── connection.rs   # TCP connection, reconnect logic
│   │   ├── queue.rs        # local disk queue for offline buffering
│   │   └── parser.rs       # extract level/timestamp from raw log lines
│   └── Cargo.toml
│
├── backend/                # Python project
│   ├── main.py             # FastAPI app, startup
│   ├── tcp_server.py       # TCP listener, agent connection manager
│   ├── websocket.py        # WebSocket manager for dashboard
│   ├── database.py         # SQLite setup and queries
│   ├── anomaly.py          # PyTorch model loading and inference
│   ├── routes/
│   │   ├── logs.py         # log query endpoints
│   │   ├── agents.py       # agent list and config endpoints
│   │   └── alerts.py       # alert query endpoints
│   └── requirements.txt
│
├── model/                  # PyTorch model
│   ├── train.py            # training script
│   ├── model.py            # Autoencoder architecture
│   ├── features.py         # feature extraction from log streams
│   └── model.pt            # saved trained model (gitignored)
│
├── frontend/               # React project
│   ├── src/
│   │   ├── App.jsx
│   │   ├── components/
│   │   │   ├── LogStream.jsx
│   │   │   ├── AlertsPanel.jsx
│   │   │   ├── AgentSettings.jsx
│   │   │   └── AgentStatus.jsx
│   │   └── hooks/
│   │       └── useWebSocket.js
│   └── package.json
│
├── cli/                    # CLI (Python or Rust, TBD)
│
├── SPEC.md                 # Software Requirements Specification
├── ARCHITECTURE.md         # This document
└── README.md

6. Ports
ServicePortBackend HTTP + WebSocket8000Agent TCP listener9000

7. Key Design Decisions and Why
DecisionReasonTCP for agent → backendPersistent, reliable, low overhead. Better than HTTP polling for continuous streamingNewline delimited JSONSimple to implement, easy to debug with basic tools like nc and catSQLite over PostgreSQLNo setup, single file, more than sufficient for a solo developer toolWebSocket for backend → dashboardPush-based, no polling, natural fit for live log streamingAutoencoder for anomaly detectionUnsupervised — no need to label "normal" vs "anomalous" data manuallyExponential backoff on reconnectAvoids hammering the backend when it's down, standard production pattern