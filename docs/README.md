# Untron V3 Mermaid diagram

- Source: `docs/untron-v3.mmd`
- Rendered: `docs/untron-v3.svg`, `docs/untron-v3.png`

## Re-render

### Docker (recommended)

```sh
docker run --rm -u "$(id -u):$(id -g)" -v "$PWD:/data" ghcr.io/mermaid-js/mermaid-cli/mermaid-cli:latest \
  -i /data/docs/untron-v3.mmd -o /data/docs/untron-v3.svg

docker run --rm -u "$(id -u):$(id -g)" -v "$PWD:/data" ghcr.io/mermaid-js/mermaid-cli/mermaid-cli:latest \
  -i /data/docs/untron-v3.mmd -o /data/docs/untron-v3.png -b transparent
```
