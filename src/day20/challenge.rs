use std::collections::BTreeMap;

const DATA: &str = include_str!( "./puzzleInput.txt" );



#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
struct Neighbour {
    key: String,
    weight: u64,
    depth: i64
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64
}

fn part_one_data_structure( data_vec_in: Vec< Vec< String > > ) -> Vec< Vec< String > > {
    let mut data_vec_char = data_vec_in.clone();

    // replace double labels with a unique symbol representing the gate
    // horizontal checks
    for l in data_vec_char.iter_mut() {
        let mut current_char = l[0].clone();
        for idx in 1.. l.len() {
            let next_char = l[idx].clone();

            if current_char.chars().all( |c| c.is_alphabetic() ) && next_char.chars().all( |c| c.is_alphabetic() ) {
                let update_label: String = current_char + &next_char;

                // check left
                let idx_update = idx as i64 - 2;
                if let Some( left ) = l.get( idx_update as usize ) {
                    if *left == " ".to_string() {
                        l[idx] = update_label;
                        l[idx-1] = " ".to_string();
                    } else {
                        l[idx-1] = update_label;
                        l[idx] = " ".to_string();
                    }
                } else {
                    l[idx] = update_label;
                    l[idx-1] = " ".to_string();
                }
            }

            current_char = next_char;
        }
    }

    // vertical checks
    for idx_x in 0.. data_vec_char[0].len() {
        let mut current_char = data_vec_char[0][idx_x].clone();

        for idx_y in 1.. data_vec_char.len() {
            let next_char = data_vec_char[idx_y][idx_x].clone();

            if current_char.clone().chars().all( |c| c.is_alphabetic() ) && next_char.chars().all( |c| c.is_alphabetic() ) {
                let update_label: String = current_char.clone() + &next_char;

                let up_idx = idx_y as i64 - 2;

                // check up
                if up_idx < 0 || data_vec_char[up_idx as usize][idx_x] == " ".to_string() {
                    data_vec_char[idx_y][idx_x] = update_label;
                    data_vec_char[idx_y - 1][idx_x] = " ".to_string();
                } else {
                    data_vec_char[idx_y - 1][idx_x] = update_label;
                    data_vec_char[idx_y][idx_x] = " ".to_string();
                }
            }

            current_char = next_char;
        }
    }

    return data_vec_char;
}

fn part_two_data_structure( data_vec_in: Vec< Vec< String > > ) -> Vec< Vec< String > > {
    let mut data_vec_char = data_vec_in.clone();

    // replace double labels with a unique symbol representing the gate
    // horizontal checks
    for l in data_vec_char.iter_mut() {
        let mut current_char = l[0].clone();
        for idx in 1.. l.len() {
            let next_char = l[idx].clone();

            if current_char.chars().all( |c| c.is_alphabetic() ) && next_char.chars().all( |c| c.is_alphabetic() ) {
                let mut update_label: String = current_char + &next_char;

                // check left
                let idx_update = idx as i64 - 2;
                if let Some( left ) = l.get( idx_update as usize ) {
                    if idx == l.len() {
                        update_label += "1"; // outer
                    } else {
                        update_label += "2"; // inner
                    }

                    if *left == " ".to_string() {
                        l[idx] = update_label;
                        l[idx-1] = " ".to_string();
                    } else {
                        l[idx-1] = update_label;
                        l[idx] = " ".to_string();
                    }
                } else {
                    // outer
                    update_label += "1";
                    l[idx] = update_label;
                    l[idx-1] = " ".to_string();
                }
            }

            current_char = next_char;
        }
    }

    // vertical checks
    for idx_x in 0.. data_vec_char[0].len() {
        let mut current_char = data_vec_char[0][idx_x].clone();

        for idx_y in 1.. data_vec_char.len() {
            let next_char = data_vec_char[idx_y][idx_x].clone();

            if current_char.clone().chars().all( |c| c.is_alphabetic() ) && next_char.chars().all( |c| c.is_alphabetic() ) {
                let mut update_label: String = current_char.clone() + &next_char;

                let up_idx = idx_y as i64 - 2;

                if up_idx < 0 || idx_y >= data_vec_char.len() - 1 {
                    update_label += "1"; // outer
                } else {
                    update_label += "2"; // inner
                }

                // check up
                if up_idx < 0 || data_vec_char[up_idx as usize][idx_x] == " ".to_string() {
                    data_vec_char[idx_y][idx_x] = update_label;
                    data_vec_char[idx_y - 1][idx_x] = " ".to_string();
                } else {
                    data_vec_char[idx_y - 1][idx_x] = update_label;
                    data_vec_char[idx_y][idx_x] = " ".to_string();
                }
            }

            current_char = next_char;
        }
    }

    return data_vec_char;
}

