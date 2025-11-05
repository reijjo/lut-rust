import { getProducts } from "@/lib/api/productApi";
import { Suspense } from "react";
import ProductList from "../_components/ProductList";

export default function Electronics() {
  const electronicsProducts = getProducts().then((products) =>
    products.filter((product) => product.category === "electronics")
  );

  return (
    <main>
      <Suspense fallback={<div>Loading electronic products...</div>}>
        <ProductList products={electronicsProducts} header="Electronics" />
      </Suspense>
    </main>
  );
}
