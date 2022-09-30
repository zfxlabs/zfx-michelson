const register_schema = {
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

module.exports = { register_schema };