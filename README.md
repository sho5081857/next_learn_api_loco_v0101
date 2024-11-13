# Introduction

# Getting started

If a local PostgreSQL instance already exists

```bash
cargo loco start
```

If a local PostgreSQL instance does not exist

```bash
task up
env $(cat .devcontainer/.env) cargo loco start
```
