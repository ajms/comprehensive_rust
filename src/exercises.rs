fn multiply(x: f32, y: f32) -> f32 {
    x * y
}

pub fn exercise1() {
    let x: i8 = 15;
    let outp: bool = true;
    let y: i16 = 1000;
    let xy: f32 = multiply(x.into(), y.into());
    if outp {
        println!("{x} * {y} = {}", xy);
    }
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        println!("{row:?}")
    }
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut mt: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            mt[j][i] = matrix[i][j];
        }
    }
    return mt;
}

pub fn exercise2() {
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
