// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

// Enum.

// Trait.

// Struct.

// Function.

// TODO Function

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
