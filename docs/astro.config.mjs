// The plissken docs (PLSKN-T-0055): Astro Starlight with the Colliery theme
// starlight-aurora, in the four Diátaxis sections. The API reference is
// written by plissken itself, with its `starlight` template.
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import starlightAurora from 'starlight-aurora';
import apiSidebar from './src/content/docs/reference/api/_sidebar.mjs';
import oldSiteRedirects from './old-site-redirects.mjs';

// The Pages workflow sets DOCS_SITE and DOCS_BASE. On a pull request and
// locally they are empty, and the site is at /.
const site = process.env.DOCS_SITE || undefined;
const base = process.env.DOCS_BASE || '/';

const PROJECT = 'plissken';
const REPO = 'https://github.com/colliery-io/plissken';

// The sidebar is written out: it keeps the order and the labels of the old
// MkDocs nav. Add a new page here.
const page = (slug, label) => ({ slug, label });

export default defineConfig({
	site,
	base,
	integrations: [
		starlight({
			title: PROJECT,
			description: 'Documentation generator for Rust-Python hybrid projects',
			logo: { src: './src/assets/logo.png', alt: '' },
			social: [{ icon: 'github', label: 'GitHub', href: REPO }],
			editLink: { baseUrl: `${REPO}/edit/main/docs/` },
			plugins: [starlightAurora({ hyper: true })],
			sidebar: [
				{ label: 'Home', link: '/' },
				{
					label: 'Tutorials',
					items: [page('tutorials/quickstart', 'Quick Start')],
				},
				{
					label: 'How-to guides',
					items: [
						page('how-to/installation', 'Installation'),
						page('how-to/python', 'Python Projects'),
						page('how-to/rust', 'Rust Projects'),
						page('how-to/hybrid', 'Hybrid Projects'),
						page('how-to/customization', 'Customization'),
						page('how-to/starlight', 'Starlight Sites'),
					],
				},
				{
					label: 'Reference',
					items: [
						page('reference', 'Overview'),
						page('reference/configuration', 'Configuration'),
						apiSidebar,
					],
				},
				{
					label: 'Explanation',
					items: [page('explanation/overview', 'Overview')],
				},
			],
		}),
		// The old MkDocs addresses (getting-started/x/, guide/x/, api/...) go to
		// the new pages.
		oldSiteRedirects(),
	],
});
