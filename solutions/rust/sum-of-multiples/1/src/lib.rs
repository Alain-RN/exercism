use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut all_sets: Vec<Vec<u32>> = Vec::new();
    for factor in factors {
        all_sets.push(multiple_set( limit, *factor ));
    }

    sum_merge_set(merge_sets(all_sets))
}

fn multiple_set( limit: u32, factor: u32 ) -> Vec<u32> {
    let mut set: Vec<u32> = Vec::new();
    let mut i: u32 = factor;
    while i < limit {
        set.push(i);
        i+=factor;
    }
    set
}

fn merge_sets( all_sets: Vec<Vec< u32 >> ) -> Vec<u32> {
    let mut seen = HashSet::new();
    let mut merge: Vec<u32> = Vec::new();
    for set in all_sets {
        for e in set {
            if seen.insert(e) {
                merge.push(e);
            }
        }
    }
    merge
}

fn sum_merge_set(set: Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    for i in set {
        sum += i;
    }
    sum
}
