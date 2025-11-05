"use client";

import "./Product.css";
import { type Product as ProductType } from "@/utils/types";
import { use } from "react";
import Image from "next/image";
import { formatPrice } from "@/utils/formatters";
import Button from "@/components/ui/button/Button";

type ProductProps = {
  product: Promise<ProductType>;
};

export default function Product({ product }: ProductProps) {
  const data = use(product);

  return (
    <section className="product-section wrapper">
      <div className="product-desc">
        <h2>{data.title}</h2>
        <h3>{formatPrice(data.price)}</h3>
        <Button className="btn-cta cart-btn">Add to Cart</Button>
        <p>{data.description}</p>
      </div>
      <div className="product-image">
        <Image
          src={data.image}
          alt={data.title}
          fill
          style={{ objectFit: "contain" }}
          sizes="200px"
        />
      </div>
    </section>
  );
}
