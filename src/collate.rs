use crate::numeric::Numeric;
use geo_types::{CoordinateType, Line, LineString, MultiLineString, MultiPolygon, Polygon};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum CollateError {
    NoValidLinesForSweep,
    HoleWithoutOutline(f64, f64),
    OutlineIsHole,
    OutlineInOutline,
    EmptyPolyStack,
    IndexNotInMaps,
}

impl error::Error for CollateError {}

impl fmt::Display for CollateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoValidLinesForSweep => write!(f, "No valid lines for sweep"),
            Self::HoleWithoutOutline(sweep, intersection) => write!(
                f,
                "Hole without outline at sweep {} {}",
                sweep, intersection
            ),
            Self::OutlineIsHole => write!(f, "Previous detected outline is a hole"),
            Self::OutlineInOutline => write!(f, "Outline directly inside outline"),
            Self::EmptyPolyStack => write!(f, "Polygon stack empty when trying to pop"),
            Self::IndexNotInMaps => write!(f, "Linestring index not in exterior or interior maps"),
        }
    }
}

pub type CollateResult<T> = Result<T, CollateError>;

pub trait Collate<T>
where
    T: CoordinateType,
{
    fn collate(&self) -> CollateResult<MultiPolygon<T>>;
    fn collate_into(self) -> CollateResult<MultiPolygon<T>>;
}

#[derive(PartialEq)]
struct TiedLine<T>
where
    T: CoordinateType,
{
    line: Line<T>,
    index: usize,
}

impl<T: CoordinateType> TiedLine<T> {
    pub fn miny(&self) -> T {
        if self.line.start.y < self.line.end.y {
            self.line.start.y
        } else {
            self.line.end.y
        }
    }

    pub fn maxy(&self) -> T {
        if self.line.start.y > self.line.end.y {
            self.line.start.y
        } else {
            self.line.end.y
        }
    }

    pub fn minx(&self) -> T {
        if self.line.start.x < self.line.end.x {
            self.line.start.x
        } else {
            self.line.end.x
        }
    }

    #[allow(dead_code)]
    pub fn maxx(&self) -> T {
        if self.line.start.x > self.line.end.x {
            self.line.start.x
        } else {
            self.line.end.x
        }
    }
}

impl<T: CoordinateType> Eq for TiedLine<T> {}

impl<T: CoordinateType> Ord for TiedLine<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.maxy().partial_cmp(&self.maxy()).unwrap()
    }
}

impl<T: CoordinateType> PartialOrd for TiedLine<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.maxy().partial_cmp(&self.maxy())
    }
}

#[derive(Debug, PartialEq)]
enum UpDown {
    Up,
    Horizontal,
    Down,
}

#[derive(Debug, PartialEq)]
struct SweepIntersection<T>
where
    T: CoordinateType,
{
    x: T,
    direction: UpDown,
    index: usize,
}

#[derive(Debug)]
struct PolyRange<T>
where
    T: CoordinateType,
{
    lower: T,
    upper: T,
    index: usize,
}

fn get_poly_ranges<T>(polys: &MultiLineString<T>) -> Vec<PolyRange<T>>
where
    T: CoordinateType + PartialOrd,
{
    polys
        .0
        .iter()
        .enumerate()
        .map(|(index, ls)| {
            let miny =
                ls.0.iter()
                    .map(|l| l.y)
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();
            let maxy =
                ls.0.iter()
                    .map(|l| l.y)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();

            PolyRange {
                lower: miny,
                upper: maxy,
                index,
            }
        })
        .collect()
}

fn get_sweep_lines<T>(ranges: Vec<PolyRange<T>>) -> Vec<T>
where
    T: CoordinateType + Numeric,
{
    let mut highest_low = ranges.first().unwrap().lower;
    let mut lowest_high = ranges.first().unwrap().upper;

    let mut sweeps = Vec::<T>::new();

    for range in ranges.iter() {
        if range.lower >= lowest_high {
            sweeps.push((lowest_high - highest_low) / T::from_int(2) + highest_low);
            highest_low = range.lower;
            lowest_high = range.upper;
        } else {
            if range.lower > highest_low {
                highest_low = range.lower;
            }
            if range.upper < lowest_high {
                lowest_high = range.upper;
            }
        }
    }
    sweeps.push((lowest_high - highest_low) / T::from_int(2) + highest_low);

    sweeps
}

fn tie_lines_to_polys<T>(polys: &MultiLineString<T>) -> Vec<TiedLine<T>>
where
    T: CoordinateType,
{
    let mut lines = Vec::<TiedLine<T>>::new();

    for (index, ls) in polys.0.iter().enumerate() {
        for line in ls.lines() {
            lines.push(TiedLine { line, index });
        }
    }
    lines
}

