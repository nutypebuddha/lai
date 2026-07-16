const PLATFORMS = ['grok', 'openai', 'anthropic', 'mistral', 'claude', 'generic'];

function detectPlatform(req, res, next) {
  const platform = (req.body.platform || req.headers['x-platform'] || 'generic').toLowerCase();
  if (!PLATFORMS.includes(platform)) {
    return res.status(400).json({ success: false, error: `unsupported platform: ${platform}` });
  }
  req.body.platform = platform;
  next();
}

module.exports = { detectPlatform, PLATFORMS };
