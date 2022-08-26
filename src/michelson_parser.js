"use strict";

const { TezosToolkit, OpKind } = require("@taquito/taquito");
const { RpcClient } = require("@taquito/rpc");
const { InMemorySigner } = require("@taquito/signer");
const { pipeline, Transform } = require("stream");
const { inspect } = require("util");

const devMode = process.env.NODE_ENV === "development";

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

//FIXME: idk if we need this async actually, just keeping the same API for now
const onEncode = async (id, content) => {
respond(id, { status: "success" });
}

//FIXME: idk if we need this async actually, just keeping the same API for now
const onDecode = async (id, content) => {
    respond(id, { status: "success" });
}

const onRequest = (id, content) => {
  if (content.kind === "Encode") {
    return onEncode(id, content);
  } else if (content.kind === "Decode") {
    return onDecode(id, content);
  } else {
    failure(new Error("invalid content.kind: " + JSON.stringify(content.kind)));
  }
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
