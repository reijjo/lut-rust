import "./ProductList.css";
import { Product } from "@/utils/types";
import { use } from "react";
import ProductCard from "./ProductCard";

type ProductListProps = {
  products: Promise<Product[]>;
  header: string;
};

export default function ProductList({ products, header }: ProductListProps) {
  const allProducts = use(products);

  return (
    <section className="wrapper product-list-section">
      <h2>{header}</h2>
      <div className="product-list">
        {allProducts.map((product) => (
          <ProductCard key={product.id} product={product} />
        ))}
      </div>
    </section>
  );
}
