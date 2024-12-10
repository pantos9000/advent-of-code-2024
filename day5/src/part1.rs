use std::cmp::Ordering;

pub fn run(input: &str) -> u32 {
    let deps = PageDependencies::load_from_input(input);
    input
        .lines()
        .filter_map(PageUpdate::parse)
        .filter(|update| update.is_ok(&deps))
        .map(|update| update.get_middle_page())
        .map(u32::from)
        .sum()
}

pub struct PageUpdate(Vec<Page>);

impl PageUpdate {
    pub fn parse(line: &str) -> Option<PageUpdate> {
        let pages: Vec<_> = line.split(',').filter_map(Page::parse).collect();
        if pages.is_empty() {
            None
        } else {
            assert_eq!(pages.len() & 1, 1);
            Some(PageUpdate(pages))
        }
    }

    pub fn into_sorted(self, rules: &PageDependencies) -> Self {
        let mut v = self.0;
        v.sort_by(|a, b| rules.compare(*a, *b));
        Self(v)
    }

    pub fn is_ok(&self, rules: &PageDependencies) -> bool {
        for i in 0..(self.0.len() - 1) {
            for ii in (i + 1)..self.0.len() {
                let a = self.0[i];
                let b = self.0[ii];
                if rules.compare(a, b) == Ordering::Greater {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_middle_page(&self) -> Page {
        self.0[self.0.len() / 2]
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Page(u8);

impl From<Page> for u32 {
    fn from(value: Page) -> Self {
        value.0.into()
    }
}

impl Page {
    fn parse(s: &str) -> Option<Page> {
        Some(Self(s.parse().ok()?))
    }
}

#[derive(Debug, Default, Clone)]
pub struct PageDependencies(Vec<u128>);

impl PageDependencies {
    pub fn load_from_input(input: &str) -> Self {
        let rules = input.lines().filter_map(Rule::parse);
        Self::construct_from_rules(rules)
    }

    pub fn construct_from_rules(rules: impl Iterator<Item = Rule>) -> Self {
        let mut dependencies = vec![0; 100];
        for Rule(dep, page_num) in rules {
            let page = usize::from(page_num);
            dependencies[page] |= 1 << dep;
        }
        Self(dependencies)
    }

    pub fn compare(&self, a: Page, b: Page) -> Ordering {
        let a = usize::from(a.0);
        let b = usize::from(b.0);

        let a_deps = self.0[a];
        let b_deps = self.0[b];

        let a_before_b = (b_deps >> a) & 1 == 1;
        let b_before_a = (a_deps >> b) & 1 == 1;

        match (a_before_b, b_before_a) {
            (true, true) => unreachable!(),
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => Ordering::Equal,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Rule(u8, u8);

impl Rule {
    pub fn parse(line: &str) -> Option<Self> {
        let (a, b) = line.split_once('|')?;
        let a = a.parse().ok()?;
        let b = b.parse().ok()?;
        assert!(a < 100);
        assert!(b < 100);
        Some(Self(a, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example = "\
            47|53\n\
            97|13\n\
            97|61\n\
            97|47\n\
            75|29\n\
            61|13\n\
            75|53\n\
            29|13\n\
            97|29\n\
            53|29\n\
            61|53\n\
            97|53\n\
            61|29\n\
            47|13\n\
            75|47\n\
            97|75\n\
            47|61\n\
            75|61\n\
            47|29\n\
            75|13\n\
            53|13\n\
            \n\
            75,47,61,53,29\n\
            97,61,53,29,13\n\
            75,29,13\n\
            75,97,47,61,53\n\
            61,13,29\n\
            97,13,75,29,47\n\
        ;";
        assert_eq!(143, run(example));
    }

    #[test]
    fn test_simple() {
        let example = "\
            12|99\n\
            \n\
            99,1,12\n\
            12,2,99\n\
        ";
        assert_eq!(2, run(example));
    }
}
