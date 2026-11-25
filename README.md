# Support Server

## Description
REST API server to handle messages from clients and propagate it to Telegram.
Authorization functionality to this REST API server implemented using `Authorization: Bearer <x-api-key>` header. List of api_keys stored in the config file. 

## Technology stack
- **Rust** language
- **Axum** server

## Use Cases
- client send to server information about critical error (device information, error message, trace information).
- for request authorization used ApiKey in Authorization Bearer header, if ApiKey not valid, server response 401 not authorized.
- server checks if the same message already presented in cache, then  ignore new duplicated message.
- if message not exists in the cache, then save message in the 10 minutes cache (cache_ttl_minutes parameter in the config file), and propagate information to the Telegram channel. 

## Example of request from client
curl -X POST http://127.0.0.1:8080/api/v1/error-report   -H "Authorization: Bearer test_api_key"   -H "Content-Type: application/json"   -d '{"app": {"name": "test", "version":"0.1.0"}, "device": {"id": "server-001", "description": "Production web server"}, "error": {"message": "Database connection timeout", "trace": ["ConnectionPool.getConnection()", "DatabaseService.connect()", "UserService.authenticate()"]}}' -v

## Notification gateway: `POST /api/v1/notify`

Generic notification endpoint used by the Argus dispatcher notifier
(`argus-notifier`, `SUPPORT_SERVER_URL=http://<host>:<port>`). Same Bearer
auth model as `/api/v1/error-report`; duplicate suppression uses the same TTL
cache (`cache_ttl_minutes`, SHA-256 over app/severity/title/message/refs/url).

Request:

```json
{
  "app": "argus",
  "severity": "warning",
  "title": "argus: human review needed for code-423n4/2023-07-basin",
  "message": "Triage: 50 candidates, 41 dedup, 10 shortlist, 3 text-fallback",
  "refs": ["report.md", "http://vega:8788/v1/jobs/<job-id>"],
  "url": "http://vega:8788/v1/jobs/<job-id>",
  "source": "argus-dispatcher",
  "job_id": "code4rena-code-423n4-2023-07-basin-73f7133b380e"
}
```

Responses:

| status | when |
|---|---|
| `200` | accepted (`duplicate:false`) or deduped (`duplicate:true`) |
| `400` | validation failure (empty app/severity/title/message) |
| `401` | missing/invalid Bearer key |
| `422` | malformed JSON / wrong types |
| `502` | Telegram delivery failed |

```bash
curl -X POST http://127.0.0.1:8080/api/v1/notify \
  -H "Authorization: Bearer test_api_key" -H "Content-Type: application/json" \
  -d '{"app":"argus","severity":"info","title":"argus: done","message":"job finished","refs":["report.md"]}'
```

`telegram_api_base` in `config.toml` (or `APP_TELEGRAM_API_BASE`) allows
pointing the client at a local mock Telegram Bot API in e2e tests.

## Docker image
rutmir/support-server

docker -run -d \
  --name support-server \
  -p 3000:3000 \
  -v <path_to_config_file>/config.toml:/app/config.toml \
  --restart always \
  rutmir/support-server:latest \
