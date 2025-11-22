# Quick Start: Test Functionality for Error Reporting

## Overview

This guide provides instructions on how to implement and run tests for the error reporting feature using the example from the README.

## Prerequisites

- Rust toolchain installed
- The support server running locally
- `curl` or another HTTP client for testing

## Test Data

The following test data is used for testing the error reporting feature:

### Valid Error Report

```json
{
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
}
```

### API Key

```text
test_api_key
```

## Running Tests

### 1. Test Successful Error Report

Send a valid error report to the server:

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
```

Expected response:

```json
{
  "status": "success",
  "message": "Error report received and processed"
}
```

### 2. Test Duplicate Error Report Handling

Send the same error report twice within 10 minutes. The second request should be ignored, and no additional message should be sent to Telegram.

### 3. Test API Key Authorization

#### Valid API Key

Send a request with a valid API key (as shown in test 1). The server should process the request and respond with success.

#### Invalid API Key

Send a request with an invalid API key:

```bash
curl -X POST http://127.0.0.1:8080/api/v1/error-report \
  -H "Authorization: Bearer invalid_api_key" \
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
```

Expected response:

```json
{
  "error": "Unauthorized",
  "message": "Invalid or missing API key"
}
```

### 4. Test Cache Functionality

#### Within 10 Minutes

Send the same error report twice within 10 minutes. The second request should be ignored, and no additional message should be sent to Telegram.

#### After 10 Minutes

Send the same error report twice, with the second request sent after 10 minutes. Both requests should result in messages being sent to Telegram.

## Running Automated Tests

To run the automated test suite:

```bash
cargo test
```

This will run all unit tests and integration tests for the error reporting feature.

## Expected Test Coverage

The test suite should cover:

- All test scenarios from README.md
- API key authorization (valid and invalid)
- Cache functionality (duplicate handling within and after 10 minutes)
- Edge cases (Telegram API unavailable, malformed JSON, cache full)