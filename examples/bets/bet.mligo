type state = Open | Closed
type bettor = key_hash
type storage = {
  state: state;
  bettors: bettor set;
  counter: nat;
  bets: (bettor, nat) big_map;
}

type parameter = unit
type return = operation list * storage

let main (_arg, store : parameter * storage) : return =
  // Store bets, etc.
  ([] : operation list), store
