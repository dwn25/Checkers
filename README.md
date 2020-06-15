# CS 451 Group 5
[![Build Status](https://travis-ci.com/ehegnes/cs-451-group-5.svg?token=9xJDZstHjNye4Zd4F9uy&branch=master)](https://travis-ci.com/ehegnes/cs-451-group-5)

This is a checkers/draughts game developed for CS 451.

## Table of Contents
- [Dependencies](#dependencies)
- [Targets](#targets)
- [Documentation](#documentation)

## Dependencies
- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)
- [GNU Make](https://www.gnu.org/software/make/)

## Targets
| Target             | Description                                  |
| ------------------ | -------------------------------------------- |
| `all`              | Format and run code                          |
| `fmt`              | Format code according to official guidelines |
| `watch`            | Watch source files and run tests on change   |
| `lint`             | Report linting warnings                      |
| `req-docs`         | Generate requirements documentation          |
| `test-docs`        | Generate test case documentation          |
| `src-docs`         | Generate and serve source documentation      |
| `todos`            | List stray TODO comments in the source       |
| `cov`              | Generate coverage report (with branches)\*   |
| `clean`            | Remove generated files                       |

\* *this currently requires `lcov`, `llvm`, and `grcov`*

## Documentation
The requirements documentation is compiled to [`docs/requirements/index.pdf`](docs/requirements/index.pdf)
```
make req-docs
```

The source code documentation can be generated and viewed by running
```
make src-docs
```
