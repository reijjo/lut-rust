import { getProducts } from "@/lib/api/productApi";
import { Suspense } from "react";
import ProductList from "../_components/ProductList";

export default function Womens() {
  const womensProducts = getProducts().then((products) =>
    products.filter((product) => product.category === "women's clothing")
  );

  return (
    <main>
      <Suspense fallback={<div>Loading women's products...</div>}>
        <ProductList products={womensProducts} header="Women's Collection" />
      </Suspense>
    </main>
  );
}
