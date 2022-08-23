// recieves table and i need to send beck who player won or if it was tie

pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    let d = diagonals("O", &table);
    let h = horizontal("O", &table);
    let v = vertical("O", &table);
    let d1 = diagonals("X", &table);
    let h1 = horizontal("X", &table);
    let v1 = vertical("X", &table);

    println!("DioO {:?}", d);
    println!("RowO {:?}", h);
    println!("VerO {:?}", v);
    println!("DioX {:?}", d1);
    println!("RowX {:?}", h1);
    println!("VerX {:?}", v1);
    println!("Table {:?}", table);
    println!();

    if d == true || h == true || v == true {
        "player O won".to_string()
    } else if d1 == true || h1 == true || v1 == true {
        "player X won".to_string()
    } else {
        "Tie".to_string()
    }
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let mut points1 = 0;
    let mut points2 = 0;
    for (i, row) in table.iter().enumerate() {
        let first = row.clone().into_iter().nth(i).unwrap();
        if first == player {
            points1 += 1;
        }

        let second = row.clone().into_iter().nth(row.len() - i - 1).unwrap();
        if second == player {
            points2 += 1;
        }
    }
    if points1 == table.len() || points2 == table.len() {
        return true;
    }
    false
}
// recieves player symbol, table and i need to send beck tru if won and false if lost
pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let mut points = 0;
    for row in table {
        for element in row {
            if element == &player {
                points += 1;
            }
        }
        if points == row.len() {
            return true;
        } else {
            points = 0;
        }
    }
    false
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let mut points = 0;
    for row in table {
        let first = row.clone().into_iter().nth(0).unwrap();
        if first == player {
            points += 1;
        }
    }
    if points == table.len() {
        return true;
    }
    points = 0;
    for row in table {
        let first = row.clone().into_iter().nth(1).unwrap();
        if first == player {
            points += 1;
        }
    }
    println!("Table len {} and {}", table.len(), points);
    if points == table.len() {
        return true;
    }
    points = 0;
    for row in table {
        let first = row.clone().into_iter().nth(2).unwrap();
        if first == player {
            points += 1;
        }
    }
    if points == table.len() {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Test<'a> {
        player: &'a str,
        table: Vec<Vec<&'a str>>,
        result: &'a str,
    }
    impl Test<'_> {
        fn init_horizontal() -> Vec<Test<'static>> {
            vec![
                Test {
                    player: "O",
                    table: vec![
                        vec!["O", "O", "O"],
                        vec!["X", "X", "O"],
                        vec!["O", "#", "X"],
                    ],
                    result: "player O won",
                },
                Test {
                    player: "O",
                    table: vec![
                        vec!["X", "X", "O"],
                        vec!["O", "O", "O"],
                        vec!["O", "#", "X"],
                    ],
                    result: "player O won",
                },
                Test {
                    player: "X",
                    table: vec![
                        vec!["O", "X", "O"],
                        vec!["O", "#", "O"],
                        vec!["X", "X", "X"],
                    ],
                    result: "player X won",
                },
                Test {
                    player: "X",
                    table: vec![
                        vec!["O", "X", "O", "O"],
                        vec!["X", "X", "X", "X"],
                        vec!["X", "#", "O", "X"],
                        vec!["X", "X", "O", "O"],
                    ],
                    result: "player X won",
                },
            ]
        }
        fn init_tie() -> Vec<Test<'static>> {
            vec![
                Test {
                    player: "none",
                    table: vec![
                        vec!["O", "X", "O"],
                        vec!["O", "X", "O"],
                        vec!["X", "#", "X"],
                    ],
                    result: "Tie",
                },
                Test {
                    player: "none",
                    table: vec![
                        vec!["O", "X", "O"],
                        vec!["X", "X", "O"],
                        vec!["X", "#", "X"],
                    ],
                    result: "Tie",
                },
                Test {
                    player: "none",
                    table: vec![
                        vec!["O", "X", "O", "O"],
                        vec!["X", "O", "X", "X"],
                        vec!["X", "#", "X", "X"],
                        vec!["X", "X", "O", "O"],
                    ],
                    result: "Tie",
                },
            ]
        }
        fn init_vertical() -> Vec<Test<'static>> {
            vec![
                Test {
                    player: "O",
                    table: vec![
                        vec!["O", "X", "O"],
                        vec!["O", "X", "O"],
                        vec!["O", "#", "X"],
                    ],
                    result: "player O won",
                },
                Test {
                    player: "O",
                    table: vec![
                        vec!["X", "O", "O"],
                        vec!["X", "O", "O"],
                        vec!["#", "O", "X"],
                    ],
                    result: "player O won",
                },
                Test {
                    player: "X",
                    table: vec![
                        vec!["O", "X", "X"],
                        vec!["O", "X", "X"],
                        vec!["X", "#", "X"],
                    ],
                    result: "player X won",
                },
            ]
        }
        fn init_diagonals() -> Vec<Test<'static>> {
            vec![
                Test {
                    player: "X",
                    table: vec![
                        vec!["O", "O", "X"],
                        vec!["O", "X", "O"],
                        vec!["X", "#", "X"],
                    ],
                    result: "player X won",
                },
                Test {
                    player: "O",
                    table: vec![
                        vec!["O", "X", "O"],
                        vec!["X", "O", "O"],
                        vec!["X", "#", "O"],
                    ],
                    result: "player O won",
                },
                Test {
                    player: "O",
                    table: vec![
                        vec!["O", "X", "O", "O"],
                        vec!["X", "O", "X", "O"],
                        vec!["X", "#", "O", "X"],
                        vec!["X", "X", "O", "O"],
                    ],
                    result: "player O won",
                },
            ]
        }
    }
    #[test]
    fn test_diagonals() {
        let new_tests = Test::init_diagonals();
        for v in new_tests {
            assert_eq!(diagonals(v.player, &v.table), true)
        }
        assert_eq!(
            diagonals(
                "O",
                &vec![
                    vec!["O", "X", "X"],
                    vec!["O", "X", "X"],
                    vec!["X", "#", "X"]
                ]
            ),
            false
        );
    }
    #[test]
    fn test_horizontal() {
        let new_tests = Test::init_horizontal();
        for v in new_tests {
            assert_eq!(horizontal(v.player, &v.table), true)
        }
        assert_eq!(
            horizontal(
                "O",
                &vec![
                    vec!["O", "X", "X"],
                    vec!["O", "O", "X"],
                    vec!["X", "#", "O"]
                ]
            ),
            false
        );
    }
    #[test]
    fn test_vertical() {
        let new_tests = Test::init_vertical();
        for v in new_tests {
            assert_eq!(vertical(v.player, &v.table), true)
        }
        assert_eq!(
            vertical(
                "O",
                &vec![
                    vec!["O", "X", "X"],
                    vec!["O", "O", "X"],
                    vec!["X", "#", "O"]
                ]
            ),
            false
        );
    }
    #[test]
    fn test_tic_tac_toe() {
        let mut new_tests = Test::init_diagonals();
        new_tests.append(&mut Test::init_horizontal());
        new_tests.append(&mut Test::init_vertical());
        new_tests.append(&mut Test::init_tie());
        for v in new_tests {
            assert_eq!(tic_tac_toe(v.table), v.result.to_string());
        }
    }
}
