use std::sync::mpsc;
use std::thread;

fn main() {
    let num: i64 = -500;
    let (tx, rx) = mpsc::channel();
    
    let tx1 = tx.clone();
    
    thread::spawn(move || {
        tx1.send(find_factors(num, 1..=250)).unwrap();
    });
    
    thread::spawn(move || {
        tx.send(find_factors(num, 251..=500)).unwrap();
    });
    
    let result: Vec<(i64, i64)> = append_results(rx);
    
    println!("{:?}", result);
}

fn find_factors(num: i64, range: std::ops::RangeInclusive<i64>) -> Vec<(i64, i64)> {
    let mut result: Vec<(i64, i64)> = Vec::new();
    
    for i in range {
        if i > (num.abs()/i) {
            return result;
        }
        if num % i == 0 {
            result.push((i, num / i));
        }
    }
    
    result
}

fn append_results(rx: mpsc::Receiver<Vec<(i64, i64)>>) -> Vec<(i64, i64)> {
    let mut result: Vec<(i64, i64)> = Vec::new();
    for mut received in rx {
        result.append(&mut received);
    }
    
    result
}
