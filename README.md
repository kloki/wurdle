# wurdle

## Run one simulation

```
cargo run --release
```

## Benchmark different strategies

```
cargo run --bin wurdle-benchmark --release
```

### Result

| Name        | Winrate |
| ----------- | ------- |
| Random      | 0.8     |
| Vowel prune | 0.77    |
| Split       | 0.84    |
| Entropy     | 0.99    |

## Entropy Flex

cargo run --bin wurdle-horror --release

## References

- [data_set](https://github.com/steve-kasica/wordle-words)
- [entropy approach](https://www.youtube.com/watch?v=v68zYyaEmEA)
