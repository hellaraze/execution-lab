#!/usr/bin/env python3
import json, sys
inp, outp, bps = sys.argv[1], sys.argv[2], float(sys.argv[3])
k = 1.0 + (bps / 10000.0)
PRICE_KEYS = {"bid","ask","best_bid","best_ask","bid_price","ask_price","price","px","mid"}
def is_num(x):
    if isinstance(x,(int,float)): return True
    if isinstance(x,str):
        try: float(x); return True
        except: return False
    return False
def walk(o):
    if isinstance(o,dict):
        for kk,vv in list(o.items()):
            if kk in PRICE_KEYS and is_num(vv):
                o[kk] = float(vv) * k
            else:
                walk(vv)
    elif isinstance(o,list):
        for vv in o: walk(vv)

with open(inp) as fi, open(outp,"w") as fo:
    for ln in fi:
        ln = ln.strip()
        if not ln: continue
        obj = json.loads(ln)
        walk(obj)
        fo.write(json.dumps(obj,separators=(",",":"))+"\n")
