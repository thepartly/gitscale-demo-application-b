/** What `GET /api/b/hello` answers. */
export interface Hello {
  message: string;
  /** Which service answered. */
  service: string;
}

/**
 * Greet `name` through application-b, at `base` — the page's own origin
 * when empty.
 */
export async function hello(name: string, base = ""): Promise<Hello> {
  const response = await fetch(
    `${base}/api/b/hello?name=${encodeURIComponent(name)}`,
  );
  if (!response.ok) {
    throw new Error(`application-b answered ${response.status}`);
  }
  return (await response.json()) as Hello;
}
