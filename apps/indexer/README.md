# Reference implementation of an indexer for Untron V3 protocol

Written in TypeScript using Ponder framework.

## Quickstart

```
pnpm install
cp .env.example .env.local
nano .env.local # configure whatever you need to configure
pnpm run build
pnpm run start
```

## Important

Currently the indexer also implements relayer-role logic needed for operation of Untron V3. Thus, this app is monolithic and can run everything you need to operate Untron V3 protocol on your own in a single service. This, however, reduces the flexibility of the system (e.g. you can't just run the indexer to read but not write protocol data). We expect to split indexer and relayer into separate, independent modules in the near future.
