const path = require('path');

module.exports = (env) => ({
    entry: {
    michelson_parser: "./src/michelson_parser.js",
  },
  output: {
      filename: "[name].bundle.js",
      path: path.resolve(__dirname, 'scripts'),
  },
    mode: env.dev ? "development" : "production",
    target: "node",
});
