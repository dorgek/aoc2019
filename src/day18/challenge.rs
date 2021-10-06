use std::collections::{
    BTreeMap,
    BTreeSet
};

const DATA: &str = include_str!( "./puzzleInput.txt" );
const DATA_PART_TWO: &str = include_str!( "./puzzleInput2.txt" );

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct Neighbour {
    key: char,
    weight: u64
}

fn get_value( x: usize, y: usize, data_vec: Vec< Vec< char > > ) -> Option< char > {
    if let Some( l ) = data_vec.get( y ) {
        if let Some( object ) = l.get( x ) {
            if *object != '#' {
                return Some( *object );
            }
        }
    }

    return None;
}

// should use bfs
fn get_neighbours( x: usize, y: usize, data_vec: Vec< Vec< char > >, checked_neighbours_in: Vec< Coordinate >, current_weight: u64 ) -> Vec< Neighbour > {
    let mut neighbours: BTreeMap< char, u64 > = BTreeMap::new();
    let mut checked_neighbours = checked_neighbours_in.clone();
    let possible_neighbours: Vec< Vec< i64 > > = vec![ vec![0, 1],
                                    vec![0, -1],
                                    vec![1, 0],
                                    vec![-1, 0] ];

    for possible_neighbour in possible_neighbours {
        let x_update = x as i64 - possible_neighbour[0];
        let y_update = y as i64 - possible_neighbour[1];

        if checked_neighbours.iter().find( | coordinate | coordinate.x == x_update as i64 && coordinate.y == y_update as i64 ).is_some() {
            continue;
        } else {
            checked_neighbours.push( Coordinate{ x: x_update, y: y_update } );
            if let Some( c ) = get_value( x_update as usize, y_update as usize, data_vec.clone() ) {
                if c == '#' {
                    continue;
                } else if c == '.' {
                    // recursively find correct neighbours
                    let reachable_neighbours = get_neighbours( x_update as usize, y_update as usize, data_vec.clone(), checked_neighbours.clone(), current_weight + 1 );
                    for reachable_neighbour in reachable_neighbours {

                        if let Some( present_neighbour_distance ) = neighbours.get( &reachable_neighbour.key ) {
                            if *present_neighbour_distance > reachable_neighbour.weight {
                                neighbours.insert( reachable_neighbour.key, reachable_neighbour.weight );
                            }
                        } else {
                            neighbours.insert( reachable_neighbour.key, reachable_neighbour.weight );
                        }
                    }
                } else {
                    neighbours.insert( c, current_weight );
                }
            }

        }
    }

    return neighbours.iter()
        .map( |( k, v )| Neighbour{ key: *k, weight: *v } )
        .collect();
}

/// Build the graph map consiting of only the starting location, the keys to open the doors and the doors them selves
/// This will remove all floor / wall nodes to generate a weighted graph. Note that nodes would then need to go through
/// specific doors before reaching certain keys. 
fn build_map( data_vec: Vec< Vec< char > > ) -> BTreeMap< char, Vec< Neighbour > > {
    let mut graph: BTreeMap< char, Vec< Neighbour > > = BTreeMap::new();

    for ( y, l ) in data_vec.clone().iter().enumerate() {
        for ( x, c ) in l.clone().iter().enumerate() {
            if *c != '#' {
                let checked_neighbours = vec![Coordinate { x: x as i64, y: y as i64 }];
                graph.insert( *c, get_neighbours( x, y, data_vec.clone(), checked_neighbours, 1 ) );
            }
        }
    }

    return graph.iter()
        .filter( |( k, _v )| **k != '.' )
        .map( |( k, v )| ( *k, v.clone() ) )
        .collect();
}

