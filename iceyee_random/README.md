
# iceyee_random

## Supported Os

- [x] linux
- [ ] macos
- [x] windows

## Example

```rust
#[test]
pub fn test_random() {
    use iceyee_random::Random;
    println!("");
    let mut counter: [usize; 10] = [0; 10];
    for _ in 0..10000 {
        let number: usize = Random::next() % 10;
        counter[number] += 1;
    }
    for x in 0..10 {
        print!("{}:{},   ", x, counter[x]);
        assert!(900 < counter[x] && counter[x] < 1100);
    }
    println!("");
    // 测试固定种子.
    Random::set_seed(0xFFFF);
    for x in 0..counter.len() {
        counter[x] = Random::next();
    }
    Random::set_seed(0xFFFF);
    for x in 0..counter.len() {
        assert!(Random::next() == counter[x]);
    }
    return;
}
```

```
test test_random ...
0:1041,   1:968,   2:993,   3:965,   4:984,   5:1011,   6:984,   7:1000,   8:1042,   9:1012,
ok
```
