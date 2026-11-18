# PATTERN_ADOPTION

api-types declares its relationship to the pattern canon
(github.com/WyattAu/pattern-standards, PATTERN-CORE + references/catalog.md).
One row per pattern with a real relationship to the repo; non-applicable
patterns are omitted. Validated by
`pattern-standards/scripts/adoption-check.py` and
`standards/scripts/voice-lint.py --strict`.

| Pattern | Status | Evidence |
|---|---|---|
| T1 | partial | src/response.rs:9 (serde derives parse wire bytes into typed values at the boundary; parsed types are trusted thereafter) but src/error.rs:36 re-maps a String code to a status at use time |
| T6 | adopted | src/response.rs:11 (ApiResponse<T>: one envelope schema over any payload), src/response.rs:94 (ApiListResponse<T>), src/jsend.rs:24 (JSend<T>); utoipa ToSchema derives ride on the same generics |
| P7 | refused | R5 names exactly what PaginationMeta ships today: offset page/per_page (src/response.rs:32); keyset cursor variant unscheduled while json-envelope wire compatibility holds |
| P64 | adopted | tests/envelope_proptest.rs:14 (proptest JSON roundtrips for all three envelopes), Cargo.toml dev-dependencies |
| P67 | adopted | fuzz/fuzz_targets/fuzz_parse.rs (libfuzzer harness: untrusted bytes parsed as JSON, embedded into ApiResponse, output re-parsed as valid JSON) |
| P70 | adopted | src/response.rs:15 (optional fields via Option with skip_serializing_if; serde defaults ignore unknown fields, no deny_unknown_fields anywhere in src/) |
| P72 | adopted | src/error.rs:61 (IntoResponse wraps every error in the same ApiResponse envelope), src/response.rs:65 |
| P73 | partial | src/response.rs:94 (ApiListResponse pins explicit page, per_page, total, total_pages); no filter or sort vocabulary, server-side allowlists are out of scope for a types crate |
