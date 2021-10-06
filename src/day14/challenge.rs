use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};

use std::collections::{
    BTreeMap,
    HashSet
};

use regex::Regex;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
struct Ingrediant {
    amount: i64,
    compound: String,
    left_over: i64
}

fn process_data( filename: impl AsRef<Path> ) -> BTreeMap<Ingrediant, Vec<Ingrediant>> {
    let file = File::open( filename ).expect( "no such file" );
    let buf  = BufReader::new( file );
    let re_num = Regex::new( "[0-9]+" ).unwrap();
    let re_compound = Regex::new( "[a-zA-Z]+" ).unwrap();
    let mut ret: BTreeMap<Ingrediant, Vec<Ingrediant>> = BTreeMap::new();
    
    buf.lines()
        .for_each( |l| {
            let line = l.expect( "could not parse line" );
            let mut num_cap  = re_num.captures_iter( &line );
            let mut comp_cap = re_compound.captures_iter( &line );

            let mut current_num  = num_cap.next().unwrap()[0].parse::<i64>().unwrap();
            let mut current_comp = comp_cap.next().unwrap()[0].to_string();

            let mut ingrediants_list: Vec<Ingrediant> = Vec::new();
            
            while let ( Some( ingrediant_num ), Some( ingrediant_comp ) ) = ( num_cap.next(), comp_cap.next() )  {
                ingrediants_list.push( to_ingrediant( current_comp, current_num ) );

                current_num  = ingrediant_num[0].parse::<i64>().unwrap();
                current_comp = ingrediant_comp[0].to_string();
            }

            let key = to_ingrediant( current_comp, current_num );
            ret.insert( key, ingrediants_list );
        });

    return ret;
}

// fn get_ingrediants_to_ore( ingrediant: Ingrediant, recipe: BTreeMap<Ingrediant, Vec<Ingrediant>> ) -> i64 {
//     // if compound == "ORE" {
//     //     // return the current value? 
//     // } else {
//     //     // recursively find the ore 

//     //     // TODO: work out the amount, only use the compound as the key
//     // }

//     if ingrediant.compound == "ORE" {
//         return ingrediant.amount;
//     }

//     let mut amount_ore_required = 0;

//     let new_ingrediants = get_ingrediants_to_ore_two( ingrediant, recipe.clone() );

//     // this following line is important
//     // let required_amount = ( ingrediant.amount + ( ( entry.0.amount - ( ingrediant.amount - entry.0.amount ) ).abs() % entry.0.amount ) ) / entry.0.amount;

//     // ingrediants.extend(new_ingrediants);

//     for new_ingrediant in new_ingrediants {
//         // let temp = get_ingrediants_to_ore(new_ingrediant.clone(), recipe.clone() );
//         // amount_ore_required += temp * required_amount;
//         // println!( "{}", new_ingrediant.clone().compound );
//         let temp = lookup_amount( new_ingrediant, recipe.clone() );
//         amount_ore_required += temp;
//     }

//     return amount_ore_required;
// }

// fn lookup_amount( ingrediant: Ingrediant, recipe: BTreeMap<Ingrediant, Vec<Ingrediant>> ) -> i64 {
//     if ingrediant.compound == "ORE" {
//         return ingrediant.amount;
//     }

//     let mut amount_ore_required = 0;

//     let entry = recipe.iter()
//         .find( |(k, _v)| k.compound == ingrediant.compound ).unwrap();

//     // TODO: how to clean up
//     let mut mod_val = ingrediant.amount % entry.0.amount;

//     if mod_val == 0 {
//         mod_val = entry.0.amount;
//     }

//     let required_amount = ( ingrediant.amount + ( entry.0.amount - ( mod_val ) ) ) / entry.0.amount;

//     let new_ingrediants = entry.1;

//     for new_ingrediant in new_ingrediants {
//         let temp = lookup_amount(new_ingrediant.clone(), recipe.clone() );
//         amount_ore_required += temp * required_amount;
//     }

//     return amount_ore_required;
// }

// fn get_ingrediants_to_ore_two( ingrediant: Ingrediant, recipe: BTreeMap<Ingrediant, Vec<Ingrediant>> ) -> Vec<Ingrediant> {
//     let mut ret = Vec::<Ingrediant>::new();

//     let entry = recipe.iter()
//         .find( |(k, _v)| k.compound == ingrediant.compound ).unwrap();

//     if entry.1.get(0).unwrap().compound != "ORE" { // TODO: clean up
//         for new_ingrediant in entry.1.clone() {
//             let compounds = get_ingrediants_to_ore_two( new_ingrediant, recipe.clone() );
//             for new_compounds in compounds {
//                 if let Some( compound_to_update ) = ret.iter_mut().find( |v| v.compound == new_compounds.compound ) {
//                     // let mut update = compound_to_update.1.clone();
//                     // update.amount += ingrediant.amount;
//                     // let required_amount = ( ingrediant.amount + ( entry.0.amount - ( mod_val ) ) ) / entry.0.amount;

//                     let mut mod_val = ingrediant.amount % entry.0.amount;

//                     if mod_val == 0 {
//                         mod_val = entry.0.amount;
//                     }
                
//                     let required_amount = ( ingrediant.amount + ( entry.0.amount - ( mod_val ) ) ) / entry.0.amount;
                    
//                     compound_to_update.amount += required_amount * new_compounds.amount;
//                     // ret[compound_to_update.0] = update;
//                 } else {
//                     let mut new_compound = new_compounds.clone();

//                     let mut mod_val = ingrediant.amount % entry.0.amount;

//                     if mod_val == 0 {
//                         mod_val = entry.0.amount;
//                     }
                
