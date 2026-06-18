# tshiu-tsian-default-dict

A character -> bopomofo (注音) lookup table for Mandarin, distributed as JSON and
exposed as a Rust string. 4808 entries, each mapping one character to its
reading(s).

## Usage

```rust
let json = tshiu_tsian_default_dict::DEFAULT_DICT_STR;
// [{"character": "一", "bopomofo": ["ㄧ"]}, {"character": "丁", "bopomofo": ["ㄓㄥ", "ㄉㄧㄥ"]}, …]
```

## Data source & attribution

The pronunciation data was extracted from the g0v
[moedict-data](https://github.com/g0v/moedict-data) project, which packages the
中華民國教育部 (Ministry of Education) Mandarin dictionaries.

- The MOE dictionary content is licensed **Creative Commons
  Attribution-NoDerivatives 3.0 Taiwan (CC BY-ND 3.0 TW)** —
  <https://creativecommons.org/licenses/by-nd/3.0/tw/>
- The format conversion by [@kcwu](https://github.com/kcwu) is dedicated to the
  public domain under CC0.

[`LICENSE`](LICENSE) for the full notice.
