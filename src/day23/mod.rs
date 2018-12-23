use crate::Solution;
use nom::types::CompleteStr;
use nom::{call, do_parse, error_position, map_res, named, tag, take_while};
use std::error::Error;
use z3::{Ast, Config, Context, Optimize};

pub(crate) const DAY23: Solution = Solution {
    part1: |input| {
        let nanobots = input
            .lines()
            .map(get_nanobot)
            .collect::<Result<Vec<_>, _>>()?;
        let Nanobot { radius, position } = nanobots
            .iter()
            .max_by_key(|n| n.radius)
            .ok_or("No nanobots")?;
        Ok(nanobots
            .iter()
            .filter(|n| *radius >= position.distance_to(&n.position))
            .count()
            .to_string())
    },
    part2: |input| {
        let ctx = Context::new(&Config::new());
        let zx = ctx.named_int_const("x");
        let zy = ctx.named_int_const("y");
        let zz = ctx.named_int_const("z");
        let mut in_ranges = Ast::from_i64(&ctx, 0);
        for line in input.lines() {
            let Nanobot {
                radius,
                position: Position { x, y, z },
            } = get_nanobot(line)?;
            in_ranges = zabssub(&ctx, &zx, x)
                .add(&[&zabssub(&ctx, &zy, y), &zabssub(&ctx, &zz, z)])
                .le(&Ast::from_i64(&ctx, radius.into()))
                .ite(&Ast::from_i64(&ctx, 1), &Ast::from_i64(&ctx, 0))
                .add(&[&in_ranges]);
        }
        let optimize = Optimize::new(&ctx);
        optimize.maximize(&in_ranges);
        optimize.check();
        let mut sum = 0;
        for var in &[zx, zy, zz] {
            sum += optimize
                .get_model()
                .eval(&var)
                .ok_or("Variable not available")?
                .as_i64()
                .ok_or("Variable not obtainable as i64")?;
        }
        Ok(sum.to_string())
    },
};

fn get_nanobot(line: &str) -> Result<Nanobot, Box<dyn Error + '_>> {
    match nanobot(CompleteStr(line))? {
        (CompleteStr(""), nanobot) => Ok(nanobot),
        _ => Err("Found text after nanobot")?,
    }
}

named!(
    nanobot(CompleteStr<'_>) -> Nanobot,
    do_parse!(
        tag!("pos=<")
            >> x: integer
            >> tag!(",")
            >> y: integer
            >> tag!(",")
            >> z: integer
            >> tag!(">, r=")
            >> radius: integer
            >> (Nanobot {
                position: Position { x, y, z },
                radius,
            })
    )
);

#[rustfmt::skip]
named!(
    integer(CompleteStr<'_>) -> i32,
    map_res!(take_while!(|c| c == '-' || char::is_digit(c, 10)), |x: CompleteStr<'_>| x.parse())
);

#[derive(Debug)]
struct Nanobot {
    position: Position,
    radius: i32,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn distance_to(&self, other: &Self) -> i32 {
        let fields: &[fn(&Self) -> i32] = &[|n| n.x, |n| n.y, |n| n.z];
        fields
            .iter()
            .map(|f| (f(self) - f(other)).abs())
            .sum::<i32>()
    }
}

fn zabssub<'ctx>(ctx: &'ctx Context, zv: &Ast<'ctx>, v: i32) -> Ast<'ctx> {
    zabs(&ctx, &zv.sub(&[&Ast::from_i64(&ctx, v.into())]))
}

fn zabs<'ctx>(ctx: &'ctx Context, v: &Ast<'ctx>) -> Ast<'ctx> {
    v.ge(&Ast::from_i64(ctx, 0)).ite(v, &v.minus())
}

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY23.part1,
        example: lines!(
            "pos=<0,0,0>, r=4"
            "pos=<1,0,0>, r=1"
            "pos=<4,0,0>, r=3"
            "pos=<0,2,0>, r=1"
            "pos=<0,5,0>, r=3"
            "pos=<0,0,3>, r=1"
            "pos=<1,1,1>, r=1"
            "pos=<1,1,2>, r=1"
            "pos=<1,3,1>, r=1"
        ) => 7,
        input: 253,
    );
    test!(
        DAY23.part2,
        example: lines!(
            "pos=<10,12,12>, r=2"
            "pos=<12,14,12>, r=2"
            "pos=<16,12,12>, r=4"
            "pos=<14,14,14>, r=6"
            "pos=<50,50,50>, r=200"
            "pos=<10,10,10>, r=5"
        ) => 36,
        input: 108_618_801,
    );
}
