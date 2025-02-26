use owsc::{print_tabulated, sorters};

fn main() {
    let initial = vec![0, 3, 7, 1, 2, 8];
    let algorithms: Vec<(&str, Option<fn(&mut Vec<i32>)>)> = vec![
        ( "Before",    None                      ),
        ( "Fung",      Some(sorters::fung_simple)),
        ( "Hamid",     Some(sorters::hamid_gnome)),
        ( "Insertion", Some(sorters::insertion)  ),
        ( "Bubble",    Some(sorters::bubble)     ),
    ];

    let mut arrs: Vec<Vec<i32>> = Vec::with_capacity(algorithms.len());
    let mut titles: Vec<&str> = Vec::with_capacity(algorithms.len());
    for i in 0..algorithms.len() {
        titles.push(algorithms[i].0);
        arrs.push(initial.clone());
    }

    for i in 0..algorithms.len() {
        if let Some(function) = algorithms[i].1 {
            function(&mut arrs[i]);
        }
    }

    print_tabulated(&titles, &arrs);
}
