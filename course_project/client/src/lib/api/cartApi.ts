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

export const updateCartItem = async (
  id: number,
  quantity: number
): Promise<Cart> => {
  try {
    const response = await fetch(`${API_URL}/api/cart/${id}`, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ quantity }),
    });

    if (!response.ok) {
      throw new Error("Failed to update cart item");
    }

    const cart: Cart = await response.json();
    return cart;
  } catch (error) {
    console.error("Error updating cart: ", error);
    throw new Error("Unknown error updating cart");
  }
};

export const deleteCartItem = async (id: number): Promise<Cart> => {
  try {
    const response = await fetch(`${API_URL}/api/cart/${id}`, {
      method: "DELETE",
    });

    if (!response.ok) {
      throw new Error("Failed to delete cart item");
    }

    const cart: Cart = await response.json();
    return cart;
  } catch (error) {
    console.error("Error deleting cart item: ", error);
    throw new Error("Unknown error deleting cart item");
  }
};
