use std::{
    cell::OnceCell,
    cmp::Ordering,
    collections::{btree_map::Entry, BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    io::stdin,
    ops::Index,
};

const CHEAT_LENGTH: usize = 20;

use itertools::Itertools;

type Position = (usize, usize);

struct Racetrack {
    map: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
    start: Position,
    end: Position,
}

impl Index<&Position> for Racetrack {
    type Output = bool;

    fn index(&self, &(row, col): &Position) -> &Self::Output {
        &self.map[row][col]
    }
}

impl Racetrack {
    fn parse() -> Self {
        let start = OnceCell::new();
        let end = OnceCell::new();
        let map = stdin()
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.unwrap()
                    .char_indices()
                    .map(|(col, c)| match c {
                        '#' => false,
                        '.' => true,
                        'S' => {
                            start.set((row, col)).unwrap();
                            true
                        }
                        'E' => {
                            end.set((row, col)).unwrap();
                            true
                        }
                        _ => panic!(),
                    })
                    .collect_vec()
            })
            .collect_vec();
        let rows = map.len();
        let cols = map.iter().map(|row| row.len()).all_equal_value().unwrap();
        Self {
            map,
            rows,
            cols,
            start: start.into_inner().unwrap(),
            end: end.into_inner().unwrap(),
        }
    }

    fn adjacent(&self, &(row, col): &Position) -> impl Iterator<Item = Position> + '_ {
        [
            (row.wrapping_sub(1), col),
            (row, col.wrapping_sub(1)),
            (row, col + 1),
            (row + 1, col),
        ]
        .into_iter()
        .filter(|&(row, col)| row < self.rows && col < self.cols)
    }

    fn shortest_without_cheat(&self) -> usize {
        let mut queue = VecDeque::from([(self.start.clone(), 0)]);
        let mut visited = BTreeSet::from([self.start.clone()]);
        while let Some((position, picoseconds)) = queue.pop_front() {
            if position == self.end {
                return picoseconds;
            }
            for adjacent in self.adjacent(&position) {
                if self[&adjacent] && visited.insert(adjacent.clone()) {
                    queue.push_back((adjacent, picoseconds + 1));
                }
            }
        }
        panic!()
    }

    fn solve(self) -> usize {
        let shortest_without_cheat = self.shortest_without_cheat();
        let mut found_cheats = HashMap::new(); //  HashSet::new();
        loop {
            if found_cheats.len() % 100 == 0 {
                println!("{}...", found_cheats.len());
            }
            let mut queue = VecDeque::from([(self.start.clone(), vec![], 0)]);
            let mut visited =
                BTreeMap::from([((self.start.clone(), 0), (0, BTreeSet::from([vec![]])))]);
            while let Some((position, cheat, picoseconds)) = queue.pop_front() {
                if picoseconds + 50 > shortest_without_cheat {
                    for (p, count) in found_cheats
                        .iter()
                        .counts_by(|(_, &p)| p)
                        .into_iter()
                        .sorted_by_key(|&(p, _)| p)
                        .rev()
                    {
                        println!(
                            "There are {count} cheats that save {} picoseconds",
                            shortest_without_cheat - p
                        );
                    }
                    return found_cheats.len();
                }
                if position == self.end {
                    // can't spread cheats
                    // cheat can only be less than 20 in this case
                    if !found_cheats.contains_key(&cheat) {
                        found_cheats.insert(cheat, picoseconds);
                        break;
                    }
                    // the ones we aren't counting are the ones that share some path with the one we do find
                    // we're successfully finding all cheats that work, but none that aren't on a shortest path
                    // from start to end...
                    // new approach: find shortest path from S to A without cheating, and shortest path from B to E without cheating,
                    // then count the number of cheat paths from A to B
                }
                for adjacent in self.adjacent(&position) {
                    let cheat = match (
                        self[&adjacent],
                        cheat.is_empty(),
                        cheat.len().cmp(&(CHEAT_LENGTH - 1)),
                    ) {
                        // if we're currently within a cheat, we must continue to cheat
                        // if this is the last step of the cheat, self[adjacent] must be true
                        (_, false, Ordering::Less)
                        | (true, _, Ordering::Equal)
                        | (false, true, _) => {
                            let mut cheat = cheat.clone();
                            cheat.push(adjacent.clone());
                            // if cheat.len() == CHEAT_LENGTH && found_cheats.contains_key(&cheat) {
                            //     continue;
                            // }
                            cheat
                        }
                        (true, _, _) => cheat.clone(),
                        _ => continue,
                    };
                    // if visited[adjacent, cheat.len()] == picoseconds + 1 (or not visited)
                    // then insert cheat into btreeset
                    // then we'll catch all cheats that share a suffix with this one?
                    if visited.insert((adjacent.clone(), cheat.len())) {
                        queue.push_back((adjacent, cheat, picoseconds + 1));
                    }
                }
            }
        }
    }
}

fn main() {
    let result = Racetrack::parse().solve();
    println!("{result}");
}

/*
- if out of bounds, ignore
- if self[adj] then insert (pico+1, rc)
- if !self[adj] and !cheated, then
    - create rc representing 0 more than shortest, because we are the shortest
    - enqueue as cheated with (pico+1, rc)

branching structure of rc tree represents convergence of various cheat paths,
if we put our rc into another one, we still need others to be able to add to our rc
without knowledge of the parent

does rc of Paths contain its offset? we don't need to mutate its offset do we?

visited_after_cheat = Map<Position, Paths>
visited_after_cheat[adjacent] => Some(paths)
paths.add(pico - paths.pico, our_paths)
our_paths is still something that other things can come along and append to, and for that they'd need to know what pico offset they find it at
so we can't just subtract from its pico field and append, because other things are depending on it having an intact pico field?


when we encounter a neighbor we've already visited:
- if we haven't cheated yet, don't enqueue and don't modify because
  we're just finding a longer path that won't be part of a best path
- if we've already cheated and are not currently cheating, then
    - the visited lookup must contain a refcell, we append to that
    - refcells can combine... when we combine, we also have a number representing the overall distance??
      is it the overall distance or the distance after cheating? -- let's go with overall for now
      is it stored alongside the other refcells? Do refcells need this number?
    - when we encounter an already visited-after-cheating path and our overall distance is higher than its...
      we need to give it our refcells in a way that makes it aware of how much that difference is...

- situations with refcell on hand:
    - just created vs dequeued -- probably same
    - encounter visited vs don't encounter visited...
        - don't encounter visited = (pico+1, rc)
        - encounter visited = store rc into visited along with difference between our pico and its pico

- refcell owns tree and allows other things to come along and mutate it
    rc owns refcell -- if we enqueue inner so that queue owns it then other things that mutate it won't get picked up
 */
