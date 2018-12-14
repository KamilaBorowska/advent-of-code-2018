use crate::Solution;
use std::error::Error;

pub(crate) const DAY14: Solution = Solution {
    part1: |input| {
        let input: usize = input.parse()?;
        let mut recipes = vec![3, 7];
        let mut elves = [0, 1];
        while recipes.len() < input + 10 {
            let mut new_recipe: u8 = elves.iter().map(|&i| recipes[i]).sum();
            if new_recipe == 0 {
                recipes.push(0);
            } else {
                let start_from = recipes.len();
                while new_recipe != 0 {
                    recipes.push(new_recipe % 10);
                    new_recipe /= 10;
                }
                recipes[start_from..].reverse();
            }
            for elf in &mut elves {
                *elf = (*elf + usize::from(recipes[*elf]) + 1) % recipes.len();
            }
        }
        Ok(recipes[input..][..10].iter().map(u8::to_string).collect())
    },
    part2: |input| {
        let input = input
            .chars()
            .map(|c| Ok(c.to_digit(10).ok_or("Input has non-digit character")? as u8))
            .collect::<Result<Vec<u8>, Box<dyn Error>>>()?;
        let mut recipes = vec![3, 7];
        let mut elves = [0, 1];
        loop {
            let mut new_recipe: u8 = elves.iter().map(|&i| recipes[i]).sum();
            let start_from = recipes.len();
            if new_recipe == 0 {
                recipes.push(0);
            } else {
                while new_recipe != 0 {
                    recipes.push(new_recipe % 10);
                    new_recipe /= 10;
                }
                recipes[start_from..].reverse();
            }
            for pos in start_from..recipes.len() {
                if pos > input.len() && input == &recipes[pos - input.len()..pos] {
                    return Ok((pos - input.len()).to_string());
                }
            }
            for elf in &mut elves {
                *elf = (*elf + usize::from(recipes[*elf]) + 1) % recipes.len();
            }
        }
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY14.part1,
        example1: "9" => "5158916779",
        example2: "5" => "0124515891",
        example3: "18" => "9251071085",
        example4: "2018" => "5941429882",
        input: "939601" => "5832873106",
    );
    test!(
        DAY14.part2,
        example1: "51589" => 9,
        example2: "01245" => 5,
        example3: "92510" => 18,
        example4: "59414" => 2018,
        input: "939601" => 20273708,
    );
}
