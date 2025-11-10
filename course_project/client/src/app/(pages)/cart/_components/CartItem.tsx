import Button from "@/components/ui/button/Button";
import "./CartItem.css";
import { CartProduct } from "@/utils/types";
import { formatPrice } from "@/utils/formatters";
import Image from "next/image";
import { deleteCartItem, updateCartItem } from "@/lib/api/cartApi";
import { useRouter } from "next/navigation";
import { useCartTotal } from "@/lib/stores/cartStore";

type CartItemProps = {
  product: CartProduct;
};

export default function CartItem({ product }: CartItemProps) {
  const router = useRouter();
  const updateTotal = useCartTotal((state) => state.updateTotal);

  const plusQuantity = async () => {
    const res = await updateCartItem(product.id, product.quantity + 1);
    updateTotal(res.total);
    router.refresh();
  };

  const minusQuantity = async () => {
    if (product.quantity === 1) return;

    const res = await updateCartItem(product.id, product.quantity - 1);
    updateTotal(res.total);
    router.refresh();
  };

  const removeItem = async () => {
    try {
      const res = await deleteCartItem(product.id);
      updateTotal(res.total);
      router.refresh();
    } catch (error) {
      console.error("Error deleting cart item: ", error);
    }
  };

  return (
    <div className="cart-item">
      <Image src={product.image} alt={product.title} width={100} height={100} />
      <div className="cart-item-info">
        <p className="cart-item-title">{product.title}</p>
        <p className="cart-item-price">Price: {formatPrice(product.price)}</p>
        <div className="cart-item-quantity">
          <Button className="btn-secondary" onClick={minusQuantity}>
            -
          </Button>
          <p>{product.quantity}</p>
          <Button className="btn-secondary" onClick={plusQuantity}>
            +
          </Button>
        </div>
      </div>
      <div className="cart-buttons">
        <Button
          className="btn-delete"
          popoverTarget={`confirm-delete-${product.id}`}
        >
          Remove
        </Button>
        <div
          popover="auto"
          id={`confirm-delete-${product.id}`}
          className="confirm-delete"
        >
          <p className="cart-item-title">
            Are you sure you want to remove {product.title}?
          </p>
          <div className="delete-item-buttons">
            <Button className="btn-delete" onClick={removeItem}>
              Yes, Delete
            </Button>
            <Button
              className="btn-secondary"
              popoverTarget={`confirm-delete-${product.id}`}
              popoverTargetAction="hide"
            >
              Cancel
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
