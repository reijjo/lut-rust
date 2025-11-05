import { getProducts } from "@/lib/api/productApi";
import { Suspense } from "react";
import ProductList from "../_components/ProductList";
import { Loading } from "@/components/ui/Loading";

export default function Electronics() {
  const electronicsProducts = getProducts().then((products) =>
    products.filter((product) => product.category === "electronics")
  );

  return (
    <main>
      <Suspense fallback={<Loading text="Loading electronics..." />}>
        <ProductList products={electronicsProducts} header="Electronics" />
      </Suspense>
    </main>
  );
}
