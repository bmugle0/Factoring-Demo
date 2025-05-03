use std::sync::mpsc;
use std::thread;

fn main() {
    let num: i64 = -9000000000000000000;
    
    let result = start_threads(num, 10);
    println!("{:?}", result);
}

fn find_factors(num: i64, range: std::ops::RangeInclusive<i64>) -> Vec<(i64, i64)> {
    let mut result: Vec<(i64, i64)> = Vec::new();
    
    for i in range {
        //println!("i={i}");
        if i > (num.abs()/i) {
            return result;
        }
        if num % i == 0 {
            result.push((i, num / i));
        }
    }
    
    result
}

fn start_threads(num: i64, threads: i64) -> Vec<(i64, i64)>{    
    let mut parts: Vec<i64> = vec![];
    parts.append(&mut (0..=threads.abs())
        .into_iter()
        .map(|i| i * ((num.abs().isqrt()+3)/threads))
        .collect::<Vec<i64>>());
        println!("{:?}", parts);
        
    let (tx, rx) = mpsc::channel();
    
    for i in 0..=threads-2 {
        start_single_thread(num, parts[i as usize]+1, parts[(i+1) as usize], tx.clone());
    }
    start_single_thread(num, parts[(threads-1) as usize]+1, parts[threads as usize], tx);
        
    append_results(rx)
}

fn start_single_thread(num: i64, start: i64, end: i64, tx: mpsc::Sender<Vec<(i64, i64)>>) {  
    thread::spawn(move || {
    tx.send(find_factors(num, start..=end)).unwrap();
    });
}

fn append_results(rx: mpsc::Receiver<Vec<(i64, i64)>>) -> Vec<(i64, i64)> {
    let mut result: Vec<(i64, i64)> = Vec::new();
    for mut received in rx {
        result.append(&mut received);
    }

    result.sort_by_key(|(f1,_)| *f1);
    result
}
