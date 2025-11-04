"use client";

import { usePathname } from "next/navigation";
import "./Navbar.css";
import Link from "next/link";
import NavCart from "./NavCart";
import Button from "@/components/ui/button/Button";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faBars, faClose } from "@fortawesome/free-solid-svg-icons";
import { useState } from "react";

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

type NavLinksProps = {
  setMobileOpen: (isOpen: boolean) => void;
};

export const NavLinks = ({ setMobileOpen }: NavLinksProps) => {
  const path = usePathname();

  return (
    <ul className="nav-links">
      {navLinks.map((link) => (
        <li key={link.name}>
          <Link
            href={link.link}
            className={path === link.link ? "active" : ""}
            onClick={() => setMobileOpen(false)}
          >
            {link.name}
          </Link>
        </li>
      ))}
    </ul>
  );
};

export default function Navbar() {
  const [mobileOpen, setMobileOpen] = useState(false);

  return (
    <nav>
      <div className="nav-content wrapper">
        <Link href="/" className="nav-logo">
          Logo
        </Link>
        <NavLinks setMobileOpen={setMobileOpen} />
        <Button
          className="btn-secondary nav-hamburger"
          onClick={() => setMobileOpen(!mobileOpen)}
        >
          <FontAwesomeIcon icon={mobileOpen ? faClose : faBars} />
        </Button>
        <NavCart />
      </div>
      {mobileOpen && (
        <div className="mobile-menu">
          <NavLinks setMobileOpen={setMobileOpen} />
        </div>
      )}
    </nav>
  );
}
