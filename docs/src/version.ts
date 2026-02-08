// Read version from Cargo.toml at build time
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

export function getVersion(): string {
  try {
    // In build context, __dirname is in docs/src
    // Cargo.toml is at project root (../../Cargo.toml relative to docs/src)
    const cargoPath = resolve(process.cwd(), '../Cargo.toml');
    const cargoContent = readFileSync(cargoPath, 'utf-8');
    const match = cargoContent.match(/^version\s*=\s*"([^"]+)"/m);
    return match ? match[1] : 'unknown';
  } catch (error) {
    console.error('Failed to read version from Cargo.toml:', error);
    return 'unknown';
  }
}

export const VERSION = getVersion();
