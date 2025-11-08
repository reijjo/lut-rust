import { Cart, CartProduct } from "@/utils/types";

const API_URL = process.env.NEXT_PUBLIC_API_URL;

export const getCart = async (): Promise<Cart> => {
  try {
    const response = await fetch(`${API_URL}/api/cart`);

    if (!response.ok) {
      throw new Error("Failed to fetch cart");
    }

    const cart: Cart = await response.json();
    return cart;
  } catch (error) {
    console.error("Error getting cart: ", error);
    throw new Error("Unknown error fetchin cart");
  }
};

export const addToCart = async (product: CartProduct): Promise<Cart> => {
  try {
    const response = await fetch(`${API_URL}/api/cart`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(product),
    });

    if (!response.ok) {
      throw new Error("Failed to add item to cart");
    }

    const cart: Cart = await response.json();
    return cart;
  } catch (error) {
    console.error("Error adding item to cart: ", error);
    throw new Error("Unknown error adding item to cart");
  }
};
