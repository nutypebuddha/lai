const http = require('http');

const BASE = 'http://localhost:3000';

async function test(path, opts) {
  return new Promise((resolve, reject) => {
    const req = http.request(`${BASE}${path}`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, ...opts }, (res) => {
      let body = '';
      res.on('data', (d) => body += d);
      res.on('end', () => {
        try { resolve({ status: res.statusCode, body: JSON.parse(body) }); }
        catch { resolve({ status: res.statusCode, body }); }
      });
    });
    req.on('error', reject);
    if (opts?.body) req.write(JSON.stringify(opts.body));
    req.end();
  });
}

async function main() {
  let ok = 0, fail = 0;

  // health
  try {
    const h = await new Promise((res) => {
      http.get(`${BASE}/health`, (r) => { let b=''; r.on('data', d=>b+=d); r.on('end',()=>res({status:r.statusCode,body:JSON.parse(b)})); });
    });
    if (h.status === 200 && h.body.status === 'ok') { console.log('✅ health'); ok++; } else { console.log('❌ health', h); fail++; }
  } catch(e) { console.log('❌ health', e.message); fail++; }

  // validate
  const v = await test('/validate', { body: { text: '2 + 2 = 4', context: 'math' } });
  if (v.status === 200 && v.body.success) { console.log('✅ /validate'); ok++; } else { console.log('❌ /validate', v); fail++; }

  // mcp grok
  const g = await test('/mcp', { body: { platform: 'grok', action: 'validate', data: { text: 'E=mc^2', context: 'fact' } } });
  if (g.status === 200 && g.body.success) { console.log('✅ /mcp grok'); ok++; } else { console.log('❌ /mcp grok', g); fail++; }

  // mcp openai
  const o = await test('/mcp', { body: { platform: 'openai', action: 'validate', data: { text: 'Earth is round', context: 'fact' } } });
  if (o.status === 200 && o.body.success) { console.log('✅ /mcp openai'); ok++; } else { console.log('❌ /mcp openai', o); fail++; }

  // mcp unknown platform
  const u = await test('/mcp', { body: { platform: 'unknown', action: 'validate', data: { text: 'x' } } });
  if (u.status === 400) { console.log('✅ /mcp unknown rejected'); ok++; } else { console.log('❌ /mcp unknown', u); fail++; }

  console.log(`\n${ok} passed, ${fail} failed`);
  process.exit(fail > 0 ? 1 : 0);
}

main();
