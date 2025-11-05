import { getProducts } from "@/lib/api/productApi";
import ProductList from "./_components/ProductList";
import { Loading } from "@/components/ui/Loading";
import { Suspense } from "react";

export default function Products() {
  const products = getProducts();

  return (
    <main>
      <Suspense fallback={<Loading text="Loading products..." />}>
        <ProductList products={products} header="Products" />
      </Suspense>
    </main>
  );
}
