/*
  JSON <-> Taquito converter.
  
  Taquito uses several non-serialisable data types, this module aims
  to convert those to JSON in both directions.

  (Most internal functions modify their input, only exported functions
   operate on a copy of the data)

  Details:

  `map` and `big_map` values <==> { "MichelsonMap": {"key":"value",...}}
  unit                       <==> JsonUnit == { "__unit__": null }
  enum                       <==> { "__enum__": "EnumVariant" }
  Numbers are deserialised as String (as they are unbounded)

*/
"use strict";

const { MichelsonMap } = require("@taquito/taquito");
const { Schema, UnitValue } = require("@taquito/michelson-encoder");
const { BigNumber } = require("bignumber.js");
const util = require("util");

exports.jsonDecode = (schema, input) => {
  const michelson = clone(input);
  const taquito_schema = new Schema(schema);
  const raw_value = taquito_schema.Execute(michelson);
  return postprocess(raw_value);
};

exports.jsonEncode = (schema, input) => {
  const data = clone(input);
  const taquito_schema = new Schema(schema);
  const preprocessed_data = preprocessEncode(data);
  return taquito_schema.Encode(preprocessed_data);
};

const preprocessEncode = (data) =>
  transform([encode_enum, encode_unit, encode_maps, encode_top_map], data);

const postprocess = (data) =>
  transform(
    [decode_top_map, decode_maps, decode_bignums, decode_unit, decode_enum],
    data
  );

// Apply several transformations **in order** to the data
const transform = (funs, init) => funs.reduce((x, f) => f(x), init);

// Handle `BigNumber`s
const decode_bignums = (data) =>
  deep_transform(data, BigNumber.isBigNumber, (bn) => bn.toString(10));

// Handle the case when the data contains a single map
const encode_top_map = (data) =>
  is_decoded_map(data) ? encode_map(data) : data;

const decode_top_map = (data) =>
  MichelsonMap.isMichelsonMap(data) ? decode_map(data) : data;

/* Convert `map`s and `big_map`s coming from Rust to Taquito's own `MichelsonMap` class

  Note that only the top-level fields of `data` are processed, there's no recursive descent
  for converting embedded maps. This is consistent with Taquito's behaviour expecting a flat object.
*/

const is_decoded_map = (data) =>
  typeof data === "object" &&
  data["MichelsonMap"] !== undefined &&
  isSingleton(data);

const encode_map = (data) => MichelsonMap.fromLiteral(data["MichelsonMap"]);

const encode_maps = (data) =>
  shallow_transform(data, is_decoded_map, encode_map);

// TODO Is `toString` OK? Or, do we need to cater for non-string keys?
const decode_map = (data) => {
  let new_data = {};
  data.forEach((v, k) => (new_data[k.toString()] = v));
  return { MichelsonMap: new_data };
};

const decode_maps = (data) =>
  shallow_transform(data, MichelsonMap.isMichelsonMap, decode_map);

// Handle `unit` values

const JsonUnit = { __unit__: null };
exports.JsonUnit = JsonUnit;

const encode_unit = (data) =>
  deep_transform(
    data,
    (x) => util.isDeepStrictEqual(x, JsonUnit),
    (_) => UnitValue
  );

const decode_unit = (data) =>
  deep_transform(
    data,
    (x) => x === UnitValue,
    (_) => JsonUnit
  );

// Handle simple enums

const is_enum = (obj) =>
  obj !== null &&
  typeof obj === "object" &&
  "__enum__" in obj &&
  util.isString(obj.__enum__) &&
  isSingleton(obj);

const is_encoded_enum = (obj) =>
  typeof obj === "object" &&
  isSingleton(obj) &&
  util.isDeepStrictEqual(obj[Object.keys(obj)[0]], JsonUnit);

const encode_enum = (data) =>
  deep_transform(data, is_enum, (data) => {
    const mich_name = firstToLowerCase(data.__enum__);
    data = {};
    data[mich_name] = UnitValue;
    return data;
  });

const decode_enum = (data) =>
  deep_transform(data, is_encoded_enum, (data) => {
    let variant = Object.keys(data)[0];
    variant = firstToUpperCase(variant);
    return { __enum__: variant };
  });

// Helper functions

const isSingleton = (obj) => obj !== null && Object.keys(obj).length === 1;

const shallow_transform = (data, matcher, transform) => {
  for (const field in data) {
    if (!data.hasOwnProperty(field)) {
      continue;
    }
    if (matcher(data[field])) {
      data[field] = transform(data[field]);
    }
  }
  return data;
};

const deep_transform = (data, matcher, transform) => {
  if (matcher(data)) {
    data = transform(data);
  } else if (Array.isArray(data)) {
    data.forEach((value, i, array) => {
      array[i] = deep_transform(value, matcher, transform);
    });
  } else if (typeof data === "object") {
    for (const field in data) {
      if (!data.hasOwnProperty(field)) {
        continue;
      }
      data[field] = deep_transform(data[field], matcher, transform);
    }
  }
  return data;
};

const firstToLowerCase = (s) => {
  const fst = s.slice(0, 1);
  const rest = s.slice(1);
  return fst.toLowerCase().concat(rest);
};

const firstToUpperCase = (s) => {
  const fst = s.slice(0, 1);
  const rest = s.slice(1);
  return fst.toUpperCase().concat(rest);
};

// Clone a serialisable object/value
const clone = (x) => JSON.parse(JSON.stringify(x));
