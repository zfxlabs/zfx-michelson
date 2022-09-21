"use strict";

const { jsonEncode, jsonDecode, JsonUnit } = require("../src/json_converter");
const assert = require("assert");

// Set `PRINT=1` in the environment to see the output
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
  // print("complex schema:", JSON.stringify(sch, null, 2));
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

function test_enum_with_parameter() {
  const sch = {
    prim: "or",
    args: [
      {
        prim: "or",
        args: [
          { prim: "int", annots: ["%decrement"] },
          { prim: "int", annots: ["%increment"] },
        ],
      },
      { prim: "unit", annots: ["%reset"] },
    ],
  };
  const data = { decrement: "1" };
  const expected = {
    prim: "Left",
    args: [{ prim: "Left", args: [{ int: "1" }] }],
  };
  const encoded = jsonEncode(sch, data);
  print("encoded", JSON.stringify(encoded));
  assert.deepStrictEqual(encoded, expected);

  const back = jsonDecode(sch, encoded);
  print("back", back);
  assert.deepStrictEqual(back, data);
}
function test_enum_with_parameter_in_record() {
  const sch = {
    prim: "pair",
    args: [
      {
        prim: "or",
        args: [
          {
            prim: "or",
            args: [
              { prim: "int", annots: ["%decrement"] },
              { prim: "int", annots: ["%increment"] },
            ],
          },
          { prim: "unit", annots: ["%reset"] },
        ],
        annots: ["%a"],
      },
      { prim: "int", annots: ["%i"] },
    ],
  };
  const data = { a: { decrement: "1" }, i: "42" };
  const expected = {
    prim: "Pair",
    args: [
      { prim: "Left", args: [{ prim: "Left", args: [{ int: "1" }] }] },
      { int: "42" },
    ],
  };
  const encoded = jsonEncode(sch, data);
  print("encoded", JSON.stringify(encoded));
  assert.deepStrictEqual(encoded, expected);

  const back = jsonDecode(sch, encoded);
  print("back", back);
  assert.deepStrictEqual(back, data);

  const data2 = { a: { __enum__: "Reset" }, i: "42" };
  // This below works for encoding, but will be decoded as above
  // const data2 = { a: {"reset": JsonUnit}, i: "42" };
  const expected2 = {
    prim: "Pair",
    args: [{ prim: "Right", args: [{ prim: "Unit" }] }, { int: "42" }],
  };
  const encoded2 = jsonEncode(sch, data2);
  print("encoded", JSON.stringify(encoded2));
  assert.deepStrictEqual(encoded2, expected2);

  const back2 = jsonDecode(sch, encoded2);
  print("back", back2);
  assert.deepStrictEqual(back2, data2);
}

function test_adt() {
  const sch = {
    prim: "pair",
    args: [
      {
        prim: "or",
        args: [
          {
            prim: "or",
            args: [
              {
                prim: "pair",
                args: [{ prim: "int" }, { prim: "string" }],
                annots: ["%decrement"],
              },
              {
                prim: "pair",
                args: [{ prim: "int" }, { prim: "int" }],
                annots: ["%increment"],
              },
            ],
          },
          { prim: "unit", annots: ["%reset"] },
        ],
        annots: ["%a"],
      },
      { prim: "int", annots: ["%i"] },
    ],
  };
  const data = { a: { decrement: { 0: "1", 1: "foo" } }, i: "42" };
  const expected = {
    prim: "Pair",
    args: [
      {
        prim: "Left",
        args: [
          {
            prim: "Left",
            args: [{ prim: "Pair", args: [{ int: "1" }, { string: "foo" }] }],
          },
        ],
      },
      { int: "42" },
    ],
  };
  const encoded = jsonEncode(sch, data);
  print("encoded", JSON.stringify(encoded));
  assert.deepStrictEqual(encoded, expected);

  const back = jsonDecode(sch, encoded);
  print("back", back);
  assert.deepStrictEqual(back, data);
}

function test_option() {
  const sch = { prim: "option", args: [{ prim: "int" }], annots: ["%a"] };
  const data_some = "1";
  const data_none = null;
  const expected_some = { prim: "Some", args: [{ int: "1" }] };
  const expected_none = { prim: "None" };

  const encoded = jsonEncode(sch, data_some);
  print("encoded", JSON.stringify(encoded));
  assert.deepStrictEqual(encoded, expected_some);

  const back = jsonDecode(sch, encoded);
  print("back", back);
  assert.deepStrictEqual(back, data_some);

  const encoded2 = jsonEncode(sch, data_none);
  print("encoded", JSON.stringify(encoded2));
  assert.deepStrictEqual(encoded2, expected_none);

  const back2 = jsonDecode(sch, encoded2);
  print("back", back2);
  assert.deepStrictEqual(back2, data_none);
}

function test_option_in_record() {
  const sch = {
    prim: "pair",
    args: [
      { prim: "option", args: [{ prim: "int" }], annots: ["%a"] },
      { prim: "int", annots: ["%b"] },
    ],
  };
  const data_none = { a: null, b: "1" };
  const data_some = { a: "1", b: "1" };

  const expected_some = {
    prim: "Pair",
    args: [{ prim: "Some", args: [{ int: "1" }] }, { int: "1" }],
  };
  const expected_none = {
    prim: "Pair",
    args: [{ prim: "None" }, { int: "1" }],
  };

  const encoded = jsonEncode(sch, data_some);
  print("encoded", JSON.stringify(encoded));
  assert.deepStrictEqual(encoded, expected_some);

  const back = jsonDecode(sch, encoded);
  print("back", back);
  assert.deepStrictEqual(back, data_some);

  const encoded2 = jsonEncode(sch, data_none);
  print("encoded", JSON.stringify(encoded2));
  assert.deepStrictEqual(encoded2, expected_none);

  const back2 = jsonDecode(sch, encoded2);
  print("back", back2);
  assert.deepStrictEqual(back2, data_none);
}

function fail() {
  assert.deepStrictEqual(0, 1);
}

// Run the tests
test_unit();
test_map();
test_enum();
test_register_storage();
test_enum_with_parameter();
test_enum_with_parameter_in_record();
test_adt();
test_option();
test_option_in_record();
