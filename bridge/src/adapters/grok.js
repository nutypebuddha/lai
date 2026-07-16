const { cidValidate } = require('./cid');

class GrokAdapter {
  async execute(action, data, metadata) {
    if (action === 'validate') {
      const result = await cidValidate(data.text, data.context || 'general', metadata);
      return { passed: result.passed, score: result.confidence, details: result };
    }
    if (action === 'search') {
      return await cidValidate(data.query, 'fact', metadata);
    }
    throw new Error(`grok adapter: unknown action '${action}'`);
  }
}

module.exports = GrokAdapter;
