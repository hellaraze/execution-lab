# md_replay: BBO compare as drift metrics (not failures)

`bookTicker` BBO and `depth@100ms` top-of-book are not semantically identical streams. Nearest-by-proc-time comparisons will show drift; treat it as metrics, not an invariant.

Outputs:
- `drift_abs_{bid,ask,mid,spread}`: min/avg/max absolute diffs
- `drift_ticks_p{50,90,99,999}`: percentiles of max(bid,ask) drift in ticks
- `nearest_dt_ns`: nearest sample time distance

Useful flags:
- `--print-threshold-usd <X>` or `--print-threshold-ticks <N>`
- `--max-print <N>`
