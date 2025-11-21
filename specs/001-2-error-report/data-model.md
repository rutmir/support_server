# Data Model: Error Report API

## Overview
This document defines the data model for the Error Report API, including entities, their fields, relationships, and validation rules.

## Entities

### Error Message
Represents a critical error reported by a client application.

#### Fields
- `device` (object, required): Information about the device reporting the error
  - `id` (string, required): Unique identifier for the device
  - `description` (string, optional): Description of the device
- `error` (object, required): Information about the error
  - `message` (string, required): The error message
  - `trace` (array of strings, required): Stack trace information

#### Validation Rules
- Device ID must be a non-empty string
- Error message must be a non-empty string
- Error trace must be an array of strings (can be empty)

#### Example
```json
{
  "device": {
    "id": "device-123",
    "description": "Production server in DC1"
  },
  "error": {
    "message": "Database connection failed",
    "trace": [
      "ConnectionPool.getConnection()",
      "DatabaseService.connect()",
      "Main.processRequest()"
    ]
  }
}
```

### API Key
Represents an authentication token used to authorize requests to the Error Report API.

#### Fields
- `key` (string, required): The API key value
- `created_at` (timestamp, required): When the key was created
- `is_active` (boolean, required): Whether the key is active

#### Validation Rules
- Key must be a non-empty string
- Key must be unique
- Active keys can be used for authentication

#### Example
```json
{
  "key": "sk_1234567890abcdef",
  "created_at": "2025-11-21T10:00:00Z",
  "is_active": true
}
```

### Message Cache
Represents temporary storage for recently received error messages to prevent duplicates.

#### Fields
- `message_hash` (string, required): Hash of the error message content
- `device_id` (string, required): ID of the device that sent the message
- `created_at` (timestamp, required): When the message was cached
- `expires_at` (timestamp, required): When the cache entry expires (10 minutes after creation)

#### Validation Rules
- Message hash must be unique for active cache entries
- Cache entries automatically expire 10 minutes after creation
- Only active cache entries are considered for duplicate detection

#### Behavior Rules
- When a new error message is received, its hash is checked against the cache
- If a matching entry exists and hasn't expired, the message is considered a duplicate
- If no matching entry exists or the entry has expired, the message is processed normally
- New messages are added to the cache with a 10-minute TTL

#### Example
```json
{
  "message_hash": "a1b2c3d4e5f6...",
  "device_id": "device-123",
  "created_at": "2025-11-21T10:00:00Z",
  "expires_at": "2025-11-21T10:10:00Z"
}
```

## Relationships
- An API Key is associated with multiple Error Messages (one-to-many)
- An Error Message is associated with one Message Cache entry (one-to-one)
- A Device (within Error Message) can have multiple Error Messages (one-to-many)
- A Device can have multiple Message Cache entries (one-to-many)