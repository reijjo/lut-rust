"use client";

import "./Product.css";
import { CartProduct, type Product as ProductType } from "@/utils/types";
import { ChangeEvent, use, useState } from "react";
import Image from "next/image";
import { formatPrice } from "@/utils/formatters";
import Button from "@/components/ui/button/Button";
import { addToCart } from "@/lib/api/cartApi";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faMinus, faPlus } from "@fortawesome/free-solid-svg-icons";

type ProductProps = {
  product: Promise<ProductType>;
};

export default function Product({ product }: ProductProps) {
  const data = use(product);

  const [add, setAdd] = useState(false);
  const [quantity, setQuantity] = useState(1);

  const addProductToCart = async () => {
    const product: CartProduct = {
      ...data,
      quantity,
    };

    setAdd(true);

    try {
      const response = await addToCart(product);
      console.log("response", response);
    } catch (err) {
      console.log(err);
    } finally {
      setAdd(false);
    }
  };

  const handleQuantityChange = (e: ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;

    if (value === "") {
      setQuantity(1);
      return;
    }

    const parsedValue = parseInt(value, 10);

    if (!Number.isNaN(parsedValue) && parsedValue >= 1) {
      setQuantity(parsedValue);
    }
  };

  return (
    <section className="product-section wrapper">
      <div className="product-desc">
        <h2>{data.title}</h2>
        <div className="quantity-price">
          <div className="quantity">
            <Button
              className="btn-secondary"
              disabled={quantity === 1}
              onClick={() => setQuantity(quantity - 1)}
            >
              <FontAwesomeIcon icon={faMinus} size="2xs" />
            </Button>
            <input
              type="text"
              value={quantity}
              className="quantity-input"
              onChange={handleQuantityChange}
            />
            <Button
              className="btn-secondary"
              onClick={() => setQuantity(quantity + 1)}
            >
              <FontAwesomeIcon icon={faPlus} size="2xs" />
            </Button>
          </div>
          <h3>{formatPrice(data.price * quantity)}</h3>
        </div>
        <Button
          className="btn-cta cart-btn"
          onClick={addProductToCart}
          disabled={add}
        >
          Add to Cart
        </Button>
        <p>{data.description}</p>
      </div>
      <div className="product-image">
        <Image
          src={data.image}
          alt={data.title}
          fill
          style={{ objectFit: "contain" }}
          sizes="200px"
        />
      </div>
    </section>
  );
}
