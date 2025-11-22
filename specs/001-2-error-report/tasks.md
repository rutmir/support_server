# Tasks: Error Report API

**Input**: Design documents from `/specs/001-2-error-report/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: The examples below include test tasks. Tests are OPTIONAL - only include them if explicitly requested in the feature specification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root
- **Web app**: `backend/src/`, `frontend/src/`
- **Mobile**: `api/src/`, `ios/src/` or `android/src/`
- Paths shown below assume single project - adjust based on plan.md structure

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create project structure per implementation plan
- [x] T002 Initialize Rust project with required dependencies
- [x] T003 [P] Configure linting and formatting tools for Rust

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Setup error handling and logging infrastructure
- [x] T005 [P] Implement API key validation middleware
- [x] T006 [P] Setup message cache with 10-minute TTL
- [x] T007 [P] Configure Telegram notification service
- [x] T008 Setup environment configuration management

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Client Reports Critical Error (Priority: P1) 🎯 MVP

**Goal**: A client application can send a critical error report that gets validated, cached, and sent to Telegram

**Independent Test**: Can be fully tested by sending a valid error report with proper authentication and verifying it appears in the Telegram channel.

### Implementation for User Story 1

- [x] T009 [P] [US1] Create ErrorReportRequest model in src/models/error_report.rs
- [x] T010 [P] [US1] Create ErrorResponse and SuccessResponse models in src/models/error_response.rs
- [x] T011 [US1] Implement error report service in src/services/error_report_service.rs
- [x] T012 [US1] Implement error report endpoint in src/api/error_report.rs
- [x] T013 [US1] Add validation for error report fields
- [x] T014 [US1] Add logging for error report operations

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Duplicate Error Handling (Priority: P2)

**Goal**: The system prevents duplicate error messages from being sent to Telegram within a 10-minute window

**Independent Test**: Can be tested by sending the same error message twice within 10 minutes and verifying only the first one triggers a Telegram notification.

### Implementation for User Story 2

- [x] T015 [P] [US2] Create MessageCache model in src/models/message_cache.rs
- [x] T016 [US2] Enhance error report service with duplicate detection in src/services/error_report_service.rs
- [x] T017 [US2] Add cache lookup before sending Telegram notification
- [x] T018 [US2] Add logging for duplicate detection

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Cache Expiration (Priority: P3)

**Goal**: Error messages are only cached for 10 minutes, after which identical messages are treated as new

**Independent Test**: Can be tested by sending an error message, waiting 10 minutes, then sending the same message again and verifying it triggers a new Telegram notification.

### Implementation for User Story 3

- [x] T019 [US3] Implement cache expiration mechanism in cache service
- [x] T020 [US3] Update error report service to handle expired cache entries
- [x] T021 [US3] Add logging for cache expiration

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T022 [P] Documentation updates in docs/
- [ ] T023 Code cleanup and refactoring
- [ ] T024 Performance optimization across all stories
- [x] T025 [P] Additional unit tests in tests/unit/
- [ ] T026 Security hardening
- [ ] T027 Run quickstart.md validation
- [ ] T028 Update observability dashboards and alerts

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - May integrate with US1 but should be independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - May integrate with US1/US2 but should be independently testable

### Within Each User Story

- Models before services
- Services before endpoints
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Models within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all models for User Story 1 together:
Task: "Create ErrorReportRequest model in src/models/error_report.rs"
Task: "Create ErrorResponse and SuccessResponse models in src/models/error_response.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence