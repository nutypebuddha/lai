class MistralAdapter {
  async execute(action, data, metadata) {
    if (action === 'validate') {
      const { cidValidate } = require('./cid');
      const result = await cidValidate(data.text, data.context || 'general', metadata);
      return { passed: result.passed, score: result.confidence, details: result };
    }
    throw new Error(`mistral adapter: unknown action '${action}'`);
  }
}

module.exports = MistralAdapter;
