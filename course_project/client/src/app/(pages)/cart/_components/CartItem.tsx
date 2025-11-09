import Button from "@/components/ui/button/Button";
import "./CartItem.css";
import { CartProduct } from "@/utils/types";
import { formatPrice } from "@/utils/formatters";
import Image from "next/image";

type CartItemProps = {
  product: CartProduct;
};

export default function CartItem({ product }: CartItemProps) {
  return (
    <div className="cart-item">
      <Image src={product.image} alt={product.title} width={100} height={100} />
      <div className="cart-item-info">
        <p className="cart-item-title">{product.title}</p>
        <p className="cart-item-price">Price: {formatPrice(product.price)}</p>
        <div className="cart-item-quantity">
          <Button className="btn-secondary">-</Button>
          <p>{product.quantity}</p>
          <Button className="btn-secondary">+</Button>
        </div>
      </div>
      <div className="cart-buttons">
        <Button className="btn-delete">Remove</Button>
      </div>
    </div>
  );
}
