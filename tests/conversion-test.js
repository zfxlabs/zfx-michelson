'use strict';

const { jsonEncode, jsonDecode, JsonUnit } = require("../src/json_converter");
const assert = require('assert');

function test_unit() {
  const enumSchema = {
    "prim": "or",
    "args": [
      {
        "prim": "unit",
        "annots": [ "%a" ]
      },
      {
        "prim": "unit",
        "annots": [ "%b" ]
      }
    ]
  };

  const data = { a: JsonUnit };
  const expected = { prim: 'Left', args: [ { prim: 'Unit' } ] }

  // Test it!
  const encoded = jsonEncode(enumSchema, data);
  console.log("encoded", encoded);
  assert.deepStrictEqual(encoded, expected);

  const back = jsonDecode(enumSchema, encoded);
  console.log("back", back);
  assert.deepStrictEqual(back, data);
}

test_unit();
