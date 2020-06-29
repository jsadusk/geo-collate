use crate::fslope::*;
use crate::numeric::*;
use geo::prelude::BoundingRect;
use geo::{CoordinateType, Line, LineString, MultiLineString, MultiPolygon, Polygon};
use quickersort;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum CollateError {
    NoValidLinesForSweep,
    HoleWithoutOutline,
    OutlineIsHole,
    OutlineInOutline,
    EmptyPolyStack,
}

impl error::Error for CollateError {}

impl fmt::Display for CollateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoValidLinesForSweep => write!(f, "No valid lines for sweep"),
            Self::HoleWithoutOutline => write!(f, "Hole without outline"),
            Self::OutlineIsHole => write!(f, "Previous detected outline is a hole"),
            Self::OutlineInOutline => write!(f, "Outline directly inside outline"),
            Self::EmptyPolyStack => write!(f, "Polygon stack empty when trying to pop"),
        }
    }
}

pub type CollateResult<T> = Result<T, CollateError>;

pub trait Collate<T>
where
    T: CoordinateType,
{
    fn collate(&self) -> CollateResult<MultiPolygon<T>>;
    //fn collate_into(self) -> MultiPolygon;
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
        return other.maxy().partial_cmp(&self.maxy()).unwrap();
    }
}

impl<T: CoordinateType> PartialOrd for TiedLine<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return other.maxy().partial_cmp(&self.maxy());
    }
}

#[derive(Debug)]
enum UpDown {
    Up,
    Horizontal,
    Down,
}

#[derive(Debug)]
struct SweepIntersection<T>
where
    T: CoordinateType,
{
    x: T,
    direction: UpDown,
    index: usize,
}

