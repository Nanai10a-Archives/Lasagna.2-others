fn main() {
    let mut collection = Vec::<uuid::Uuid>::new();

    let mut collition_time = 0u128;
    let mut all_time = 0u128;

    let mut elapsed_times = Vec::<_>::new();
    let start_time = std::time::Instant::now();

    loop {
        let current = uuid::Uuid::new_v4();
        all_time += 1;

        let time = std::time::Instant::now();

        let mut is_matched = false;
        for other in collection.iter() {
            if *other == current {
                is_matched = true;
                break;
            }
        }

        let elapsed = time.elapsed().as_secs_f64();
        elapsed_times.push(elapsed);
        let avr = {
            let mut all = 0f64;
            elapsed_times.iter().for_each(|v| all += v);
            all / elapsed_times.len() as f64
        };

        let res = if is_matched {
            collition_time += 1;
            "collition"
        } else {
            collection.push(current);
            "ok"
        };

        println!(
            "current: {} | {:<12}ms / avr. {:<25}ms | all: {:<10} / collition: {:<3} | elapsed: {:<6}sec | {}.",
            current, elapsed, avr, all_time, collition_time, start_time.elapsed().as_secs(), res
        );
    }
}
