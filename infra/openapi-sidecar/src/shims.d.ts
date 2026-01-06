declare module "swagger2openapi" {
  export function convertObj(
    swagger: Record<string, unknown>,
    options: Record<string, unknown>
  ): Promise<{ openapi: any }>;
}