impl<T> Collate<T> for MultiLineString<T>
where
    T: CoordinateType + Numeric + fmt::Display + fmt::Debug,
{
    fn collate(&self) -> CollateResult<MultiPolygon<T>> {
        let mut sweeps = Vec::<T>::new();
        sweeps.reserve(self.0.len());

        let mut lines = Vec::<TiedLine<T>>::new();

        for (index, ls) in self.0.iter().enumerate() {
            let rect = ls.bounding_rect().unwrap();
            sweeps.push(rect.min().y + (rect.max().y - rect.min().y).half());

            for line in ls.lines() {
                lines.push(TiedLine { line, index });
            }
        }

        quickersort::sort_by(&mut lines, &|a, b| a.miny().partial_cmp(&b.miny()).unwrap());

        quickersort::sort_by(&mut sweeps, &|a, b| a.partial_cmp(&b).unwrap());

        let mut valid_lines = BinaryHeap::new();
        let mut cur_line_iter = lines.iter().peekable();

        let mut hole_of = HashMap::<usize, usize>::new();
        let mut exteriors = HashSet::<usize>::new();

        for sweep in sweeps {
            println!("Sweep {}", sweep);

            while cur_line_iter.peek() != None && cur_line_iter.peek().unwrap().miny() <= sweep {
                println!(
                    "Adding {} {} to heap",
                    cur_line_iter.peek().unwrap().miny(),
                    cur_line_iter.peek().unwrap().maxy()
                );
                valid_lines.push(cur_line_iter.next().unwrap())
            }

            println!("Top of heap is {}", valid_lines.peek().unwrap().maxy());
            while !valid_lines.is_empty() && valid_lines.peek().unwrap().maxy() < sweep {
                println!("Popping {}", valid_lines.peek().unwrap().maxy());
                valid_lines.pop();
            }

            if valid_lines.is_empty() {
                return Err(CollateError::NoValidLinesForSweep);
            }

            for valid_line in valid_lines.iter() {
                println!(
                    "Testing intersection with {} {}",
                    valid_line.miny(),
                    valid_line.maxy()
                );
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
                        let miny = line.line.start.y;
                        let x = line.minx()
                            + T::from_float(
                                (line.line.inv_fslope()) * (sweep - miny).as_float().abs(),
                            );

                        let direction = if line.line.start.y < line.line.end.y {
                            UpDown::Up
                        } else {
                            UpDown::Down
                        };

                        println!(
                            "poly {} {:?} intersection at {} dir {:?} 1/slope {} miny {}",
                            line.index,
                            line.line,
                            x,
                            direction,
                            line.line.inv_fslope(),
                            sweep - miny
                        );
                        SweepIntersection {
                            x,
                            direction,
                            index: line.index,
                        }
                    }
                })
                .collect();
            quickersort::sort_by(
                &mut intersections,
                &|a: &SweepIntersection<T>, b: &SweepIntersection<T>| {
                    a.x.partial_cmp(&b.x).unwrap()
                },
            );

            println!("Intersections {:?}", intersections);

            let mut poly_stack = Vec::<usize>::new();
            let mut inside = false;

            for intersection in intersections {
                let last = poly_stack.last();
                println!(
                    "Inside {:?} intersection {:?} last {:?}",
                    inside, intersection, last
                );

                if !inside {
                    match intersection.direction {
                        UpDown::Up => {
                            if last == None || intersection.index != *last.unwrap() {
                                println!("Adding {} to exteriors", intersection.index);
                                exteriors.insert(intersection.index);
                            }
                            inside = true;
                        }
                        UpDown::Down => return Err(CollateError::HoleWithoutOutline),
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

        println!("Exteriors {:?}", exteriors);
        println!("Hole of {:?}", hole_of);
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
}

#[cfg(test)]
mod tests {
    use crate::collate::*;
    use geo::prelude::Translate;
    #[test]
    fn one_square() {
        let uncollated = MultiLineString::from(vec![
            (0.0, 0.0),
            (0.0, 1.0),
            (1.0, 1.0),
            (1.0, 0.0),
            (0.0, 0.0),
        ]);

        let collated = uncollated.collate().unwrap();
        assert_eq!(collated.0.len(), 1);
        assert_eq!(collated.0.first().unwrap().exterior().0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors().len(), 0);
    }

    #[test]
    fn square_hole() {
        let exterior: LineString<f64> =
            vec![(0.0, 0.0), (0.0, 3.0), (3.0, 3.0), (3.0, 0.0), (0.0, 0.0)].into();
        let hole: LineString<f64> =
            vec![(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0), (1.0, 1.0)].into();
        let uncollated: MultiLineString<f64> = (vec![exterior, hole]).into_iter().collect();
        let collated = uncollated.collate().unwrap();
        println!("Collated {:?}", collated);

        assert_eq!(collated.0.len(), 1);
        assert_eq!(collated.0.first().unwrap().exterior().0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors().len(), 1);
        assert_eq!(
            collated
                .0
                .first()
                .unwrap()
                .interiors()
                .first()
                .unwrap()
                .0
                .len(),
            5
        );
    }

    #[test]
    fn square_with_diamond_hole() {
        let exterior: LineString<f64> =
            vec![(0.0, 0.0), (0.0, 4.0), (4.0, 4.0), (4.0, 0.0), (0.0, 0.0)].into();
        let hole: LineString<f64> =
            vec![(2.0, 1.0), (3.0, 2.0), (2.0, 3.0), (1.0, 2.0), (2.0, 1.0)].into();
        let uncollated: MultiLineString<f64> = (vec![exterior, hole]).into_iter().collect();
        let collated = uncollated.collate().unwrap();
        println!("Collated {:?}", collated);

        assert_eq!(collated.0.len(), 1);
        assert_eq!(collated.0.first().unwrap().exterior().0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors().len(), 1);
        assert_eq!(
            collated
                .0
                .first()
                .unwrap()
                .interiors()
                .first()
                .unwrap()
                .0
                .len(),
            5
        );
    }

    #[test]
    fn square_two_holes() {
        let exterior: LineString<f64> =
            vec![(0.0, 0.0), (0.0, 6.0), (6.0, 6.0), (6.0, 0.0), (0.0, 0.0)].into();
        let hole1: LineString<f64> =
            vec![(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0), (1.0, 1.0)].into();
        let hole2: LineString<f64> =
            vec![(3.0, 3.0), (4.0, 3.0), (4.0, 4.0), (3.0, 4.0), (3.0, 3.0)].into();
        let uncollated: MultiLineString<f64> = (vec![exterior, hole1, hole2]).into_iter().collect();
        let collated = uncollated.collate().unwrap();
        println!("Collated {:?}", collated);

        assert_eq!(collated.0.len(), 1);
        assert_eq!(collated.0.first().unwrap().exterior().0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors().len(), 2);
        assert_eq!(collated.0.first().unwrap().interiors()[0].0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors()[1].0.len(), 5);
    }

    #[test]
    fn square_two_holes_in_line() {
        let exterior: LineString<f64> =
            vec![(0.0, 0.0), (0.0, 6.0), (6.0, 6.0), (6.0, 0.0), (0.0, 0.0)].into();
        let hole1: LineString<f64> =
            vec![(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0), (1.0, 1.0)].into();
        let hole2: LineString<f64> =
            vec![(3.0, 1.0), (4.0, 1.0), (4.0, 2.0), (3.0, 2.0), (3.0, 1.0)].into();
        let uncollated: MultiLineString<f64> = (vec![exterior, hole1, hole2]).into_iter().collect();
        let collated = uncollated.collate().unwrap();
        println!("Collated {:?}", collated);

        assert_eq!(collated.0.len(), 1);
        assert_eq!(collated.0.first().unwrap().exterior().0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors().len(), 2);
        assert_eq!(collated.0.first().unwrap().interiors()[0].0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors()[1].0.len(), 5);
    }

    #[test]
    fn two_polys_square_hole() {
        let exterior1: LineString<f64> =
            vec![(0.0, 0.0), (0.0, 3.0), (3.0, 3.0), (3.0, 0.0), (0.0, 0.0)].into();
        let hole1: LineString<f64> =
            vec![(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0), (1.0, 1.0)].into();
        let exterior2: LineString<f64> = exterior1.translate(4.0, 0.0);

        let hole2: LineString<f64> = hole1.translate(4.0, 0.0);

        let uncollated: MultiLineString<f64> = (vec![exterior1, hole1, exterior2, hole2])
            .into_iter()
            .collect();
        let collated = uncollated.collate().unwrap();
        println!("Collated {:?}", collated);

        assert_eq!(collated.0.len(), 2);
        assert_eq!(collated.0[0].exterior().0.len(), 5);
        assert_eq!(collated.0[0].interiors().len(), 1);
        assert_eq!(collated.0[0].interiors().first().unwrap().0.len(), 5);

        assert_eq!(collated.0[1].exterior().0.len(), 5);
        assert_eq!(collated.0[1].interiors().len(), 1);
        assert_eq!(collated.0[1].interiors().first().unwrap().0.len(), 5);
    }

    #[test]
    fn hole_in_line_with_sweep() {
        let exterior: LineString<f64> =
            vec![(0.0, 0.0), (0.0, 4.0), (3.0, 4.0), (3.0, 0.0), (0.0, 0.0)].into();
        let hole: LineString<f64> =
            vec![(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0), (1.0, 1.0)].into();
        let uncollated: MultiLineString<f64> = (vec![exterior, hole]).into_iter().collect();
        let collated = uncollated.collate().unwrap();
        println!("Collated {:?}", collated);

        assert_eq!(collated.0.len(), 1);
        assert_eq!(collated.0.first().unwrap().exterior().0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors().len(), 1);
        assert_eq!(
            collated
                .0
                .first()
                .unwrap()
                .interiors()
                .first()
                .unwrap()
                .0
                .len(),
            5
        );
    }

    #[test]
    fn poly_in_hole() {
        let exterior1: LineString<f64> =
            vec![(0.0, 0.0), (0.0, 6.0), (6.0, 6.0), (6.0, 0.0), (0.0, 0.0)].into();
        let hole1: LineString<f64> =
            vec![(1.0, 1.0), (5.0, 1.0), (5.0, 5.0), (1.0, 5.0), (1.0, 1.0)].into();
        let exterior2: LineString<f64> =
            vec![(2.0, 2.0), (2.0, 4.0), (4.0, 4.0), (4.0, 2.0), (2.0, 2.0)].into();
        let hole2: LineString<f64> =
            vec![(2.5, 2.5), (3.5, 2.5), (3.5, 3.5), (2.5, 3.5), (2.5, 2.5)].into();

        let uncollated: MultiLineString<f64> = (vec![exterior1, hole1, exterior2, hole2])
            .into_iter()
            .collect();
        let collated = uncollated.collate().unwrap();
        println!("Collated {:?}", collated);

        assert_eq!(collated.0.len(), 2);
        assert_eq!(collated.0[0].exterior().0.len(), 5);
        assert_eq!(collated.0[0].interiors().len(), 1);
        assert_eq!(collated.0[0].interiors().first().unwrap().0.len(), 5);

        assert_eq!(collated.0[1].exterior().0.len(), 5);
        assert_eq!(collated.0[1].interiors().len(), 1);
        assert_eq!(collated.0[1].interiors().first().unwrap().0.len(), 5);
    }

    #[test]
    fn one_square_int() {
        let uncollated = MultiLineString::from(vec![(0, 0), (0, 10), (10, 10), (10, 0), (0, 0)]);

        let collated = uncollated.collate().unwrap();
        assert_eq!(collated.0.len(), 1);
        assert_eq!(collated.0.first().unwrap().exterior().0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors().len(), 0);
    }

    #[test]
    fn one_unit_square_int() {
        let uncollated = MultiLineString::from(vec![(0, 0), (0, 1), (1, 1), (1, 0), (0, 0)]);

        let collated = uncollated.collate().unwrap();
        assert_eq!(collated.0.len(), 1);
        assert_eq!(collated.0.first().unwrap().exterior().0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors().len(), 0);
    }

    #[test]
    fn square_hole_int() {
        let exterior: LineString<i64> = vec![(0, 0), (0, 30), (30, 30), (30, 0), (0, 0)].into();
        let hole: LineString<i64> = vec![(10, 10), (20, 10), (20, 20), (10, 20), (10, 10)].into();
        let uncollated: MultiLineString<i64> = (vec![exterior, hole]).into_iter().collect();
        let collated = uncollated.collate().unwrap();
        println!("Collated {:?}", collated);

        assert_eq!(collated.0.len(), 1);
        assert_eq!(collated.0.first().unwrap().exterior().0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors().len(), 1);
        assert_eq!(
            collated
                .0
                .first()
                .unwrap()
                .interiors()
                .first()
                .unwrap()
                .0
                .len(),
            5
        );
    }

    #[test]
    fn square_with_diamond_hole_int() {
        let exterior: LineString<i64> = vec![(0, 0), (0, 40), (40, 40), (40, 0), (0, 0)].into();
        let hole: LineString<i64> = vec![(20, 10), (30, 20), (20, 30), (10, 20), (20, 10)].into();
        let uncollated: MultiLineString<i64> = (vec![exterior, hole]).into_iter().collect();
        let collated = uncollated.collate().unwrap();
        println!("Collated {:?}", collated);

        assert_eq!(collated.0.len(), 1);
        assert_eq!(collated.0.first().unwrap().exterior().0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors().len(), 1);
        assert_eq!(
            collated
                .0
                .first()
                .unwrap()
                .interiors()
                .first()
                .unwrap()
                .0
                .len(),
            5
        );
    }

    #[test]
    fn square_two_holes_int() {
        let exterior: LineString<i64> = vec![(0, 0), (0, 60), (60, 60), (60, 0), (0, 0)].into();
        let hole1: LineString<i64> = vec![(10, 10), (20, 10), (20, 20), (10, 20), (10, 10)].into();
        let hole2: LineString<i64> = vec![(30, 30), (40, 30), (40, 40), (30, 40), (30, 30)].into();
        let uncollated: MultiLineString<i64> = (vec![exterior, hole1, hole2]).into_iter().collect();
        let collated = uncollated.collate().unwrap();
        println!("Collated {:?}", collated);

        assert_eq!(collated.0.len(), 1);
        assert_eq!(collated.0.first().unwrap().exterior().0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors().len(), 2);
        assert_eq!(collated.0.first().unwrap().interiors()[0].0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors()[1].0.len(), 5);
    }

    #[test]
    fn square_two_holes_in_line_int() {
        let exterior: LineString<i64> = vec![(0, 0), (0, 60), (60, 60), (60, 0), (0, 0)].into();
        let hole1: LineString<i64> = vec![(10, 10), (20, 10), (20, 20), (10, 20), (10, 10)].into();
        let hole2: LineString<i64> = vec![(30, 10), (40, 10), (40, 20), (30, 20), (30, 10)].into();
        let uncollated: MultiLineString<i64> = (vec![exterior, hole1, hole2]).into_iter().collect();
        let collated = uncollated.collate().unwrap();
        println!("Collated {:?}", collated);

        assert_eq!(collated.0.len(), 1);
        assert_eq!(collated.0.first().unwrap().exterior().0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors().len(), 2);
        assert_eq!(collated.0.first().unwrap().interiors()[0].0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors()[1].0.len(), 5);
    }

    #[test]
    fn two_polys_square_hole_int() {
        let exterior1: LineString<i64> = vec![(0, 0), (0, 30), (30, 30), (30, 0), (0, 0)].into();
        let hole1: LineString<i64> = vec![(10, 10), (20, 10), (20, 20), (10, 20), (10, 10)].into();
        let exterior2: LineString<i64> = exterior1.translate(40, 0);

        let hole2: LineString<i64> = hole1.translate(40, 0);

        let uncollated: MultiLineString<i64> = (vec![exterior1, hole1, exterior2, hole2])
            .into_iter()
            .collect();
        let collated = uncollated.collate().unwrap();
        println!("Collated {:?}", collated);

        assert_eq!(collated.0.len(), 2);
        assert_eq!(collated.0[0].exterior().0.len(), 5);
        assert_eq!(collated.0[0].interiors().len(), 1);
        assert_eq!(collated.0[0].interiors().first().unwrap().0.len(), 5);

        assert_eq!(collated.0[1].exterior().0.len(), 5);
        assert_eq!(collated.0[1].interiors().len(), 1);
        assert_eq!(collated.0[1].interiors().first().unwrap().0.len(), 5);
    }

    #[test]
    fn hole_in_line_with_sweep_int() {
        let exterior: LineString<i64> = vec![(0, 0), (0, 40), (30, 40), (30, 0), (0, 0)].into();
        let hole: LineString<i64> = vec![(10, 10), (20, 10), (20, 20), (10, 20), (10, 10)].into();
        let uncollated: MultiLineString<i64> = (vec![exterior, hole]).into_iter().collect();
        let collated = uncollated.collate().unwrap();
        println!("Collated {:?}", collated);

        assert_eq!(collated.0.len(), 1);
        assert_eq!(collated.0.first().unwrap().exterior().0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors().len(), 1);
        assert_eq!(
            collated
                .0
                .first()
                .unwrap()
                .interiors()
                .first()
                .unwrap()
                .0
                .len(),
            5
        );
    }

    #[test]
    fn poly_in_hole_int() {
        let exterior1: LineString<i64> = vec![(0, 0), (0, 60), (60, 60), (60, 0), (0, 0)].into();
        let hole1: LineString<i64> = vec![(10, 10), (50, 10), (50, 50), (10, 50), (10, 10)].into();
        let exterior2: LineString<i64> =
            vec![(20, 20), (20, 40), (40, 40), (40, 20), (20, 20)].into();
        let hole2: LineString<i64> = vec![(25, 25), (35, 25), (35, 35), (25, 35), (25, 25)].into();

        let uncollated: MultiLineString<i64> = (vec![exterior1, hole1, exterior2, hole2])
            .into_iter()
            .collect();
        let collated = uncollated.collate().unwrap();
        println!("Collated {:?}", collated);

        assert_eq!(collated.0.len(), 2);
        assert_eq!(collated.0[0].exterior().0.len(), 5);
        assert_eq!(collated.0[0].interiors().len(), 1);
        assert_eq!(collated.0[0].interiors().first().unwrap().0.len(), 5);

        assert_eq!(collated.0[1].exterior().0.len(), 5);
        assert_eq!(collated.0[1].interiors().len(), 1);
        assert_eq!(collated.0[1].interiors().first().unwrap().0.len(), 5);
    }
}
