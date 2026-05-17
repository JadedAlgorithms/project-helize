Software Requirements Specification
1. Overview
A developer tool for autonomous log aggregation, real-time monitoring, and anomaly detection across a local machine or multiple machines. The system is designed for a solo developer who wants full visibility into what their running services are doing, without manual configuration or third party dependencies.

2. Goals

Zero-config log collection — the agent figures out what to watch on its own
Real-time visibility into logs from a web dashboard
Anomaly detection powered by a locally trained PyTorch model
Full control via a CLI without any authentication overhead
Resilient log delivery — no log lines lost during network outages


3. Components
3.1 Agent (Rust)
A background program that runs on any machine being monitored.
Responsibilities:

Watch a configured directory (default: /var/log) for any .log files and begin tailing them automatically when they appear
Scan running processes on the machine periodically and detect associated log files
Stream log lines to the backend over TCP as they are written
On connection loss, buffer undelivered logs to a local queue on disk
On reconnect, drain the local queue in order before resuming live streaming
Identify itself to the backend with a unique agent ID and hostname on connection
Accept configuration updates pushed from the backend (watched directories, sensitivity settings)

3.2 Backend (FastAPI + Python)
The central server that all agents report to.
Responsibilities:

Accept incoming TCP streams from one or more agents simultaneously
Parse and store log lines in a local database with metadata (timestamp, level, message, source file, agent ID)
Expose a REST API for the frontend and CLI to query logs
Maintain a WebSocket connection to the frontend for pushing live log lines
Load the trained PyTorch anomaly detection model and run inference on incoming log batches
Generate anomaly alerts and push them to the frontend via WebSocket
Push configuration changes to agents when updated from the dashboard

3.3 Anomaly Detection Model (PyTorch)
A locally trained Autoencoder that learns what normal log patterns look like and flags deviations.
Responsibilities:

Accept numerical features derived from log streams — log frequency per minute, distribution of log levels (INFO/WARN/ERROR), inter-log time intervals
Learn to reconstruct normal patterns during training
At inference time, compute a reconstruction error — high error means the pattern looks abnormal
Expose a threshold above which a pattern is flagged as an anomaly
Be retrained periodically as the system accumulates more data

3.4 Dashboard (React)
A web interface for monitoring and control.
Responsibilities:

Display a live stream of incoming log lines via WebSocket
Allow filtering by agent, log level, source file, and time range
Display an anomaly alerts panel with timestamp, affected agent, and anomaly score
Provide an agent settings panel — configure watched directories, anomaly sensitivity threshold, reconnect behavior per agent
Reflect agent connection status (online, offline, reconnecting)

3.5 CLI
The primary way the developer interacts with the system. Replaces the need for authentication.
Commands:
project start            # starts the backend server
project stop             # stops the backend server
project agent attach     # runs the agent on the current machine
project agent list       # lists all connected agents and their status
project logs tail        # streams live logs in the terminal
project logs search      # search logs with filters (level, time, agent)
project dashboard        # opens the web dashboard in the browser
project alerts list      # lists recent anomaly alerts
project model train      # triggers a retraining of the anomaly detection model

4. Data
4.1 Log Entry
Each log line stored in the database has the following fields:
FieldTypeDescriptionidintegerunique identifiertimestampdatetimewhen the log line was writtenlevelstringINFO, WARN, ERROR, DEBUG, or UNKNOWNmessagestringthe raw log line contentsource_filestringwhich file the line came fromagent_idstringwhich agent sent itreceived_atdatetimewhen the backend received it
4.2 Agent Record
FieldTypeDescriptionagent_idstringunique identifier generated on first runhostnamestringmachine namestatusstringonline, offline, reconnectinglast_seendatetimelast heartbeat receivedwatched_dirslistdirectories currently being watchedsensitivityfloatanomaly detection threshold (0.0 to 1.0)
4.3 Anomaly Alert
FieldTypeDescriptionidintegerunique identifieragent_idstringwhich agent triggered ittimestampdatetimewhen it was detectedscorefloatreconstruction error from the modelwindowstringthe log window that triggered it

5. Tech Stack
ComponentTechnologyAgentRust, Tokio (async), Serde (serialization)BackendPython, FastAPI, SQLite, WebSocketsAnomaly DetectionPython, PyTorchFrontendReact, Tailwind CSS, Recharts (for anomaly scores)CLIRust or Python (TBD)Communication (agent → backend)TCP with a simple custom message formatCommunication (backend → frontend)WebSockets

6. Out of Scope

User authentication and multi-user support
Cloud deployment or remote access
Log parsing into structured formats (logs are stored as raw strings)
External alerting (email, Slack, webhooks)
Support for Windows (Linux and macOS only)


7. Constraints

The system must function entirely offline — no external API calls
The PyTorch model must be trained on data the user's own system generates
No AI-generated code in the implementation