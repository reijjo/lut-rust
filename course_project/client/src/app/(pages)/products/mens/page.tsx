import { getProducts } from "@/lib/api/productApi";
import { Suspense } from "react";
import ProductList from "../_components/ProductList";
import { Loading } from "@/components/ui/Loading";

export default function Mens() {
  const mensProducts = getProducts().then((products) =>
    products.filter((product) => product.category === "men's clothing")
  );

  return (
    <main>
      <Suspense fallback={<Loading text="Loading men's products..." />}>
        <ProductList products={mensProducts} header="Men's Collection" />
      </Suspense>
    </main>
  );
}
