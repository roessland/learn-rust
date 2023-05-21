
// Solution uses a loop here.
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    [
        [matrix[0][0],matrix[1][0],matrix[2][0]],
        [matrix[0][1],matrix[1][1],matrix[2][1]],
        [matrix[0][2],matrix[1][2],matrix[2][2]],
    ]
}

// Solution just prints every row instead of every cell.
fn pretty_print(matrix: &[[i32; 3]; 3]) {
    println!("┌─────┬─────┬─────┐");
    for row in matrix.iter() {
        print!("│");
        for val in row.iter() {
            print!(" {:3} │", val);
        }
        println!();
        println!("└─────┸─────┸─────┘");
    }
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}


#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}