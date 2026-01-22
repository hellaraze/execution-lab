use std::process::Command;

#[test]
fn no_direct_core_event_writes() {
    // Разрешённые пути (tooling / eventlog internals / tests)
    let allow = [
        "eventlog/",
        "replay/src/bin/",
        "tests/",
        "examples/",
    ];

    // Ищем прямые записи core::Event в eventlog
    let patterns = [
        "EventLogWriter::open(",
        "writer.write(&ev)",
        "append_bytes(\"event\"",
        "append_json_value(\"event\"",
    ];

    let out = Command::new("rg")
        .arg("-n")
        .args(&patterns)
        .arg(".")
        .output()
        .expect("rg failed");

    let stdout = String::from_utf8_lossy(&out.stdout);

    let mut violations = Vec::new();
    for line in stdout.lines() {
        if allow.iter().any(|p| line.contains(p)) {
            continue;
        }
        violations.push(line.to_string());
    }

    if !violations.is_empty() {
        panic!(
            "Direct core::Event writes are forbidden in runtime:\n{}",
            violations.join("\n")
        );
    }
}
