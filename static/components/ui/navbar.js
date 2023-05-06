import styles from './navbar.module.css';
import Link from "next/link";


const Navbar = () => {
    return (
        <ul className={styles.navContainer}>
            <li className={styles.leftItem}>
                <Link className={styles.itemLink} href="/auth">TODO</Link>
            </li>
            <li className={styles.rightItem}>
                <Link className={styles.itemLink} href="/auth">Sign in</Link>
            </li>
        </ul>
    )
}
export default Navbar;