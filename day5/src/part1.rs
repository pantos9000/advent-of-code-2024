pub fn run(input: &str) -> u32 {
    let deps = PageDependencies::load_from_input(input);
    input
        .lines()
        .filter_map(PageUpdate::parse)
        .filter(|update| update.is_ok(&deps))
        .map(|update| update.get_middle_page().0)
        .map(u32::from)
        .sum()
}

struct PageUpdate(Vec<Page>);

impl PageUpdate {
    fn parse(line: &str) -> Option<PageUpdate> {
        let pages: Vec<_> = line.split(',').filter_map(Page::parse).collect();
        if pages.is_empty() {
            None
        } else {
            assert_eq!(pages.len() & 1, 1);
            Some(PageUpdate(pages))
        }
    }
}

impl PageUpdate {
    fn is_ok(&self, rules: &PageDependencies) -> bool {
        for i in 0..(self.0.len() - 1) {
            for ii in (i + 1)..self.0.len() {
                let a = self.0[i];
                let b = self.0[ii];
                if rules.a_before_b(b, a) {
                    return false;
                }
            }
        }
        true
    }

    fn get_middle_page(&self) -> &Page {
        &self.0[self.0.len() / 2]
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Page(u8);

impl Page {
    fn parse(s: &str) -> Option<Page> {
        Some(Self(s.parse().ok()?))
    }
}

#[derive(Debug, Default, Clone)]
struct PageDependencies(Vec<u128>);

impl PageDependencies {
    fn load_from_input(input: &str) -> Self {
        let rules = input.lines().filter_map(Rule::parse);
        Self::construct_from_rules(rules)
    }

    fn construct_from_rules(rules: impl Iterator<Item = Rule>) -> Self {
        let mut dependencies = vec![0; 100];
        for Rule(dep, page_num) in rules {
            let page = usize::from(page_num);
            dependencies[page] |= 1 << dep;
        }
        Self(dependencies)
    }

    fn a_before_b(&self, a: Page, b: Page) -> bool {
        let a = usize::from(a.0);
        let b = usize::from(b.0);

        let b_deps = self.0[b];

        (b_deps >> a) & 1 != 0
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Rule(u8, u8);

impl Rule {
    fn parse(line: &str) -> Option<Self> {
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
