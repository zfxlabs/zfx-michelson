"use strict";

const { Schema } = require("@taquito/michelson-encoder"); 
const { pipeline, Transform } = require("stream");
const { inspect } = require("util");

const failure = (err) => {
  console.error(err);
  process.exit(1);
};

/** @param {Response} message */
const write = (message) => {
  const callback = (err) => {
    if (err) {
      failure(err);
    }
  };
  const messageString = JSON.stringify(message);
  process.stdout.write(messageString + "\n", callback);
};

const respond = (id, content) => write({ id, content });

/**
 * @callback RequestCallback
 * @param {Request} request
 */

/** @param {RequestCallback} callback */
const read = (callback) => {
  let buf = "";
  /* WARNING: inputs are separated by new lines,
    that means each input must be contained in a single line

    similar to http://ndjson.org/
  */
  const parseStream = Transform({
    objectMode: true,
    transform(chunk, encoding, done) {
      if (encoding !== "utf8") {
        throw TypeError("encoding expected to be utf8");
      }
      buf = buf + chunk;

      const parts = buf.split("\n");
      const messages = parts.slice(0, -1); // everything except last

      buf = parts.slice(-1)[0]; // last

      for (const message of messages) {
        try {
          const json = JSON.parse(message);
          this.push(json);
        } catch (err) {
          return done(err);
        }
      }
      return done();
    },
  });
  const errorHandler = (err = "close") => failure(err);

  pipeline(process.stdin, parseStream, errorHandler).on("data", callback);
};

const onRequest = async (id, content) => {
  const { schema, michelson } = content;
  const taquito_schema = new Schema(schema);
  const data = taquito_schema.Execute(michelson);
  respond(id, { data });
};

read((request) => {
  const { id, content } = request;
  onRequest(id, content)
    .catch((err) => {
      const status = "error";
      const error = inspect(err);
      respond(id, { status, error });
    })
    .catch(failure);
});
