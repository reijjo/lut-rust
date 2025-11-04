import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import "./Nav-Footer.css";
import { faLinkedin, faGithub } from "@fortawesome/free-brands-svg-icons";

export default function Footer() {
  return (
    <footer>
      <div className="footer-content wrapper">
        <p>&copy; {new Date().getFullYear()} Teemu Aitomeri</p>
        <div className="footer-links">
          <a href="/" target="_blank" title="LinkedIn">
            <FontAwesomeIcon icon={faLinkedin} size="lg" />
          </a>
          <a href="/" target="_blank" title="GitHub">
            <FontAwesomeIcon icon={faGithub} size="lg" />
          </a>
        </div>
      </div>
    </footer>
  );
}
