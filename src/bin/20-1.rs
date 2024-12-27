use std::{
    cell::OnceCell,
    collections::{btree_map::Entry, BTreeMap, VecDeque},
    io::stdin,
    ops::Index,
};

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

    fn solve(self) -> usize {
        let mut queue = VecDeque::from([(self.start.clone(), None, 0)]);
        let mut visited = BTreeMap::from([(self.start.clone(), BTreeMap::from([(None, 0)]))]);
        while let Some((position, cheat, picoseconds)) = queue.pop_front() {
            if position == self.end && cheat.is_none() {
                return visited[&self.end]
                    .iter()
                    .filter(|&(_, &p)| p + 100 <= picoseconds)
                    .count();
            }
            for adjacent in self.adjacent(&position) {
                if let Some(cheat) = if self[&adjacent] {
                    Some(cheat)
                } else if cheat.is_none() {
                    Some(Some(adjacent.clone()))
                } else {
                    None
                } {
                    if let Entry::Vacant(vacant) = visited
                        .entry(adjacent.clone())
                        .or_default()
                        .entry(cheat.clone())
                    {
                        vacant.insert(picoseconds + 1);
                        queue.push_back((adjacent, cheat, picoseconds + 1));
                    }
                }
            }
        }
        panic!()
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
