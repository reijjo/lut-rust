import { getProducts } from "@/lib/api/productApi";
import { Suspense } from "react";
import ProductList from "../_components/ProductList";
import { Loading } from "@/components/ui/Loading";

export default function Jewelery() {
  const jeweleryProducts = getProducts().then((products) =>
    products.filter((product) => product.category === "jewelery")
  );

  return (
    <main>
      <Suspense fallback={<Loading text="Loading jewelery..." />}>
        <ProductList products={jeweleryProducts} header="Jewelry" />
      </Suspense>
    </main>
  );
}
