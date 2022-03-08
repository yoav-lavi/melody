#![macro_use]

macro_rules! pattern_either {
    ($first: expr, $second: expr) => {
        $first | $second
    };
}

macro_rules! format_command {
    ($short: expr, $long: expr) => {
        pattern_either!(concat!(":", $short), concat!(":", $long))
    };
}
