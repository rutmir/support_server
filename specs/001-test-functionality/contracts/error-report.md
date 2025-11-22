# API Contract: Error Report Endpoint

## Overview

This document describes the API contract for the error report endpoint. This endpoint allows clients to send error reports to the server, which will then be propagated to a Telegram channel.

## Endpoint

### URL
`POST /api/v1/error-report`

### Headers
- `Authorization: Bearer {api_key}` - Required. API key for authentication.
- `Content-Type: application/json` - Required. Specifies the content type of the request body.

### Request Body

The request body should be a JSON object with the following structure:

```json
{
  "app": {
    "name": "string",
    "version": "string"
  },
  "device": {
    "id": "string",
    "description": "string (optional)"
  },
  "error": {
    "message": "string",
    "trace": ["string"]
  }
}
```

#### Field Descriptions

- `app`: Object containing application information
  - `name`: String representing the application name (required, non-empty)
  - `version`: String representing the application version (required, non-empty)
- `device`: Object containing device information
  - `id`: String representing the device ID (required, non-empty)
  - `description`: String describing the device (optional)
- `error`: Object containing error details
  - `message`: String representing the error message (required, non-empty)
  - `trace`: Array of strings representing the error trace (optional)

### Responses

#### Success Response
- **Status Code**: `200 OK`
- **Body**:
```json
{
  "status": "success",
  "message": "Error report received and processed"
}
```

#### Unauthorized Response
- **Status Code**: `401 Unauthorized`
- **Body**:
```json
{
  "error": "Unauthorized",
  "message": "Invalid or missing API key"
}
```

#### Bad Request Response
- **Status Code**: `400 Bad Request`
- **Body**:
```json
{
  "error": "Bad Request",
  "message": "Invalid request payload"
}
```

#### Unprocessable Entity Response
- **Status Code**: `422 Unprocessable Entity`
- **Body**:
```json
{
  "error": "Unprocessable Entity",
  "message": "Malformed JSON in request"
}
```

#### Internal Server Error Response
- **Status Code**: `500 Internal Server Error`
- **Body**:
```json
{
  "error": "Internal Server Error",
  "message": "An error occurred while processing the request"
}
```

## Example Request

```bash
curl -X POST http://127.0.0.1:8080/api/v1/error-report \
  -H "Authorization: Bearer test_api_key" \
  -H "Content-Type: application/json" \
  -d '{
    "app": {
      "name": "test",
      "version": "0.1.0"
    },
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