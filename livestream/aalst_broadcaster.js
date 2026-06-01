const { execSync } = require('child_process');
const REPO_DIR = '/Users/sac/process-intelligence';
const stream = () => {
  const log = execSync('git log --pretty=format:"%H|%an|%ai|%s"', { cwd: REPO_DIR }).toString().split('\n');
  log.forEach(line => {
    const [hash, author, date, message] = line.split('|');
    const event = {
      'ocel:activity': 'Git Commit',
      'ocel:timestamp': new Date(date).toISOString(),
      'ocel:vmap': { hash, author, message },
      'ocel:omap': [author]
    };
    process.stdout.write(JSON.stringify(event) + '\n');
  });
};
stream();