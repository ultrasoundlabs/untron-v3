openapi-generator generate \
  -i ./apps/indexer/openapi.json \
  -g rust \
  -o ./crates/indexer-client \
  --additional-properties=library=reqwest,packageName=untron-v3-indexer-client \
  --type-mappings number=serde_json::Number,integer=i64
