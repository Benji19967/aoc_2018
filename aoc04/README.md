# Performance

I created a file of 1.3GB (40'348'800 lines) with similar content as the input.txt file.

Running it using one thread, the program performs as follows.

```bash
./target/release/aoc04 < input/input.txt  31.85s user 0.40s system 99% cpu 32.369 total
```

## Parallelizing the work using Rayon

With just a few simple changes we can parse in parallel using Rayon.

```Rust
use std::io::{BufRead, BufReader, Read};
use rayon::prelude::*;

let input = BufReader::new(io::stdin());

input
    .lines()
    .map(|l| l.unwrap().to_owned())
    .par_bridge()
    .for_each(move |line| {
        let event = line.parse::<Event>();
    });
```

and the results:
```bash
# This used 400% CPU on my laptop that has 8 logical cores (`sysctl -n hw.ncpu`)
./target/release/aoc04 < input/input.txt  48.91s user 24.20s system 396% cpu 18.460 total
```

```bash
export RAYON_NUM_THREADS=1 
./target/release/aoc04 < input/input.txt  35.99s user 0.27s system 99% cpu 36.260 total
```

## Comparing a similar Python implementation
```bash
python python/main.py  147.45s user 0.86s system 98% cpu 2:30.63 total
```

... roughly 5 times slower when compared to the single-threaded Rust version.
