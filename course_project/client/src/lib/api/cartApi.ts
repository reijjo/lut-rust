import { Cart } from "@/utils/types";

const API_URL = process.env.NEXT_PUBLIC_API_URL;

export const getCart = async (): Promise<Cart> => {
  console.log("API_URL", API_URL);
  const response = await fetch(`${API_URL}/api/cart`);

  if (!response.ok) {
    throw new Error("Failed to fetch cart");
  }

  const cart = await response.json();
  return cart;
};
