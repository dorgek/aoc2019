use std::collections::BTreeSet;
use std::collections::BTreeMap;

const DATA: &str = include_str!( "./puzzleInput.txt" );
const X: i64 = 5;
const Y: i64 = 5;

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
struct Coordinate {
    x: i64,
    y: i64
}

enum Row {
    TOP_ROW,
    BOTTOM_ROW,
    LEFT_ROW,
    RIGHT_ROW
}

fn process_input( data: &str ) -> BTreeSet< Coordinate > {
    let mut bugs: BTreeSet< Coordinate > = BTreeSet::new();
    data.lines()
        .enumerate()
        .for_each( | ( y, l ) | {
            l.chars()
                .enumerate()
                .for_each( |( x, c ) | {
                    if c == '#' {
                        bugs.insert( Coordinate{ x: x as i64, y: y as i64 } );
                    }
                });
        });

    return bugs;
}

fn get_neighbours( coordinate: Coordinate ) -> Vec< Coordinate > {
    let possible_neighbours: Vec< ( i64, i64 ) > = vec![ ( -1, 0 ), ( 1, 0 ), ( 0, -1 ), ( 0, 1 ) ];
    let mut neighbours: Vec<Coordinate> = Vec::new();

    for possible_neighbour in possible_neighbours {
        let mut neighbour_coordinate = Coordinate{ x: coordinate.x, y: coordinate.y };
        neighbour_coordinate.x += possible_neighbour.0;
        neighbour_coordinate.y += possible_neighbour.1;
        neighbours.push( neighbour_coordinate );
    }

    return neighbours;
}

fn num_adjacent_bug_tiles( neighbours: Vec< Coordinate >, bugs: BTreeSet< Coordinate > ) -> u64 {
    let mut num_bugs: u64 = 0;

    for possible_neighbour in neighbours {
        if let Some( _ ) = bugs.get( &possible_neighbour ) {
            num_bugs += 1;
        }
    }

    return num_bugs;
}

/// - A bug dies (becoming an empty space) unless there is exactly one bug adjacent to it.
/// - An empty space becomes infested with a bug if exactly one or two bugs are adjacent to it.
fn step_state( bugs: BTreeSet< Coordinate > ) -> BTreeSet< Coordinate > {
    let mut new_bugs: BTreeSet< Coordinate > = BTreeSet::new();

    for x in 0..X {
        for y in 0.. Y {
            let current_coordiante = Coordinate{ x: x as i64, y: y as i64 };
            if let Some( coordinate ) = bugs.get( &current_coordiante ) {
                // check if bug dies 
                let possible_neighbours = get_neighbours( *coordinate );
                let num_bugs = num_adjacent_bug_tiles( possible_neighbours, bugs.clone() );
                
                if num_bugs == 1 {
                    new_bugs.insert( coordinate.clone() );
                }
            } else {
                // no bug, check if it becomes a bug 
                let possible_neighbours = get_neighbours( current_coordiante );
                let num_bugs = num_adjacent_bug_tiles( possible_neighbours, bugs.clone() );
                
                if num_bugs > 0 && num_bugs < 3  {
                    new_bugs.insert( current_coordiante.clone() );
                }
            }
        }
    }

    return new_bugs;
}

fn print_grid( bugs: BTreeSet< Coordinate > ) {
    for y in 0.. Y {
        for x in 0.. X {
            let current_coordiante = Coordinate{ x: x as i64, y: y as i64 };

            if let Some( _ ) = bugs.get( &current_coordiante ) {
                print!( "#" );
            } else {
                print!( "." );
            }
        }
        println!( "" );
    }

    println!();
}

fn calculate_biodiversity_rating( bugs: BTreeSet< Coordinate > ) -> u64 {
    let mut biodirversity_rating = 0;

    for bug_tile in bugs {
        let tile_num = bug_tile.y * X as i64 + bug_tile.x;
        biodirversity_rating += u64::pow( 2, tile_num as u32 );
    }

    return biodirversity_rating;
}

