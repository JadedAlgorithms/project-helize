API Contracts
Base URL
http://localhost:8000

Logs
GET /logs
Fetch stored logs with optional filters.
Query parameters:
ParameterTypeRequiredDescriptionagent_idstringnofilter by agentlevelstringnoINFO, WARN, ERROR, DEBUG, UNKNOWNsource_filestringnofilter by source file pathfromstringnoISO 8601 datetime, lower boundtostringnoISO 8601 datetime, upper boundlimitintegernodefault 100, max 1000offsetintegernofor pagination, default 0
Response:
json{
  "total": 2430,
  "limit": 100,
  "offset": 0,
  "logs": [
    {
      "id": 1,
      "agent_id": "a1b2c3d4",
      "timestamp": "2026-05-17T10:42:01Z",
      "received_at": "2026-05-17T10:42:01Z",
      "level": "ERROR",
      "message": "Database connection failed",
      "source_file": "/var/log/postgres.log"
    }
  ]
}

GET /logs/stream
Server-sent stream of live logs for the CLI project logs tail command.
Response: newline delimited JSON stream, one log object per line, stays open until client disconnects.

Agents
GET /agents
List all known agents and their current status.
Response:
json{
  "agents": [
    {
      "agent_id": "a1b2c3d4",
      "hostname": "johandev",
      "status": "online",
      "last_seen": "2026-05-17T10:42:30Z",
      "watched_dirs": ["/var/log", "/home/johan/projects/app/logs"],
      "sensitivity": 0.75,
      "version": "0.1.0",
      "created_at": "2026-05-17T09:00:00Z"
    }
  ]
}

GET /agents/{agent_id}
Get a single agent's details.
Response: same shape as one agent object above.

PUT /agents/{agent_id}/config
Update an agent's configuration from the dashboard. Backend forwards the new config to the agent over the existing TCP connection.
Request body:
json{
  "watched_dirs": ["/var/log", "/tmp/logs"],
  "sensitivity": 0.80
}
Response:
json{
  "success": true,
  "agent_id": "a1b2c3d4"
}

Alerts
GET /alerts
Fetch anomaly alerts.
Query parameters:
ParameterTypeRequiredDescriptionagent_idstringnofilter by agentfromstringnoISO 8601 datetimetostringnoISO 8601 datetimelimitintegernodefault 50offsetintegernodefault 0
Response:
json{
  "total": 12,
  "limit": 50,
  "offset": 0,
  "alerts": [
    {
      "id": 1,
      "agent_id": "a1b2c3d4",
      "timestamp": "2026-05-17T10:45:00Z",
      "score": 0.91,
      "threshold": 0.75,
      "window_start": "2026-05-17T10:44:00Z",
      "window_end": "2026-05-17T10:45:00Z"
    }
  ]
}

Model
POST /model/train
Triggers a retraining of the anomaly detection model using all logs currently in the database.
Response:
json{
  "success": true,
  "message": "Training started",
  "logs_used": 15420
}

WebSocket
WS /ws
Single WebSocket endpoint for the dashboard. Backend pushes three types of messages over this connection:
Log message:
json{
  "type": "log",
  "data": {
    "id": 1,
    "agent_id": "a1b2c3d4",
    "timestamp": "2026-05-17T10:42:01Z",
    "level": "ERROR",
    "message": "Database connection failed",
    "source_file": "/var/log/postgres.log"
  }
}
Alert message:
json{
  "type": "alert",
  "data": {
    "id": 1,
    "agent_id": "a1b2c3d4",
    "timestamp": "2026-05-17T10:45:00Z",
    "score": 0.91,
    "threshold": 0.75,
    "window_start": "2026-05-17T10:44:00Z",
    "window_end": "2026-05-17T10:45:00Z"
  }
}
Agent status message:
json{
  "type": "agent_status",
  "data": {
    "agent_id": "a1b2c3d4",
    "status": "offline",
    "timestamp": "2026-05-17T10:50:00Z"
  }
}