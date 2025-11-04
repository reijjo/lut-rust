import { Product } from "@/utils/types";

const API_URL = process.env.API_URL;

console.log("API_URL", API_URL);

export const getProducts = async (): Promise<Product[]> => {
  const response = await fetch(`${API_URL}/api/products`);

  if (!response.ok) {
    throw new Error("Failed to fetch products");
  }

  const products = await response.json();
  return products;
};
