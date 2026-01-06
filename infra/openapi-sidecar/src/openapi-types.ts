export type OpenAPIObject = {
  openapi: string;
  info?: unknown;
  servers?: unknown;
  tags?: unknown[];
  externalDocs?: unknown;
  paths?: Record<string, unknown>;
  components?: {
    schemas?: Record<string, unknown>;
    parameters?: Record<string, unknown>;
    responses?: Record<string, unknown>;
    requestBodies?: Record<string, unknown>;
    [k: string]: unknown;
  };
  [k: string]: unknown;
};
