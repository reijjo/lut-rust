import { getProducts } from "@/lib/api/productApi";
import ProductList from "./_components/ProductList";
import { Suspense } from "react";

export default function Products() {
  const products = getProducts();

  return (
    <main>
      <Suspense fallback={<div>Loading...</div>}>
        <ProductList products={products} header="Products" />
      </Suspense>
    </main>
  );
}
