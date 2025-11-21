# Quickstart: Error Report API

This guide provides instructions on how to use the Error Report API to submit critical error messages from client applications.

## Prerequisites

- A valid API key for authentication
- An HTTPS client (curl, Postman, etc.)
- Network access to the Error Report API server

## API Endpoint

```
POST /api/v1/error-report
```

## Authentication

All requests must include an Authorization header with a valid API key:

```
Authorization: Bearer YOUR_API_KEY
```

## Submitting an Error Report

To submit an error report, send a POST request with a JSON payload containing the device information and error details.

### Request Format

```json
{
  "device": {
    "id": "unique-device-identifier",
    "description": "optional-description-of-device"
  },
  "error": {
    "message": "description-of-error",
    "trace": [
      "stack-trace-line-1",
      "stack-trace-line-2"
    ]
  }
}
```

### Example using curl

```bash
curl -X POST https://your-api-server.com/api/v1/error-report \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "device": {
      "id": "server-001",
      "description": "Production web server"
    },
    "error": {
      "message": "Database connection timeout",
      "trace": [
        "ConnectionPool.getConnection()",
        "DatabaseService.connect()",
        "UserService.authenticate()"
      ]
    }
  }'
```

### Example Response

A successful request will return a 200 OK response:

```json
{
  "status": "success",
  "message": "Error report received"
}
```

## Error Responses

### 401 Unauthorized

Returned when the API key is missing or invalid:

```json
{
  "status": "error",
  "message": "Unauthorized"
}
```

### 400 Bad Request

Returned when the request format is invalid or required fields are missing:

```json
{
  "status": "error",
  "message": "Invalid request format"
}
```

## Duplicate Handling

The system automatically detects and ignores duplicate error messages within a 10-minute window. Duplicate messages will still return a 200 OK response but will not trigger a Telegram notification.

## Best Practices

1. **Secure your API key**: Never expose your API key in client-side code or public repositories
2. **Handle errors gracefully**: Implement proper error handling for 401 and 400 responses
3. **Include meaningful stack traces**: Provide detailed stack trace information to help with debugging
4. **Use descriptive device IDs**: Use consistent, descriptive identifiers for your devices/servers
5. **Rate limiting**: Avoid sending excessive error reports in a short period