fn build_graph( part_one: bool ) -> BTreeMap< String, Vec< Neighbour > > {
    let mut data_vec_char: Vec< Vec< String > > = DATA.lines()
        .map( |l| l.chars().map( |c| ( c.to_string() )).collect::<Vec<String>>() )
        .collect();
    let mut graph: BTreeMap< String, Vec< Neighbour > > = BTreeMap::new();

    if part_one {
        data_vec_char = part_one_data_structure( data_vec_char );
    } else {
        data_vec_char = part_two_data_structure( data_vec_char );
    }

    let data_vec: Vec< Vec< ( Coordinate, String ) > > = data_vec_char.iter()
        .enumerate()
        .map( |( y, v )| v.iter().enumerate().map( |( x, s )| ( Coordinate{ x: x as i64, y: y as i64 }, s.clone() ) ).collect() )
        .collect();


    let to_visit: Vec< ( String, Coordinate ) > = data_vec.iter()
            .flat_map( |l| l.iter().filter( |( _, c )| c.chars().next().unwrap().is_alphabetic() ).map( |( coord, c )| ( c.clone(), *coord ) ) )
            .collect();
    let mut temp_graph: Vec< ( String, Vec< Neighbour > ) > = Vec::new();

    for ( key, visit ) in to_visit {
        let distances = bfs( data_vec.clone(), visit );
        let mut neighbours: Vec< Neighbour > = Vec::new();

        for ( _, d, dk ) in distances {
            if dk.chars().next().unwrap().is_alphabetic() {
                let depth = match dk.chars().last() {
                    Some( '2' ) => 1,
                    _ => -1
                };

                neighbours.push( Neighbour{ key: dk.to_string(), weight: d, depth: depth })
            }
        }

        temp_graph.push( ( key.to_string(), neighbours ) );
    }

    if !part_one {
        // strip numbers from all nodes
        for ( k, v ) in temp_graph.iter_mut() {
            let new_label = k.clone().chars().filter( |c| !c.is_digit( 10 ) ).collect::<String>();
            *k = new_label.clone();


            for n in v.iter_mut() {
                let new_label_n = n.key.chars().filter( |c| !c.is_digit( 10 ) ).collect::<String>();
                n.key = new_label_n;
            }
        }
    }

    for ( k, v )  in temp_graph.clone().iter_mut().filter( |( k, _ ) | k.len() > 1 ) {
        for n in v.iter_mut() {
            n.weight -= 1;
        }

        if let Some( uv ) = graph.get_mut( &k.clone() ) {
            uv.append( v );
        } else {
            graph.insert( k.to_string(), v.to_vec() );
        }
    }

    return graph;
}

fn bfs( graph: Vec< Vec< ( Coordinate, String ) > >, root: Coordinate ) -> Vec< ( Coordinate, u64, String ) > {
    let mut visited: Vec< Coordinate > = Vec::new();
    let mut queue: Vec< ( Coordinate, u64 ) > = Vec::new();
    let mut distances: Vec< ( Coordinate, u64, String ) > = Vec::new();

    visited.push( root );
    queue.push( ( root, 0 ) );

    while let Some( ( p, d ) ) = queue.pop() {
        let neighbours = find_neighbours( graph.clone(), p );

        for ( neighbour, key ) in neighbours {

            if !visited.contains( &neighbour ) {
                visited.push( neighbour );

                distances.push( ( neighbour, d + 1, key.clone() ) );

                if !key.clone().chars().next().unwrap().is_alphabetic() {
                    queue.push( ( neighbour, d + 1 ) );
                }
            }
        }
    }

    return distances;
}

