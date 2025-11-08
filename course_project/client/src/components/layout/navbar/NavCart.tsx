import "./NavCart.css";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCartShopping } from "@fortawesome/free-solid-svg-icons";
import Link from "next/link";
import { formatPrice } from "@/utils/formatters";
import { use } from "react";
import { Cart } from "@/utils/types";

type NavCartProps = {
  cart: Promise<Cart>;
};

export default function NavCart({ cart }: NavCartProps) {
  const cartTotal = use(cart);

  return (
    <div className="nav-cart">
      <Link href="/cart" className="nav-cart-content">
        <FontAwesomeIcon icon={faCartShopping} />
        {formatPrice(cartTotal.total)}
      </Link>
    </div>
  );
}
