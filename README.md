# Support Server

## Description
REST API server to handle messages from clients and propagate it to Telegram.
Authorization functionality to this REST API server implemented using Authorization header with Bearer api-key parameter. List of api_keys stored in the config file. 

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

## Docker image
rutmir/support-server:latest

