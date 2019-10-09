import classes from "./Header.module.scss"
import logo from "../../assets/logov3.svg"

const Header = () => (
  <header className={classes.header}>
    <img className={classes.logo} src={logo} alt="Coffeed logo" />
    <nav className={classes.navbar}>
      <ul className={classes.menu}>
        <li className={classes.item}>
          <a>Homepage</a>
        </li>
        <li className={classes.item}>
          <a>About</a>
        </li>
        <li className={classes.item}>
          <a>Shop</a>
        </li>
        <li className={classes.item}>
          <a>Blog</a>
        </li>
        <li className={classes.item}>
          <a>Contact us</a>
        </li>
      </ul>
    </nav>
  </header>
)

export default Header