fn part_one( bugs: BTreeSet< Coordinate > ) {
    let mut current_bugs: BTreeSet< Coordinate > = bugs.clone();
    let mut bugs_match: bool = false;
    let mut previous_layers: BTreeSet< BTreeSet< Coordinate > > = BTreeSet::new();

    while !bugs_match {
        print_grid( current_bugs.clone() );
        previous_layers.insert( current_bugs.clone() );
        current_bugs = step_state( current_bugs.clone() );

        if let Some( _ ) = previous_layers.get( &current_bugs ){
            bugs_match = true;
        }

    }

    print_grid( current_bugs.clone() );
    println!( "Part one: biodiversity rating = {}", calculate_biodiversity_rating( current_bugs.clone() ) );
}

fn get_internal_neighbours( row: Row, level: i64 ) -> Vec< ( i64, Coordinate ) > {
    let mut neighbours: Vec< ( i64, Coordinate ) > = Vec::new();

    match row {
        Row::RIGHT_ROW => {
            for y in 0.. Y {
                neighbours.push( ( level, Coordinate { x: X - 1, y: y } ) );
            }
        } 
        Row::LEFT_ROW => {
            for y in 0.. Y {
                neighbours.push( ( level, Coordinate { x: 0, y: y } ) );
            }
        }
        Row::BOTTOM_ROW => {
            for x in 0.. X {
                neighbours.push( ( level, Coordinate { x: x, y: Y - 1 } ) );
            }
        }
        Row::TOP_ROW => {
            for x in 0.. X {
                neighbours.push( ( level, Coordinate { x: x, y: 0 } ) );
            }
        }
    }

    return neighbours;
}

fn get_neighbours_two( coordinate: Coordinate, level: i64 ) -> Vec< ( i64, Coordinate ) > {
    let possible_neighbours: Vec< ( i64, i64 ) > = vec![ ( -1, 0 ), ( 1, 0 ), ( 0, -1 ), ( 0, 1 ) ];
    let mut neighbours: Vec< ( i64, Coordinate ) > = Vec::new();

    for possible_neighbour in possible_neighbours {
        let mut neighbour_coordinate = Coordinate{ x: coordinate.x, y: coordinate.y };
        neighbour_coordinate.x += possible_neighbour.0;
        neighbour_coordinate.y += possible_neighbour.1;

        // if point is outside of grid then step out
        if neighbour_coordinate.x < 0 {
            // neighbours include left of middle
            let new_level = level - 1;
            let new_coordinate: Coordinate = Coordinate{ x: 1, y: 2 };
            neighbours.push( ( new_level, new_coordinate ) );
        } else if neighbour_coordinate.x >= X as i64 {
            // neighbours include right of middle 
            let new_level = level - 1;
            let new_coordinate: Coordinate = Coordinate{ x: 3, y: 2 };
            neighbours.push( ( new_level, new_coordinate ) );
        } else if neighbour_coordinate.y < 0 {
            // neighbours include top of middle
            let new_level = level - 1;
            let new_coordinate: Coordinate = Coordinate{ x: 2, y: 1 };
            neighbours.push( ( new_level, new_coordinate ) );
        } else if neighbour_coordinate.y >= Y as i64 {
            // neighbours include entire bottom of middle
            let new_level = level - 1;
            let new_coordinate: Coordinate = Coordinate{ x: 2, y: 3 };
            neighbours.push( ( new_level, new_coordinate ) );
        } else if neighbour_coordinate.x == 2 && neighbour_coordinate.y == 2 {
            // middle square is hit, which one is used depends on the current coordinate position
            let new_level = level + 1;
            if coordinate.x < 2 {
                neighbours.append( &mut get_internal_neighbours( Row::LEFT_ROW, new_level ) );
            } else if coordinate.x > 2 {
                neighbours.append( &mut get_internal_neighbours( Row::RIGHT_ROW, new_level ) );
            } else if coordinate.y < 2 {
                neighbours.append( &mut get_internal_neighbours( Row::TOP_ROW, new_level ) );
            } else {
                neighbours.append( &mut get_internal_neighbours( Row::BOTTOM_ROW, new_level ) );
            }
        } else {
            // normal neighbour
            neighbours.push( ( level, neighbour_coordinate ) );
        }
    }

    return neighbours;
}

