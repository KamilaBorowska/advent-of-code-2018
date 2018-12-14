use crate::Solution;
use rayon::prelude::*;

pub(super) const DAY11: Solution = Solution {
    part1: |serial| {
        let serial: i32 = serial.parse()?;
        let (x, y) = (1..=300 - 3 + 1)
            .flat_map(|x| (1..=300 - 3 + 1).map(move |y| (x, y)))
            .max_by_key(|&(x, y)| -> i32 {
                (x..x + 3)
                    .flat_map(|x| (y..y + 3).map(move |y| (x, y)))
                    .map(|(x, y)| get_power(serial, x, y))
                    .sum()
            })
            .unwrap();
        Ok(format!("{},{}", x, y))
    },
    part2: |serial| {
        let serial: i32 = serial.parse()?;
        let (x, y, size, _) = (0..300 * 300)
            .into_par_iter()
            .map(|pos| {
                let x = pos % 300 + 1;
                let y = pos / 300 + 1;
                let mut max_size = 1;
                let mut max_sum = get_power(serial, x, y);
                let mut sum = max_sum;
                for size in 1..300 - x.max(y) {
                    sum += get_power(serial, x + size, y + size);
                    for x_mod in 0..size {
                        sum += get_power(serial, x + x_mod, y + size);
                    }
                    for y_mod in 0..size {
                        sum += get_power(serial, x + size, y + y_mod);
                    }
                    if sum > max_sum {
                        max_size = size + 1;
                        max_sum = sum;
                    }
                }
                (x, y, max_size, max_sum)
            })
            .max_by_key(|&(_, _, _, value)| value)
            .unwrap();
        Ok(format!("{},{},{}", x, y, size))
    },
};

fn get_power(serial: i32, x: i32, y: i32) -> i32 {
    let rack_id = x + 10;
    (rack_id * y + serial) * rack_id / 100 % 10 - 5
}

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY11.part1,
        example1: "18" => "33,45",
        example2: "42" => "21,61",
        input: "9798" => "44,37",
    );
    test!(
        DAY11.part2,
        example1: "18" => "90,269,16",
        example2: "42" => "232,251,12",
        input: "9798" => "235,87,13",
    );
}
