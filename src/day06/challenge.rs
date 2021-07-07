use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    collections::HashMap,
    collections::HashSet
};

fn lines_from_file( filename: impl AsRef<Path> ) -> Vec<String> {
    let file = File::open( filename ).expect( "no such file" );
    let buf = BufReader::new( file );

    return buf.lines()
        .map( |l| l.expect( "could not parse line" ) ) 
        .collect();
}


fn construct_adjacency_list( input: Vec< String > ) -> HashMap<String, String> {
    let mut ret: HashMap< String, String > = HashMap::new();

    for line in input {
        let split_orbit: Vec<String> = line.split( ')' ).map( |v| String::from( v ) ).collect();
        let orbit       = split_orbit[0].clone();
        let orbiting    = split_orbit[1].clone();

        ret.insert( orbiting, orbit );
    }

    return ret;
}

fn count_indirect_orbits( adjaceny_list: HashMap<String, String>, mut cache: HashMap<String, u64> ) -> u64 {
    let mut ret = 0;

    for ( k, _v ) in adjaceny_list.clone() {
        let cache_val = count_objects_orbits( adjaceny_list.clone(), k.clone(), cache.clone() );
        ret += cache_val;
        cache.insert( k.clone(), cache_val );
    }

    return ret;
}

fn count_objects_orbits( adjaceny_list: HashMap<String, String>, key: String, cache: HashMap<String, u64> ) -> u64 {
    if cache.contains_key( &key ) {
        return cache[&key];
    }

    return if adjaceny_list.contains_key( &key ) { 1 + count_objects_orbits( adjaceny_list.clone(), adjaceny_list[&key].clone(), cache ) } else { 0 }
}


fn dijkstras( adjaceny_list: HashMap<String, String>, src: String, dest: String ) -> i64 {
    let mut list: HashMap<String, (i64, String)> = HashMap::new();
    let mut current_node: String;
    let mut next_check: HashSet<String> = HashSet::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut iter_vals: Vec<String> = adjaceny_list.keys().cloned().collect();
    iter_vals.append( &mut adjaceny_list.values().cloned().collect() );
    let iter_set: HashSet<String> = iter_vals.into_iter().collect();
    
    for k in iter_set {
        list.insert( k.to_string(), ( i64::max_value(), "None".to_string() ) );
    }

    list.insert( src.clone(), ( 0, src.clone() ) );
    next_check.insert( src.clone() );

    // find the shortest path for all verticies - do while
    while  {
        current_node = next_check.iter().next().clone().unwrap().to_string();
        next_check.remove( &current_node );
        visited.insert( current_node.clone() );

        let mut neighbours: Vec<String> = adjaceny_list.clone().iter()
                            .filter_map( | ( key_value, entry ) | if entry == &current_node { Some( key_value.into() ) } else { None } )
                            .collect();

        if adjaceny_list.contains_key( &current_node ) {
            neighbours.push( adjaceny_list[&current_node].clone() )
        } 

        for neighbour in neighbours.clone() {
            let new_dist = 1 + list[&current_node].0;

            if new_dist < list[&neighbour].0 {
                list.insert( neighbour.clone(), ( new_dist, current_node.clone() ) );
                next_check.insert( neighbour.clone() );
            }
        }

        neighbours.clone().iter()
            .filter_map( |j| if !visited.contains( j ) { Some( j.into() ) } else { None } )
            .for_each( |k: String| { 
                next_check.insert( k.to_string() );
            } );


        // condition
        !next_check.is_empty()
    } {}

    return list[&dest].0 - 2;
}


pub fn day_06( args: Vec<String> ) {
    let input_data = lines_from_file( args[1].clone() );
    let cache: HashMap<String, u64> = HashMap::new();

    let adjaceny_list = construct_adjacency_list( input_data );

    println!( "Part One: {}", count_indirect_orbits( adjaceny_list.clone(), cache ) );
    println!( "Part Two: {}", dijkstras( adjaceny_list.clone(), "YOU".to_string(), "SAN".to_string() ) );
}
