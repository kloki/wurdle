# wurdle

Solve wordle using entropy approach.

Found "fried" in 4 guesses

          ⬜⬜🟨🟩⬜
          ⬜⬜🟩⬜🟩
          🟨⬜⬜🟨🟨
          🟩🟩🟩🟩🟩

## Solve a specific word

```
cargo run --release <input>
```

## Compare entropy with naive solutions

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

```
cargo run --bin wurdle-horror --release
```

## Cheater assistant

Solve the daily wordle.

```
cargo run --bin wurdle-cheater --release
```

## Opener

Given a data set, the opener is always the same. Therefore I hardcode the opener everywhere
This one recalculates it.

```
cargo run --bin wurdle-cheater --release
```

## Endless

```
cargo run --bin wurdle-loop --release
```

## References

- [data_set](https://github.com/steve-kasica/wordle-words)
- [entropy approach](https://www.youtube.com/watch?v=v68zYyaEmEA)
