const fs = require('fs');
const path = require('path');
const REPLAY_DIR = '/Users/sac/zoeapp/replays/';
const TARGET = '/Users/sac/zoeapp/master_conversation.ocel';
const aggregate = () => {
  const master = { 'ocel:events': [] };
  const files = fs.readdirSync(REPLAY_DIR).filter(f => f.endsWith('.json'));
  files.forEach(file => {
    try {
      const data = JSON.parse(fs.readFileSync(path.join(REPLAY_DIR, file), 'utf-8'));
      const events = data.input?.ocel2?.event_log?.events;
      if (events && Array.isArray(events)) {
        master['ocel:events'].push(...events);
      }
    } catch(e) {}
  });
  fs.writeFileSync(TARGET, JSON.stringify(master, null, 2));
  console.log('Aggregated ' + master['ocel:events'].length + ' events from ' + files.length + ' replays into ' + TARGET);
};
aggregate();