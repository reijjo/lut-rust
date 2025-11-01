import "./Nav-Footer.css";

export default function Footer() {
  return (
    <footer>
      <div className="footer-content wrapper">
        <p>&copy; {new Date().getFullYear()} Teemu Aitomeri</p>
        <div className="footer-links">
          <p>linkedin</p>
          <p>github</p>
        </div>
      </div>
    </footer>
  );
}
