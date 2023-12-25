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

#[test]
pub fn test_random() {
    use iceyee_random::Random;

    println!("");
    println!("测试随机数, 从[0,9]当中取值, 每次取值都是独立, 执行10_000次, 理论上预期结果是, 每个值平均命中1000次, 但是毕竟随机数存在波动, 所以预期每个值的命中次数在[900,1100]之间.");
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

    println!("");
    println!("测试固定种子.");
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
