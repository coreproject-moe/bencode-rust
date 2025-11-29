import { readdirSync, statSync, unlinkSync } from 'fs';
import { join, extname, basename } from 'path';
import { execSync } from 'child_process';

// Recursively minify all JS files in a directory, delete originals, generate source maps
function minifyDir(dir: string) {
    const files = readdirSync(dir);

    for (const file of files) {
        const fullPath = join(dir, file);
        const stat = statSync(fullPath);

        if (stat.isDirectory()) {
            minifyDir(fullPath); // Recurse into subdirectories
        } else if (extname(file) === '.js') {
            const minFile = join(dir, basename(file, '.js') + '.min.js');
            const mapFile = minFile + '.map';
            console.log(
                `Minifying ${fullPath} → ${minFile} (source map: ${mapFile})`
            );

            // Run terser with source map
            execSync(
                `terser "${fullPath}" -c -m -o "${minFile}" --source-map "url='${basename(
                    mapFile
                )}'" --ecma 2025`
            );

            // Delete original JS
            unlinkSync(fullPath);
        }
    }
}

// Example: minify all JS files in "pkg" folder
minifyDir('pkg');
