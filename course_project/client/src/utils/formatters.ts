export const formatPrice = (
  price: number,
  currency = "EUR",
  locale = "fi-FI"
): string => {
  return new Intl.NumberFormat(locale, {
    style: "currency",
    currency,
  }).format(price);
};
