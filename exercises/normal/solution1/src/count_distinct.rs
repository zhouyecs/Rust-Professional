pub fn new_count_distinct(input_str: &str) -> usize {
    // todo!()
    let mut set = std::collections::HashSet::new();
    input_str.split(',').for_each(|x| {
        set.insert(x);
    });
    set.len()
}
