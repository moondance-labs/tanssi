# Chain Log Alerting Rules

This directory contains JSON definitions of patterns used to monitor node logs.
If a matching log entry is detected, an alert is sent to the configured communication channel (primarily Slack).

The check runs hourly and scans logs from the previous hour against the defined patterns.

### Chains monitored:
- Dancelight
- Moonlight
- Tanssi

### Node types monitored:
- Validators
- Collators
- Boot Nodes