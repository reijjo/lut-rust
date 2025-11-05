import { getProducts } from "@/lib/api/productApi";
import { Suspense } from "react";
import ProductList from "../_components/ProductList";

export default function Mens() {
  const mensProducts = getProducts().then((products) =>
    products.filter((product) => product.category === "men's clothing")
  );

  return (
    <main>
      <Suspense fallback={<div>Loading men's products...</div>}>
        <ProductList products={mensProducts} header="Men's Collection" />
      </Suspense>
    </main>
  );
}
