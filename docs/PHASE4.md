# PHASE 4 - MULTI-EXCHANGE MARKET DATA (TIER-1 COVERAGE) (IN PROGRESS)

## Goal
Provide a product-grade MD control surface:
- start market data ingest for an exchange
- write canonical eventlog output
- record evidence JSON for audit
- keep multi-exchange capability registry (tier-1 coverage)

## Commands

### List MD-capable exchanges
elctl md list

### Start MD ingest (Binance depth; writes eventlog)
elctl md start binance --out md_out/binance_depth.eventlog

### Status
elctl status
elctl health

## Notes
- Phase 4 provides a single live ingest path (Binance depth) and a registry for tier-1 coverage.
- Additional exchanges are represented as capabilities and will be implemented in later phases.

