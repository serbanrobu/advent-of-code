use std::ops::BitOr;

pub fn part_1(rows: Vec<Vec<u8>>) -> usize {
    if rows.len() < 3 {
        return rows.iter().map(|r| r.len()).sum();
    }

    let mut count = rows[0].len() + rows[rows.len() - 1].len();

    for (i, row) in rows.iter().enumerate().skip(1).take(rows.len() - 2) {
        if row.len() < 3 {
            count += row.len();
            continue;
        }

        count += 2;

        for (j, tree) in row.iter().enumerate().skip(1).take(row.len() - 2) {
            let up: Vec<_> = rows[..i].iter().map(|r| &r[j]).collect();
            let down: Vec<_> = rows[i + 1..].iter().map(|r| &r[j]).collect();
            let left: Vec<_> = row[..j].iter().collect();
            let right: Vec<_> = row[j + 1..].iter().collect();

            if [up, down, left, right]
                .into_iter()
                .map(|ts| ts.into_iter().all(|t| t < tree))
                .reduce(BitOr::bitor)
                .unwrap_or(false)
            {
                count += 1;
            }
        }
    }

    count
}

pub fn part_2(rows: Vec<Vec<u8>>) -> usize {
    rows.iter()
        .enumerate()
        .map(|(i, row)| {
            if i == 0 || i == rows.len() - 1 {
                return 0;
            }

            row.iter()
                .enumerate()
                .map(|(j, tree)| {
                    if j == 0 || j == row.len() - 1 {
                        return 0;
                    }

                    let up: Vec<_> = rows[..i].iter().rev().map(|r| &r[j]).collect();
                    let down: Vec<_> = rows[i + 1..].iter().map(|r| &r[j]).collect();
                    let left: Vec<_> = row[..j].iter().rev().collect();
                    let right: Vec<_> = row[j + 1..].iter().collect();

                    [up, down, left, right]
                        .into_iter()
                        .map(|ts| {
                            let len = ts.len();
                            let smaller = ts.into_iter().take_while(|&t| t < tree).count();

                            if smaller < len {
                                smaller + 1
                            } else {
                                len
                            }
                        })
                        .product()
                })
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TREES: &str = "\
30373
25512
65332
33549
35390
";

    #[test]
    fn test_part_1() {
        for (i, o) in [
            ("", 0),
            ("1", 1),
            ("222", 3),
            ("333\n333", 6),
            ("444\n444\n444", 8),
            (TREES, 21),
        ] {
            let answer = part_1(i.lines().map(|l| l.as_bytes().to_vec()).collect());

            assert_eq!(answer, o);
        }
    }

    #[test]
    fn test_part_2() {
        for (i, o) in [
            ("", 0),
            ("1", 0),
            ("22", 0),
            ("33\n33", 0),
            ("44\n44\n44", 0),
            ("555\n555\n555", 1),
            (
                "6666\n\
                 6106\n\
                 6666",
                2,
            ),
            (TREES, 8),
        ] {
            let answer = part_2(i.lines().map(|l| l.as_bytes().to_vec()).collect());

            assert_eq!(answer, o);
        }
    }
}
