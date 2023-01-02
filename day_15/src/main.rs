use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: &Pos) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn dist_from_row(&self, row: i64) -> u64 {
        self.y.abs_diff(row)
    }

    fn segment_at_row(&self, beacon: &Pos, row: i64) -> Option<Segment> {
        let dist_from_beacon= self.dist(beacon);
        let dist_from_row = self.dist_from_row(row);
        if dist_from_row > dist_from_beacon {
            None
        } else {
            let delta = dist_from_beacon - dist_from_row;
            Some(Segment::new(self.x - delta as i64, self.x + delta as i64))
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Segment {
    start: i64,
    end: i64
}

impl Segment {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    fn len(&self) -> u64 {
        (self.end - self.start + 1) as u64
    }

    fn overlaps(&self, other: &Segment) -> bool {
        (self.start <= other.start && self.end >= other.start) ||
            (other.start <= self.start && other.end >= self.start)
    }

    // Requires self.overlaps(other)
    fn merge(&self, other: &Segment) -> Segment {
        let start = min(self.start, other.start);
        let end = max(self.end, other.end);
        Segment::new(start, end)
    }

    fn merge_vec(segments: &mut Vec<Segment>) -> Vec<Segment> {
        let len = segments.len();
        segments.sort_by(|s1, s2| s1.start.cmp(&s2.start));

        let mut res = Vec::new();
        let mut current_seg = segments[0];
        for idx in 1..len {
            let next_seg = segments[idx];
            if current_seg.overlaps(&next_seg) {
                current_seg = current_seg.merge(&next_seg);
                if idx == len - 1 {
                    res.push(current_seg);
                }
            } else {
                res.push(current_seg);
                current_seg = next_seg;
                if idx == len - 1 {
                    res.push(next_seg);
                }
            }
        }

        res
    }
}

fn parse_input() -> Vec<(Pos, Pos)> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    lines.map(|line| {
        let line = line.unwrap();
        let mut split = line.split_whitespace();

        let sx = split.nth(2).unwrap();
        let sx = sx
            .replace("x=", "")
            .replace(",", "")
            .parse::<i64>().unwrap();

        let sy = split.nth(0).unwrap();
        let sy = sy
            .replace("y=", "")
            .replace(":", "")
            .parse::<i64>().unwrap();

        let bx = split.nth(4).unwrap();
        let bx = bx
            .replace("x=", "")
            .replace(",", "")
            .parse::<i64>().unwrap();

        let by = split.nth(0).unwrap();
        let by = by
            .replace("y=", "")
            .replace("", "")
            .parse::<i64>().unwrap();

        (Pos::new(sx, sy), Pos::new(bx, by))
    }).collect()
}

fn segments_at_row(input: &Vec<(Pos, Pos)>, row: i64) -> Vec<Segment> {
    let mut segments: Vec<Segment> = input.iter().filter_map(|&(sensor, beacon)| {
        sensor.segment_at_row(&beacon, row)
    }).collect();

    let merged_segments = Segment::merge_vec(&mut segments);
    merged_segments
}

fn main() {
    let input = parse_input();

    // Part 1
    // TODO: -1 is hardcoded, should be the existing beacon
    let segments = segments_at_row(&input, 2000000);
    let count = segments.iter()
        .map(|seg| seg.len())
        .sum::<u64>() - 1;
    println!("Positions that cannot contain a beacon: {}", count);

    // TODO: shame on me for iterating over every possible row!
    for row in 0..4000001 {
        if row % 100000 == 0 {
            println!("... row {} ...", row);
        }
        let segments = segments_at_row(&input, row);
        let mut segments: Vec<_> = segments.iter().filter(|s| {
            s.end > 0 && s.start <= 4000000
        }).collect();
        if segments.len() > 1 {
            segments.sort_by(|s1, s2| s1.start.cmp(&s2.start));
            let x = segments[0].end + 1;
            let y = row;
            let freq = x * 4000000 + y;
            println!("Found! x= {}, y={}, tuning frequency={}", x, y, freq);
        }
    }
}
