# Feature Specification: Error Report API

**Feature Branch**: `001-2-error-report`  
**Created**: 2025-11-21  
**Status**: Draft  
**Input**: User description: "Implement a REST API endpoint to receive critical error messages from clients. The server must authorize requests using an ApiKey in the Authorization Bearer header, returning 401 if invalid. The server must check a 10-minute cache for message duplication; if duplicated, ignore the message. If new, save the message to the cache and propagate the error information to the configured Telegram channel."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Client Reports Critical Error (Priority: P1)

A client application encounters a critical error and needs to report it to the server for notification via Telegram.

**Why this priority**: This is the core functionality of the feature - receiving error reports and notifying the team via Telegram.

**Independent Test**: Can be fully tested by sending a valid error report with proper authentication and verifying it appears in the Telegram channel.

**Acceptance Scenarios**:

1. **Given** a client has a valid API key, **When** it sends a critical error report to the endpoint, **Then** the server accepts the request, caches the message, and sends a notification to the Telegram channel.
2. **Given** a client has an invalid API key, **When** it attempts to send an error report, **Then** the server returns a 401 Unauthorized response and does not process the message.

---

### User Story 2 - Duplicate Error Handling (Priority: P2)

The system prevents duplicate error messages from being sent to Telegram within a 10-minute window.

**Why this priority**: This prevents spamming the Telegram channel with repeated error messages while still allowing legitimate new errors to be reported.

**Independent Test**: Can be tested by sending the same error message twice within 10 minutes and verifying only the first one triggers a Telegram notification.

**Acceptance Scenarios**:

1. **Given** an error message has been received and processed, **When** the same message is sent again within 10 minutes, **Then** the server ignores the duplicate and does not send another Telegram notification.

---

### User Story 3 - Cache Expiration (Priority: P3)

Error messages are only cached for 10 minutes, after which identical messages are treated as new.

**Why this priority**: This ensures that recurring errors that were temporarily resolved and reappeared are properly reported.

**Independent Test**: Can be tested by sending an error message, waiting 10 minutes, then sending the same message again and verifying it triggers a new Telegram notification.

**Acceptance Scenarios**:

1. **Given** an error message was received and cached, **When** the same message is sent after 10 minutes have passed, **Then** the server treats it as a new message and sends a Telegram notification.

---

### Edge Cases

- What happens when the Telegram API is unavailable?
- How does the system handle malformed error messages?
- What happens when the cache system is unavailable?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a REST API endpoint to receive critical error messages from clients
- **FR-002**: System MUST validate API key in Authorization Bearer header for all requests
- **FR-003**: System MUST return 401 Unauthorized for invalid API keys
- **FR-004**: System MUST check cache for duplicate messages within 10 minutes
- **FR-005**: System MUST ignore duplicate messages within the 10-minute cache window
- **FR-006**: System MUST cache new messages for 10 minutes
- **FR-007**: System MUST send error information to configured Telegram channel for new messages
- **FR-008**: System MUST handle cache expiration properly (10-minute TTL)

### Key Entities

- **Error Message**: Contains device information, error message, and trace information
  - Structure: {"device": {"id": "string"}, "error": {"message": "string", "trace": "string"}}
- **API Key**: Authentication token used to authorize requests
- **Message Cache**: Temporary storage for recently received error messages

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 95% of valid error reports are successfully processed and sent to Telegram within 5 seconds
- **SC-002**: 100% of invalid API key requests receive 401 responses
- **SC-003**: Duplicate messages within 10 minutes are blocked 99% of time

## Clarifications

1. **Q**: What is the structure of an error message?
   **A**: {"device": {"id": "string", "description": "string"}, "error": {"message": "string", "trace": ["string"]}}

2. **Q**: How should the system handle malformed error messages?
   **A**: Log the error and return 200 OK to avoid client retries

3. **Q**: What happens when the Telegram API is unavailable?
   **A**: Retry with exponential backoff, then log failure

4. **Q**: How should the system handle cache unavailability?
   **A**: Allow all messages through without duplication checking
