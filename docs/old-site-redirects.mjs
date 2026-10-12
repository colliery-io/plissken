// The old MkDocs site served each page at <path>/. Some pages moved to a
// Diátaxis section, and the API reference moved under reference/. After the
// build, this writes a small page at each old address that sends the reader
// to the new one (PLSKN-T-0055). GitHub Pages is static, so a server redirect
// is not possible.
//
// Each target is relative, so the pages work under any base path. The page
// comes from starlight-aurora: a script keeps the #anchor of the old link, and
// a meta refresh is for a browser with no JavaScript.
import { existsSync } from 'node:fs';
import { mkdir, writeFile } from 'node:fs/promises';
import { dirname, join, posix } from 'node:path';
import { fileURLToPath } from 'node:url';
import { redirectPage } from 'starlight-aurora/mdbook-redirects';

// Old route -> new route, both relative to the base path.
const MOVED = {
	'getting-started/installation/': 'how-to/installation/',
	'getting-started/quickstart/': 'tutorials/quickstart/',
	'getting-started/configuration/': 'reference/configuration/',
	'guide/overview/': 'explanation/overview/',
	'guide/python/': 'how-to/python/',
	'guide/rust/': 'how-to/rust/',
	'guide/hybrid/': 'how-to/hybrid/',
	'guide/customization/': 'how-to/customization/',
	'guide/starlight/': 'how-to/starlight/',
};

// "a/b/" seen from "x/y/z/" -> "../../../a/b/"
function relative(from, to) {
	const up = from.split('/').filter(Boolean).length;
	return '../'.repeat(up) + to;
}

export function redirectFiles(pathnames) {
	const moved = { ...MOVED };
	// Each API page: api/rust/... -> reference/api/rust/...
	for (const pathname of pathnames) {
		if (pathname.startsWith('reference/api/')) {
			moved[pathname.slice('reference/'.length)] = pathname;
		}
	}
	const files = {};
	for (const [from, to] of Object.entries(moved)) {
		files[posix.join(from, 'index.html')] = relative(from, to);
	}
	return files;
}

export default function oldSiteRedirects() {
	return {
		name: 'plissken-old-site-redirects',
		hooks: {
			'astro:build:done': async ({ dir, pages, logger }) => {
				const out = fileURLToPath(dir);
				const files = redirectFiles(pages.map((p) => p.pathname));
				let written = 0;
				for (const [file, target] of Object.entries(files)) {
					const path = join(out, file);
					// Never write over a page of the site.
					if (existsSync(path)) continue;
					await mkdir(dirname(path), { recursive: true });
					await writeFile(path, redirectPage(target));
					written++;
				}
				logger.info(`${written} redirects from the old MkDocs addresses`);
			},
		},
	};
}
