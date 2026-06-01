const fs = require('fs');
const path = require('path');
const LOG_DIR = '/Users/sac/.gemini/tmp/zoeapp/chats';
const parse = () => {
  const events = [];
  const files = fs.readdirSync(LOG_DIR).filter(f => f.endsWith('.jsonl'));
  files.forEach(file => {
    const lines = fs.readFileSync(path.join(LOG_DIR, file), 'utf-8').split('\n');
    lines.forEach(line => {
      if (!line.trim()) return;
      try {
        const entry = JSON.parse(line);
        if (entry.thoughts) entry.thoughts.forEach(t => events.push({ activity: 'Admissibility Guard', timestamp: t.timestamp, type: 'intent', body: t.description }));
        if (entry.toolCalls) entry.toolCalls.forEach(tc => {
          events.push({ activity: tc.name, timestamp: entry.timestamp, type: 'activity', body: tc.args });
          if (tc.result) events.push({ activity: 'Event', timestamp: tc.timestamp || entry.timestamp, type: 'event', body: tc.result });
        });
      } catch(e) }
    });
  });
  return events.sort((a,b) => new Date(a.timestamp) - new Date(b.timestamp));
};
const log = parse();
console.log(JSON.stringify({ "ocel:events": log }, null, 2));