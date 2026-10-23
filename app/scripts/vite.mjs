// Resolve the physical working directory for consistent Vite/esbuild paths.
import { realpathSync } from 'node:fs';
process.chdir(realpathSync(process.cwd()));
await import('../node_modules/vite/bin/vite.js');
