import { ButtonHTMLAttributes } from "react";
import "./Button.css";

type ButtonProps = {
  children: React.ReactNode;
  type?: "button" | "submit" | "reset";
} & ButtonHTMLAttributes<HTMLButtonElement>;

export default function Button({
  children,
  type = "button",
  ...rest
}: ButtonProps) {
  return (
    <button type={type} {...rest}>
      {children}
    </button>
  );
}
