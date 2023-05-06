import Head from 'next/head';
import Link from "next/link";
import Layout from "../components/layout";

export default function Auth() {
    return (
        <>
            <Layout>
                <Head>
                    <title>Auth</title>
                </Head>
                <h1>Auth page</h1>
                <h2>
                    <Link href="/">Back home</Link>
                </h2>
            </Layout>
        </>
    )
}