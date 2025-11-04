import { Product } from "@/utils/types";

export const getProducts = async (): Promise<Product[]> => {
  const response = await fetch("http://localhost:3001/api/products");
  const products = await response.json();

  if (!response.ok) {
    throw new Error("Failed to fetch products");
  }

  return products;
};
