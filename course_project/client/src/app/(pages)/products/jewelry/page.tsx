import { getProducts } from "@/lib/api/productApi";
import { Suspense } from "react";
import ProductList from "../_components/ProductList";

export default function Jewelery() {
  const jeweleryProducts = getProducts().then((products) =>
    products.filter((product) => product.category === "jewelery")
  );

  return (
    <main>
      <Suspense fallback={<div>Loading jewelry...</div>}>
        <ProductList products={jeweleryProducts} header="Jewelery" />
      </Suspense>
    </main>
  );
}