fn dijkstra( graph: BTreeMap< char, Vec< Neighbour > >, start_node: char, key_check: Vec< char > ) -> BTreeMap< ( char, BTreeSet< char > ), u64 > {
    let mut distances: BTreeMap< ( char, BTreeSet<char> ), u64 > = BTreeMap::new();

    let starting_node = ( start_node.clone(), BTreeSet::new() );
    let mut to_visit: Vec< ( char, BTreeSet<char> ) > = Vec::new();
    distances.insert( starting_node.clone(), 0 );
    to_visit.push( starting_node.clone() );

    while let Some( current_node ) = to_visit.pop() {
        let neighbours = graph.get( &current_node.clone().0 ).unwrap();

        for neighbour in neighbours {
            let mut current_distance: u64 = *distances.get( &current_node ).unwrap();
            let mut visited_keys: BTreeSet<char> = current_node.1.clone();

            if key_check.contains( &neighbour.key.to_ascii_lowercase() ) && neighbour.key.is_uppercase() && !visited_keys.contains( &neighbour.key.to_ascii_lowercase() ) {
                continue; // skip over as we don't have access to this door yet
            }

            current_distance += neighbour.weight;
            if neighbour.key.is_lowercase() {
                visited_keys.insert( neighbour.key );
            }

            if let Some( visited_node ) = distances.get_mut( &( neighbour.key, visited_keys.clone() ) ) {
                if current_distance < *visited_node  {
                    *visited_node = current_distance;
                    to_visit.push( ( neighbour.key, visited_keys ) );
                }
            } else {
                distances.insert( ( neighbour.key, visited_keys.clone() ), current_distance );
                to_visit.push( ( neighbour.key, visited_keys.clone() ) );
            }
        }
    }

    return distances;
}

fn get_graph( graph: BTreeMap< char, Vec< Neighbour > >, start_node: char ) -> BTreeMap< char, Vec< Neighbour > > {
    let mut to_check: Vec< char > = Vec::new();
    let mut checked: Vec< char > = Vec::new();
    let mut new_graph: BTreeMap< char, Vec< Neighbour > > = BTreeMap::new();

    to_check.push( start_node );

    while let Some( node ) = to_check.pop() {
        let neighbours = graph.get( &node ).unwrap().clone();
        new_graph.insert( node, neighbours.clone() );

        neighbours.iter().for_each( |n| {
            if !checked.contains( &n.key ) {
                to_check.push( n.key );
            }
        });

        checked.push( node );
    }

    return new_graph;
}

fn get_four_quadrants( data: &str ) -> Vec< BTreeMap< char, Vec< Neighbour > > > {
    let data_vec: Vec< Vec< char > > = data.lines()
            .map( |l| l.chars().collect() )
            .collect();
    let mut maps: Vec< BTreeMap< char, Vec< Neighbour > > > = Vec::new();
    let start_nodes = vec!['@', '$', '&', '*' ];

    let graph = build_map( data_vec );

    for start_node in start_nodes {
        maps.push( get_graph( graph.clone(), start_node ) );
    }

    return maps;
}
 
fn part_one( map: BTreeMap< char, Vec< Neighbour > > ) {
    // get keys required to open door for this map 
    let keys = map.clone()
        .iter()
        .filter( |( k, _ )| k.is_lowercase() )
        .map( |( k, _ )| *k )
        .collect();

    let distances = dijkstra( map.clone(), '@', keys );

    let key_set: BTreeSet< char > = map.keys()
        .filter( |k| k.is_lowercase() )
        .map( |k| *k )
        .collect();

    // find min distance that matches the required hash
    let min_distance = distances.iter()
        .filter( |(k, _)| k.1 == key_set )
        .map( |(_, v)| v )
        .min()
        .unwrap();

    println!( "Part one: minimum distance = {}", min_distance );
}

fn part_two( maps: Vec< BTreeMap< char, Vec< Neighbour > > > ) {
    let mut min_distance_total = 0;
    let start_nodes = vec!['@', '$', '&', '*' ];

    for ( idx, map ) in maps.iter().enumerate() {
        let keys: Vec< char > = map.clone()
            .iter()
            .filter( |( k, _ )| k.is_lowercase() )
            .map( |( k, _ )| *k )
            .collect();


        let distances = dijkstra( map.clone(), start_nodes[idx], keys.clone() );

        let key_set: BTreeSet< char > = keys.clone().iter()
            .map( |k| *k )
            .collect();

        // find min distance that matches the required hash
        let min_distance = distances.iter()
            .filter( |(k, _)| k.1 == key_set )
            .map( |(_, v)| v )
            .min()
            .unwrap();

        min_distance_total += min_distance;
    }

    println!( "Part two: minimum distance = {}", min_distance_total );
}  

#[allow(dead_code)]
pub fn day_18( _args: Vec< String > ) {
    let data_vec: Vec< Vec< char > > = DATA.lines()
            .map( |l| l.chars().collect() )
            .collect();

    let map = build_map( data_vec );
    part_one( map );

    // part two
    let maps = get_four_quadrants( DATA_PART_TWO );
    part_two( maps );
}