use crate::part1::{PageDependencies, PageUpdate};

pub fn run(input: &str) -> u32 {
    let deps = PageDependencies::load_from_input(input);
    input
        .lines()
        .filter_map(PageUpdate::parse)
        .filter(|update| !update.is_ok(&deps))
        .map(|update| update.into_sorted(&deps))
        .map(|update| update.get_middle_page())
        .map(u32::from)
        .sum()
}
