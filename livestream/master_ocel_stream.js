const fs = require('fs');
const path = require('path');
const REPLAYS_DIR = '/Users/sac/zoeapp/replays';
const seen = new Set();
function poll() {
    fs.readdirSync(REPLAYS_DIR).filter(f => f.endsWith('.json')).sort().forEach(file => {
        if (!seen.has(file)) {
            try {
                const data = JSON.parse(fs.readFileSync(path.join(REPLAYS_DIR, file), 'utf8'));
                process.stdout.write(JSON.stringify(data) + '\n');
            } catch (e) {}
            seen.add(file);
        }
    });
    setTimeout(poll, 1000);
}
console.log('--- AALST LIVE MASTER STREAM ACTIVE ---');
poll();