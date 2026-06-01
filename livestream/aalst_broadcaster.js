const { execSync } = require('child_process');
const repoPath = '/Users/sac/process-intelligence';
try {
  const log = execSync('git -C ' + repoPath + ' log --reverse --pretty=format:"%H|%an|%ad|%s" --date=iso', { encoding: 'utf-8' });
  const commits = log.trim().split('\n').filter(Boolean).map(line => {
      const [hash, author, date, message] = line.split('|');
      return { hash, author, date, message };
  });
  
  const events = commits.map((commit, i) => ({
      "ocel:activity": "Git Commit",
      "ocel:timestamp": new Date(commit.date).toISOString(),
      "ocel:vmap": { hash: commit.hash, author: commit.author, message: commit.message },
      "ocel:omap": [commit.author]
  }));

  (async function stream() {
      for (const e of events) {
          console.log(JSON.stringify(e));
          await new Promise(r => setTimeout(r, 100));
      }
  })();
} catch (e) {
  console.error("Error: ", e.message);
}