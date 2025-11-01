import "./Nav-Footer.css";

export default function Navbar() {
  return (
    <nav>
      <div className="nav-content wrapper">
        <p className="nav-logo">Logo</p>
        <ul className="nav-links">
          <li>mens</li>
          <li>womens</li>
        </ul>
        <div className="nav-cart">
          <p>Cart total: 545€</p>
        </div>
      </div>
    </nav>
  );
}
