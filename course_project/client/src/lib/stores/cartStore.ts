import { create } from "zustand";

type CartStore = {
  total: number;
  updateTotal: (total: number) => void;
};

export const useCartTotal = create<CartStore>()((set) => ({
  total: 0,
  updateTotal: (total) => set({ total }),
}));
