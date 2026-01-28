import json, os, subprocess, hashlib, datetime, pathlib, sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
OUT = ROOT / "demo" / "out" / "last_run"
OUT.mkdir(parents=True, exist_ok=True)

def sha256_bytes(b: bytes) -> str:
    h = hashlib.sha256()
    h.update(b)
    return h.hexdigest()

def main() -> int:
    ts = datetime.datetime.utcnow().replace(microsecond=0).isoformat() + "Z"
    bin_path = ROOT / "target" / "release" / "app"
    if not bin_path.exists():
        subprocess.check_call(["cargo", "build", "-q", "--release", "-p", "app"], cwd=str(ROOT))

    # run demo
    p = subprocess.run([str(bin_path), "demo"], cwd=str(ROOT), stdout=subprocess.PIPE, stderr=subprocess.STDOUT, check=True)
    stdout = p.stdout
    (OUT / "stdout.txt").write_bytes(stdout)
    (OUT / "run.log").write_bytes(stdout)

    # manifest
    manifest = {
        "ts_utc": ts,
        "app_bin": str(bin_path.relative_to(ROOT)),
        "cmd": [str(bin_path.relative_to(ROOT)), "demo"],
        "cwd": ".",
        "files": {
            "stdout.txt": {"sha256": sha256_bytes((OUT / "stdout.txt").read_bytes()), "bytes": (OUT / "stdout.txt").stat().st_size},
            "run.log": {"sha256": sha256_bytes((OUT / "run.log").read_bytes()), "bytes": (OUT / "run.log").stat().st_size},
        },
    }
    (OUT / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")

    # bundle hash list
    lines = []
    for name in ["manifest.json", "stdout.txt", "run.log"]:
        b = (OUT / name).read_bytes()
        lines.append(f"{sha256_bytes(b)}  {name}")
    (OUT / "sha256.txt").write_text("\n".join(lines) + "\n", encoding="utf-8")

    print("BUNDLE_OK")
    print(f"OUT={OUT.relative_to(ROOT)}")
    return 0

if __name__ == "__main__":
    sys.exit(main())
