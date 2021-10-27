
# Othello bot

This project aims to implement a high-performing opening book generator for othello.

## How to use
```sh
# compile
cargo +nightly build --release

# run
./targets/release/othello_bot
```

## Develop build
```sh
cargo +nightly build

# run
./targets/debug/othello_bot
```

## Run tests
```sh
cargo +nightly test
```

---

#### TODO
- [x] move board to separate file
- [x] implement bot with alpha-beta pruning
- [x] print stats on calculation speed
- [x] simd
    - [x] do move
    - [x] get valid moves
    - [x] potential move difference
    - [x] corner difference
- [ ] implement pvs
- [ ] implement board normalization [edax](https://github.com/abulmo/edax-reversi/blob/master/src/board.c#L319)
- [ ] use transposition table
- [ ] xot openings
