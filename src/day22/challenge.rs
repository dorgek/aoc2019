use regex::Regex;

// am able to simplify the resets into y = bx
// cuts can be simplified into y = (x + z) % L, where z = N, if positive or z = L - N if negative 
// the current simplification for the re-deal becomes y = -x

// next will need to simplify the series of equations that get created from the input instructions

const DATA: &str = include_str!( "./puzzleInput.txt" );

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct LinearEquation {
    b: i128,
    c: i128
}

/// The three rules get reduced down into 3 different linear equations thats ban be applied
/// cut       => y = x + i (if cut is < 0 then i = n - cut, else i = -cut)
/// iterate   => y = ix 
/// new stack => y = -x + n - 1
fn map_instruction_to_linear( line: &str, n: i128, re: Regex ) -> LinearEquation {

    if line.contains( "new stack" ) {
        return LinearEquation { b: -1, c: n - 1 };
    }
    else if line.contains( "increment" ) {
        let mut cap = re.captures_iter( &line );

        if let Some( i ) = cap.next() {
            return LinearEquation { b: i[0].parse::<i128>().unwrap(), c: 0 };
        }
    } 

    // else cut
    let mut cap = re.captures_iter( &line );

    if let Some( i ) = cap.next() {
        let val = i[0].parse::<i128>().unwrap();

        if val < 0 {
            return LinearEquation { b: 1, c: -val };
        } else {
            return LinearEquation { b: 1, c: n - val };
        }
    }

    panic!( "Unsupported deck shuffle" );
}

fn process_input_instructions( input: &str, n: i128 ) -> LinearEquation {
    let re = Regex::new( "-*[0-9]+" ).unwrap();

    let linear_equations: Vec<LinearEquation> = input.lines()
        .map( |l| map_instruction_to_linear( l, n, re.clone() ) )
         .collect();

    // reduce linear equations into one
    // as for each equation is the input to the next, i.e. f(x) and g(x) is g(f(x)), this can be simplified down
    // into one equation
    let mut reduced_linear_equation = linear_equations[0].clone();

    for i in 1.. linear_equations.len() {
        // to prevent integer overflow (a + b) % c = (a % c + b % c) % c, and (a * b) % c = (a % c * b % c) % c assuming x < n
        reduced_linear_equation.b = ( linear_equations[i].b * reduced_linear_equation.b ).rem_euclid( n );
        reduced_linear_equation.c = ( linear_equations[i].b * reduced_linear_equation.c + linear_equations[i].c ).rem_euclid( n );
    }

    return reduced_linear_equation;
}

fn part_one( linear_equation: LinearEquation, start_index: i128, n: i128 ) {
    let res = ( linear_equation.b * start_index + linear_equation.c ).rem_euclid( n );

    println!( "Part one: for start index {}, end index is {}", start_index, res );
}

fn part_two( linear_equation: LinearEquation, n: i128, m: i128, y: i128 ) {
    // the iterations can be continuous shuffles can be simplified down into a geometric progression such as 
    // yn = b^n xn + c b ^(n - 1), where n is the number of shuffles

    let c = ( linear_equation.c * ( 1 - linear_equation.b.pow( ( n ) as u32 ) ) / ( 1 - linear_equation.b ) ).rem_euclid( m ); // TODO: complete modular exponential 
    let b = linear_equation.b.pow( n as u32 ).rem_euclid( m );
    println!( "y = {}x + {}", b, c );

    // TODO: find the multiplicative inverse of the 2020 = bx + c mod m and solve for x
}

pub fn day_22( _: Vec< String > ) {
    let n = 10007;
    let i = 2019;
    let linear_equation_one = process_input_instructions( DATA.clone(), n );

    part_one( linear_equation_one, i, n );
}