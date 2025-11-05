import Link from "next/link";
import Image from "next/image";
import { Product } from "@/utils/types";
import { formatPrice } from "@/utils/formatters";

type ProductCardProps = {
  product: Product;
};

export default function ProductCard({ product }: ProductCardProps) {
  return (
    <Link href={`/products/${product.id}`} className="product">
      <div className="product-image-wrapper">
        <Image
          src={product.image}
          alt={product.title}
          fill
          style={{ objectFit: "contain" }}
          sizes="200px"
        />
      </div>
      <div className="product-card-info">
        <p>{product.title}</p>
        <p>{formatPrice(product.price)}</p>
      </div>
    </Link>
  );
}
