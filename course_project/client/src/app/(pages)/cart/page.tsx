import { getCart } from "@/lib/api/cartApi";
import CartList from "./_components/CartList";
import { Suspense } from "react";
import { Loading } from "@/components/ui/Loading";

export default function Cart() {
  const cart = getCart();

  return (
    <main>
      <Suspense fallback={<Loading text="Loading cart..." />}>
        <CartList cart={cart} />
      </Suspense>
    </main>
  );
}
