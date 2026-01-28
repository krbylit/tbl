# Tasks: JSON Input Parsing

**Input**: Design documents from `specs/001-json-input/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, quickstart.md

**Tests**: Unit and integration tests included per Testing Discipline (constitution requirement)

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add dependencies and create example JSON files

- [x] T001 [P] Add serde_json = "1.0" to Cargo.toml dependencies
- [x] T002 [P] Add indexmap = "2.0" to Cargo.toml dependencies
- [x] T003 [P] Create examples/api_response.json with array of objects example data
- [x] T004 [P] Create examples/config.json with single object example data
- [x] T005 [P] Create examples/columnar.json with columnar format example data

**Checkpoint**: ✅ Dependencies added, example files created

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core JSON parsing infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T006 Create src/json_parser.rs module with public JSONFormat enum
- [x] T007 Implement DepthLimitedDeserializer in src/json_parser.rs with 128 level limit
- [x] T008 Implement FormatDetector in src/json_parser.rs for auto-detection
- [x] T009 Implement ColumnIndex in src/json_parser.rs using IndexMap for first-occurrence order
- [x] T010 Implement value_to_cell_string helper in src/json_parser.rs for JSON-to-string conversion
- [x] T011 Export parse_json function in src/lib.rs

**Checkpoint**: ✅ Foundation ready - JSON parsing infrastructure complete, user story implementation can now begin

---

## Phase 3: User Story 1 - Parse Array of Objects (Priority: P1) 🎯 MVP

**Goal**: Format JSON arrays of objects (most common use case) into tables with automatic column detection

**Independent Test**: `echo '[{"name":"Alice","age":30},{"name":"Bob","age":25}]' | cargo run | grep "Alice"` should display formatted table with name and age columns

### Unit Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T012 [P] [US1] Create tests/unit/json_parser_tests.rs test file
- [x] T013 [P] [US1] Write test_parse_array_of_objects_consistent_keys in tests/unit/json_parser_tests.rs
- [x] T014 [P] [US1] Write test_parse_array_of_objects_inconsistent_keys in tests/unit/json_parser_tests.rs
- [x] T015 [P] [US1] Write test_first_occurrence_column_order in tests/unit/json_parser_tests.rs
- [x] T016 [P] [US1] Write test_missing_keys_empty_cells in tests/unit/json_parser_tests.rs
- [x] T017 [P] [US1] Write test_nested_objects_stringified in tests/unit/json_parser_tests.rs
- [x] T018 [P] [US1] Write test_nested_arrays_stringified in tests/unit/json_parser_tests.rs
- [x] T019 [P] [US1] Write test_null_values_empty_cells in tests/unit/json_parser_tests.rs
- [x] T020 [P] [US1] Write test_boolean_values_as_text in tests/unit/json_parser_tests.rs
- [x] T021 [P] [US1] Write test_number_values_formatted in tests/unit/json_parser_tests.rs

### Implementation for User Story 1

- [x] T022 [US1] Implement classify_json_format function in src/json_parser.rs to detect ArrayOfObjects variant
- [x] T023 [US1] Implement collect_column_headers function in src/json_parser.rs using ColumnIndex for first-occurrence order
- [x] T024 [US1] Implement convert_array_of_objects_to_table function in src/json_parser.rs
- [x] T025 [US1] Add depth limit validation to array parsing in src/json_parser.rs
- [x] T026 [US1] Add main parse_json_to_table function in src/json_parser.rs for array of objects path
- [x] T027 [US1] Update src/main.rs to add --json flag to CLI arguments
- [x] T028 [US1] Update src/main.rs to implement format auto-detection using FormatDetector
- [x] T029 [US1] Update src/main.rs to call parse_json_to_table when JSON format detected
- [x] T030 [US1] Verify all unit tests for US1 pass (T013-T021)

### Integration Tests for User Story 1

- [x] T031 [P] [US1] Create tests/integration/json_tests.rs test file
- [x] T032 [P] [US1] Write test_json_array_from_file in tests/integration/json_tests.rs
- [x] T033 [P] [US1] Write test_json_array_from_stdin in tests/integration/json_tests.rs
- [x] T034 [P] [US1] Write test_json_flag_explicit in tests/integration/json_tests.rs
- [x] T035 [P] [US1] Write test_auto_detection_json in tests/integration/json_tests.rs
- [x] T036 [P] [US1] Write test_styling_works_with_json in tests/integration/json_tests.rs
- [x] T037 [P] [US1] Write test_conditional_formatting_with_json in tests/integration/json_tests.rs
- [x] T038 [US1] Verify all integration tests for US1 pass (T032-T037)

**Checkpoint**: ✅ User Story 1 (array of objects) is fully functional and testable independently. MVP complete!

---

## Phase 4: User Story 2 - Parse Single Object as Row (Priority: P2)

**Goal**: Format single JSON objects as one-row tables for config file inspection

**Independent Test**: `echo '{"host":"localhost","port":8080}' | cargo run | grep "localhost"` should display single-row table with host and port columns

### Unit Tests for User Story 2

- [ ] T039 [P] [US2] Write test_parse_single_object in tests/unit/json_parser_tests.rs
- [ ] T040 [P] [US2] Write test_single_object_with_arrays in tests/unit/json_parser_tests.rs
- [ ] T041 [P] [US2] Write test_single_object_empty in tests/unit/json_parser_tests.rs

### Implementation for User Story 2

- [ ] T042 [US2] Update classify_json_format function in src/json_parser.rs to detect SingleObject variant
- [ ] T043 [US2] Implement convert_single_object_to_table function in src/json_parser.rs
- [ ] T044 [US2] Update parse_json_to_table function in src/json_parser.rs to handle single object path
- [ ] T045 [US2] Verify all unit tests for US2 pass (T039-T041)

### Integration Tests for User Story 2

- [ ] T046 [P] [US2] Write test_single_object_from_file in tests/integration/json_tests.rs
- [ ] T047 [P] [US2] Write test_single_object_from_stdin in tests/integration/json_tests.rs
- [ ] T048 [US2] Verify all integration tests for US2 pass (T046-T047)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Parse Object of Arrays (Columnar Format) (Priority: P3)

**Goal**: Format columnar JSON (data science format) by transposing into row-based tables

**Independent Test**: `echo '{"name":["Alice","Bob"],"age":[30,25]}' | cargo run | grep "Alice"` should display transposed table with name and age columns

### Unit Tests for User Story 3

- [ ] T049 [P] [US3] Write test_parse_columnar_format_equal_length in tests/unit/json_parser_tests.rs
- [ ] T050 [P] [US3] Write test_parse_columnar_format_mismatched_length in tests/unit/json_parser_tests.rs
- [ ] T051 [P] [US3] Write test_columnar_with_non_array_values in tests/unit/json_parser_tests.rs
- [ ] T052 [P] [US3] Write test_columnar_empty_arrays in tests/unit/json_parser_tests.rs

### Implementation for User Story 3

- [ ] T053 [US3] Update classify_json_format function in src/json_parser.rs to detect ColumnarObject variant
- [ ] T054 [US3] Implement transpose_columnar_to_rows function in src/json_parser.rs
- [ ] T055 [US3] Implement convert_columnar_to_table function in src/json_parser.rs with padding for mismatched lengths
- [ ] T056 [US3] Update parse_json_to_table function in src/json_parser.rs to handle columnar path
- [ ] T057 [US3] Verify all unit tests for US3 pass (T049-T052)

### Integration Tests for User Story 3

- [ ] T058 [P] [US3] Write test_columnar_from_file in tests/integration/json_tests.rs
- [ ] T059 [P] [US3] Write test_columnar_from_stdin in tests/integration/json_tests.rs
- [ ] T060 [US3] Verify all integration tests for US3 pass (T058-T059)

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Edge Cases & Error Handling

**Purpose**: Handle edge cases and error conditions across all user stories

### Unit Tests for Edge Cases

- [ ] T061 [P] Write test_depth_limit_exceeded in tests/unit/json_parser_tests.rs
- [ ] T062 [P] Write test_empty_json_array in tests/unit/json_parser_tests.rs
- [ ] T063 [P] Write test_empty_json_object in tests/unit/json_parser_tests.rs
- [ ] T064 [P] Write test_invalid_json_syntax in tests/unit/json_parser_tests.rs
- [ ] T065 [P] Write test_special_characters_in_keys in tests/unit/json_parser_tests.rs
- [ ] T066 [P] Write test_numeric_keys_converted in tests/unit/json_parser_tests.rs

### Implementation for Edge Cases

- [ ] T067 Implement empty JSON handling in src/json_parser.rs (return empty TableData, exit 0)
- [ ] T068 Implement depth limit error handling in src/json_parser.rs with clear message
- [ ] T069 Implement invalid JSON error handling in src/json_parser.rs with line/column info
- [ ] T070 Add special character handling for JSON keys in src/json_parser.rs
- [ ] T071 Add numeric key conversion to string in src/json_parser.rs
- [ ] T072 Verify all edge case tests pass (T061-T066)

### Integration Tests for Edge Cases

- [ ] T073 [P] Write test_empty_json_exit_code_zero in tests/integration/json_tests.rs
- [ ] T074 [P] Write test_depth_limit_error_message in tests/integration/json_tests.rs
- [ ] T075 [P] Write test_invalid_json_error_message in tests/integration/json_tests.rs
- [ ] T076 [P] Write test_large_file_100mb in tests/integration/json_tests.rs
- [ ] T077 Verify all edge case integration tests pass (T073-T076)

**Checkpoint**: All edge cases handled correctly

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, shell completions, and final validation

- [ ] T078 [P] Update README.md with JSON format examples (3 formats: array, single, columnar)
- [ ] T079 [P] Add JSON pipeline examples to README.md (curl | tbl patterns)
- [ ] T080 [P] Add JSON error handling examples to README.md
- [ ] T081 [P] Update src/main.rs --help text to document --json flag with examples
- [ ] T082 [P] Update shell completions to include --json flag
- [ ] T083 [P] Update CHANGELOG.md with JSON input feature description
- [ ] T084 [P] Add JSON format limitations to README.md (100MB limit, 128 depth, no streaming)
- [ ] T085 Run cargo test to verify all 30+ unit tests pass
- [ ] T086 Run cargo test --test integration to verify all integration tests pass
- [ ] T087 Manual test: curl https://jsonplaceholder.typicode.com/users | cargo run
- [ ] T088 Manual test: cargo run examples/api_response.json
- [ ] T089 Manual test: cargo run --json examples/config.json
- [ ] T090 Manual test: cargo run examples/columnar.json --style markdown
- [ ] T091 Manual validation using quickstart.md examples
- [ ] T092 Performance benchmark: JSON vs CSV on equivalent data (verify <10% difference)
- [ ] T093 Memory test: 100MB JSON file doesn't exceed 200MB peak memory

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion (T001-T005) - BLOCKS all user stories
- **User Stories (Phase 3-5)**: All depend on Foundational phase completion (T006-T011)
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Edge Cases (Phase 6)**: Can start after Foundational, benefits from user story implementations
- **Polish (Phase 7)**: Depends on all user stories and edge cases being complete

### User Story Dependencies

- **User Story 1 (P1)**: Depends only on Foundational (T006-T011) - No dependencies on other stories
- **User Story 2 (P2)**: Depends only on Foundational (T006-T011) - Reuses US1 infrastructure but independently testable
- **User Story 3 (P3)**: Depends only on Foundational (T006-T011) - Reuses US1 infrastructure but independently testable

### Within Each User Story

**User Story 1 (T012-T038)**:
1. Unit tests (T012-T021) can all run in parallel - write first, ensure they fail
2. Core implementation (T022-T026) sequential - classify → collect → convert → validate → parse
3. CLI integration (T027-T029) sequential - flag → detection → call
4. Test verification (T030) after implementation
5. Integration tests (T031-T037) can all run in parallel
6. Final verification (T038) after integration tests

**User Story 2 (T039-T048)**:
1. Unit tests (T039-T041) in parallel
2. Implementation (T042-T044) sequential
3. Test verification (T045)
4. Integration tests (T046-T047) in parallel
5. Final verification (T048)

**User Story 3 (T049-T060)**:
1. Unit tests (T049-T052) in parallel
2. Implementation (T053-T056) sequential
3. Test verification (T057)
4. Integration tests (T058-T059) in parallel
5. Final verification (T060)

### Parallel Opportunities

- **Setup Phase**: All 5 tasks (T001-T005) can run in parallel
- **Foundational Phase**: T006-T011 are sequential due to module dependencies (T007 needs T006's module, T008-T010 need T007's deserializer, T011 needs all prior implementations)
- **User Story Tests**: All unit tests within a story can run in parallel (marked [P])
- **User Stories**: After Foundational complete, US1/US2/US3 can all be developed in parallel by different developers
- **Integration Tests**: All integration tests within a story can run in parallel (marked [P])
- **Polish Phase**: Most documentation tasks (T078-T084) can run in parallel

---

## Parallel Example: User Story 1

```bash
# After Foundational phase complete, launch all US1 unit tests together:
Task T013: "Write test_parse_array_of_objects_consistent_keys"
Task T014: "Write test_parse_array_of_objects_inconsistent_keys"
Task T015: "Write test_first_occurrence_column_order"
Task T016: "Write test_missing_keys_empty_cells"
Task T017: "Write test_nested_objects_stringified"
Task T018: "Write test_nested_arrays_stringified"
Task T019: "Write test_null_values_empty_cells"
Task T020: "Write test_boolean_values_as_text"
Task T021: "Write test_number_values_formatted"

