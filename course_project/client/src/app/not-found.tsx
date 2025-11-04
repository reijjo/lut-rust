import Button from "@/components/ui/button/Button";
import Link from "next/link";
import "./layout.css";

export default function NotFound() {
  return (
    <main>
      <div className="not-found wrapper">
        <h1>404</h1>
        <h2>Nothing here.</h2>
        <Link href="/">
          <Button className="btn-cta">Back to Homepage</Button>
        </Link>
      </div>
    </main>
  );
}
