#!/usr/bin/env python3
import argparse, json, sys, urllib.request, urllib.error, re

def fetch(url: str):
    req = urllib.request.Request(url, headers={"Accept":"application/vnd.github+json","User-Agent":"execution-lab-phase16"})
    with urllib.request.urlopen(req, timeout=20) as r:
        return json.loads(r.read().decode("utf-8"))

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--slug", required=True, help="owner/repo")
    ap.add_argument("--tag", default="", help="vX.Y.Z")
    ap.add_argument("--latest", action="store_true")
    args = ap.parse_args()

    if args.latest and args.tag:
        print("ERROR: use --latest OR --tag, not both")
        return 2

    if args.latest:
        url = f"https://api.github.com/repos/{args.slug}/releases/latest"
    else:
        if not args.tag:
            print("ERROR: provide --tag vX.Y.Z or use --latest")
            return 2
        url = f"https://api.github.com/repos/{args.slug}/releases/tags/{args.tag}"

    try:
        rel = fetch(url)
    except urllib.error.HTTPError as e:
        print(f"ERROR: HTTP {e.code} {e.reason} for {url}")
        return 1
    except Exception as e:
        print(f"ERROR: {e}")
        return 1

    assets = rel.get("assets", [])
    names = [a.get("name","") for a in assets]
    print("Release:", rel.get("tag_name",""), "-", rel.get("name",""))
    print("Assets:", len(names))
    for n in names:
        print(" -", n)

    have_latest = any(n == "latest.json" for n in names)
    installers = [n for n in names if n.lower().endswith(".exe") or n.lower().endswith(".msi")]
    sigs = [n for n in names if n.lower().endswith(".sig")]

    ok = True
    if not have_latest:
        print("MISSING: latest.json")
        ok = False
    if not installers:
        print("MISSING: installer (.exe or .msi)")
        ok = False
    if not sigs:
        print("MISSING: installer signature (.sig)")
        ok = False

    # stronger: ensure at least one signature matches an installer name + ".sig"
    if installers and sigs:
        matches = 0
        for ins in installers:
            if f"{ins}.sig" in sigs:
                matches += 1
        if matches == 0:
            print("WARN: no '<installer>.sig' found (signing may not have produced expected asset naming)")
    if ok:
        print("VERIFY_OK")
        return 0
    else:
        print("VERIFY_FAIL")
        return 1

if __name__ == "__main__":
    sys.exit(main())