# After US1 implementation, launch all US1 integration tests together:
Task T032: "Write test_json_array_from_file"
Task T033: "Write test_json_array_from_stdin"
Task T034: "Write test_json_flag_explicit"
Task T035: "Write test_auto_detection_json"
Task T036: "Write test_styling_works_with_json"
Task T037: "Write test_conditional_formatting_with_json"
```

---

## Parallel Example: All User Stories

```bash
# After Foundational phase complete, if you have 3 developers:
Developer A: User Story 1 (T012-T038) - Array of objects (MVP)
Developer B: User Story 2 (T039-T048) - Single object
Developer C: User Story 3 (T049-T060) - Columnar format

# Each story is independently implementable and testable
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T005)
2. Complete Phase 2: Foundational (T006-T011) - CRITICAL
3. Complete Phase 3: User Story 1 (T012-T038)
4. **STOP and VALIDATE**: Test US1 independently with real API data
5. Deploy/demo if ready - array of objects covers 80% of JSON use cases

**MVP Scope**: Tasks T001-T038 (38 tasks)
**MVP Value**: Format JSON from APIs and data exports (most common use case)

### Incremental Delivery

1. **Foundation**: T001-T011 → JSON parsing infrastructure ready
2. **MVP**: Add T012-T038 → Array of objects format working → Demo with `curl api | tbl`
3. **Increment 2**: Add T039-T048 → Single object format → Demo config file inspection
4. **Increment 3**: Add T049-T060 → Columnar format → Demo data science workflows
5. **Hardening**: Add T061-T077 → Edge cases and error handling
6. **Release**: Add T078-T093 → Documentation and validation → v1.0 with JSON support

