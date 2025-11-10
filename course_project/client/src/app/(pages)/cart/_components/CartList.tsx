"use client";
import { Cart } from "@/utils/types";
import "./CartList.css";
import { use } from "react";
import { formatPrice } from "@/utils/formatters";
import Button from "@/components/ui/button/Button";
import CartItem from "./CartItem";
import { useCartTotal } from "@/lib/stores/cartStore";
import { useRouter } from "next/navigation";
import { clearCart } from "@/lib/api/cartApi";

type CartListProps = {
  cart: Promise<Cart>;
};

export default function CartList({ cart }: CartListProps) {
  const fullCart = use(cart);
  const updateTotal = useCartTotal((state) => state.updateTotal);
  const router = useRouter();

  const handleClearCart = async () => {
    const res = await clearCart();
    updateTotal(res.total);
    router.refresh();
  };

  return (
    <section className="cart-list wrapper">
      {fullCart.products.length === 0 ? (
        <h3 className="empty-cart">Your cart is empty.</h3>
      ) : (
        fullCart.products.map((product) => (
          <CartItem key={product.id} product={product} />
        ))
      )}
      {fullCart.products.length > 0 && (
        <div className="cart-total">
          <p>Total: {formatPrice(fullCart.total)}</p>
          <Button className="btn-cta" popoverTarget="buy-items">
            Buy items!
          </Button>
          <div popover="auto" id="buy-items" className="bought">
            <p>Thanks for your purchase!</p>
            <Button
              className="btn-cta"
              popoverTarget="buy-items"
              popoverTargetAction="hide"
              onClick={handleClearCart}
            >
              Ok, bye!
            </Button>
          </div>
        </div>
      )}
    </section>
  );
}