//                     let required_amount = ( ingrediant.amount + ( entry.0.amount - ( mod_val ) ) ) / entry.0.amount;

//                     new_compound.amount *= required_amount; // TODO: this division isn't right
//                     ret.push( new_compound );
//                 }
//             }
//         }
//     } else {
//         let mut mod_val = ingrediant.amount % entry.0.amount;

//         if mod_val == 0 {
//             mod_val = entry.0.amount;
//         }
    
//         let required_amount = ( ingrediant.amount + ( entry.0.amount - ( mod_val ) ) ) / entry.0.amount;

//         let mut ingrediant_temp = ingrediant.clone();
//         ingrediant_temp.amount *= required_amount;
        
//         ret.push( ingrediant_temp );
//     }

//     return ret;
// }

fn get_ingrediants_to_ore( ingrediant: Ingrediant, recipe: BTreeMap<Ingrediant, Vec<Ingrediant>>, ores_list_val: Vec<Ingrediant> ) -> Vec<Ingrediant> { 

    let mut ores_list = ores_list_val.clone();

    // ores_list.push( ingrediant.clone() );

    // if ingrediant.compound == "ORE" {
    //     return ores_list_val;
    // }

    let entry = recipe.iter()
        .find( |(k, _v)| k.compound == ingrediant.compound ).unwrap();

    // check to see if current compound results in left over
    if let Some( current_value ) = ores_list.iter_mut().find( |v| v.compound == ingrediant.compound ) {
        let mut mod_val = current_value.amount % entry.0.amount;

        if mod_val == 0 {
            mod_val = entry.0.amount;
        }
        current_value.left_over = entry.0.amount - mod_val;
        current_value.amount += current_value.left_over;
    }

    for recipe_ingrediant in entry.1.clone() {
        if recipe_ingrediant.compound == "ORE" {
            continue;
        }

        // check to see if it has already been added to the list 
        if let Some( current_ore_info ) = ores_list.iter_mut().find( |v| v.compound == recipe_ingrediant.compound ) {
            let mut mod_val = ingrediant.amount % entry.0.amount;

            if mod_val == 0 {
                mod_val = entry.0.amount;
            }

            let factor = ( ingrediant.amount + ( entry.0.amount - ( mod_val ) ) ) / entry.0.amount;

            let mut required_amount = factor * recipe_ingrediant.amount;

            if recipe_ingrediant.compound == "ORE" {
                println!( "here for debugging" );
            }

            if required_amount < current_ore_info.left_over {
                current_ore_info.left_over -= required_amount;
                continue;
            }

            required_amount -= current_ore_info.left_over;

            current_ore_info.left_over = 0;
            current_ore_info.amount += required_amount;
        } else {
            let mut recipe_ingrediant_clone = recipe_ingrediant.clone();
            let mut factor = 1;

            if recipe_ingrediant.compound == "ORE" {
                println!( "here for debugging" );
            }

            // if let Some( key_ingrediant ) = recipe.iter().find( |(k, _v)| k.compound == recipe_ingrediant_clone.compound ) {
                // factor = get_factor( key_ingrediant.0.clone(), recipe_ingrediant_clone.clone() ); // TODO: try reverting this change
                let mut mod_val = ingrediant.amount % entry.0.amount;

                if mod_val == 0 {
                    mod_val = entry.0.amount;
                }
    
                factor = ( ingrediant.amount + ( entry.0.amount - ( mod_val ) ) ) / entry.0.amount;
            // }
            recipe_ingrediant_clone.amount *= factor;
            // recipe_ingrediant_clone.left_over = recipe_ingrediant_clone.amount - recipe_ingrediant.amount;
            ores_list.push( recipe_ingrediant_clone.clone() );
        }


        ores_list = get_ingrediants_to_ore( recipe_ingrediant.clone(), recipe.clone(), ores_list );
    }

    // TODO: lookup value

    return ores_list;
}

fn get_factor( key_ingrediant: Ingrediant, entry_ingrediant: Ingrediant ) -> i64 {
    let mut mod_val = entry_ingrediant.amount % key_ingrediant.amount;

    if mod_val == 0 {
        mod_val = entry_ingrediant.amount;
    } else if entry_ingrediant.amount > key_ingrediant.amount { // Does this work?
        return 1;
    }

    return ( key_ingrediant.amount + ( entry_ingrediant.amount - ( mod_val ) ) ) / key_ingrediant.amount;
}

fn calculate_ore( recipe: BTreeMap<Ingrediant, Vec<Ingrediant>>, ore_list: &mut Vec<Ingrediant> ) -> i64 {
    let mut ore_count = 0;

    while let Some( ore ) = ore_list.pop() {
        if let Some( req_ore ) = recipe.iter().find( |(k, v)| k.compound == ore.compound && v.get(0).unwrap().compound == "ORE" ) {
            let factor = ore.amount / req_ore.0.amount;
            ore_count += factor * req_ore.1.get(0).unwrap().amount;
        }
    }

    return ore_count;
}


fn part_one( ingrediants: BTreeMap<Ingrediant, Vec<Ingrediant>> ) {
    let fuel = Ingrediant{ compound: "FUEL".to_string(), amount: 1, left_over: 0 };
    let mut compound_numbers = get_ingrediants_to_ore( fuel, ingrediants.clone(), vec![] );

    println!( "Part One: required amount of ore = {}", calculate_ore( ingrediants.clone(), &mut compound_numbers ) );
}

fn to_ingrediant( compound: String, amount: i64 ) -> Ingrediant {
    Ingrediant { amount: amount, compound: compound, left_over: 0 }
}

#[allow(dead_code)]
pub fn day_14( args: Vec<String> ) {
    let data = process_data( args[1].clone() );

    part_one( data );
}