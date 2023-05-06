import Head from 'next/head';
import Layout, { meta } from '../components/layout';
import utilStyles from '../styles/utils.module.css';
import List from "../components/ui/list";

export default function Home() {
  return (
      <Layout>
        <Head>
          <title>{meta.title}</title>
        </Head>
        <section className={utilStyles.headingMd}>
            <List/>
        </section>
      </Layout>
  );
}