"use client";

import { usePathname } from "next/navigation";
import "./Navbar.css";
import Link from "next/link";
import NavCart from "./NavCart";

export default function Navbar() {
  const path = usePathname();

  const navLinks = [
    {
      name: "home",
      link: "/",
    },
    {
      name: "mens",
      link: "/products/mens",
    },
    {
      name: "womens",
      link: "/products/womens",
    },
    {
      name: "jewelry",
      link: "/products/jewelry",
    },
    {
      name: "electronics",
      link: "/products/electronics",
    },
  ];
  return (
    <nav>
      <div className="nav-content wrapper">
        <Link href="/" className="nav-logo">
          Logo
        </Link>
        <ul className="nav-links">
          {navLinks.map((link) => (
            <li key={link.name}>
              <Link
                href={link.link}
                className={path == link.link ? "active" : ""}
              >
                {link.name}
              </Link>
            </li>
          ))}
        </ul>
        <NavCart />
      </div>
    </nav>
  );
}
