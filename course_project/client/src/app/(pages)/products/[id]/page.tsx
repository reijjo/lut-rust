import { getProductById } from "@/lib/api/productApi";
import Product from "./_components/Product";
import { Suspense } from "react";

type ProductPageProps = {
  params: Promise<{ id: string }>;
};

export default async function ProductPage({ params }: ProductPageProps) {
  const { id } = await params;

  const product = getProductById(Number(id));

  return (
    <Suspense fallback={<div>Loading product...</div>}>
      <Product product={product} />
    </Suspense>
  );
}
