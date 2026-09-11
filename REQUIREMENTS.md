# Requirements — api-types

Numbered, testable requirements. Every requirement maps to at least one named
test or doc-comment contract; security-relevant items cite threat-model rows.

Scope: Shared API envelope types (`api-types`) — `ApiResponse`, JSend, `ApiListResponse`, RFC 7807 `ProblemDetail`

## Functional

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-AT-001 | `ApiResponse::success`/`error` serialize to the documented JSON shapes; `success` is always present | MUST |
| REQ-AT-002 | JSend envelope strictly validates `status` values (`success`/`fail`/`error`) | MUST |
| REQ-AT-003 | `ProblemDetail` serializes with `type`/`title` per RFC 7807 and omits `None` fields | MUST |
| REQ-AT-004 | Serde implementations are feature-gated (`serde_impl`); `--no-default-features` builds compile | MUST |

## Security

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-AT-100 | Deserialization of hostile JSON yields typed errors, never panics | MUST |
| REQ-AT-101 | No heap exhaustion hazard in envelope parsing (lengths inherited from serde's limits) | SHOULD |

## Observability & API hygiene

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-AT-900 | All fallible public APIs return typed errors; production `unwrap`/`expect` is denied or explicitly justified with an invariant comment | MUST |
| REQ-AT-901 | Public items carry doc comments with runnable examples where practical | SHOULD |