fn get_poly_hole_map<T>(
    polys: &MultiLineString<T>,
) -> CollateResult<(HashMap<usize, usize>, HashSet<usize>)>
where
    T: CoordinateType + Numeric,
{
    let mut poly_ranges = get_poly_ranges(polys);

    poly_ranges.sort_unstable_by(|a, b| a.lower.partial_cmp(&b.lower).unwrap());

    let mut sweeps = get_sweep_lines(poly_ranges);
    sweeps.sort_unstable_by(|a, b| a.partial_cmp(&b).unwrap());

    let mut lines = tie_lines_to_polys(polys);
    lines.sort_unstable_by(|a, b| a.miny().partial_cmp(&b.miny()).unwrap());

    let mut valid_lines = BinaryHeap::new();
    let mut cur_line_iter = lines.iter().peekable();

    let mut hole_of = HashMap::<usize, usize>::new();
    let mut exteriors = HashSet::<usize>::new();

    for sweep in sweeps {
        while cur_line_iter.peek() != None {
            let peeked = &cur_line_iter.peek().unwrap();
            if peeked.miny() > sweep {
                break;
            }
            let next = cur_line_iter.next().unwrap();

            if next.maxy() >= sweep {
                valid_lines.push(next);
            }
        }

        while !valid_lines.is_empty() && valid_lines.peek().unwrap().maxy() < sweep {
            valid_lines.pop();
        }

        if valid_lines.is_empty() {
            return Err(CollateError::NoValidLinesForSweep);
        }

        let mut intersections: Vec<SweepIntersection<T>> = valid_lines
            .iter()
            .map(|line| {
                if line.line.start.y == line.line.end.y {
                    SweepIntersection {
                        x: line.minx(),
                        direction: UpDown::Horizontal,
                        index: line.index,
                    }
                } else {
                    let lefty = if line.line.start.x < line.line.end.x {
                        line.line.start.y
                    } else {
                        line.line.end.y
                    };

                    let x = line.minx() + (sweep - lefty) * line.line.dx() / line.line.dy();

                    let direction = if line.line.start.y < line.line.end.y {
                        UpDown::Up
                    } else {
                        UpDown::Down
                    };

                    SweepIntersection {
                        x,
                        direction,
                        index: line.index,
                    }
                }
            })
            .collect();
        intersections.sort_unstable_by(|a: &SweepIntersection<T>, b: &SweepIntersection<T>| {
            a.x.partial_cmp(&b.x).unwrap()
        });

        intersections.dedup();

        let mut poly_stack = Vec::<usize>::new();
        let mut inside = false;

        for intersection in intersections {
            let last = poly_stack.last();

            if !inside {
                match intersection.direction {
                    UpDown::Up => {
                        if last == None || intersection.index != *last.unwrap() {
                            exteriors.insert(intersection.index);
                        }
                        inside = true;
                    }
                    UpDown::Down => {
                        return Err(CollateError::HoleWithoutOutline(
                            sweep.as_float(),
                            intersection.x.as_float(),
                        ))
                    }
                    UpDown::Horizontal => continue,
                }
            } else {
                match intersection.direction {
                    UpDown::Up => return Err(CollateError::OutlineInOutline),
                    UpDown::Down => match poly_stack.last() {
                        None => return Err(CollateError::EmptyPolyStack),
                        Some(last) => {
                            if intersection.index != *last {
                                if hole_of.contains_key(last) {
                                    return Err(CollateError::OutlineIsHole);
                                } else {
                                    hole_of.insert(intersection.index, *last);
                                }
                            }
                            inside = false;
                        }
                    },
                    UpDown::Horizontal => continue,
                }
            }

            if last != None && intersection.index == *poly_stack.last().unwrap() {
                poly_stack.pop();
            } else {
                poly_stack.push(intersection.index);
            }
        }
    }

    Ok((hole_of, exteriors))
}

impl<T> Collate<T> for MultiLineString<T>
where
    T: CoordinateType + Numeric + fmt::Display + fmt::Debug,
{
    fn collate(&self) -> CollateResult<MultiPolygon<T>> {
        let (hole_of, exteriors) = get_poly_hole_map(self)?;

        let mut polys = HashMap::<usize, Vec<LineString<T>>>::new();
        for outer_index in exteriors {
            polys.insert(outer_index, Vec::new());
        }

        for (inner_index, outer_index) in hole_of {
            let inner = polys.get_mut(&outer_index).unwrap();
            inner.push(self.0[inner_index].clone());
        }

        Ok(MultiPolygon::from(
            polys
                .into_iter()
                .map(|(outer_index, inner)| Polygon::new(self.0[outer_index].clone(), inner))
                .collect::<Vec<Polygon<T>>>(),
        ))
    }

    fn collate_into(self) -> CollateResult<MultiPolygon<T>> {
        let (hole_of, exteriors) = get_poly_hole_map(&self)?;

        let mut polys = HashMap::<usize, Polygon<T>>::new();

        for (i, ls) in self.into_iter().enumerate() {
            if exteriors.contains(&i) {
                polys
                    .entry(i)
                    .and_modify(|poly| poly.exterior_mut(|exterior| *exterior = ls.clone()))
                    .or_insert(Polygon::<T>::new(ls, vec![]));
            } else {
                let exterior_i = hole_of.get(&i).ok_or(CollateError::IndexNotInMaps)?;
                let poly = polys
                    .entry(*exterior_i)
                    .or_insert(Polygon::<T>::new(LineString(vec![]), vec![]));
                poly.interiors_push(ls);
            }
        }

        Ok(MultiPolygon(
            polys
                .into_iter()
                .map(|(_i, p)| p)
                .collect::<Vec<Polygon<T>>>(),
        ))
    }
}
