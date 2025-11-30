macro_rules! run_days {
    ($($day:ident),* $(,)?) => {
        $(
            pub mod $day;
        )*

        pub fn run_all() {
            $(

                println!("--- Running {} ---", stringify!($day));
                println!("Part 1:");
                //dry run for cache warmup
                let _ = $day::part1();
                let t = std::time::Instant::now();
                let result = $day::part1();
                let time = t.elapsed();
                println!("\tResult: {:?}", result);
                println!("\tTime elapsed: {:.3?}", time);

                println!("Part 2:");
                //dry run for cache warmup
                let _ = $day::part2();
                let t = std::time::Instant::now();
                let result = $day::part2();
                let time = t.elapsed();
                println!("\tResult: {:?}", result);
                println!("\tTime elapsed: {:.3?}", time);
            )*
        }
    };
}

run_days!(day01);
