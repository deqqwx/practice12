
use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, usize, i32) {
    let (index, min_sum) = data
        .windows(2)
        .enumerate()
        .map(|(i, w)| (i, w[0] + w[1]))
        .min_by_key(|&(_, sum)| sum)
        .unwrap();
    (index, index + 1, min_sum)
}

fn print_vector_with_min_sum(data: &[i32]) {
    // Вивід індексів
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    // Вивід даних
    print!("data:    ");
    for value in data {
        print!("{:>3} ", value);
    }
    println!();

    // Знаходимо мінімальну пару
    let (idx1, idx2, min_sum) = min_adjacent_sum(data);

    // Позначення мінімальної пари
    print!("indexes: ");
    for i in 0..data.len() {
        if i == idx1 {
            print!("\\__");
        } else if i == idx2 {
            print!("__/ ");
        } else {
            print!("    ");
        }
    }
    println!();

    // Вивід мінімальної суми
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[idx1], data[idx2], min_sum, idx1, idx2
    );
}
#[test]
fn main() {
    let data = gen_random_vector(20);
    print_vector_with_min_sum(&data);
}
