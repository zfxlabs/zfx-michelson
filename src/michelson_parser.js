"use strict";

const { MichelsonMap } = require("@taquito/taquito");
const { Schema } = require("@taquito/michelson-encoder");
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

/* Convert `map`s and `big_map`s coming from Rust to Taquito's own `MichelsonMap` class

  Note that only the top-level fields of `data` are processed, there's no recursive descent
  for converting embedded maps. This is consistent with Taquito's behaviour expecting a flat object.
*/
const convert_maps = (data) => {
  for(const field in data) {
    if(typeof data[field] === 'object') {
      let mmap = data[field]["MichelsonMap"];
      if(mmap !== undefined) {
        data[field] = MichelsonMap.fromLiteral(mmap);
      }
    }
  }
  return data;
};

//FIXME: idk if we need this async actually, just keeping the same API for now
const onEncode = async (id, content) => {
  const { schema, data } = content;
  const taquito_schema = new Schema(schema);
  const preprocessed_data = convert_maps(data);
  const value = taquito_schema.Encode(preprocessed_data);
  respond(id, { status: "Success" , value });
}

//FIXME: idk if we need this async actually, just keeping the same API for now
const onDecode = async (id, content) => {
  const { schema, michelson } = content;
  const taquito_schema = new Schema(schema);
  const value = taquito_schema.Execute(michelson);
  respond(id, { status: "Success", value });
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
      const status = "Error";
      const error = inspect(err);
      respond(id, { status, error });
    })
    .catch(failure);
});
