"use strict";

const { jsonEncode, jsonDecode, JsonUnit } = require("../src/json_converter");
const assert = require("assert");

const PRINT = process.env["PRINT"] ? true : false;
const print = PRINT ? console.log : () => undefined;

function test_unit() {
  const enumSchema = {
    prim: "unit",
    annots: ["%a"],
  };

  const data = JsonUnit;
  const expected = { prim: "Unit" };

  const encoded = jsonEncode(enumSchema, data);
  print("encoded", encoded);
  assert.deepStrictEqual(encoded, expected);

  const back = jsonDecode(enumSchema, encoded);
  print("back", back);
  assert.deepStrictEqual(back, data);
}

function test_map() {
  const sch = { prim: "map", args: [{ prim: "string" }, { prim: "int" }] };
  const data = { MichelsonMap: { field: "1" } };
  const expected = [{ prim: "Elt", args: [{ string: "field" }, { int: "1" }] }];

  const encoded = jsonEncode(sch, data);
  print("encoded", JSON.stringify(encoded));
  assert.deepStrictEqual(encoded, expected);

  const back = jsonDecode(sch, encoded);
  print("back", back);
  assert.deepStrictEqual(back, data);
}

function test_enum() {
  const sch = {
    prim: "or",
    args: [
      { prim: "unit", annots: ["%aaA"] },
      { prim: "unit", annots: ["%ccC"] },
    ],
  };
  const data = { __enum__: "AaA" };
  const expected = { prim: "Left", args: [{ prim: "Unit" }] };

  const encoded = jsonEncode(sch, data);
  print("encoded", JSON.stringify(encoded));
  assert.deepStrictEqual(encoded, expected);

  const back = jsonDecode(sch, encoded);
  print("back", back);
  assert.deepStrictEqual(back, data);
}

function test_register_storage() {
  const sch = {
    prim: "pair",
    args: [
      {
        prim: "pair",
        args: [
          {
            prim: "pair",
            args: [
              {
                prim: "big_map",
                args: [
                  { prim: "key_hash" },
                  {
                    prim: "list",
                    args: [
                      {
                        prim: "pair",
                        args: [
                          {
                            prim: "pair",
                            args: [
                              {
                                prim: "pair",
                                args: [
                                  {
                                    prim: "key_hash",
                                    annots: ["%baking_account"],
                                  },
                                  { prim: "key", annots: ["%public_key"] },
                                ],
                              },
                              { prim: "bytes", annots: ["%tls_cert"] },
                            ],
                          },
                          { prim: "timestamp" },
                        ],
                      },
                    ],
                  },
                ],
                annots: ["%old_validator_map"],
              },
              {
                prim: "set",
                args: [{ prim: "key_hash" }],
                annots: ["%old_validators"],
              },
            ],
          },
          { prim: "address", annots: ["%owner"] },
          {
            prim: "or",
            args: [
              {
                prim: "or",
                args: [
                  { prim: "unit", annots: ["%genesis"] },
                  { prim: "unit", annots: ["%open"] },
                ],
              },
              { prim: "unit", annots: ["%sealed"] },
            ],
            annots: ["%state"],
          },
        ],
      },
      {
        prim: "big_map",
        args: [
          { prim: "key_hash" },
          {
            prim: "pair",
            args: [
              {
                prim: "pair",
                args: [
                  { prim: "key_hash", annots: ["%baking_account"] },
                  { prim: "key", annots: ["%public_key"] },
                ],
              },
              { prim: "bytes", annots: ["%tls_cert"] },
            ],
          },
        ],
        annots: ["%validator_map"],
      },
      { prim: "set", args: [{ prim: "key_hash" }], annots: ["%validators"] },
    ],
  };
  /*
  {
    state = Genesis;
    owner = ("tz1burnburnburnburnburnburnburjAYjjX" : address);
    validators = (Set.literal [("tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c": key_hash)] : validator set);
    validator_map = (Big_map.literal [
      (("tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c": key_hash),
       {baking_account = ("tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c" : key_hash);
       public_key = ("edpku2tvek7QFRYm12819P8RwSY8m7zSzKV9RMnWHy3xVbrBwN5zAg" : key) ;
       tls_cert = 0x}) ]
      : (validator, register) big_map);
    old_validators = (Set.empty : validator set);
    old_validator_map = (Big_map.empty : history_map);
  }
  */
  const data = {
    state: { __enum__: "Genesis" },
    owner: "tz1burnburnburnburnburnburnburjAYjjX",
    validators: ["tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c"],
    old_validators: [],
    old_validator_map: { MichelsonMap: {} },
    validator_map: {
      MichelsonMap: {
        tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c: {
          baking_account: "tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c",
          public_key: "edpku2tvek7QFRYm12819P8RwSY8m7zSzKV9RMnWHy3xVbrBwN5zAg",
          tls_cert: "",
        },
      },
    },
  };
  const expected = {
    prim: "Pair",
    args: [
      {
        prim: "Pair",
        args: [
          {
            prim: "Pair",
            args: [[], []],
          },
          {
            prim: "Pair",
            args: [
              { string: "tz1burnburnburnburnburnburnburjAYjjX" },
              {
                prim: "Left",
                args: [{ prim: "Left", args: [{ prim: "Unit" }] }],
              },
            ],
          },
        ],
      },
      {
        prim: "Pair",
        args: [
          [
            {
              prim: "Elt",
              args: [
                { string: "tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c" },
                {
                  prim: "Pair",
                  args: [
                    {
                      prim: "Pair",
                      args: [
                        { string: "tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c" },
                        {
                          string:
                            "edpku2tvek7QFRYm12819P8RwSY8m7zSzKV9RMnWHy3xVbrBwN5zAg",
                        },
                      ],
                    },
                    { bytes: "" },
                  ],
                },
              ],
            },
          ],
          [{ string: "tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c" }],
        ],
      },
    ],
  };
  const encoded = jsonEncode(sch, data);
  print("encoded", JSON.stringify(encoded));
  assert.deepStrictEqual(encoded, expected);

  const back = jsonDecode(sch, encoded);
  print("back", back);
  print(
    "back value",
    JSON.stringify(
      back.validator_map.MichelsonMap.tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c
    )
  );
  assert.deepStrictEqual(back, data);
}

function fail() {
  assert.deepStrictEqual(0, 1);
}

// Run the tests
test_unit();
test_map();
test_enum();
test_register_storage();
