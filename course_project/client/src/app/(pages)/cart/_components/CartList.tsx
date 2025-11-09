"use client";
import { Cart } from "@/utils/types";
import "./CartList.css";
import { use } from "react";
import { formatPrice } from "@/utils/formatters";
import Button from "@/components/ui/button/Button";
import CartItem from "./CartItem";

type CartListProps = {
  cart: Promise<Cart>;
};

export default function CartList({ cart }: CartListProps) {
  const fullCart = use(cart);

  console.log("cart", fullCart);

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
          <Button className="btn-cta">Buy items!</Button>
        </div>
      )}
    </section>
  );
}
