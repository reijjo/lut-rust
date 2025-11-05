import { getProducts } from "@/lib/api/productApi";
import { Suspense } from "react";
import ProductList from "../_components/ProductList";
import { Loading } from "@/components/ui/Loading";

export default function Womens() {
  const womensProducts = getProducts().then((products) =>
    products.filter((product) => product.category === "women's clothing")
  );

  return (
    <main>
      <Suspense fallback={<Loading text="Loading women's products..." />}>
        <ProductList products={womensProducts} header="Women's Collection" />
      </Suspense>
    </main>
  );
}
