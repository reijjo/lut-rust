import "./NavCart.css";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCartShopping } from "@fortawesome/free-solid-svg-icons";
import Link from "next/link";
import { formatPrice } from "@/utils/formatters";
import { useCartTotal } from "@/lib/stores/cartStore";

export default function NavCart() {
  const total = useCartTotal((state) => state.total);

  return (
    <div className="nav-cart">
      <Link href="/cart" className="nav-cart-content">
        <FontAwesomeIcon icon={faCartShopping} />
        {formatPrice(total)}
      </Link>
    </div>
  );
}
