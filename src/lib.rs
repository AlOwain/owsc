pub mod sorters {
    pub fn merge(a: &mut Vec<i32>) {
        todo!("Merge sort not been implemented yet!");
    }

    pub fn insertion(a: &mut Vec<i32>) {
        for i in 0..a.len() {
            // TODO(optimization): Insteaad of decrementing one by one, since A[0..i] is sorted, we can find the position of j using binary search.
            for j in (1..i).rev() {
                if a[j] > a[j - 1] {
                    break;
                }

                a.swap(j, j - 1);
            }
        }
    }

    pub fn bubble(a: &mut Vec<i32>) {
        for i in 0..a.len() {
            for j in i..a.len() {
                if a[i] > a[j] {
                    a.swap(i, j);
                }
            }
        }
    }

    pub fn fung_simple(a: &mut Vec<i32>) {
        for i in 0..a.len() {
            for j in 0..a.len() {
                if a[i] < a[j] {
                    a.swap(i, j);
                }
            }
        }
    }

    pub fn hamid_gnome(a: &mut Vec<i32>) {
        let mut i = 0;
        while i < a.len() - 1 {
            i = if a[i] > a[i + 1] {
                // Would (a[i], a[i+1]) = (a[i+1], a[i]) be compiled the same?
                a.swap(i, i + 1);

                // Decrement only to 0, I don't think this is necessary
                i.saturating_sub(1)
            } else {
                i + 1
            }
        }
    }
}

pub fn print_tabulated(titles: &[&str], arrs: &[Vec<i32>]) {
    let left_space: usize = titles.iter().map(|title| title.len()).max().unwrap() + 1;
    for i in 0..arrs.len() {
        println!(
            "{}:{}{:?}",
            titles[i],
            " ".repeat(left_space - titles[i].len()),
            arrs[i]
        );
    }
}
