import { Product } from "@/utils/types";

const API_URL = process.env.NEXT_PUBLIC_API_URL;

export const getProducts = async (): Promise<Product[]> => {
  try {
    const response = await fetch(`${API_URL}/api/products`);

    if (!response.ok) {
      throw new Error("Failed to fetch products");
    }

    const products = await response.json();
    return products;
  } catch (error) {
    console.error("Error fetchin products: ", error);
    throw new Error("Unknown error fetchin products");
  }
};

export const getProductById = async (id: number): Promise<Product> => {
  try {
    const response = await fetch(`${API_URL}/api/products/${id}`);

    if (!response.ok) {
      throw new Error(
        `Failed to fetch product ${id}: ${response.status} ${response.statusText}`
      );
    }

    const product = await response.json();
    return product;
  } catch (error) {
    console.error(`Error fetchin product ${id}: `, error);
    throw new Error(`Unknown error fetchin product ${id}`);
  }
};
