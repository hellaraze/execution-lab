use std::path::PathBuf;
use std::process::Command;

fn find_value_span<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    // returns the raw value substring for key=... until next " <word>=" or end
    let start = line.find(key)? + key.len();
    let rest = &line[start..];
    let mut end = rest.len();

    // look for next " <something>=" pattern
    let bytes = rest.as_bytes();
    for i in 0..rest.len() {
        if bytes[i] == b' ' {
            // scan ahead for '=' with no spaces in between (token key)
            if let Some(eq_pos) = rest[i + 1..].find('=') {
                let j = i + 1 + eq_pos;
                // ensure the segment between space and '=' has no spaces
                if !rest[i + 1..j].contains(' ') && j > i + 1 {
                    end = i;
                    break;
                }
            }
        }
    }
    Some(rest[..end].trim())
}

fn parse_u64(v: &str) -> u64 {
    v.parse::<u64>()
        .unwrap_or_else(|e| panic!("bad u64 '{v}': {e}"))
}

fn parse_f64_opt(v: &str) -> Option<f64> {
    if v == "None" {
        return None;
    }
    let inner = v
        .strip_prefix("Some(")
        .and_then(|x| x.strip_suffix(')'))
        .unwrap_or_else(|| panic!("expected Some(..), got '{v}'"));
    Some(
        inner
            .parse::<f64>()
            .unwrap_or_else(|e| panic!("bad f64 '{inner}': {e}")),
    )
}

fn parse_f64_tuple_opt(v: &str) -> Option<(f64, f64, f64)> {
    // Some((min, Some(avg), max))
    if v == "None" {
        return None;
    }
    let inner = v
        .strip_prefix("Some((")
        .and_then(|x| x.strip_suffix("))"))
        .unwrap_or_else(|| panic!("expected Some((..)), got '{v}'"));

    let parts: Vec<&str> = inner.split(", ").collect();
    if parts.len() != 3 {
        panic!("expected 3 parts in tuple, got {:?} from '{inner}'", parts);
    }
    let p0 = parts[0];
    let p1 = parts[1];
    let p2 = parts[2];

    let min = p0
        .parse::<f64>()
        .unwrap_or_else(|e| panic!("min bad '{p0}': {e}"));
    let avg_s = p1
        .strip_prefix("Some(")
        .and_then(|x| x.strip_suffix(')'))
        .unwrap_or_else(|| panic!("avg expected Some(..), got '{p1}'"));
    let avg = avg_s
        .parse::<f64>()
        .unwrap_or_else(|e| panic!("avg bad '{avg_s}': {e}"));
    let max = p2
        .parse::<f64>()
        .unwrap_or_else(|e| panic!("max bad '{p2}': {e}"));

    Some((min, avg, max))
}

#[test]
fn md_replay_drift_fixture_invariants_locked() {
    let exe = env!("CARGO_BIN_EXE_md_replay");

    let mut fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fixture.push("tests/fixtures/binance_md_fixture.eventlog");
    assert!(fixture.exists(), "fixture missing: {:?}", fixture);

    let out = Command::new(exe)
        .args([
            fixture.to_string_lossy().as_ref(),
            "--mode",
            "compare",
            "--tick",
            "0.01",
            "--window-ms",
            "250",
            "--ring",
            "4096",
            "--max-print",
            "0",
            "--drift-keep",
            "200000",
        ])
        .output()
        .expect("run md_replay");

    assert!(
        out.status.success(),
        "md_replay failed: status={:?}\nstdout:\n{}\nstderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let s = String::from_utf8_lossy(&out.stdout);
    let ok = s
        .lines()
        .find(|l| l.starts_with("OK md_replay "))
        .expect("OK line");

    assert!(ok.contains("mode=Compare"), "bad ok line: {ok}");

    let bbo = parse_u64(find_value_span(ok, "bbo=").expect("bbo="));
    let compared = parse_u64(find_value_span(ok, "compared=").expect("compared="));
    let skipped = parse_u64(find_value_span(ok, "skipped=").expect("skipped="));

    assert_eq!(bbo, 1450, "bbo changed: {ok}");
    assert_eq!(compared, 1443, "compared changed: {ok}");
    assert_eq!(skipped, 7, "skipped changed: {ok}");

    let p50 = parse_f64_opt(find_value_span(ok, "drift_ticks_p50=").expect("p50"));
    let p90 = parse_f64_opt(find_value_span(ok, "drift_ticks_p90=").expect("p90")).expect("p90");
    let p99 = parse_f64_opt(find_value_span(ok, "drift_ticks_p99=").expect("p99")).expect("p99");
    let p999 =
        parse_f64_opt(find_value_span(ok, "drift_ticks_p999=").expect("p999")).expect("p999");

    assert_eq!(p50, Some(0.0), "p50 not zero: {ok}");

    assert!(
        (1020.0..=1045.0).contains(&p90),
        "p90 out of band (expected ~1032): {p90} | {ok}"
    );
    assert!(
        (1070.0..=1090.0).contains(&p99),
        "p99 out of band (expected ~1082): {p99} | {ok}"
    );
    assert!(
        (1070.0..=1090.0).contains(&p999),
        "p999 out of band (expected ~1082): {p999} | {ok}"
    );

    let db = parse_f64_tuple_opt(find_value_span(ok, "drift_abs_bid=").expect("drift_abs_bid"))
        .expect("drift_abs_bid Some");
    let (_min, _avg, max) = db;

    assert!(
        (10.6..=11.2).contains(&max),
        "drift_abs_bid max out of band (expected ~10.82): {max} | {ok}"
    );
}
