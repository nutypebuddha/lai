const { execSync } = require('child_process');
const path = require('path');

const CID_BINARY = process.env.CID_BINARY || path.resolve(__dirname, '../../../target/release/lai');

async function cidValidate(text, context, metadata) {
  try {
    const stdout = execSync(`"${CID_BINARY}" gate validate "${text.replace(/"/g, '\\"')}" ${context}`, {
      encoding: 'utf-8',
      timeout: 10000,
    });
    return JSON.parse(stdout);
  } catch (err) {
    const stdout = err.stdout ? err.stdout.toString() : '';
    try { return JSON.parse(stdout); } catch (_) {}
    return { validated_text: text, confidence: 0.5, passed: false, fix_count: 0, error: err.message };
  }
}

async function validate(text, context) {
  return cidValidate(text, context);
}

module.exports = { cidValidate, validate };
