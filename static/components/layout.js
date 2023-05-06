import Head from 'next/head';
import styles from './layout.module.css';


export const meta = {
    title: "Online todo board",
    description: "Creating and managing tasks online",
}

export default function Layout({ children}) {
    return (
        <div className={styles.container}>
            <Head>
                <title>{meta.title}</title>
                <meta name="robots" content="follow, index" />
                <meta content={meta.description} name="description" />
                <meta property="og:type" content="website" />
                <meta property="og:site_name" content={meta.title} />
                <meta property="og:description" content={meta.description} />
                <meta property="og:title" content={meta.title} />
            </Head>
            <main>{children}</main>
        </div>
    );
}