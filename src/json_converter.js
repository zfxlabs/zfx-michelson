"use strict";

const { MichelsonMap } = require("@taquito/taquito");
const { Schema, UnitValue } = require("@taquito/michelson-encoder");

exports.jsonDecode = (schema, input) => {
  const michelson = clone(input);
  const taquito_schema = new Schema(schema);
  const raw_value = taquito_schema.Execute(michelson);
  return postprocess(raw_value);
}

exports.jsonEncode = (schema, input) => {
  const data = clone(input);
  const taquito_schema = new Schema(schema);
  const preprocessed_data = preprocessEncode(data);
  return taquito_schema.Encode(preprocessed_data);
}

const preprocessEncode = (data) => convert_maps(encode_unit(data));

const postprocess = (data) => decode_unit(data);


/* Convert `map`s and `big_map`s coming from Rust to Taquito's own `MichelsonMap` class

  Note that only the top-level fields of `data` are processed, there's no recursive descent
  for converting embedded maps. This is consistent with Taquito's behaviour expecting a flat object.
*/
const convert_maps = (data) => {
  for(const field in data) {
    if(!data.hasOwnProperty(field)) {
      continue;
    }
    if(typeof data[field] === 'object') {
      let mmap = data[field]["MichelsonMap"];
      if(mmap !== undefined) {
        data[field] = MichelsonMap.fromLiteral(mmap);
      }
    }
  }
  return data;
};

const JsonUnit = { __unit__: null };
exports.JsonUnit = JsonUnit;

const is_unit = (obj) => ("__unit__" in obj) && obj.__unit__ === null;

const encode_unit = (data) => {
  if (Array.isArray(data)) {
    data.forEach((value, i, array) => {
      array[i] = encode_unit(value);
    });
  } else if (typeof data === 'object') {
    if (is_unit(data)) { data = UnitValue; }
    else for (const field in data) {
      if(!data.hasOwnProperty(field)) {
        continue;
      }
      data[field] = encode_unit(data[field]);
    }
  }
  return data;
};

const decode_unit = (data) => {
  // console.log("data", data, data == UnitValue);
  if (data === UnitValue) { 
    data = JsonUnit;
  } else if (Array.isArray(data)) {
    data.forEach((value, i, array) => {
      array[i] = decode_unit(value);
    });
  } else if (typeof data === 'object') {
     for (const field in data) {
      if(!data.hasOwnProperty(field)) {
        continue;
      }
      // console.log("field", data[field]);
      data[field] = decode_unit(data[field]);
    }
  }
  return data;
};

function clone(a) {
   return JSON.parse(JSON.stringify(a));
}
