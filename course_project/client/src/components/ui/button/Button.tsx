import { ButtonHTMLAttributes } from "react";
import "./Button.css";

type ButtonProps = {
  type?: "button" | "submit" | "reset";
} & ButtonHTMLAttributes<HTMLButtonElement>;

export default function Button({ type = "button", ...rest }: ButtonProps) {
  return (
    <button type={type} {...rest}>
      {rest.children}
    </button>
  );
}
