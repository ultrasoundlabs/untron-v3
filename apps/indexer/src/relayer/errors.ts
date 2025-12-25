export class RetryLaterError extends Error {
  readonly _tag = "RetryLaterError";

  constructor(message: string) {
    super(message);
    this.name = "RetryLaterError";
  }
}

export const isRetryLaterError = (error: unknown): error is RetryLaterError =>
  error instanceof RetryLaterError;
