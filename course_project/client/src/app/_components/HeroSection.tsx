import "./HeroSection.css";
import Image from "next/image";
import Button from "@/components/ui/button/Button";
import Link from "next/link";

export default function HeroSection() {
  return (
    <section className="hero-section wrapper">
      <div className="hero-text">
        <h1>We got the best fake store in town!</h1>
        <div className="hero-desc">
          <div>
            <p>
              Find all the <span>best products!</span>{" "}
            </p>{" "}
            <p>Clothes, jewelry, electronics, you name it...</p>
          </div>
          <div className="hero-buttons">
            <Link href="/products" className="hero-link">
              <Button className="btn-cta">See All Products</Button>
            </Link>
          </div>
        </div>
      </div>
      <div className="hero-image">
        <Image
          src="/images/shophero.webp"
          alt="Fake store for all the fake people"
          title="fake store"
          fill
          sizes="(max-width: 800px) 100vw"
          priority
          style={{ objectFit: "cover" }}
        />
      </div>
    </section>
  );
}
