import { Suspense } from "react";
import HeroSection from "./_components/HeroSection";
import HomeProducts from "./_components/home-products/HomeProducts";
import { getProducts } from "@/lib/api/productApi";

export default function Home() {
  const products = getProducts();

  return (
    <main>
      <HeroSection />
      <Suspense fallback={<div>Loading...</div>}>
        <HomeProducts products={products} />
      </Suspense>
    </main>
  );
}
