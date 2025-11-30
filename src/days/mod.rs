#[cfg(not(feature = "bench"))]
macro_rules! run_days {
    ($($day:ident),* $(,)?) => {
        $(
            pub mod $day;
        )*

        pub fn run_all() {
            $(

                println!("--- Running {} ---", stringify!($day));
                println!("Part 1:");
                let t = std::time::Instant::now();
                for _ in  0..300 {
                    let _ = $day::part1();
                }
                let avg = t.elapsed() / 300;
                println!("\tResult: {:?}", $day::part1());
                println!("\tTime elapsed: {:.3?}", avg);

                println!("Part 2:");
                let t = std::time::Instant::now();
                for _ in  0..300 {
                    let _ = $day::part2();
                }
                let avg = t.elapsed() / 300;
                println!("\tResult: {:?}", $day::part2());
                println!("\tTime elapsed: {:.3?}", avg);
            )*
        }
    };
}

#[cfg(not(feature = "bench"))]
run_days!(day01);

#[cfg(feature = "bench")]
macro_rules! run_days_benchmark {
    ($($day:ident),* $(,)?) => {
        $(
            pub mod $day;
        )*

        pub fn run_all() {
            use criterion::{criterion_group, criterion_main, Criterion};
            use std::hint::black_box;
            let mut c = Criterion::default();

            $(

                let mut group = c.benchmark_group(stringify!($day));

                println!("--- Running {} ---", stringify!($day));
                println!("Part 1:");
                println!("\tResult: {:?}", $day::part1());
                group.bench_function("Avg Part 1: ", |x| {
                    x.iter(|| {
                        let _ = $day::part1();
                    })
                });
                // println!("\tTime elapsed: {:.3?}", time);

                println!("Part 2:");
                println!("\tResult: {:?}", $day::part2());
                group.bench_function("Avg Part 2: ", |x| {
                    x.iter(|| {
                        let _ = $day::part2();
                    })
                });
                // println!("\tTime elapsed: {:.3?}", time);

                group.finish();
            )*
        }
    };
}

#[cfg(feature = "bench")]
run_days_benchmark!(day01);
