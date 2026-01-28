# PHASE 3 - EXCHANGE REGISTRY + CONNECT WORKFLOW (IN PROGRESS)

## Goal
Provide a product-grade onboarding workflow:
- list supported exchanges and capabilities
- connect an exchange via CLI
- validate required secrets without leaking them
- run basic health checks (stubbed where appropriate)

## Commands

### List exchanges
elctl exchange list

### Connect exchange (offline validation + stubbed health checks)
elctl exchange connect okx --secrets-file secrets/okx.toml

## Secrets
Secrets must never be printed.
Secrets files should be excluded from version control.

