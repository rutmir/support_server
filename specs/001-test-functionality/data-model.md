# Data Model: Test Functionality for Error Reporting

## Overview

This document describes the data model for implementing test functionality for the error reporting feature. The entities described here are used in the tests to verify the system's behavior.

## Entities

### ErrorReport

Represents an error report from a client, including app information, device information, and error details.

#### Fields

- `app`: Object containing application information
  - `name`: String representing the application name
  - `version`: String representing the application version
- `device`: Object containing device information
  - `id`: String representing the device ID
  - `description`: String describing the device
- `error`: Object containing error details
  - `message`: String representing the error message
  - `trace`: Array of strings representing the error trace

#### Example

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

### ApiKey

Represents an API key used for authorization.

#### Fields

- `key`: String representing the API key value

#### Example

```text
test_api_key
```

### MessageCache

Represents a cache of recently received messages to prevent duplicates.

#### Fields

- `message_hash`: String representing a hash of the message content
- `timestamp`: Timestamp representing when the message was received

#### Behavior

- Messages are stored in the cache for 10 minutes
- When a new message is received, its hash is checked against the cache
- If a matching hash is found and the timestamp is within 10 minutes, the message is ignored
- If the cache is full, new entries are rejected (no LRU eviction)