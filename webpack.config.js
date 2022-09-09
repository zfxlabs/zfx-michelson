
module.exports = (env) => ({
  entry: {
    michelson_parser: "./src/michelson_parser.js",
  },
  output: {
    filename: "[name].bundle.js",
    path: __dirname,
  },
  mode: env.dev ? "development" : "production",
  target: "node",
});
