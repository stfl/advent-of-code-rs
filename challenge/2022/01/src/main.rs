use fetch::Input;

fn main() {
    let input = Input::try_get(2022, 1).unwrap();

    dbg!(&input.text);

    let elves = split_by_elves(&input.text);
    let mut elves = calc_elves_cal(elves);
    elves.sort();
    let res1 = elves.iter().max().unwrap();

    let res2: i64 = elves[elves.len() - 3..].iter().sum();

    println!("{res1}\n{res2}");
}

fn split_by_elves(text: &str) -> Vec<Vec<i64>> {
    text.split("\n\n")
        .map(|x| {
            x.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn calc_elves_cal(elves: Vec<Vec<i64>>) -> Vec<i64> {
    elves.iter().map(|e| e.iter().sum()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_RESULT1: i64 = 24000;
    static EXAMPLE_RESULT2: i64 = 45000;

    static EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn test_tttt() {
        let elves = split_by_elves(EXAMPLE_INPUT);
        let elves = calc_elves_cal(elves);
        let res = elves.iter().max().unwrap();
        assert_eq!(res, &EXAMPLE_RESULT1);
    }

    #[test]
    fn test_top_3() {
        let elves = split_by_elves(EXAMPLE_INPUT);
        let mut elves = calc_elves_cal(elves);
        elves.sort();

        let res: i64 = elves[elves.len() - 3..].iter().sum();
        assert_eq!(res, EXAMPLE_RESULT2);
    }
}
