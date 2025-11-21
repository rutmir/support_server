# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

[Extract from feature spec: primary requirement + technical approach from research]

## Technical Context

**Language/Version**: [Rust 1.90]  
**Testing**: [cargo test]  
**Target Platform**: [Linux server]
**Project Type**: [single/web/mobile - determines source structure]  
**Performance Goals**: [domain-specific, 1000 req/s]  
**Constraints**: [domain-specific, <200ms p95, <100MB memory]  
**Scale/Scope**: [domain-specific, 10k users, 1M LOC]

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

Based on the Support Server Constitution, all implementations must adhere to these principles:
- I. Rust-First Development: All features must be implemented in Rust
- II. REST API Interface: All functionality exposed via REST API endpoints
- III. Test-First (NON-NEGOTIABLE): TDD mandatory with comprehensive unit tests
- IV. Integration Testing: Required for API contracts and external integrations
- V. Observability & Error Handling: Structured logging and proper error categorization

Additionally, all work must follow the Development Workflow and Quality Gates defined in the Constitution.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

```

**Structure Decision**: [Document the selected structure and reference the real
directories captured above]

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
