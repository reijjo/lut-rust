import "./NavCart.css";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCartShopping } from "@fortawesome/free-solid-svg-icons";
import Link from "next/link";

export default function NavCart() {
  return (
    <div className="nav-cart">
      <Link href="/cart" className="nav-cart-content">
        <FontAwesomeIcon icon={faCartShopping} />
        0€
      </Link>
    </div>
  );
}
