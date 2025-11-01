import "./HeroSection.css";
import Image from "next/image";
import Button from "@/components/ui/button/Button";

export default function HeroSection() {
  return (
    <section className="hero-section wrapper">
      <div className="hero-text">
        <h1>We got the best fake store in town!</h1>
        <div className="hero-desc">
          <p>
            Find all the <span>best products!</span> Clothes, jewelry,
            electronics, you name it...
          </p>
          <div className="hero-buttons">
            <Button className="btn-cta">See the products</Button>
          </div>
        </div>
      </div>
      <div className="hero-image">
        <Image
          src="/images/shophero.webp"
          alt="Hero image"
          title="fake store"
          fill
          style={{ objectFit: "cover" }}
        />
      </div>
    </section>
  );
}
