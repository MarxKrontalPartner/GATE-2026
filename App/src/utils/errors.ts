export class MisconfigurationError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "MisconfigurationError";
  }
}

export function getErrorMessage(error: unknown, fallback: string): string {
  return error instanceof Error && error.message ? error.message : fallback;
}