Each increment adds value without breaking previous functionality.

### Parallel Team Strategy

With 3 developers:

1. **Together**: Phase 1 (Setup) + Phase 2 (Foundational) - T001-T011
2. **Parallel User Stories**:
   - Developer A: User Story 1 (T012-T038) - MVP priority
   - Developer B: User Story 2 (T039-T048)
   - Developer C: User Story 3 (T049-T060)
3. **Reconvene**: Edge Cases (T061-T077) together
4. **Parallel Polish**: Documentation tasks (T078-T084) in parallel
5. **Validation**: Testing and benchmarking (T085-T093) together

---

## Task Summary

**Total Tasks**: 93 tasks

**By Phase**:
- Phase 1 (Setup): 5 tasks
- Phase 2 (Foundational): 6 tasks (BLOCKING)
- Phase 3 (User Story 1 - Array of Objects): 27 tasks (MVP)
- Phase 4 (User Story 2 - Single Object): 10 tasks
- Phase 5 (User Story 3 - Columnar Format): 12 tasks
- Phase 6 (Edge Cases): 17 tasks
- Phase 7 (Polish): 16 tasks

**Parallel Opportunities**: 52 tasks marked [P] can run in parallel within their phase

**Independent Test Criteria**:
- **US1**: `curl https://jsonplaceholder.typicode.com/users | tbl` displays formatted user table
- **US2**: `echo '{"name":"Alice","age":30}' | tbl` displays single-row table
- **US3**: `echo '{"x":[1,2],"y":[3,4]}' | tbl` displays transposed 2x2 table

**MVP Scope**: 38 tasks (T001-T038) delivers array-of-objects format (80% of use cases)

---

## Notes

- [P] tasks = different files, no dependencies, can run in parallel
- [US1/US2/US3] label maps task to specific user story for traceability
- Each user story is independently completable and testable per spec requirements
- Write tests FIRST, ensure they FAIL before implementing (TDD)
- Commit after each task or logical group (follow conventional commit style)
- Stop at any checkpoint to validate story independently
- Foundational phase (T006-T011) MUST complete before any user story work begins
- All 30+ unit tests expected per Testing Discipline in constitution
