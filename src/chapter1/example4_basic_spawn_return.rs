use std::thread;

pub fn example4_basic_spawn_return() {
    let numbers = Vec::from_iter(0..=1000);

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();

        sum / len
    });

    // t.join will give a result
    let average = t.join().unwrap();
    println!("The average is {}", average);
}