fn num_adjacent_bug_tiles_two( coordinates: Vec< ( i64, Coordinate ) >, levels: BTreeMap< i64, BTreeSet< Coordinate > > ) -> u64 {
    let mut num_bugs = 0;

    for ( l, coordinate ) in coordinates {
        if let Some( level ) = levels.get( &l ) {
            if let Some ( _ ) = level.get( &coordinate ) {
                num_bugs += 1;
            }
        }

    }

    return num_bugs;
}

/// - A bug dies (becoming an empty space) unless there is exactly one bug adjacent to it.
/// - An empty space becomes infested with a bug if exactly one or two bugs are adjacent to it.
/// Note: will only need to calculate as many levels as there are bugs, i.e. outer levels will
///       always be empty, and no need to calculate further
fn step_state_two( levels: BTreeMap< i64, BTreeSet< Coordinate > > ) -> BTreeMap< i64, BTreeSet< Coordinate > > {
    let min = *levels.keys().min().unwrap();
    let max = *levels.keys().max().unwrap();
    let mut new_level: BTreeMap< i64, BTreeSet< Coordinate > > = BTreeMap::new();

    for l in min..=max {
        let current_bug_level = levels.get( &l ).unwrap().clone();
        let mut next_bug_level: BTreeSet< Coordinate > = BTreeSet::new();

        for x in 0..X {
            for y in 0.. Y {
                if x == 2 && y == 2 {
                    continue;
                }

                let current_coordinate = Coordinate{ x: x as i64, y: y as i64 };
                let neighbours = get_neighbours_two( current_coordinate, l );

                if let Some( _ ) = current_bug_level.get( &current_coordinate ) {
                    // bug present - check if it lives
                    let num_bugs = num_adjacent_bug_tiles_two( neighbours.clone(), levels.clone() );

                    if num_bugs == 1 {
                        next_bug_level.insert( current_coordinate.clone() );
                    }
                } else {
                    // no bug, check if it becomes a bug 
                    let num_bugs = num_adjacent_bug_tiles_two( neighbours.clone(), levels.clone() );
                
                    if num_bugs > 0 && num_bugs < 3  {
                        next_bug_level.insert( current_coordinate.clone() );
                    }
                }
            }
        }

        new_level.insert( l, next_bug_level );
    }

    // check if it needs padding around for next iteration
    if new_level.get( &min ).unwrap().len() > 0 {
        new_level.insert( min - 1, BTreeSet::new() );
    } 
    
    if new_level.get( &max ).unwrap().len() > 0 {
        new_level.insert( max + 1, BTreeSet::new() );
    }

    return new_level;
}

fn part_two( bugs: BTreeSet< Coordinate > ) {
    let mut levels: BTreeMap< i64, BTreeSet< Coordinate > > = BTreeMap::new();
    let mut num_bugs = 0;

    levels.insert( -1, BTreeSet::new() );
    levels.insert( 0, bugs.clone() );
    levels.insert( 1, BTreeSet::new() );

    for i in 0.. 200 {
        levels = step_state_two( levels );
        println!( "{} minutes have passed", i );
    }

    for ( depth, level ) in levels.clone() {
        println!( "Depth: {}", depth );
        print_grid( level.clone() );
    }

    for ( _depth, level ) in levels.clone() {
        num_bugs += level.len();
    }

    println!( "Part 2: Total number of bugs = {}", num_bugs );
}


#[allow(dead_code)]
pub fn day_24( _: Vec< String > ) {
    let bugs = process_input( DATA );
    part_one( bugs.clone() );
    part_two( bugs.clone() );
}