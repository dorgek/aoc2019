
static MIN_VAL: u64 = 246515;
static MAX_VAL: u64 = 739105;

fn meets_requirements( num: String ) -> bool {
    // rules
    // - it is a six-digit number
    // - the value is within the range given in your puzzle input
    // - two adjacent digits are the same (like 22 in 122345)
    // - going from left to right, the digits never decrease; they only ever increase or stay the same

    let num_int: u64    = num.parse().unwrap();
    let mut previous_val: i64 = -1;
    let mut adj_digits  = false;
    let mut adj_count   = 0;
    let mut two_adj_count = false;

    // six digit number
    if num.len() > 6 {
        return false;
    }

    // within range of puzzle input
    if num_int > MAX_VAL || num_int < MIN_VAL {
        return false;
    }

    for val in num.chars() {
        let val_digit: i64 = val.to_digit( 10 ).unwrap().into();

        // at least two adjacent digits are the same 
        if previous_val == val_digit {
            adj_digits = true;
            adj_count += 1;

        } else {
            // part two
            if adj_count == 1 {
                two_adj_count = true;
            }

            adj_count = 0;
        }

        // left to right the numbers never decrease 
        if previous_val > val_digit {
            return false;
        }

        previous_val = val_digit;
    }

    // part two
    if adj_count == 1 {
        two_adj_count = true;
    }

    // check if adjacent digits are present
    if !adj_digits || !two_adj_count {
        return false;
    }


    return true; 
}


fn check_range() -> u64 {
    let mut count = 0;

    for x in MIN_VAL..=MAX_VAL {
        if meets_requirements( x.to_string() ) {
            count += 1;
        }
    }

    return count;
}

fn main() {
    println!( "ANS: {}", check_range() );
}