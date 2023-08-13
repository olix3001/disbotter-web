import { sentrySvelteKit } from '@sentry/sveltekit';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import * as child_process from 'child_process';

const commit_hash = child_process.execSync('git rev-parse --short HEAD').toString();
console.log(`Commit hash: ${commit_hash}`);

export default defineConfig({
	plugins: [
		sentrySvelteKit({
			sourceMapsUploadOptions: {
				org: 'olix3001',
				project: 'disbotter'
			}
		}),
		sveltekit()
	],
	define: {
		'import.meta.env.VITE_BUILD_TIME': JSON.stringify(new Date().toUTCString()),
		'import.meta.env.VITE_COMMIT_HASH': JSON.stringify(commit_hash)
	}
});
