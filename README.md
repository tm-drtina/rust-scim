# rust-scim
Implementation of *System for Cross-domain Identity Management* (SCIM) protocol as a rust library folowing RFCs [7642](https://datatracker.ietf.org/doc/html/rfc7642) (definitions), [7643](https://datatracker.ietf.org/doc/html/rfc7643) (core schema) and [7644](https://datatracker.ietf.org/doc/html/rfc7644) (protocol).

## Testing
There are currently two test suites testing the code: static schema and tests against [directory-scimple](https://github.com/apache/directory-scimple/) project.

To run all test suites simply run `cargo t`

### Static schema tests
Tests whether the lib is able to parse examples from [RFC7643 Section 8](https://datatracker.ietf.org/doc/html/rfc7643#section-8).
These tests have no prerequisites and should always pass.

### Directory-scimple tests
This suite can be considered as integration tests and requires running server to test against.

To start directory-scimple spring boot example server:
```
./mvnw install
./mvnw spring-boot:run -amd -pl scim-server-examples/scim-server-spring-boot/
```

Note that we accessing single stateful server instance and there might be cases, where one test leaves the server in broken state. Ideally we want all tests to cleanup any resource created by them. If you get unexpected test failure, please try to restart the SCIM server and run just the single (failing) test to verify it's not affected by other test executions.
