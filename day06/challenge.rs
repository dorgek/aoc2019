use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    env,
    collections::HashMap
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


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_data = lines_from_file( args[1].clone() );
    let cache: HashMap<String, u64> = HashMap::new();

    let adjaceny_list = construct_adjacency_list( input_data );

    println!( "Part One: {}", count_indirect_orbits( adjaceny_list, cache ) );
}