fn find_neighbours( graph: Vec< Vec< ( Coordinate, String ) > >, current: Coordinate ) -> Vec< ( Coordinate, String ) > {
    let possible_neighbours: Vec< ( i64, i64 ) > = vec![(-1, 0), 
                                                        (1, 0),
                                                        (0, -1),
                                                        (0, 1)];
    let mut neighbours: Vec< ( Coordinate, String ) > = Vec::new();

    for possible_neighbour in possible_neighbours {
        let neighbour = Coordinate {
            x: current.x + possible_neighbour.0,
            y: current.y + possible_neighbour.1
        };

        if let Some( neighbour_value ) = graph.iter()
            .flat_map( | v | v.iter().filter( | ( coord, _ ) | *coord == neighbour ) )
                .map( | ( _, c ) | ( c.clone() ) )
                .collect::< Vec< String > >()
                .pop()
        {
            if neighbour_value.clone().chars().next().unwrap().is_alphabetic() || neighbour_value == "."  {
                neighbours.push( ( neighbour, neighbour_value ) );
            }
        }
            
    }

    return neighbours;
}

fn dijkstra( graph: BTreeMap< String, Vec< Neighbour > >, start_node: String ) -> BTreeMap< String ,u64 > {
    let mut distances: BTreeMap< String, u64 > = BTreeMap::new();
    let mut to_visit: Vec< String > = Vec::new();

    distances.insert( start_node.clone(), 0 );
    to_visit.push( start_node.clone() );

    while let Some( current_node ) = to_visit.pop() {
        if let Some( neighbours ) = graph.get( &current_node ) {
            for neighbour in neighbours {
                let current_distance: u64 = *distances.get( &current_node ).unwrap();
                let new_distance = current_distance + neighbour.weight;

                if let Some( next_neighbour_distance ) = distances.get( &neighbour.key ) {
                    if new_distance < *next_neighbour_distance {
                        distances.insert( neighbour.key.clone(), new_distance );
                        to_visit.push( neighbour.key.clone() );
                    }
                } else {
                    distances.insert( neighbour.key.clone(), new_distance );
                    to_visit.push( neighbour.key.clone() );
                }
            }
        }
    }

    return distances;
}

fn dijkstra_two( graph: BTreeMap< String, Vec< Neighbour > >, start_node: String ) -> BTreeMap< ( String, i64 ), u64 > {
    let mut distances: BTreeMap< ( String, i64 ), u64 > = BTreeMap::new();
    let mut to_visit: Vec< ( String, i64 ) > = Vec::new();

    distances.insert( ( start_node.clone(), 0 ), 0 );
    to_visit.push( ( start_node.clone(), 0 ) );

    while let Some( current_node ) = to_visit.pop() {
        if let Some( neighbours ) = graph.get( &current_node.0 ) {
            for neighbour in neighbours {
                let current_distance: u64 = *distances.get( &current_node ).unwrap();
                let new_distance = current_distance + neighbour.weight;

                if current_node.1 == 0 && neighbour.depth < 0 {
                    continue;
                }

                let new_neighbour = ( neighbour.key.clone(), current_node.1 + neighbour.depth );

                if let Some( next_neighbour_distance ) = distances.get( &new_neighbour ) {
                    if new_distance < *next_neighbour_distance {
                        distances.insert( new_neighbour.clone(), new_distance );
                        to_visit.push( new_neighbour.clone() );
                    }
                } else {
                    distances.insert( new_neighbour.clone(), new_distance );
                    to_visit.push( new_neighbour.clone() );
                }
            }
        }
    }

    return distances;
}

fn part_one( graph: BTreeMap< String, Vec< Neighbour > > ) {
    let distances = dijkstra( graph.clone(), "AA".to_string() );
    let distance = distances.get( "ZZ" ).unwrap() - 1;

    println!( "Part One: minimum distance to exit = {}", distance );
}

fn part_two( graph: BTreeMap< String, Vec< Neighbour > > ) {
    let distances = dijkstra_two( graph.clone(), "AA".to_string() );
    let distance = distances.get( &( "ZZ".to_string(), 0 ) ).unwrap() - 1;

    println!( "Part Two: minimum distance to exit = {}", distance );
}

pub fn day_20( _args: Vec<String> ) {
    let graph = build_graph( true );
    part_one( graph.clone() );

    let graph_two = build_graph( false );
    part_two( graph_two.clone() );
}