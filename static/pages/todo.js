import Layout from "../components/layout";
import Head from "next/head";
import Link from "next/link";


export default function Todo() {
    return (
        <>
            <Layout>
                <Head>
                    <title>Todo</title>
                </Head>
                <h1>Todo page</h1>
                <h2>
                    <Link href="/">Back home</Link>
                </h2>
            </Layout>
        </>
    )
}