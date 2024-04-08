pub fn sort() {
    let strings = get_random_strings();
    print_strings("unsorted: ".into(), &strings);
    let sorted: Vec<String> = get_sorted_strings(&strings);
    print_strings("sorted:   ".into(), &sorted);
}

fn get_sorted_strings(unsorted: &Vec<String>) -> Vec<String> {
    let mut sorted = vec![];
    // copy borrowed array in to-be-sorted array
    for u in unsorted {
        sorted.push(u.clone());
    }
    // apply bubble sort
    let mut keep_cycling: bool = true;
    while keep_cycling {
        keep_cycling = false;
        for i in 0..sorted.len()-1 {
            if sorted[i] > sorted[i+1] {
                let tmp = sorted[i].clone();
                sorted[i] = sorted[i+1].clone();
                sorted[i+1] = tmp;
                keep_cycling = true;
            }
        }
    }
    return sorted;
}

fn get_random_strings() -> Vec<String> {
    return vec!["Wayne".into(), "Craig".into(), "Mick".into(), "Simon".into()];
}

fn print_strings(caption: String, strings: &Vec<String>) {
    print!("{}", caption);
    for s in strings {
        print!("{} ", s);
    }
    println!("");
}

