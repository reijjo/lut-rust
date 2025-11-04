"use client";

import "./HomeProducts.css";
import { use } from "react";
import { Product } from "@/utils/types";
import Image from "next/image";
import Link from "next/link";

type HomeProductsProps = {
  products: Promise<Product[]>;
};

export default function HomeProducts({ products }: HomeProductsProps) {
  const allProducts = use(products);

  return (
    <div className="home-products wrapper">
      <section className="home-product-section">
        <h2>{allProducts[1].category}</h2>
        <Link href="/products/mens" className="home-product-item">
          <Image
            src={allProducts[1].image}
            alt={allProducts[1].category}
            fill
            sizes="(max-width: 800px) 100vw"
            style={{ objectFit: "contain" }}
          />
        </Link>
      </section>
      <section className="home-product-section">
        <h2>{allProducts[15].category}</h2>
        <Link href="/products/womens" className="home-product-item">
          <Image
            src={allProducts[15].image}
            alt={allProducts[15].category}
            fill
            sizes="(max-width: 800px) 100vw"
            style={{ objectFit: "contain" }}
          />
        </Link>
      </section>
      <section className="home-product-section">
        <h2>{allProducts[5].category}</h2>
        <Link href="/products/jewelery" className="home-product-item">
          <Image
            src={allProducts[5].image}
            alt={allProducts[5].category}
            fill
            sizes="(max-width: 800px) 100vw"
            style={{ objectFit: "contain" }}
          />
        </Link>
      </section>
      <section className="home-product-section">
        <h2>{allProducts[13].category}</h2>
        <Link href="/products/electronics" className="home-product-item">
          <Image
            src={allProducts[13].image}
            alt={allProducts[13].category}
            fill
            sizes="(max-width: 800px) 100vw"
            style={{ objectFit: "contain" }}
          />
        </Link>
      </section>
    </div>
  );
}
