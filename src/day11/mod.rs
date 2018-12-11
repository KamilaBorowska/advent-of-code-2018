use crate::Solution;

pub(super) const DAY11: Solution = Solution {
    part1: |serial| {
        let serial: i32 = serial.parse()?;
        let (x, y) = (1..=300 - 3 + 1)
            .flat_map(|x| (1..=300 - 3 + 1).map(move |y| (x, y)))
            .max_by_key(|&(x, y)| -> i32 {
                (x..x + 3)
                    .flat_map(|x| (y..y + 3).map(move |y| (x, y)))
                    .map(|(x, y)| {
                        let rack_id = x + 10;
                        (rack_id * y + serial) * rack_id / 100 % 10 - 5
                    })
                    .sum()
            })
            .unwrap();
        Ok(format!("{},{}", x, y))
    },
    part2: |serial| {
        let serial: i32 = serial.parse()?;
        let (x, y, size) = (1..=300)
            .flat_map(|size| (1..=300 - size + 1).map(move |x| (x, size)))
            .flat_map(|(x, size)| (1..=300 - size + 1).map(move |y| (x, y, size)))
            .max_by_key(|&(x, y, size)| -> i32 {
                (x..x + size)
                    .flat_map(|x| (y..y + size).map(move |y| (x, y)))
                    .map(|(x, y)| {
                        let rack_id = x + 10;
                        (rack_id * y + serial) * rack_id / 100 % 10 - 5
                    })
                    .sum()
            })
            .unwrap();
        Ok(format!("{},{},{}", x, y, size))
    },
};

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
