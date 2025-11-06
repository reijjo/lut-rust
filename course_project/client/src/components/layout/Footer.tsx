import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import "./Nav-Footer.css";
import { faLinkedin, faGithub } from "@fortawesome/free-brands-svg-icons";

export default function Footer() {
  return (
    <footer>
      <div className="footer-content wrapper">
        <p>&copy; 2025 Teemu Aitomeri</p>
        <div className="footer-links">
          <a
            href="https://www.linkedin.com/in/teemu-aitomeri/"
            target="_blank"
            rel="noopener noreferrer"
            title="LinkedIn"
          >
            <FontAwesomeIcon icon={faLinkedin} size="lg" />
          </a>
          <a
            href="https://github.com/reijjo"
            target="_blank"
            rel="noopener noreferrer"
            title="GitHub"
          >
            <FontAwesomeIcon icon={faGithub} size="lg" />
          </a>
        </div>
      </div>
    </footer>
  );
}
