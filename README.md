# Maelstrom problems solved with Rust

See: [Fly.io / Maelstrom distributed system challenge](https://fly.io/dist-sys/1/)

## Challenges
- [x] [Echo](https://fly.io/dist-sys/1/)
- [x] [Unique ID generations](https://fly.io/dist-sys/2/)
- [ ] [Broadcast messages](https://fly.io/dist-sys/3a/)
- [ ] [Grow-only counter](https://fly.io/dist-sys/4/)
- [ ] [Kafka-style log](https://fly.io/dist-sys/5a/)
- [ ] [Single-node key-value store](https://fly.io/dist-sys/6a/)

## Test
First build the binary with cargo.
```bash
$ cargo build --release
```

After this, you can grab the respective command from fly.io challenge and run it. Example for generate id challenge:
```bash
$ MAELSTROM_PATH/maelstrom test -w unique-ids --bin ./target/release/unique-ids --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
```

## TODO
- [ ] include Maelstrom in Nix