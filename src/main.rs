use std::sync::mpsc;
use std::{thread, env};

fn main() {
    let num: i64 = get_argument().expect("Error");
    
    let result = start_threads(num, 10);
    println!("{:?}", result);
}

fn get_argument() -> Result<i64, &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Not enough arguments. Please enter a number.")
    }
    if args.len() > 2 {
        return Err("Too many arguments. Please enter a number.")
    }
    match &args[1].parse::<i64>() {
        Ok(num) => Ok(*num),
        Err(_) => Err("Failure parsing value. Please enter a number.")
    }
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

fn split_work(num: i64, threads: i64) -> (Vec<i64>, i64) {
    let flt_num: f64 = num as f64;
    
    let flt_threads = if threads > num {flt_num} else {threads as f64};
    
    let mut result: Vec<i64> = vec![];
    result.append(&mut (0..=(flt_threads.abs() as i64))
        .into_iter()
        .map(
            |i| (i as f64 * ((flt_num.abs().sqrt())/flt_threads)) as i64
        )
        .collect::<Vec<i64>>());
        (result, flt_threads as i64)
}

fn start_threads(num: i64, threads: i64) -> Vec<(i64, i64)>{    
    let (parts, new_threads) = split_work(num, threads);
    //Uncomment below to see the intervals that are given to the seperate threads.
    //dbg!(&parts);
    
    let (tx, rx) = mpsc::channel();
    
    for i in 0..=new_threads-2 {
        start_single_thread(num, parts[i as usize]+1, parts[(i+1) as usize], tx.clone());
    }
    start_single_thread(num, parts[(new_threads-1) as usize]+1, parts[new_threads as usize], tx);
        
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
