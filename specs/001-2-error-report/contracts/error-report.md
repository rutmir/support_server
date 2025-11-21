# OpenAPI Specification: Error Report API

## Overview
This document defines the REST API contract for the Error Report service, which allows client applications to report critical errors that are then forwarded to a Telegram channel.

## API Information
- **Title**: Error Report API
- **Version**: 1.0.0
- **Base URL**: `/api/v1`

## Authentication
All requests to the Error Report API require authentication using an API key in the Authorization Bearer header:

```
Authorization: Bearer {api_key}
```

## Endpoints

### Report Error
Submit a critical error report from a client application.

#### Request
- **Method**: POST
- **Path**: `/error-report`
- **Headers**:
  - `Authorization: Bearer {api_key}` (required)
  - `Content-Type: application/json` (required)
- **Body**:
  ```json
  {
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

#### Responses
- **200 OK**: Error report was successfully received and processed
  ```json
  {
    "status": "success",
    "message": "Error report received"
  }
  ```

- **400 Bad Request**: Invalid request format or missing required fields
  ```json
  {
    "status": "error",
    "message": "Invalid request format"
  }
  ```

- **401 Unauthorized**: Invalid or missing API key
  ```json
  {
    "status": "error",
    "message": "Unauthorized"
  }
  ```

## Data Models

### ErrorReportRequest
The request payload for reporting an error.

- `device` (object, required)
  - `id` (string, required): Unique identifier for the device
  - `description` (string, optional): Description of the device
- `error` (object, required)
  - `message` (string, required): The error message
  - `trace` (array of strings, required): Stack trace information

### SuccessResponse
Standard response for successful requests.

- `status` (string): "success"
- `message` (string): Descriptive message

### ErrorResponse
Standard response for error conditions.

- `status` (string): "error"
- `message` (string): Descriptive error message

## Security
- All API requests must be made over HTTPS
- API keys should be kept secure and not exposed in client-side code
- Rate limiting may be implemented to prevent abuse

## Implementation Notes
- The server checks for duplicate messages within a 10-minute window
- Duplicate messages are silently ignored (still return 200 OK)
- New messages are cached for 10 minutes for duplicate detection
- Error reports are forwarded to a configured Telegram channel