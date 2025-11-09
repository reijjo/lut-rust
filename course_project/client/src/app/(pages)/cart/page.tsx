import { getCart } from "@/lib/api/cartApi";

export default async function Cart() {
  const cart = await getCart();

  console.log("cart", cart);
  return (
    <main>
      <h1>Cart</h1>
    </main>
  );
}
