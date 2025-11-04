"use client";

export default function Error({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  console.log("error", error);

  return (
    <main>
      <h2>Something went wrong loading products</h2>
      <button onClick={() => reset()}>Try again</button>
    </main>
  );
}
