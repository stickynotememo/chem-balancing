use super::*;
#[test]
fn test_is_balanced_balanced() {
    // Arrange
    let args = parse_input(&vec![
        String::from("CO2"),
        String::from("->"),
        String::from("CO"),
        String::from("O"),
    ]);

    // Act
    let balance = is_balanced(&args);

    // Assert
    assert!(balance);
}

#[test]
fn test_is_balanced_unbalanced() {
    // Arrange
    let args = parse_input(&vec![
        String::from("CO2"),
        String::from("->"),
        String::from("CF"),
        String::from("O2"),
    ]);

    // Act
    let balance = is_balanced(&args);

    // Assert
    assert!(!balance);
}

#[test]
fn test_parse_input() {
    // Arrange
    let input = &vec![
        String::from("CO2"),
        String::from("->"),
        String::from("CF"),
        String::from("O2"),
    ];

    // Act
    let actual_output = parse_input(input);

    // Assert
    let expected_output: Equation = (
        vec![Molecule {
            coefficient: 1,
            hashmap: HashMap::from([('C', 1), ('O', 2)]),
        }],
        vec![
            Molecule {
                coefficient: 1,
                hashmap: HashMap::from([('C', 1), ('F', 1)]),
            },
            Molecule {
                coefficient: 1,
                hashmap: HashMap::from([('O', 2)]),
            },
        ],
    );

    assert_eq!(expected_output, actual_output);
}
