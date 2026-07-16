const path = require('path');

function auth(req, res, next) {
  const key = req.headers['x-api-key'] || process.env.CID_BRIDGE_KEY;
  if (process.env.CID_BRIDGE_KEY && key !== process.env.CID_BRIDGE_KEY) {
    return res.status(401).json({ success: false, error: 'unauthorized' });
  }
  next();
}

module.exports = { auth };
