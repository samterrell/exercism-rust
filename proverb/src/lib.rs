pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        String::new()
    } else {
        list.windows(2)
            .map(|args| format!("For want of a {} the {} was lost.\n", args[0], args[1]))
            .chain(Some(format!("And all for the want of a {}.", list[0])))
            .reduce(|l, r| l + &r)
            .unwrap()
    }
}
