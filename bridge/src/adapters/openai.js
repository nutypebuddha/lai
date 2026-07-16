class OpenAIAdapter {
  async execute(action, data, metadata) {
    if (action === 'validate') {
      const { cidValidate } = require('./cid');
      const result = await cidValidate(data.text, data.context || 'general', metadata);
      return { passed: result.passed, score: result.confidence, details: result };
    }
    throw new Error(`openai adapter: unknown action '${action}'`);
  }
}

module.exports = OpenAIAdapter;
