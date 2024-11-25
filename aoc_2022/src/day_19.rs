use common::{solution, Answer};
use hashbrown::HashMap;

solution!("Not Enough Minerals", 19);

fn part_a(input: &str) -> Answer {
    let robots = parse(input);

    let mut geodes = Vec::new();
    for i in robots.into_iter() {
        geodes.push(simulate(State::new(), 24, i, &mut HashMap::new(), &mut 0));
    }

    geodes
        .iter()
        .enumerate()
        .map(|(i, &e)| e as u32 * (1 + i as u32))
        .sum::<u32>()
        .into()
}

fn part_b(input: &str) -> Answer {
    let robots = parse(input);

    let mut result = 1;
    for i in robots.into_iter().take(3) {
        result *= simulate(State::new(), 32, i, &mut HashMap::new(), &mut 0) as u32;
    }

    result.into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    resources: [u8; 4],
    robots: [u8; 4],
}

impl State {
    fn new() -> Self {
        Self {
            resources: [0, 0, 0, 0],
            robots: [1, 0, 0, 0],
        }
    }

    fn tick(self) -> Self {
        let mut resources = self.resources;
        for (i, resource) in resources.iter_mut().enumerate() {
            *resource += self.robots[i];
        }

        Self { resources, ..self }
    }

    fn cant_beat(&self, ticks: u8, max_geodes: u8) -> bool {
        self.resources[3] as u32
            + (ticks as u32 * self.robots[3] as u32 + ticks as u32 * (ticks as u32 - 1) / 2)
            <= max_geodes as u32
    }
}

fn simulate(
    state: State,
    ticks: u8,
    costs: [RobotType; 4],
    cache: &mut HashMap<State, (u8, u8)>,
    max: &mut u8,
) -> u8 {
    if let Some(res) = cache.get(&state) {
        if res.0 >= ticks {
            return res.1;
        }
    }

    if ticks == 0 {
        cache.insert(state, (ticks, state.resources[3]));
        return state.resources[3];
    }

    if state.cant_beat(ticks, *max) {
        return 0;
    }

    let mut best = 0;

    for i in costs {
        let can_build = match i {
            RobotType::Ore(c) => state.resources[0] >= c,
            RobotType::Clay(c) => state.resources[0] >= c,
            RobotType::Obsidian(o, c) => state.resources[0] >= o && state.resources[1] >= c,
            RobotType::Geode(o, c) => state.resources[0] >= o && state.resources[2] >= c,
        };

        if can_build {
            let mut new_state = state.tick();
            i.build(&mut new_state.resources);
            new_state.robots[i.index()] += 1;
            best = best.max(simulate(new_state, ticks - 1, costs, cache, max));
        }
    }

    best = best.max(simulate(state.tick(), ticks - 1, costs, cache, max));
    *max = (*max).max(best);

    cache.insert(state, (ticks, best));
    best
}

#[derive(Debug, Clone, Copy)]
enum RobotType {
    // (ore cost)
    Ore(u8),
    // (ore cost)
    Clay(u8),
    // (ore cost, clay cost)
    Obsidian(u8, u8),
    // (ore cost, obsidian cost)
    Geode(u8, u8),
}

fn parse(raw: &str) -> Vec<[RobotType; 4]> {
    raw.lines().map(RobotType::parse).collect()
}

impl RobotType {
    fn parse(raw: &str) -> [Self; 4] {
        let mut out = [Self::Ore(0); 4];

        fn first_word(s: &str) -> &str {
            s.split_whitespace().next().unwrap()
        }

        for (i, e) in raw
            .split(['.', ':'])
            .skip(1)
            .filter(|x| !x.is_empty())
            .enumerate()
        {
            let cost = e.split_once("costs ").unwrap();
            match i {
                0 => out[0] = Self::Ore(first_word(cost.1).parse().unwrap()),
                1 => out[1] = Self::Clay(first_word(cost.1).parse().unwrap()),
                2 => {
                    let cost = cost.1.split_once(" and ").unwrap();
                    out[2] = Self::Obsidian(
                        first_word(cost.0).parse().unwrap(),
                        first_word(cost.1).parse().unwrap(),
                    );
                }
                3 => {
                    let cost = cost.1.split_once(" and ").unwrap();
                    out[3] = Self::Geode(
                        first_word(cost.0).parse().unwrap(),
                        first_word(cost.1).parse().unwrap(),
                    );
                }
                _ => unreachable!(),
            }
        }

        out
    }

    fn index(&self) -> usize {
        match self {
            Self::Ore(_) => 0,
            Self::Clay(_) => 1,
            Self::Obsidian(_, _) => 2,
            Self::Geode(_, _) => 3,
        }
    }

    fn build(&self, resources: &mut [u8; 4]) {
        match self {
            Self::Ore(c) => resources[0] -= c,
            Self::Clay(c) => resources[0] -= c,
            Self::Obsidian(o, c) => {
                resources[0] -= o;
                resources[1] -= c;
            }
            Self::Geode(o, c) => {
                resources[0] -= o;
                resources[2] -= c;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const CASE: &str = indoc! {"
        Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
    "};

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(CASE), 33.into());
    }
}
