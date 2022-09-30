const { jsonEncode, jsonDecode, JsonUnit } = require("../../src/json_converter");
const assert = require("chai").assert;
const register_schema = require("./test_data").register_schema;

describe("Conversion tests", function() {
    this.timeout(10000);

    it("test_unit", function() {
        const enumSchema = {
            prim: "unit",
            annots: ["%a"],
          };

        const data = JsonUnit;
        const expected = { prim: "Unit" };

        const encoded = jsonEncode(enumSchema, data);
        assert.deepEqual(encoded, expected);

        const back = jsonDecode(enumSchema, encoded);
        assert.deepEqual(back, data);
    });

    it("test_map", function() {
        const sch = { prim: "map", args: [{ prim: "string" }, { prim: "int" }] };
        const data = { MichelsonMap: { field: "1" } };
        const expected = [{ prim: "Elt", args: [{ string: "field" }, { int: "1" }] }];

        const encoded = jsonEncode(sch, data);
        assert.deepEqual(encoded, expected);

        const back = jsonDecode(sch, encoded);
        assert.deepEqual(back, data);
    });

    it("test_enum", function() {
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
          assert.deepStrictEqual(encoded, expected);
        
          const back = jsonDecode(sch, encoded);
          assert.deepStrictEqual(back, data);
    });

    it("test_register_storage", function() {
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
          const encoded = jsonEncode(register_schema, data);
          assert.deepStrictEqual(encoded, expected);
        
          const back = jsonDecode(register_schema, encoded);
          assert.deepEqual(back, data);
    });

    it("test_register_schema_with_history", function() {
        const data = {
            state: { __enum__: "Genesis" },
            owner: "tz1burnburnburnburnburnburnburjAYjjX",
            validators: ["tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c"],
            old_validators: ["tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c"],
            old_validator_map: {
              MichelsonMap: {
                tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c: [
                  {
                    baking_account: "tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c",
                    public_key:
                      "edpku2tvek7QFRYm12819P8RwSY8m7zSzKV9RMnWHy3xVbrBwN5zAg",
                    tls_cert: "",
                    // Taquito returns timestamps with fractions of seconds: `.000`
                    3: "2022-09-23T00:00:00.000Z",
                  },
                ],
              },
            },
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
        
          // This is the output of Taquito, not the version generated by the Ligo compiler,
          // the latter is using the `Pair` constructor with more than two arguments
          const expected = {
            prim: "Pair",
            args: [
              {
                prim: "Pair",
                args: [
                  {
                    prim: "Pair",
                    args: [
                      [
                        {
                          prim: "Elt",
                          args: [
                            {
                              string: "tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c",
                            },
                            [
                              {
                                prim: "Pair",
                                args: [
                                  {
                                    prim: "Pair",
                                    args: [
                                      {
                                        prim: "Pair",
                                        args: [
                                          {
                                            string:
                                              "tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c",
                                          },
                                          {
                                            string:
                                              "edpku2tvek7QFRYm12819P8RwSY8m7zSzKV9RMnWHy3xVbrBwN5zAg",
                                          },
                                        ],
                                      },
                                      {
                                        bytes: "",
                                      },
                                    ],
                                  },
                                  {
                                    string: "2022-09-23T00:00:00.000Z",
                                  },
                                ],
                              },
                            ],
                          ],
                        },
                      ],
                      [
                        {
                          string: "tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c",
                        },
                      ],
                    ],
                  },
                  {
                    prim: "Pair",
                    args: [
                      {
                        string: "tz1burnburnburnburnburnburnburjAYjjX",
                      },
                      {
                        prim: "Left",
                        args: [
                          {
                            prim: "Left",
                            args: [
                              {
                                prim: "Unit",
                              },
                            ],
                          },
                        ],
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
                        {
                          string: "tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c",
                        },
                        {
                          prim: "Pair",
                          args: [
                            {
                              prim: "Pair",
                              args: [
                                {
                                  string: "tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c",
                                },
                                {
                                  string:
                                    "edpku2tvek7QFRYm12819P8RwSY8m7zSzKV9RMnWHy3xVbrBwN5zAg",
                                },
                              ],
                            },
                            {
                              bytes: "",
                            },
                          ],
                        },
                      ],
                    },
                  ],
                  [
                    {
                      string: "tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c",
                    },
                  ],
                ],
              },
            ],
          };
          const encoded = jsonEncode(register_schema, data);
          assert.deepEqual(encoded, expected);
        
          const back = jsonDecode(register_schema, encoded);
          assert.deepEqual(back, data);
    });

    it("test_enum_with_parameter", function() {
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
          assert.deepEqual(encoded, expected);
        
          const back = jsonDecode(sch, encoded);
          assert.deepEqual(back, data);
    });

    it("test_enum_with_parameter_in_record", function() {
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
          assert.deepEqual(encoded, expected);
        
          const back = jsonDecode(sch, encoded);
          assert.deepEqual(back, data);
        
          const data2 = { a: { __enum__: "Reset" }, i: "42" };
          // This below works for encoding, but will be decoded as above
          // const data2 = { a: {"reset": JsonUnit}, i: "42" };
          const expected2 = {
            prim: "Pair",
            args: [{ prim: "Right", args: [{ prim: "Unit" }] }, { int: "42" }],
          };
          const encoded2 = jsonEncode(sch, data2);
          assert.deepEqual(encoded2, expected2);
        
          const back2 = jsonDecode(sch, encoded2);
          assert.deepEqual(back2, data2);
    });

    it("test_record_in_record", function() {
        const sch = {
            prim: "pair",
            args: [
              { prim: "int", annots: ["%a"] },
              {
                prim: "pair",
                args: [
                  { prim: "int", annots: ["%c"] },
                  { prim: "int", annots: ["%d"] },
                ],
                annots: ["%b"],
              },
            ],
          };
          const data = {
            a: "0",
            b: { c: "2", d: "3" },
          };
          const expected = {
            prim: "Pair",
            args: [{ int: "0" }, { prim: "Pair", args: [{ int: "2" }, { int: "3" }] }],
          };
          const encoded = jsonEncode(sch, data);
          assert.deepEqual(encoded, expected);
        
          const back = jsonDecode(sch, encoded);
          assert.deepEqual(back, data);
    });

    it("test_adt", function() {
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
          assert.deepEqual(encoded, expected);
        
          const back = jsonDecode(sch, encoded);
          assert.deepEqual(back, data);
    });

    it("test_option", function() {
        const sch = { prim: "option", args: [{ prim: "int" }], annots: ["%a"] };
        const data_some = "1";
        const data_none = null;
        const expected_some = { prim: "Some", args: [{ int: "1" }] };
        const expected_none = { prim: "None" };
      
        const encoded = jsonEncode(sch, data_some);
        assert.deepEqual(encoded, expected_some);
      
        const back = jsonDecode(sch, encoded);
        assert.deepEqual(back, data_some);
      
        const encoded2 = jsonEncode(sch, data_none);
        assert.deepEqual(encoded2, expected_none);
      
        const back2 = jsonDecode(sch, encoded2);
        assert.deepEqual(back2, data_none);
    });

    it("test_option_in_record", function() {
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
          assert.deepEqual(encoded, expected_some);
        
          const back = jsonDecode(sch, encoded);
          assert.deepEqual(back, data_some);
        
          const encoded2 = jsonEncode(sch, data_none);
          assert.deepEqual(encoded2, expected_none);
        
          const back2 = jsonDecode(sch, encoded2);
          assert.deepEqual(back2, data_none);
    });
});