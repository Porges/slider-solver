use slider_solver::{parse_board, solve};

// Format:
// a # is a wall, must be around outside,
// a space is free space,
// capital letters are blocks with identity (non-fungible),
// lowercase letters or numbers etc are blocks without identity (can be interchanged).
//
// In the target, spaces are ignored.
const EXAMPLES: &[(&str, &str)] = &[
    (
        "
######
#1AA2#
#1AA2#
#4335#
#4675#
#8  9#
######
",
        "
######
#    #
#    #
#    #
# AA #
# AA #
######
",
    ),
    (
        "
######
#MAAN#
#MAAN#
#OWWP#
#ObcP#
#a  d#
######
",
        "
######
# MN #
#OMNP#
#OWWP#
#aAAb#
#cAAd#
######
",
    ),
    (
        "
######
#AA11#
#AA22#
#34  #
#5677#
#5688#
######
",
        "
######
#    #
#    #
#    #
#AA  #
#AA  #
######",
    ),
];

fn main() {
    for (source, dest) in EXAMPLES.iter() {
        let board = parse_board(source);
        let target = parse_board(dest);

        println!("----");
        println!("Source:");
        println!("{}", source);
        println!("----");

        println!("Target:");
        println!("{}", target);
        println!("----");

        let (visited, generated, result) = solve(&board, &target);

        if let Some((_boards, cost)) = result {
            println!("Found a solution in {} moves:", cost);
            println!(
                "Visited {} board positions (generated {} total).",
                visited, generated
            );

            println!("----");
            println!();
        } else {
            println!("No solution found");
        }
    }
}
