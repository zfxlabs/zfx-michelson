{
  state = Open;
  bettors = (Set.literal 
    [("tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c": key_hash)] 
    : bettor set);
  counter = 1n;
  bets = (Big_map.literal 
    [(("tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c": key_hash), 42n)]
    : (bettor, nat) big_map);
}
