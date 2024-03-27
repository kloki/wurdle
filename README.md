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

| Name        | Avg    | Max | Min | Winrate |
| ----------- | ------ | --- | --- | ------- |
| Random      | 5.1946 | 15  | 2   | 0.8246  |
| Vowel prune | 5.1834 | 18  | 2   | 0.8204  |
| Split       | 5.0324 | 15  | 1   | 0.855   |
| Entropy     | 4.5952 | 13  | 1   | 0.9078  |

## You will lose

cargo run --bin wurdle-horror --release

## References

- [data_set](https://github.com/steve-kasica/wordle-words)
- [entropy approach](https://www.youtube.com/watch?v=v68zYyaEmEA)
