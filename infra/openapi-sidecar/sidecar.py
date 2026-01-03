import os
import json
import time
import requests
from flask import Flask, Response

UPSTREAM = os.environ.get("UPSTREAM_OPENAPI_URL", "http://postgrest:3000/")
CACHE_SECONDS = int(os.environ.get("CACHE_SECONDS", "10"))

app = Flask(__name__)
_cached = {"body": None, "at": 0.0}

def fetch_and_filter():
    r = requests.get(
        UPSTREAM,
        headers={"Accept": "application/openapi+json"},
        timeout=10,
    )
    r.raise_for_status()
    spec = r.json()

    paths = spec.get("paths", {})
    for path, ops in list(paths.items()):
        if not isinstance(ops, dict):
            continue
        get_op = ops.get("get")
        if get_op is None:
            del paths[path]
        else:
            paths[path] = {"get": get_op}

    # compact output
    return json.dumps(spec, separators=(",", ":"))

@app.get("/openapi.json")
def openapi_json():
    now = time.time()
    if _cached["body"] is None or (now - _cached["at"]) > CACHE_SECONDS:
        _cached["body"] = fetch_and_filter()
        _cached["at"] = now
    return Response(_cached["body"], mimetype="application/json")

@app.get("/healthz")
def healthz():
    return {"ok": True}

if __name__ == "__main__":
    # IMPORTANT: bind to 0.0.0.0 so Caddy/other containers can reach it
    app.run(host="0.0.0.0", port=5000)
