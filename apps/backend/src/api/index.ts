import { db } from "ponder:api";
import schema from "ponder:schema";
import { Hono } from "hono";
import { client, graphql } from "ponder";

import { untronV3Api } from "./untronV3";

const app = new Hono();

app.use("/sql/*", client({ db, schema }));

app.route("/", untronV3Api);

app.use("/graphql", graphql({ db, schema }));

export default app;
