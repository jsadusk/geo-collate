#[cfg(test)]
mod test {
    use crate::collate::*;
    use geo::prelude::Translate;
    use geo_types::{Coordinate, LineString, MultiLineString};

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
        let collated2 = uncollated.collate_into().unwrap();

        assert_eq!(collated.0.len(), 1);
        assert_eq!(collated.0.first().unwrap().exterior().0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors().len(), 2);
        assert_eq!(collated.0.first().unwrap().interiors()[0].0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors()[1].0.len(), 5);

        assert_eq!(collated2.0.len(), 1);
        assert_eq!(collated2.0.first().unwrap().exterior().0.len(), 5);
        assert_eq!(collated2.0.first().unwrap().interiors().len(), 2);
        assert_eq!(collated2.0.first().unwrap().interiors()[0].0.len(), 5);
        assert_eq!(collated2.0.first().unwrap().interiors()[1].0.len(), 5);
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

        assert_eq!(collated.0.len(), 1);
        assert_eq!(collated.0.first().unwrap().exterior().0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors().len(), 2);
        assert_eq!(collated.0.first().unwrap().interiors()[0].0.len(), 5);
        assert_eq!(collated.0.first().unwrap().interiors()[1].0.len(), 5);
    }

    #[test]
    fn square_two_holes_overlapping_range() {
        let exterior: LineString<f64> =
            vec![(0.0, 0.0), (0.0, 6.0), (6.0, 6.0), (6.0, 0.0), (0.0, 0.0)].into();
        let hole1: LineString<f64> =
            vec![(1.0, 1.0), (2.0, 1.0), (2.0, 3.0), (1.0, 3.0), (1.0, 1.0)].into();
        let hole2: LineString<f64> =
            vec![(3.0, 2.0), (4.0, 2.0), (4.0, 4.0), (3.0, 4.0), (3.0, 2.0)].into();
        let uncollated: MultiLineString<f64> = (vec![exterior, hole1, hole2]).into_iter().collect();
        let collated = uncollated.collate().unwrap();

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

        assert_eq!(collated.0.len(), 2);
        assert_eq!(collated.0[0].exterior().0.len(), 5);
        assert_eq!(collated.0[0].interiors().len(), 1);
        assert_eq!(collated.0[0].interiors().first().unwrap().0.len(), 5);

        assert_eq!(collated.0[1].exterior().0.len(), 5);
        assert_eq!(collated.0[1].interiors().len(), 1);
        assert_eq!(collated.0[1].interiors().first().unwrap().0.len(), 5);
    }

    #[test]
    fn octopus_layer() {
        let layer = MultiLineString(vec![LineString(vec![
            Coordinate {
                x: -0.06148776779719839,
                y: 21.08726538461537,
            },
            Coordinate {
                x: 0.010888609340072575,
                y: 20.83853598277212,
            },
            Coordinate {
                x: 0.01352752935146273,
                y: 20.83109032889585,
            },
            Coordinate {
                x: 0.7007118528744496,
                y: 20.867004577589256,
            },
            Coordinate {
                x: 1.7651915109257719,
                y: 20.774226781933663,
            },
            Coordinate {
                x: 3.073542794187464,
                y: 20.60272918136909,
            },
            Coordinate {
                x: 3.8929185744680854,
                y: 20.352614361702127,
            },
            Coordinate {
                x: 4.49005372582205,
                y: 20.17874095744681,
            },
            Coordinate {
                x: 5.2982631296572285,
                y: 19.7519105296343,
            },
            Coordinate {
                x: 6.200597303679927,
                y: 19.369241803278687,
            },
            Coordinate {
                x: 6.812109696610124,
                y: 18.711550355977455,
            },
            Coordinate {
                x: 7.612515777891643,
                y: 18.042605324829427,
            },
            Coordinate {
                x: 8.193687496580027,
                y: 17.00482975376197,
            },
            Coordinate {
                x: 8.913517761348091,
                y: 15.498490355677156,
            },
            Coordinate {
                x: 9.172079470141991,
                y: 14.402422961844634,
            },
            Coordinate {
                x: 9.930007194544956,
                y: 12.595277065605272,
            },
            Coordinate {
                x: 10.138493225240707,
                y: 11.589890444044403,
            },
            Coordinate {
                x: 10.472352601178299,
                y: 10.554994374437443,
            },
            Coordinate {
                x: 12.340197833783378,
                y: 9.52010015926138,
            },
            Coordinate {
                x: 12.969037033492823,
                y: 8.966239602870813,
            },
            Coordinate {
                x: 14.187640299959465,
                y: 8.939444052806131,
            },
            Coordinate {
                x: 15.286206615322254,
                y: 8.762651024836938,
            },
            Coordinate {
                x: 16.55244514056225,
                y: 8.853177662285507,
            },
            Coordinate {
                x: 17.236801365461847,
                y: 8.895279670317635,
            },
            Coordinate {
                x: 18.7936282626539,
                y: 9.148506849023754,
            },
            Coordinate {
                x: 19.012936060191517,
                y: 9.224552410396717,
            },
            Coordinate {
                x: 19.25887409028728,
                y: 9.39700782862828,
            },
            Coordinate {
                x: 20.426542608695648,
                y: 10.175913671936756,
            },
            Coordinate {
                x: 21.00605789029536,
                y: 10.538736660529343,
            },
            Coordinate {
                x: 21.114733161520483,
                y: 10.608078822733424,
            },
            Coordinate {
                x: 21.989499144542773,
                y: 12.292867050147493,
            },
            Coordinate {
                x: 22.186401318848866,
                y: 12.702478860936408,
            },
            Coordinate {
                x: 22.15583181126332,
                y: 13.518502511415525,
            },
            Coordinate {
                x: 22.03867094887179,
                y: 16.13062137733142,
            },
            Coordinate {
                x: 21.77238904,
                y: 17.063638,
            },
            Coordinate {
                x: 21.188697432744043,
                y: 19.157146848578016,
            },
            Coordinate {
                x: 20.929558710717163,
                y: 20.23260777598711,
            },
            Coordinate {
                x: 20.49831397324941,
                y: 22.25549319433517,
            },
            Coordinate {
                x: 20.477551548117155,
                y: 23.447674351464435,
            },
            Coordinate {
                x: 20.455800901639346,
                y: 25.38585393442623,
            },
            Coordinate {
                x: 20.82074742857143,
                y: 26.474966408163265,
            },
            Coordinate {
                x: 21.032764362730294,
                y: 27.166926910299004,
            },
            Coordinate {
                x: 21.606130629139074,
                y: 28.170946688741722,
            },
            Coordinate {
                x: 21.97977507346189,
                y: 28.83267777777778,
            },
            Coordinate {
                x: 23.076982200230148,
                y: 29.828628396624474,
            },
            Coordinate {
                x: 23.795319546247818,
                y: 30.472989965095987,
            },
            Coordinate {
                x: 25.01754561018437,
                y: 31.07793937664618,
            },
            Coordinate {
                x: 25.8657391598916,
                y: 31.492427235772357,
            },
            Coordinate {
                x: 26.925092752962627,
                y: 31.5383112579763,
            },
            Coordinate {
                x: 27.687997134935305,
                y: 31.581126340110906,
            },
            Coordinate {
                x: 28.657738030888034,
                y: 31.405857335907335,
            },
            Coordinate {
                x: 29.888076387536515,
                y: 31.19783169425511,
            },
            Coordinate {
                x: 30.77959560669456,
                y: 30.667062343096234,
            },
            Coordinate {
                x: 31.824922842105263,
                y: 30.022619157894738,
            },
            Coordinate {
                x: 32.384647458563535,
                y: 29.308962707182317,
            },
            Coordinate {
                x: 32.930236932070144,
                y: 28.62972915742794,
            },
            Coordinate {
                x: 33.40243915775401,
                y: 27.417105882352942,
            },
            Coordinate {
                x: 33.78587905982906,
                y: 26.456192735042737,
            },
            Coordinate {
                x: 33.804014294330514,
                y: 25.554209589867313,
            },
            Coordinate {
                x: 33.802290403903164,
                y: 24.424874048913043,
            },
            Coordinate {
                x: 33.55289204951857,
                y: 23.805794841815683,
            },
            Coordinate {
                x: 33.13353828571428,
                y: 22.869603714285713,
            },
            Coordinate {
                x: 31.844447168271582,
                y: 21.56045053859964,
            },
            Coordinate {
                x: 31.589993333333332,
                y: 21.265352500000002,
            },
            Coordinate {
                x: 29.89847020202021,
                y: 20.314774242424246,
            },
            Coordinate {
                x: 29.776206603773584,
                y: 20.24805471698113,
            },
            Coordinate {
                x: 29.683012025316447,
                y: 20.240690506329113,
            },
            Coordinate {
                x: 27.497459345794393,
                y: 20.127992056074767,
            },
            Coordinate {
                x: 26.892066022099442,
                y: 20.369673756906078,
            },
            Coordinate {
                x: 25.738433644859814,
                y: 20.8454285046729,
            },
            Coordinate {
                x: 25.502154292671616,
                y: 21.09019107142857,
            },
            Coordinate {
                x: 25.154493258426967,
                y: 21.451698876404492,
            },
            Coordinate {
                x: 25.56378841724942,
                y: 21.153132307692307,
            },
            Coordinate {
                x: 25.827658018867925,
                y: 21.004933490566035,
            },
            Coordinate {
                x: 26.50416591836735,
                y: 20.757312857142853,
            },
            Coordinate {
                x: 27.576205458813487,
                y: 20.389938262910796,
            },
            Coordinate {
                x: 28.081310122574056,
                y: 20.45590767790262,
            },
            Coordinate {
                x: 29.60730164319249,
                y: 20.763275352112675,
            },
            Coordinate {
                x: 29.648015175718854,
                y: 20.79281517571885,
            },
            Coordinate {
                x: 29.730729591836745,
                y: 20.85393265306123,
            },
            Coordinate {
                x: 30.96186148325359,
                y: 21.70242033492823,
            },
            Coordinate {
                x: 31.18817417218543,
                y: 21.822496854304635,
            },
            Coordinate {
                x: 31.365036702127657,
                y: 22.035047872340424,
            },
            Coordinate {
                x: 32.39781927272727,
                y: 23.247991276595744,
            },
            Coordinate {
                x: 32.50551139664804,
                y: 23.599085754189943,
            },
            Coordinate {
                x: 32.822329282814614,
                y: 24.42639303112314,
            },
            Coordinate {
                x: 32.820663370744484,
                y: 25.020313511659808,
            },
            Coordinate {
                x: 32.77088164301552,
                y: 26.185701463414635,
            },
            Coordinate {
                x: 32.50398729726715,
                y: 26.902169754601225,
            },
            Coordinate {
                x: 32.04126911111111,
                y: 27.987764555555557,
            },
            Coordinate {
                x: 31.647999944506104,
                y: 28.46753923418424,
            },
            Coordinate {
                x: 31.127455396825397,
                y: 29.0939826984127,
            },
            Coordinate {
                x: 30.44923255813954,
                y: 29.55751353065539,
            },
            Coordinate {
                x: 29.682444412264235,
                y: 29.995819911504427,
            },
            Coordinate {
                x: 28.69701233880266,
                y: 30.2022816097561,
            },
            Coordinate {
                x: 27.790147198879552,
                y: 30.359385294117647,
            },
            Coordinate {
                x: 26.961438682745825,
                y: 30.306906586270873,
            },
            Coordinate {
                x: 26.2434147166362,
                y: 30.25297851919561,
            },
            Coordinate {
                x: 25.37167597876576,
                y: 29.835370437956204,
            },
            Coordinate {
                x: 24.604307272727272,
                y: 29.44911819787986,
            },
            Coordinate {
                x: 23.937540303030303,
                y: 28.77823205128205,
            },
            Coordinate {
                x: 23.344010996369814,
                y: 28.19831873406967,
            },
            Coordinate {
                x: 22.910568189581554,
                y: 27.39731293766012,
            },
            Coordinate {
                x: 22.587128661087867,
                y: 26.794778870292888,
            },
            Coordinate {
                x: 22.38626590985485,
                y: 25.92412142857143,
            },
            Coordinate {
                x: 22.229949829545454,
                y: 25.29813125,
            },
            Coordinate {
                x: 22.331526400274786,
                y: 23.55394084802687,
            },
            Coordinate {
                x: 22.396756209856232,
                y: 22.45136526357199,
            },
            Coordinate {
                x: 22.718026916167663,
                y: 21.438030538922153,
            },
            Coordinate {
                x: 23.399811080025117,
                y: 19.482226362240983,
            },
            Coordinate {
                x: 23.740263903080393,
                y: 18.613442011019284,
            },
            Coordinate {
                x: 24.60390506437768,
                y: 16.474625250357654,
            },
            Coordinate {
                x: 24.92816869801316,
                y: 12.659147962052003,
            },
            Coordinate {
                x: 24.173935210402608,
                y: 9.956412491091061,
            },
            Coordinate {
                x: 24.13456312396694,
                y: 9.8310492446281,
            },
            Coordinate {
                x: 23.578766363636362,
                y: 9.208967285714282,
            },
            Coordinate {
                x: 21.760740188045666,
                y: 7.283062592832285,
            },
            Coordinate {
                x: 21.313857405824532,
                y: 6.828671995115697,
            },
            Coordinate {
                x: 19.744859045792914,
                y: 5.8500657829912015,
            },
            Coordinate {
                x: 18.805989032258065,
                y: 5.2491379914279275,
            },
            Coordinate {
                x: 17.470490517023958,
                y: 4.467765943979518,
            },
            Coordinate {
                x: 16.31292654056326,
                y: 3.7327158415300543,
            },
            Coordinate {
                x: 14.987631483474251,
                y: 3.0072801050241074,
            },
            Coordinate {
                x: 13.63669804765565,
                y: 2.156483744811683,
            },
            Coordinate {
                x: 13.273114880860875,
                y: 1.9496852820906998,
            },
            Coordinate {
                x: 12.252034990964944,
                y: 1.3257203411637153,
            },
            Coordinate {
                x: 12.061943194795807,
                y: 1.0096926760291751,
            },
            Coordinate {
                x: 11.434426101694914,
                y: 0.548328214329738,
            },
            Coordinate {
                x: 12.049012598225602,
                y: 0.09493810424780114,
            },
            Coordinate {
                x: 12.238187249683143,
                y: -0.10339011314667595,
            },
            Coordinate {
                x: 14.16732037735849,
                y: -0.4584178799313895,
            },
            Coordinate {
                x: 14.687316603773585,
                y: -0.6179386974271013,
            },
            Coordinate {
                x: 17.763016335282654,
                y: -0.5705853020142951,
            },
            Coordinate {
                x: 18.43331769980507,
                y: -0.5793399651899109,
            },
            Coordinate {
                x: 20.956411442406058,
                y: -0.21543464658664663,
            },
            Coordinate {
                x: 21.075134246061516,
                y: -0.19698205843960997,
            },
            Coordinate {
                x: 23.040759272727264,
                y: 0.4511722563636331,
            },
            Coordinate {
                x: 23.19587550387597,
                y: 0.49963446779422105,
            },
            Coordinate {
                x: 23.205687111111114,
                y: 0.5039109645656565,
            },
            Coordinate {
                x: 24.810370080744754,
                y: 1.2419652641873276,
            },
            Coordinate {
                x: 24.936022249431144,
                y: 1.341469282479142,
            },
            Coordinate {
                x: 26.290122921348313,
                y: 2.5278789662921346,
            },
            Coordinate {
                x: 27.724430976658475,
                y: 4.114721394963144,
            },
            Coordinate {
                x: 27.945226017615603,
                y: 4.351800262661214,
            },
            Coordinate {
                x: 29.50286204625763,
                y: 6.00793946321876,
            },
            Coordinate {
                x: 29.70316395348837,
                y: 6.217383340380549,
            },
            Coordinate {
                x: 31.6052374251497,
                y: 7.760636314643439,
            },
            Coordinate {
                x: 31.772148468786806,
                y: 7.8958668189313626,
            },
            Coordinate {
                x: 33.4417129827832,
                y: 8.82275788814563,
            },
            Coordinate {
                x: 33.53797617901101,
                y: 8.877894686460808,
            },
            Coordinate {
                x: 35.9828087890053,
                y: 9.69223429582066,
            },
            Coordinate {
                x: 36.01735785202864,
                y: 9.704519049251465,
            },
            Coordinate {
                x: 36.93926000000002,
                y: 9.937350787878794,
            },
            Coordinate {
                x: 37.54194852459016,
                y: 10.085045628415301,
            },
            Coordinate {
                x: 37.559828787878786,
                y: 10.084569672131149,
            },
            Coordinate {
                x: 40.24268772204806,
                y: 10.03719827586207,
            },
            Coordinate {
                x: 40.28153394636015,
                y: 10.027840495646117,
            },
            Coordinate {
                x: 41.77933794676806,
                y: 9.681891956100932,
            },
            Coordinate {
                x: 43.401696170212766,
                y: 8.867055127659574,
            },
            Coordinate {
                x: 43.45783201160542,
                y: 8.838263605415861,
            },
            Coordinate {
                x: 44.73855283442793,
                y: 7.794885079847909,
            },
            Coordinate {
                x: 44.78137558935361,
                y: 7.759170283097131,
            },
            Coordinate {
                x: 46.00845375,
                y: 5.556670755681818,
            },
            Coordinate {
                x: 46.09560285714285,
                y: 5.201223428571447,
            },
            Coordinate {
                x: 46.409446776859504,
                y: 3.8459839559228652,
            },
            Coordinate {
                x: 46.40417259720063,
                y: 3.774479371695178,
            },
            Coordinate {
                x: 46.32473324675325,
                y: 2.046916139941691,
            },
            Coordinate {
                x: 46.26792260504202,
                y: 1.8490084705882346,
            },
            Coordinate {
                x: 45.84123174336533,
                y: 0.2830372484472051,
            },
            Coordinate {
                x: 45.6397089010989,
                y: -0.04822107692307773,
            },
            Coordinate {
                x: 44.895276033057854,
                y: -1.290085685950413,
            },
            Coordinate {
                x: 44.452212929292926,
                y: -1.7099684040404046,
            },
            Coordinate {
                x: 43.63621501284934,
                y: -2.4977271519434625,
            },
            Coordinate {
                x: 42.71382027164686,
                y: -2.8895553005093375,
            },
            Coordinate {
                x: 42.021440099673484,
                y: -3.146603387523629,
            },
            Coordinate {
                x: 40.313682535211264,
                y: -3.1216282816901404,
            },
            Coordinate {
                x: 39.76690159929008,
                y: -3.1409050498915394,
            },
            Coordinate {
                x: 38.41420016393443,
                y: -2.7361684918032796,
            },
            Coordinate {
                x: 37.78954163265306,
                y: -2.521901061224489,
            },
            Coordinate {
                x: 37.004571937229436,
                y: -1.8467063333333287,
            },
            Coordinate {
                x: 36.11590434275185,
                y: -1.0443628648648646,
            },
            Coordinate {
                x: 35.4777408,
                y: 0.37502871701818696,
            },
            Coordinate {
                x: 35.348282448979596,
                y: 0.6730294510204083,
            },
            Coordinate {
                x: 35.32911,
                y: 0.8354519136363611,
            },
            Coordinate {
                x: 35.179274059775835,
                y: 1.98016295890411,
            },
            Coordinate {
                x: 35.1833274131016,
                y: 1.9995644999999997,
            },
            Coordinate {
                x: 35.1889990976431,
                y: 2.0030520208754203,
            },
            Coordinate {
                x: 35.194963643147894,
                y: 1.9992811044776118,
            },
            Coordinate {
                x: 35.20239685415304,
                y: 1.9796570359712236,
            },
            Coordinate {
                x: 35.368704966442955,
                y: 0.8430525854179354,
            },
            Coordinate {
                x: 35.394885088293,
                y: 0.6872540276651405,
            },
            Coordinate {
                x: 35.99148641975309,
                y: -0.5391715387766589,
            },
            Coordinate {
                x: 36.22007639657444,
                y: -0.9858103509222663,
            },
            Coordinate {
                x: 37.053937611940306,
                y: -1.5795664613297182,
            },
            Coordinate {
                x: 37.97409768115942,
                y: -2.1419871304347833,
            },
            Coordinate {
                x: 38.2161711969112,
                y: -2.1659966486486493,
            },
            Coordinate {
                x: 38.47635833333334,
                y: -2.1883396666666677,
            },
            Coordinate {
                x: 39.5894663670412,
                y: -2.3286056779026225,
            },
            Coordinate {
                x: 39.869493057851244,
                y: -2.395053371900827,
            },
            Coordinate {
                x: 40.27501322800194,
                y: -2.3360116363636374,
            },
            Coordinate {
                x: 41.69292425696075,
                y: -2.133550583025831,
            },
            Coordinate {
                x: 42.185477159763316,
                y: -1.8576483944773186,
            },
            Coordinate {
                x: 42.75553809694794,
                y: -1.568571339317774,
            },
            Coordinate {
                x: 43.35626628803245,
                y: -0.9338686737967923,
            },
            Coordinate {
                x: 43.734110989528,
                y: -0.4944010235847626,
            },
            Coordinate {
                x: 44.24983444846293,
                y: 0.500327599901363,
            },
            Coordinate {
                x: 44.42846730644935,
                y: 0.8720306662919768,
            },
            Coordinate {
                x: 44.68988003338898,
                y: 1.9413655225375614,
            },
            Coordinate {
                x: 44.73777311953353,
                y: 2.165594565597668,
            },
            Coordinate {
                x: 44.79438872670807,
                y: 3.3254998260869555,
            },
            Coordinate {
                x: 44.79598586445366,
                y: 3.43459397749277,
            },
            Coordinate {
                x: 44.551636197387516,
                y: 4.579184494920174,
            },
            Coordinate {
                x: 44.54376824468085,
                y: 4.608298744680852,
            },
            Coordinate {
                x: 44.088449173553705,
                y: 5.486198931129499,
            },
            Coordinate {
                x: 43.598063178294574,
                y: 6.4572989478505995,
            },
            Coordinate {
                x: 42.52226148510668,
                y: 7.3128364237025565,
            },
            Coordinate {
                x: 42.50638091402014,
                y: 7.325229593127244,
            },
            Coordinate {
                x: 41.365619259259255,
                y: 7.990255451178451,
            },
            Coordinate {
                x: 40.21929826197005,
                y: 8.30932831301413,
            },
            Coordinate {
                x: 40.191910481614286,
                y: 8.3105873387471,
            },
            Coordinate {
                x: 38.373479641465316,
                y: 8.380649535109473,
            },
            Coordinate {
                x: 37.51849825174829,
                y: 8.199783349650358,
            },
            Coordinate {
                x: 37.29047026537997,
                y: 8.153300885733085,
            },
            Coordinate {
                x: 35.5739868973262,
                y: 7.57506523828877,
            },
            Coordinate {
                x: 35.518505652173914,
                y: 7.556146305335969,
            },
            Coordinate {
                x: 33.93036149195302,
                y: 6.692874525445847,
            },
            Coordinate {
                x: 31.944797801710486,
                y: 5.236909567627495,
            },
            Coordinate {
                x: 31.78258482352941,
                y: 5.113927253796792,
            },
            Coordinate {
                x: 30.18008338244514,
                y: 3.308276926123303,
            },
            Coordinate {
                x: 30.000945714285713,
                y: 3.1050520995671,
            },
            Coordinate {
                x: 28.37655575757576,
                y: 1.1181568614718627,
            },
            Coordinate {
                x: 28.197941004838874,
                y: 0.9051253861834657,
            },
            Coordinate {
                x: 26.238901999127968,
                y: -0.9290026762589918,
            },
            Coordinate {
                x: 26.156780508474576,
                y: -1.005625550154083,
            },
            Coordinate {
                x: 24.59227592592591,
                y: -2.005733185185194,
            },
            Coordinate {
                x: 23.879666279069767,
                y: -2.447816217054264,
            },
            Coordinate {
                x: 23.84657093023256,
                y: -2.4702627286821706,
            },
            Coordinate {
                x: 21.40088863298663,
                y: -3.538301937592868,
            },
            Coordinate {
                x: 21.260826671619615,
                y: -3.6118359643387814,
            },
            Coordinate {
                x: 19.013587550439116,
                y: -4.486142778067885,
            },
            Coordinate {
                x: 18.526589765013053,
                y: -4.689803705435557,
            },
            Coordinate {
                x: 15.75972842105263,
                y: -5.470108403508772,
            },
            Coordinate {
                x: 14.71079050367855,
                y: -5.7870362440705865,
            },
            Coordinate {
                x: 14.130166904357669,
                y: -5.87134540891084,
            },
            Coordinate {
                x: 11.396836252723311,
                y: -6.79111423073876,
            },
            Coordinate {
                x: 12.836953866760663,
                y: -7.314210095267088,
            },
            Coordinate {
                x: 13.430737899189285,
                y: -7.665385187169545,
            },
            Coordinate {
                x: 15.891967692307693,
                y: -8.278695832167832,
            },
            Coordinate {
                x: 17.614775384615385,
                y: -8.69158813986014,
            },
            Coordinate {
                x: 20.508739439252338,
                y: -10.285825400169923,
            },
            Coordinate {
                x: 21.034942242990653,
                y: -10.562739719626167,
            },
            Coordinate {
                x: 21.33170522514868,
                y: -10.839488317757008,
            },
            Coordinate {
                x: 22.943262058547067,
                y: -12.314627400295421,
            },
            Coordinate {
                x: 23.2693244911264,
                y: -13.196186254980075,
            },
            Coordinate {
                x: 23.87570205288796,
                y: -15.025043110647182,
            },
            Coordinate {
                x: 23.88745512820513,
                y: -15.329726244343894,
            },
            Coordinate {
                x: 23.960685734265734,
                y: -17.477934475524474,
            },
            Coordinate {
                x: 23.6133433965311,
                y: -19.34018309210526,
            },
            Coordinate {
                x: 23.49620768022841,
                y: -19.898904389721626,
            },
            Coordinate {
                x: 23.29821013039044,
                y: -20.73196482371795,
            },
            Coordinate {
                x: 22.744271335379892,
                y: -22.771500805832694,
            },
            Coordinate {
                x: 22.167913473053893,
                y: -24.710688473053892,
            },
            Coordinate {
                x: 21.899988487232672,
                y: -25.71286526357199,
            },
            Coordinate {
                x: 21.785349664147773,
                y: -26.8158580604534,
            },
            Coordinate {
                x: 21.62434375,
                y: -28.56073125,
            },
            Coordinate {
                x: 21.778491344537816,
                y: -29.254584957983194,
            },
            Coordinate {
                x: 21.9928726215291,
                y: -30.22546573221757,
            },
            Coordinate {
                x: 22.298107250859108,
                y: -30.830159278350514,
            },
            Coordinate {
                x: 22.703645194250193,
                y: -31.61432564102564,
            },
            Coordinate {
                x: 23.150684988776653,
                y: -32.04120379188713,
            },
            Coordinate {
                x: 23.666477301561873,
                y: -32.53162973568282,
            },
            Coordinate {
                x: 24.352727837076323,
                y: -32.8549336827712,
            },
            Coordinate {
                x: 25.12504182648402,
                y: -33.18867739726027,
            },
            Coordinate {
                x: 26.256443471758022,
                y: -33.42552113970588,
            },
            Coordinate {
                x: 27.18248221033868,
                y: -33.62357801120448,
            },
            Coordinate {
                x: 27.94577306949807,
                y: -33.50700048262548,
            },
            Coordinate {
                x: 28.809313944395413,
                y: -33.351192718446605,
            },
            Coordinate {
                x: 29.51190471226022,
                y: -33.14606467889909,
            },
            Coordinate {
                x: 30.20681809815951,
                y: -32.91714938650307,
            },
            Coordinate {
                x: 30.772219131701632,
                y: -32.48469230769231,
            },
            Coordinate {
                x: 31.269684328633932,
                y: -32.07654005347594,
            },
            Coordinate {
                x: 31.814980518018018,
                y: -31.44050518018018,
            },
            Coordinate {
                x: 32.21083703009828,
                y: -30.958643693693695,
            },
            Coordinate {
                x: 32.55321905118602,
                y: -29.88120468164794,
            },
            Coordinate {
                x: 32.73880441494149,
                y: -29.18182376237624,
            },
            Coordinate {
                x: 32.80154325208307,
                y: -28.19137339261286,
            },
            Coordinate {
                x: 32.81496887317013,
                y: -27.68899303112314,
            },
            Coordinate {
                x: 32.50266570848146,
                y: -26.861685754189942,
            },
            Coordinate {
                x: 32.39781927272727,
                y: -26.510591276595743,
            },
            Coordinate {
                x: 31.365036702127657,
                y: -25.29672393617021,
            },
            Coordinate {
                x: 31.18817417218543,
                y: -25.083996854304633,
            },
            Coordinate {
                x: 30.96186148325359,
                y: -24.96392033492823,
            },
            Coordinate {
                x: 29.730729591836745,
                y: -24.11543265306123,
            },
            Coordinate {
                x: 29.656726744186052,
                y: -24.05927093023256,
            },
            Coordinate {
                x: 29.60786635220126,
                y: -24.02293364779874,
            },
            Coordinate {
                x: 28.20101297055058,
                y: -23.729857981220658,
            },
            Coordinate {
                x: 27.573002405063292,
                y: -23.641001265822783,
            },
            Coordinate {
                x: 26.692397564593303,
                y: -23.939743684210526,
            },
            Coordinate {
                x: 25.820720253164556,
                y: -24.25473101265823,
            },
            Coordinate {
                x: 25.080241257689675,
                y: -24.83151992481203,
            },
            Coordinate {
                x: 24.77587464968153,
                y: -25.06351433121019,
            },
            Coordinate {
                x: 24.731795656401946,
                y: -25.055063776337114,
            },
            Coordinate {
                x: 24.7241,
                y: -25.01283,
            },
            Coordinate {
                x: 25.390740501228503,
                y: -24.453634324324323,
            },
            Coordinate {
                x: 25.755809375000002,
                y: -24.138696875,
            },
            Coordinate {
                x: 26.646257086614167,
                y: -23.764686220472445,
            },
            Coordinate {
                x: 27.50551998284734,
                y: -23.41652547169811,
            },
            Coordinate {
                x: 29.63244676963812,
                y: -23.502961650485435,
            },
            Coordinate {
                x: 29.774809493670883,
                y: -23.512658860759494,
            },
            Coordinate {
                x: 29.89847020202021,
                y: -23.575248989898995,
            },
            Coordinate {
                x: 31.589993333333332,
                y: -24.5268525,
            },
            Coordinate {
                x: 31.844447168271582,
                y: -24.82213276481149,
            },
            Coordinate {
                x: 33.13353828571428,
                y: -26.132203714285712,
            },
            Coordinate {
                x: 33.55147932974866,
                y: -27.06839484181568,
            },
            Coordinate {
                x: 33.80086793478261,
                y: -27.687474048913042,
            },
            Coordinate {
                x: 33.78803503067485,
                y: -28.661053803680982,
            },
            Coordinate {
                x: 33.76582250647741,
                y: -29.40860724907063,
            },
            Coordinate {
                x: 33.466707086846114,
                y: -30.482701955307263,
            },
            Coordinate {
                x: 33.17527069741282,
                y: -31.51056507311586,
            },
            Coordinate {
                x: 32.62682653504744,
                y: -32.168572222222224,
            },
            Coordinate {
                x: 32.04635688990115,
                y: -32.880526652452026,
            },
            Coordinate {
                x: 31.34967225342598,
                y: -33.43459076433121,
            },
            Coordinate {
                x: 30.726345612527716,
                y: -33.93201798780488,
            },
            Coordinate {
                x: 29.777996060606064,
                y: -34.28924343434343,
            },
            Coordinate {
                x: 28.998282084534104,
                y: -34.55946325648415,
            },
            Coordinate {
                x: 27.99265371443442,
                y: -34.716202671755724,
            },
            Coordinate {
                x: 27.1951319038817,
                y: -34.845439648798525,
            },
            Coordinate {
                x: 25.876675432088497,
                y: -34.62063536139067,
            },
            Coordinate {
                x: 24.867799862283793,
                y: -34.43011844003607,
            },
            Coordinate {
                x: 23.773830313901346,
                y: -33.904452959641254,
            },
            Coordinate {
                x: 22.976305681818182,
                y: -33.530075,
            },
            Coordinate {
                x: 22.444271147351376,
                y: -33.047147455470736,
            },
            Coordinate {
                x: 21.63122400508044,
                y: -32.30165245554615,
            },
            Coordinate {
                x: 21.281324027589324,
                y: -31.631710447761193,
            },
            Coordinate {
                x: 20.747026334164588,
                y: -30.60282306733167,
            },
            Coordinate {
                x: 20.559623673469385,
                y: -29.841175714285715,
            },
            Coordinate {
                x: 20.25604131147541,
                y: -28.64845393442623,
            },
            Coordinate {
                x: 20.333511631799162,
                y: -26.710274351464435,
            },
            Coordinate {
                x: 20.3675689693155,
                y: -25.517690991345397,
            },
            Coordinate {
                x: 20.734327929089442,
                y: -23.524123730862208,
            },
            Coordinate {
                x: 20.95530039410244,
                y: -22.46300903151422,
            },
            Coordinate {
                x: 20.953675346708465,
                y: -21.604954551724138,
            },
            Coordinate {
                x: 20.94049112578903,
                y: -19.61750153901217,
            },
            Coordinate {
                x: 20.905276528799444,
                y: -18.09694435114504,
            },
            Coordinate {
                x: 20.905728963117607,
                y: -17.62720490605428,
            },
            Coordinate {
                x: 20.47834354620586,
                y: -16.216294402704733,
            },
            Coordinate {
                x: 20.4089721751026,
                y: -15.972046306429549,
            },
            Coordinate {
                x: 19.577352033310202,
                y: -15.131401908396946,
            },
            Coordinate {
                x: 19.278795269485347,
                y: -14.808011812778602,
            },
            Coordinate {
                x: 19.222147721194112,
                y: -14.776514338781576,
            },
            Coordinate {
                x: 18.170260000000003,
                y: -14.160816666666667,
            },
            Coordinate {
                x: 17.890593333333335,
                y: -14.09365,
            },
            Coordinate {
                x: 16.24134816542015,
                y: -13.747537725472943,
            },
            Coordinate {
                x: 14.940157492300925,
                y: -13.895180268367795,
            },
            Coordinate {
                x: 12.854184330616997,
                y: -14.07213533178114,
            },
            Coordinate {
                x: 11.234649095847885,
                y: -14.794774291812185,
            },
            Coordinate {
                x: 9.542914913294798,
                y: -15.460211849710984,
            },
            Coordinate {
                x: 9.328692727272728,
                y: -16.28455,
            },
            Coordinate {
                x: 9.180059,
                y: -16.65895,
            },
            Coordinate {
                x: 9.838116487922704,
                y: -19.835157729468598,
            },
            Coordinate {
                x: 9.838535471366066,
                y: -19.84048090909091,
            },
            Coordinate {
                x: 9.843282090909092,
                y: -19.884057692307714,
            },
            Coordinate {
                x: 10.16075425981873,
                y: -23.12924003021148,
            },
            Coordinate {
                x: 10.125056978851964,
                y: -23.26589501510574,
            },
            Coordinate {
                x: 9.216824990328819,
                y: -26.40247340425532,
            },
            Coordinate {
                x: 9.136165382978724,
                y: -26.491198936170214,
            },
            Coordinate {
                x: 6.752372333333357,
                y: -28.91480333333331,
            },
            Coordinate {
                x: 6.659293893617023,
                y: -29.007941063829787,
            },
            Coordinate {
                x: 6.6529661459415586,
                y: -29.014493392857144,
            },
            Coordinate {
                x: 4.698182098569158,
                y: -30.416987678855325,
            },
            Coordinate {
                x: 4.458449083887657,
                y: -30.560658536585365,
            },
            Coordinate {
                x: 2.8004379935562524,
                y: -31.483349042145594,
            },
            Coordinate {
                x: 2.571658720671444,
                y: -31.619197302904563,
            },
            Coordinate {
                x: 0.045879814072229796,
                y: -32.72151242661448,
            },
            Coordinate {
                x: -0.49560151719367596,
                y: -32.97893695652174,
            },
            Coordinate {
                x: -1.9678976851248429,
                y: -33.70592518557795,
            },
            Coordinate {
                x: -2.429041752688172,
                y: -33.97243752688172,
            },
            Coordinate {
                x: -3.6893136415694587,
                y: -35.124571420996816,
            },
            Coordinate {
                x: -4.110945325031134,
                y: -35.50573082191781,
            },
            Coordinate {
                x: -4.854566009861325,
                y: -36.52081101694915,
            },
            Coordinate {
                x: -5.093412482276898,
                y: -36.9308627293578,
            },
            Coordinate {
                x: -5.193135560076288,
                y: -37.35553531468531,
            },
            Coordinate {
                x: -5.439305760997067,
                y: -38.368831451612905,
            },
            Coordinate {
                x: -5.392446314131414,
                y: -39.9667301980198,
            },
            Coordinate {
                x: -5.358187656260306,
                y: -40.839122914147524,
            },
            Coordinate {
                x: -4.679698569417581,
                y: -42.22456702127659,
            },
            Coordinate {
                x: -4.243529647869674,
                y: -43.07155764411027,
            },
            Coordinate {
                x: -3.1862447623642947,
                y: -43.82638148371532,
            },
            Coordinate {
                x: -2.4289130139122093,
                y: -44.394928100263854,
            },
            Coordinate {
                x: -1.2471657190606964,
                y: -44.64322220543807,
            },
            Coordinate {
                x: -0.20417073018284831,
                y: -44.8486902266289,
            },
            Coordinate {
                x: 1.208624985002309,
                y: -44.64445592216582,
            },
            Coordinate {
                x: 1.903140907668232,
                y: -44.52301025039124,
            },
            Coordinate {
                x: 3.056799810093822,
                y: -43.88302046263345,
            },
            Coordinate {
                x: 3.3824261181054243,
                y: -43.6993631092437,
            },
            Coordinate {
                x: 4.3699307269155225,
                y: -42.633768860510806,
            },
            Coordinate {
                x: 4.461576455696203,
                y: -42.52936265822785,
            },
            Coordinate {
                x: 4.89566191204589,
                y: -41.32762131931166,
            },
            Coordinate {
                x: 4.9119162962962974,
                y: -41.28979888888889,
            },
            Coordinate {
                x: 4.922251366906475,
                y: -41.034779496402884,
            },
            Coordinate {
                x: 5.033826523605151,
                y: -40.4017517167382,
            },
            Coordinate {
                x: 5.207429190909091,
                y: -39.690000000000005,
            },
            Coordinate {
                x: 5.55887566666667,
                y: -40.828550000000014,
            },
            Coordinate {
                x: 5.678565039348712,
                y: -41.205163432835825,
            },
            Coordinate {
                x: 5.688562474903475,
                y: -41.35776196911197,
            },
            Coordinate {
                x: 5.330239621572211,
                y: -42.800707038391224,
            },
            Coordinate {
                x: 5.291449197939172,
                y: -42.92854634369287,
            },
            Coordinate {
                x: 4.003079095121095,
                y: -44.28245579150579,
            },
            Coordinate {
                x: 3.983055202531645,
                y: -44.309933544303796,
            },
            Coordinate {
                x: 3.9488388358995996,
                y: -44.335744895591645,
            },
            Coordinate {
                x: 2.4847500390624995,
                y: -45.4532466796875,
            },
            Coordinate {
                x: 1.9082936874325696,
                y: -45.60591659340659,
            },
            Coordinate {
                x: -0.1710241228142293,
                y: -46.13201956521739,
            },
            Coordinate {
                x: -1.2513467282225224,
                y: -45.96203432835821,
            },
            Coordinate {
                x: -2.8306697750071246,
                y: -45.7819539184953,
            },
            Coordinate {
                x: -3.950447106035888,
                y: -45.01115391517129,
            },
            Coordinate {
                x: -4.96529236522661,
                y: -44.33111093294461,
            },
            Coordinate {
                x: -6.138661928620546,
                y: -42.76974954614221,
            },
            Coordinate {
                x: -6.908923115074798,
                y: -41.711520886075945,
            },
            Coordinate {
                x: -7.259640909090908,
                y: -39.401450000000004,
            },
            Coordinate {
                x: -7.451395713666466,
                y: -38.22152523178808,
            },
            Coordinate {
                x: -7.366029692307692,
                y: -37.57332692307692,
            },
            Coordinate {
                x: -7.1679019554437655,
                y: -35.93239308300395,
            },
            Coordinate {
                x: -6.869847152211772,
                y: -35.35990095785441,
            },
            Coordinate {
                x: -6.127569640176337,
                y: -33.698801900393185,
            },
            Coordinate {
                x: -5.594131811461387,
                y: -33.10744596774193,
            },
            Coordinate {
                x: -4.130437571779572,
                y: -31.59611344916345,
            },
            Coordinate {
                x: -3.5173405850843755,
                y: -31.17561491017964,
            },
            Coordinate {
                x: -1.7785765537820957,
                y: -30.151353435114505,
            },
            Coordinate {
                x: -1.1048699961685808,
                y: -29.784088314176245,
            },
            Coordinate {
                x: 0.7570252456965089,
                y: -28.7491283269962,
            },
            Coordinate {
                x: 1.7070045284552842,
                y: -28.072885772357726,
            },
            Coordinate {
                x: 1.946997506913446,
                y: -27.883432035928145,
            },
            Coordinate {
                x: 3.16385630744868,
                y: -26.735243161290324,
            },
            Coordinate {
                x: 3.3197360270270275,
                y: -26.577503378378378,
            },
            Coordinate {
                x: 4.069082329283111,
                y: -24.967709538274605,
            },
            Coordinate {
                x: 4.096999657912458,
                y: -24.918623015873013,
            },
            Coordinate {
                x: 4.241111983086683,
                y: -23.781145348837192,
            },
            Coordinate {
                x: 4.391154632569903,
                y: -22.70646916537867,
            },
            Coordinate {
                x: 4.393993719685261,
                y: -22.671712751159195,
            },
            Coordinate {
                x: 2.7795412443181813,
                y: -20.17895,
            },
            Coordinate {
                x: 2.77126,
                y: -20.165075,
            },
            Coordinate {
                x: 2.7542037534090915,
                y: -20.153912500000004,
            },
            Coordinate {
                x: 0.025835602247999356,
                y: -17.955661078140455,
            },
            Coordinate {
                x: -0.8259763841472227,
                y: -17.43118473585788,
            },
            Coordinate {
                x: -1.2812667311827954,
                y: -17.171464913510988,
            },
            Coordinate {
                x: -1.7098254975137062,
                y: -16.923327793361384,
            },
            Coordinate {
                x: -2.395120673306773,
                y: -16.536210779105797,
            },
            Coordinate {
                x: -3.2639064452493054,
                y: -16.02521759628154,
            },
            Coordinate {
                x: -4.389581278085515,
                y: -15.442883293508563,
            },
            Coordinate {
                x: -5.31830235207567,
                y: -15.060421098265897,
            },
            Coordinate {
                x: -6.605387522858644,
                y: -14.437084682080926,
            },
            Coordinate {
                x: -7.681890009171838,
                y: -14.286312611275964,
            },
            Coordinate {
                x: -8.847372463717292,
                y: -14.112098961424332,
            },
            Coordinate {
                x: -10.738085244752531,
                y: -14.072642355452077,
            },
            Coordinate {
                x: -12.613717997671712,
                y: -14.007504637175009,
            },
            Coordinate {
                x: -15.055914748544204,
                y: -13.978453493912124,
            },
            Coordinate {
                x: -16.04459367919534,
                y: -13.971993885653784,
            },
            Coordinate {
                x: -17.804206666666666,
                y: -14.369250000000001,
            },
            Coordinate {
                x: -18.10954,
                y: -14.436416666666666,
            },
            Coordinate {
                x: -19.24129447791436,
                y: -15.033712555720653,
            },
            Coordinate {
                x: -19.30189418073754,
                y: -15.06425193164933,
            },
            Coordinate {
                x: -19.600141603053434,
                y: -15.387701908396945,
            },
            Coordinate {
                x: -20.431102881482403,
                y: -16.22834630642955,
            },
            Coordinate {
                x: -20.50017615531726,
                y: -16.468695078888054,
            },
            Coordinate {
                x: -20.92749130322009,
                y: -17.85862807933194,
            },
            Coordinate {
                x: -20.927514009715473,
                y: -18.31097824427481,
            },
            Coordinate {
                x: -20.964373285612027,
                y: -19.771699821045097,
            },
            Coordinate {
                x: -20.976758344827587,
                y: -21.67049137931034,
            },
            Coordinate {
                x: -20.976913481936972,
                y: -22.490506341275943,
            },
            Coordinate {
                x: -20.749315906526995,
                y: -23.54270503626108,
            },
            Coordinate {
                x: -20.367555312924683,
                y: -25.517690991345397,
            },
            Coordinate {
                x: -20.333497742868012,
                y: -26.710274351464435,
            },
            Coordinate {
                x: -20.256021311475408,
                y: -28.64845393442623,
            },
            Coordinate {
                x: -20.559603673469386,
                y: -29.841175714285715,
            },
            Coordinate {
                x: -20.74700633416459,
                y: -30.60282306733167,
            },
            Coordinate {
                x: -21.28130990049751,
                y: -31.631710447761193,
            },
            Coordinate {
                x: -21.63121400508044,
                y: -32.30165245554615,
            },
            Coordinate {
                x: -22.444255351993213,
                y: -33.047147455470736,
            },
            Coordinate {
                x: -22.97629,
                y: -33.530075,
            },
            Coordinate {
                x: -23.773810313901347,
                y: -33.904452959641254,
            },
            Coordinate {
                x: -24.867785446348062,
                y: -34.43011844003607,
            },
            Coordinate {
                x: -25.876665432088497,
                y: -34.62063536139067,
            },
            Coordinate {
                x: -27.195116328348178,
                y: -34.845439648798525,
            },
            Coordinate {
                x: -27.99263828244275,
                y: -34.716202671755724,
            },
            Coordinate {
                x: -28.998272084534104,
                y: -34.55946325648415,
            },
            Coordinate {
                x: -29.77798606060606,
                y: -34.28924343434343,
            },
            Coordinate {
                x: -30.726330345528456,
                y: -33.93201798780488,
            },
            Coordinate {
                x: -31.34965719745223,
                y: -33.43459076433121,
            },
            Coordinate {
                x: -32.04634181236674,
                y: -32.880526652452026,
            },
            Coordinate {
                x: -32.62681171717172,
                y: -32.168572222222224,
            },
            Coordinate {
                x: -33.17526552612741,
                y: -31.51056507311586,
            },
            Coordinate {
                x: -33.46669229050279,
                y: -30.482701955307263,
            },
            Coordinate {
                x: -33.76580827757125,
                y: -29.40860724907063,
            },
            Coordinate {
                x: -33.78802921807027,
                y: -28.661053803680982,
            },
            Coordinate {
                x: -33.80086793478261,
                y: -27.687474048913042,
            },
            Coordinate {
                x: -33.55147211454295,
                y: -27.06839484181568,
            },
            Coordinate {
                x: -33.13351828571428,
                y: -26.132203714285712,
            },
            Coordinate {
                x: -31.84444385506773,
                y: -24.82213276481149,
            },
            Coordinate {
                x: -31.589991795454544,
                y: -24.5268525,
            },
            Coordinate {
                x: -29.898460881542707,
                y: -23.575248989898995,
            },
            Coordinate {
                x: -29.77479991944764,
                y: -23.512658860759494,
            },
            Coordinate {
                x: -29.63243676963812,
                y: -23.502961650485435,
            },
            Coordinate {
                x: -27.50550998284734,
                y: -23.41652547169811,
            },
            Coordinate {
                x: -26.646247086614167,
                y: -23.764686220472445,
            },
            Coordinate {
                x: -25.755799375000002,
                y: -24.138696875,
            },
            Coordinate {
                x: -25.390730501228504,
                y: -24.453634324324323,
            },
            Coordinate {
                x: -24.724082454545456,
                y: -25.01283,
            },
            Coordinate {
                x: -24.731775656401947,
                y: -25.055063776337114,
            },
            Coordinate {
                x: -24.77585464968153,
                y: -25.06351433121019,
            },
            Coordinate {
                x: -25.08022716336295,
                y: -24.83151992481203,
            },
            Coordinate {
                x: -25.820714321058688,
                y: -24.25473101265823,
            },
            Coordinate {
                x: -26.69239263157895,
                y: -23.939743684210526,
            },
            Coordinate {
                x: -27.572986311852706,
                y: -23.641001265822783,
            },
            Coordinate {
                x: -28.20099876653863,
                y: -23.729857981220658,
            },
            Coordinate {
                x: -29.607856838193253,
                y: -24.02293364779874,
            },
            Coordinate {
                x: -29.65671674418605,
                y: -24.05927093023256,
            },
            Coordinate {
                x: -29.730719591836742,
                y: -24.11543265306123,
            },
            Coordinate {
                x: -30.96185148325359,
                y: -24.96392033492823,
            },
            Coordinate {
                x: -31.18816566676701,
                y: -25.083996854304633,
            },
            Coordinate {
                x: -31.36502830270793,
                y: -25.29672393617021,
            },
            Coordinate {
                x: -32.3978058336557,
                y: -26.510591276595743,
            },
            Coordinate {
                x: -32.50265216861351,
                y: -26.861685754189942,
            },
            Coordinate {
                x: -32.81495575470538,
                y: -27.68899303112314,
            },
            Coordinate {
                x: -32.801526730506154,
                y: -28.19137339261286,
            },
            Coordinate {
                x: -32.738788514851485,
                y: -29.18182376237624,
            },
            Coordinate {
                x: -32.55319905118602,
                y: -29.88120468164794,
            },
            Coordinate {
                x: -32.21082231981982,
                y: -30.958643693693695,
            },
            Coordinate {
                x: -31.81497051801802,
                y: -31.44050518018018,
            },
            Coordinate {
                x: -31.269669304812833,
                y: -32.07654005347594,
            },
            Coordinate {
                x: -30.772204102564103,
                y: -32.48469230769231,
            },
            Coordinate {
                x: -30.206812911321805,
                y: -32.91714938650307,
            },
            Coordinate {
                x: -29.511894370308593,
                y: -33.14606467889909,
            },
            Coordinate {
                x: -28.809298543689323,
                y: -33.351192718446605,
            },
            Coordinate {
                x: -27.94576306949807,
                y: -33.50700048262548,
            },
            Coordinate {
                x: -27.182467731092437,
                y: -33.62357801120448,
            },
            Coordinate {
                x: -26.256433471758022,
                y: -33.42552113970588,
            },
            Coordinate {
                x: -25.125026271481943,
                y: -33.18867739726027,
            },
            Coordinate {
                x: -24.352723381950774,
                y: -32.8549336827712,
            },
            Coordinate {
                x: -23.666468689627553,
                y: -32.53162973568282,
            },
            Coordinate {
                x: -23.15067638688472,
                y: -32.04120379188713,
            },
            Coordinate {
                x: -22.703636857031857,
                y: -31.61432564102564,
            },
            Coordinate {
                x: -22.29808725085911,
                y: -30.830159278350514,
            },
            Coordinate {
                x: -21.992856652719666,
                y: -30.22546573221757,
            },
            Coordinate {
                x: -21.778481344537816,
                y: -29.254584957983194,
            },
            Coordinate {
                x: -21.62433375,
                y: -28.56073125,
            },
            Coordinate {
                x: -21.785339664147774,
                y: -26.8158580604534,
            },
            Coordinate {
                x: -21.899974933123524,
                y: -25.71286526357199,
            },
            Coordinate {
                x: -22.184268443113773,
                y: -24.71292005988024,
            },
            Coordinate {
                x: -22.752349900230236,
                y: -22.79079144282425,
            },
            Coordinate {
                x: -23.282681666666665,
                y: -20.824299599358977,
            },
            Coordinate {
                x: -23.487229578872235,
                y: -20.01993600999286,
            },
            Coordinate {
                x: -23.89409766590389,
                y: -18.150529252479025,
            },
            Coordinate {
                x: -23.982254615384615,
                y: -17.684429510489508,
            },
            Coordinate {
                x: -23.90917224941725,
                y: -15.580675867269987,
            },
            Coordinate {
                x: -23.897830795217306,
                y: -15.281213187195547,
            },
            Coordinate {
                x: -23.264533277797895,
                y: -13.47033804780876,
            },
            Coordinate {
                x: -22.925378552437223,
                y: -12.597459453471197,
            },
            Coordinate {
                x: -21.04146242990654,
                y: -11.18046308411215,
            },
            Coordinate {
                x: -20.687960195412064,
                y: -10.91777523364486,
            },
            Coordinate {
                x: -20.145544672897195,
                y: -10.618529138487682,
            },
            Coordinate {
                x: -17.103963076923076,
                y: -8.933795272727272,
            },
            Coordinate {
                x: -15.58712132867133,
                y: -8.42688758041958,
            },
            Coordinate {
                x: -13.4088886579934,
                y: -7.674483001762425,
            },
            Coordinate {
                x: -13.050429249206909,
                y: -7.448857199859006,
            },
            Coordinate {
                x: -12.091443787878788,
                y: -7.107898378787879,
            },
            Coordinate {
                x: -11.850916666666667,
                y: -6.956199,
            },
            Coordinate {
                x: -11.396822440087146,
                y: -6.79111423073876,
            },
            Coordinate {
                x: -14.130149701600041,
                y: -5.87134540891084,
            },
            Coordinate {
                x: -14.710770503678551,
                y: -5.7870362440705865,
            },
            Coordinate {
                x: -15.759711218295005,
                y: -5.470108403508772,
            },
            Coordinate {
                x: -18.526571457393782,
                y: -4.689803705435557,
            },
            Coordinate {
                x: -19.01356924281984,
                y: -4.486142778067885,
            },
            Coordinate {
                x: -21.26081667161961,
                y: -3.6118359643387814,
            },
            Coordinate {
                x: -21.400879177360526,
                y: -3.538301937592868,
            },
            Coordinate {
                x: -23.84656106412967,
                y: -2.4702627286821706,
            },
            Coordinate {
                x: -23.879656279069767,
                y: -2.447816217054264,
            },
            Coordinate {
                x: -24.59226592592591,
                y: -2.005733185185194,
            },
            Coordinate {
                x: -26.156770508474576,
                y: -1.005625550154083,
            },
            Coordinate {
                x: -26.238891558752997,
                y: -0.9290026762589918,
            },
            Coordinate {
                x: -28.197939063111296,
                y: 0.9051253861834657,
            },
            Coordinate {
                x: -28.376553766233766,
                y: 1.1181568614718627,
            },
            Coordinate {
                x: -30.000925714285714,
                y: 3.1050520995671,
            },
            Coordinate {
                x: -30.180065353187043,
                y: 3.308276926123303,
            },
            Coordinate {
                x: -31.782575596791443,
                y: 5.113927253796792,
            },
            Coordinate {
                x: -31.944787801710486,
                y: 5.236909567627495,
            },
            Coordinate {
                x: -33.930351491953026,
                y: 6.692874525445847,
            },
            Coordinate {
                x: -35.518485938735175,
                y: 7.556146305335969,
            },
            Coordinate {
                x: -35.57396745561497,
                y: 7.57506523828877,
            },
            Coordinate {
                x: -37.29046029827832,
                y: 8.153300885733085,
            },
            Coordinate {
                x: -37.518488251748295,
                y: 8.199783349650358,
            },
            Coordinate {
                x: -38.37346956139729,
                y: 8.380649535109473,
            },
            Coordinate {
                x: -40.19190032482599,
                y: 8.3105873387471,
            },
            Coordinate {
                x: -40.21928826197005,
                y: 8.30932831301413,
            },
            Coordinate {
                x: -41.365599259259255,
                y: 7.990255451178451,
            },
            Coordinate {
                x: -42.50636091402014,
                y: 7.325229593127244,
            },
            Coordinate {
                x: -42.52224176818534,
                y: 7.3128364237025565,
            },
            Coordinate {
                x: -43.598063178294574,
                y: 6.4572989478505995,
            },
            Coordinate {
                x: -44.08843870523415,
                y: 5.486198931129499,
            },
            Coordinate {
                x: -44.543748453820115,
                y: 4.608298744680852,
            },
            Coordinate {
                x: -44.55161642564982,
                y: 4.579184494920174,
            },
            Coordinate {
                x: -44.795975864453666,
                y: 3.43459397749277,
            },
            Coordinate {
                x: -44.794378726708075,
                y: 3.3254998260869555,
            },
            Coordinate {
                x: -44.73775445666578,
                y: 2.165594565597668,
            },
            Coordinate {
                x: -44.68986003338898,
                y: 1.9413655225375614,
            },
            Coordinate {
                x: -44.42846298721372,
                y: 0.8720306662919768,
            },
            Coordinate {
                x: -44.24983444846293,
                y: 0.500327599901363,
            },
            Coordinate {
                x: -43.7340971907725,
                y: -0.4944010235847626,
            },
            Coordinate {
                x: -43.35625005531993,
                y: -0.9338686737967923,
            },
            Coordinate {
                x: -42.755528096947934,
                y: -1.568571339317774,
            },
            Coordinate {
                x: -42.18546223417608,
                y: -1.8576483944773186,
            },
            Coordinate {
                x: -41.69291375041932,
                y: -2.133550583025831,
            },
            Coordinate {
                x: -40.27500561497326,
                y: -2.3360116363636374,
            },
            Coordinate {
                x: -39.8694812133734,
                y: -2.395053371900827,
            },
            Coordinate {
                x: -39.589456367041194,
                y: -2.3286056779026225,
            },
            Coordinate {
                x: -38.47634833333334,
                y: -2.1883396666666677,
            },
            Coordinate {
                x: -38.2161611969112,
                y: -2.1659966486486493,
            },
            Coordinate {
                x: -37.974087681159425,
                y: -2.1419871304347833,
            },
            Coordinate {
                x: -37.0539276119403,
                y: -1.5795664613297182,
            },
            Coordinate {
                x: -36.22006639657444,
                y: -0.9858103509222663,
            },
            Coordinate {
                x: -35.99177703703704,
                y: -0.5391715387766589,
            },
            Coordinate {
                x: -35.39504343361674,
                y: 0.6872540276651405,
            },
            Coordinate {
                x: -35.36869496644295,
                y: 0.8430525854179354,
            },
            Coordinate {
                x: -35.202396043165464,
                y: 1.9796570359712236,
            },
            Coordinate {
                x: -35.19496343283582,
                y: 1.9992811044776118,
            },
            Coordinate {
                x: -35.18899888888889,
                y: 2.0030520208754203,
            },
            Coordinate {
                x: -35.18332720588235,
                y: 1.9995644999999997,
            },
            Coordinate {
                x: -35.17927251556662,
                y: 1.98016295890411,
            },
            Coordinate {
                x: -35.32909863636363,
                y: 0.8354519136363611,
            },
            Coordinate {
                x: -35.34827244897959,
                y: 0.6730294510204083,
            },
            Coordinate {
                x: -35.477730799999996,
                y: 0.37502871701818696,
            },
            Coordinate {
                x: -36.115894342751844,
                y: -1.0443628648648646,
            },
            Coordinate {
                x: -37.00456193722943,
                y: -1.8467063333333287,
            },
            Coordinate {
                x: -37.789531632653066,
                y: -2.521901061224489,
            },
            Coordinate {
                x: -38.414190163934435,
                y: -2.7361684918032796,
            },
            Coordinate {
                x: -39.766893383947945,
                y: -3.1409050498915394,
            },
            Coordinate {
                x: -40.31367253521127,
                y: -3.1216282816901404,
            },
            Coordinate {
                x: -42.02143525519848,
                y: -3.146603387523629,
            },
            Coordinate {
                x: -42.713810271646864,
                y: -2.8895553005093375,
            },
            Coordinate {
                x: -43.63621060070671,
                y: -2.4977271519434625,
            },
            Coordinate {
                x: -44.45220292929293,
                y: -1.7099684040404046,
            },
            Coordinate {
                x: -44.89526307888805,
                y: -1.290085685950413,
            },
            Coordinate {
                x: -45.6396889010989,
                y: -0.04822107692307773,
            },
            Coordinate {
                x: -45.84121546160361,
                y: 0.2830372484472051,
            },
            Coordinate {
                x: -46.26792260504202,
                y: 1.8490084705882346,
            },
            Coordinate {
                x: -46.32473221574344,
                y: 2.046916139941691,
            },
            Coordinate {
                x: -46.404162597200624,
                y: 3.774479371695178,
            },
            Coordinate {
                x: -46.409436394941146,
                y: 3.8459839559228652,
            },
            Coordinate {
                x: -46.095584935064934,
                y: 5.201223428571447,
            },
            Coordinate {
                x: -46.00843380681818,
                y: 5.556670755681818,
            },
            Coordinate {
                x: -44.78135558935361,
                y: 7.759170283097131,
            },
            Coordinate {
                x: -44.738533155893535,
                y: 7.794885079847909,
            },
            Coordinate {
                x: -43.457822011605415,
                y: 8.838263605415861,
            },
            Coordinate {
                x: -43.40168584139265,
                y: 8.867055127659574,
            },
            Coordinate {
                x: -41.77931794676806,
                y: 9.681891956100932,
            },
            Coordinate {
                x: -40.28151394636015,
                y: 10.027840495646117,
            },
            Coordinate {
                x: -40.24266796934866,
                y: 10.03719827586207,
            },
            Coordinate {
                x: -37.559808852459014,
                y: 10.084569672131149,
            },
            Coordinate {
                x: -37.54192852459016,
                y: 10.085045628415301,
            },
            Coordinate {
                x: -36.939243939393954,
                y: 9.937350787878794,
            },
            Coordinate {
                x: -36.01734770015187,
                y: 9.704519049251465,
            },
            Coordinate {
                x: -35.98279894671623,
                y: 9.69223429582066,
            },
            Coordinate {
                x: -33.53795731267545,
                y: 8.877894686460808,
            },
            Coordinate {
                x: -33.441694134225244,
                y: 8.82275788814563,
            },
            Coordinate {
                x: -31.772139266516756,
                y: 7.8958668189313626,
            },
            Coordinate {
                x: -31.6052274251497,
                y: 7.760636314643439,
            },
            Coordinate {
                x: -29.70315395348837,
                y: 6.217383340380549,
            },
            Coordinate {
                x: -29.50285204625763,
                y: 6.00793946321876,
            },
            Coordinate {
                x: -27.945216017615603,
                y: 4.351800262661214,
            },
            Coordinate {
                x: -27.724419729729732,
                y: 4.114721394963144,
            },
            Coordinate {
                x: -26.290111654749744,
                y: 2.5278789662921346,
            },
            Coordinate {
                x: -24.93602077798245,
                y: 1.341469282479142,
            },
            Coordinate {
                x: -24.810368790728603,
                y: 1.2419652641873276,
            },
            Coordinate {
                x: -23.20566711111111,
                y: 0.5039109645656565,
            },
            Coordinate {
                x: -23.19585550387597,
                y: 0.49963446779422105,
            },
            Coordinate {
                x: -23.04073999999999,
                y: 0.4511722563636331,
            },
            Coordinate {
                x: -21.075124246061517,
                y: -0.19698205843960997,
            },
            Coordinate {
                x: -20.956400990247563,
                y: -0.21543464658664663,
            },
            Coordinate {
                x: -18.43329769980507,
                y: -0.5793399651899109,
            },
            Coordinate {
                x: -17.762996335282654,
                y: -0.5705853020142951,
            },
            Coordinate {
                x: -14.686541886792453,
                y: -0.6179386974271013,
            },
            Coordinate {
                x: -14.16654566037736,
                y: -0.4584178799313895,
            },
            Coordinate {
                x: -12.238167249683144,
                y: -0.10339011314667595,
            },
            Coordinate {
                x: -12.048992598225603,
                y: 0.09493810424780114,
            },
            Coordinate {
                x: -11.434406533127888,
                y: 0.548328214329738,
            },
            Coordinate {
                x: -12.061923194795808,
                y: 1.0096926760291751,
            },
            Coordinate {
                x: -12.252020391300062,
                y: 1.3257203411637153,
            },
            Coordinate {
                x: -13.273099989518553,
                y: 1.9496852820906998,
            },
            Coordinate {
                x: -13.63667804765565,
                y: 2.156483744811683,
            },
            Coordinate {
                x: -14.98761148347425,
                y: 3.0072801050241074,
            },
            Coordinate {
                x: -16.31290654056326,
                y: 3.7327158415300543,
            },
            Coordinate {
                x: -17.47047051702396,
                y: 4.467765943979518,
            },
            Coordinate {
                x: -18.805969032258066,
                y: 5.2491379914279275,
            },
            Coordinate {
                x: -19.744846413264153,
                y: 5.8500657829912015,
            },
            Coordinate {
                x: -21.313840310763783,
                y: 6.828671995115697,
            },
            Coordinate {
                x: -21.760720188045667,
                y: 7.283062592832285,
            },
            Coordinate {
                x: -23.578762207792206,
                y: 9.208967285714282,
            },
            Coordinate {
                x: -24.13456233057851,
                y: 9.8310492446281,
            },
            Coordinate {
                x: -24.173934264159527,
                y: 9.956412491091061,
            },
            Coordinate {
                x: -24.92816475244362,
                y: 12.659147962052003,
            },
            Coordinate {
                x: -24.603885064377682,
                y: 16.474625250357654,
            },
            Coordinate {
                x: -23.740258135487103,
                y: 18.613442011019284,
            },
            Coordinate {
                x: -23.399807866462012,
                y: 19.482226362240983,
            },
            Coordinate {
                x: -22.718016916167663,
                y: 21.438030538922153,
            },
            Coordinate {
                x: -22.396746209856232,
                y: 22.45136526357199,
            },
            Coordinate {
                x: -22.331516400274786,
                y: 23.55394084802687,
            },
            Coordinate {
                x: -22.22993375,
                y: 25.29813125,
            },
            Coordinate {
                x: -22.386249915966385,
                y: 25.92412142857143,
            },
            Coordinate {
                x: -22.587118661087864,
                y: 26.794778870292888,
            },
            Coordinate {
                x: -22.910558189581554,
                y: 27.39731293766012,
            },
            Coordinate {
                x: -23.344000996369815,
                y: 28.19831873406967,
            },
            Coordinate {
                x: -23.9375303030303,
                y: 28.77823205128205,
            },
            Coordinate {
                x: -24.60429159010601,
                y: 29.44911819787986,
            },
            Coordinate {
                x: -25.371660437956205,
                y: 29.835370437956204,
            },
            Coordinate {
                x: -26.243404716636196,
                y: 30.25297851919561,
            },
            Coordinate {
                x: -26.961428682745826,
                y: 30.306906586270873,
            },
            Coordinate {
                x: -27.790137198879552,
                y: 30.359385294117647,
            },
            Coordinate {
                x: -28.696997658536585,
                y: 30.2022816097561,
            },
            Coordinate {
                x: -29.682429773844643,
                y: 29.995819911504427,
            },
            Coordinate {
                x: -30.449222558139535,
                y: 29.55751353065539,
            },
            Coordinate {
                x: -31.127445396825397,
                y: 29.0939826984127,
            },
            Coordinate {
                x: -31.647989944506104,
                y: 28.46753923418424,
            },
            Coordinate {
                x: -32.04125911111111,
                y: 27.987764555555557,
            },
            Coordinate {
                x: -32.50397153374233,
                y: 26.902169754601225,
            },
            Coordinate {
                x: -32.770865853658535,
                y: 26.185701463414635,
            },
            Coordinate {
                x: -32.82064685871056,
                y: 25.020313511659808,
            },
            Coordinate {
                x: -32.822309282814615,
                y: 24.42639303112314,
            },
            Coordinate {
                x: -32.50549139664804,
                y: 23.599085754189943,
            },
            Coordinate {
                x: -32.3978058336557,
                y: 23.247991276595744,
            },
            Coordinate {
                x: -31.36502830270793,
                y: 22.035047872340424,
            },
            Coordinate {
                x: -31.18816566676701,
                y: 21.822496854304635,
            },
            Coordinate {
                x: -30.96185148325359,
                y: 21.70242033492823,
            },
            Coordinate {
                x: -29.730719591836742,
                y: 20.85393265306123,
            },
            Coordinate {
                x: -29.64800517571885,
                y: 20.79281517571885,
            },
            Coordinate {
                x: -29.607291643192486,
                y: 20.763275352112675,
            },
            Coordinate {
                x: -28.08129243445693,
                y: 20.45590767790262,
            },
            Coordinate {
                x: -27.576188356807513,
                y: 20.389938262910796,
            },
            Coordinate {
                x: -26.504162092764382,
                y: 20.757312857142853,
            },
            Coordinate {
                x: -25.827658018867925,
                y: 21.004933490566035,
            },
            Coordinate {
                x: -25.563782351981352,
                y: 21.153132307692307,
            },
            Coordinate {
                x: -25.154486613891727,
                y: 21.451698876404492,
            },
            Coordinate {
                x: -25.502151275510204,
                y: 21.09019107142857,
            },
            Coordinate {
                x: -25.738428118096856,
                y: 20.8454285046729,
            },
            Coordinate {
                x: -26.892059487694624,
                y: 20.369673756906078,
            },
            Coordinate {
                x: -27.497449345794394,
                y: 20.127992056074767,
            },
            Coordinate {
                x: -29.683002025316448,
                y: 20.240690506329113,
            },
            Coordinate {
                x: -29.776196603773585,
                y: 20.24805471698113,
            },
            Coordinate {
                x: -29.898460881542707,
                y: 20.314774242424246,
            },
            Coordinate {
                x: -31.589991795454544,
                y: 21.265352500000002,
            },
            Coordinate {
                x: -31.84444385506773,
                y: 21.56045053859964,
            },
            Coordinate {
                x: -33.13351828571428,
                y: 22.869603714285713,
            },
            Coordinate {
                x: -33.55287204951857,
                y: 23.805794841815683,
            },
            Coordinate {
                x: -33.80227753087945,
                y: 24.424874048913043,
            },
            Coordinate {
                x: -33.80400857988815,
                y: 25.554209589867313,
            },
            Coordinate {
                x: -33.785873397713395,
                y: 26.456192735042737,
            },
            Coordinate {
                x: -33.40242480392157,
                y: 27.417105882352942,
            },
            Coordinate {
                x: -32.93022714775247,
                y: 28.62972915742794,
            },
            Coordinate {
                x: -32.384642582621794,
                y: 29.308962707182317,
            },
            Coordinate {
                x: -31.824917723444972,
                y: 30.022619157894738,
            },
            Coordinate {
                x: -30.77959560669456,
                y: 30.667062343096234,
            },
            Coordinate {
                x: -29.888071813755865,
                y: 31.19783169425511,
            },
            Coordinate {
                x: -28.65772803088803,
                y: 31.405857335907335,
            },
            Coordinate {
                x: -27.68799155940178,
                y: 31.581126340110906,
            },
            Coordinate {
                x: -26.925082752962627,
                y: 31.5383112579763,
            },
            Coordinate {
                x: -25.8657291598916,
                y: 31.492427235772357,
            },
            Coordinate {
                x: -25.01753990102961,
                y: 31.07793937664618,
            },
            Coordinate {
                x: -23.795313871965732,
                y: 30.472989965095987,
            },
            Coordinate {
                x: -23.07697220023015,
                y: 29.828628396624474,
            },
            Coordinate {
                x: -21.97976507346189,
                y: 28.83267777777778,
            },
            Coordinate {
                x: -21.6061247576761,
                y: 28.170946688741722,
            },
            Coordinate {
                x: -21.0327525807913,
                y: 27.166926910299004,
            },
            Coordinate {
                x: -20.820727428571427,
                y: 26.474966408163265,
            },
            Coordinate {
                x: -20.455786887481374,
                y: 25.38585393442623,
            },
            Coordinate {
                x: -20.477531548117156,
                y: 23.447674351464435,
            },
            Coordinate {
                x: -20.49829397324941,
                y: 22.25549319433517,
            },
            Coordinate {
                x: -20.929545207677094,
                y: 20.23260777598711,
            },
            Coordinate {
                x: -21.188684091258473,
                y: 19.157146848578016,
            },
            Coordinate {
                x: -21.77237597018182,
                y: 17.063638,
            },
            Coordinate {
                x: -22.03866094887179,
                y: 16.13062137733142,
            },
            Coordinate {
                x: -22.15582181126332,
                y: 13.518502511415525,
            },
            Coordinate {
                x: -22.18638919636618,
                y: 12.702478860936408,
            },
            Coordinate {
                x: -21.989489144542773,
                y: 12.292867050147493,
            },
            Coordinate {
                x: -21.114713748308528,
                y: 10.608078822733424,
            },
            Coordinate {
                x: -21.00603789029536,
                y: 10.538736660529343,
            },
            Coordinate {
                x: -20.42652575098814,
                y: 10.175913671936756,
            },
            Coordinate {
                x: -19.25886409028728,
                y: 9.39700782862828,
            },
            Coordinate {
                x: -19.012926060191518,
                y: 9.224552410396717,
            },
            Coordinate {
                x: -18.7936169680388,
                y: 9.148506849023754,
            },
            Coordinate {
                x: -17.236781365461848,
                y: 8.895279670317635,
            },
            Coordinate {
                x: -16.55242514056225,
                y: 8.853177662285507,
            },
            Coordinate {
                x: -15.286186615322254,
                y: 8.762651024836938,
            },
            Coordinate {
                x: -14.187620299959466,
                y: 8.939444052806131,
            },
            Coordinate {
                x: -12.969017033492824,
                y: 8.966239602870813,
            },
            Coordinate {
                x: -12.340177833783379,
                y: 9.52010015926138,
            },
            Coordinate {
                x: -10.472334455609197,
                y: 10.554994374437443,
            },
            Coordinate {
                x: -9.945373190300849,
                y: 11.377743549354935,
            },
            Coordinate {
                x: -9.726739964464851,
                y: 12.371978245951139,
            },
            Coordinate {
                x: -9.172063614403713,
                y: 14.402422961844634,
            },
            Coordinate {
                x: -8.913498413132695,
                y: 15.498490355677156,
            },
            Coordinate {
                x: -8.131291709986321,
                y: 16.903737961696308,
            },
            Coordinate {
                x: -7.552932397723902,
                y: 17.946063616137646,
            },
            Coordinate {
                x: -7.071593240203878,
                y: 18.22610304064076,
            },
            Coordinate {
                x: -6.05370751627385,
                y: 19.13448091168091,
            },
            Coordinate {
                x: -6.047818870620203,
                y: 19.140960330361825,
            },
            Coordinate {
                x: -6.044582331695332,
                y: 19.14263108108108,
            },
            Coordinate {
                x: -6.057877559139785,
                y: 19.20112634408602,
            },
            Coordinate {
                x: -6.346836413347981,
                y: 20.49469259818731,
            },
            Coordinate {
                x: -6.070152605415861,
                y: 23.854260638297873,
            },
            Coordinate {
                x: -6.063316796905222,
                y: 23.97553936170213,
            },
            Coordinate {
                x: -5.398866444444451,
                y: 26.473267777777757,
            },
            Coordinate {
                x: -5.37000798303737,
                y: 26.57189839650146,
            },
            Coordinate {
                x: -5.367740358768007,
                y: 26.575741803278692,
            },
            Coordinate {
                x: -3.9538550711382108,
                y: 28.821279878048777,
            },
            Coordinate {
                x: -3.842939595480225,
                y: 28.956972372881356,
            },
            Coordinate {
                x: -2.28435846031746,
                y: 30.68602439153439,
            },
            Coordinate {
                x: -2.0441190693834885,
                y: 30.94827842215256,
            },
            Coordinate {
                x: -0.07693699236901018,
                y: 32.52823669950739,
            },
            Coordinate {
                x: 1.5832581253109121,
                y: 33.44101878612717,
            },
            Coordinate {
                x: 2.0860712304687503,
                y: 33.759799609375,
            },
            Coordinate {
                x: 3.6439736933461906,
                y: 35.11893457087753,
            },
            Coordinate {
                x: 4.102558532721655,
                y: 35.52245731204943,
            },
            Coordinate {
                x: 4.923202312523191,
                y: 36.57459959183674,
            },
            Coordinate {
                x: 5.0777096894409945,
                y: 36.98563260869565,
            },
            Coordinate {
                x: 5.167354957983194,
                y: 37.36068361344538,
            },
            Coordinate {
                x: 5.422091373936874,
                y: 38.39082692307692,
            },
            Coordinate {
                x: 5.371153718843471,
                y: 40.04495299102692,
            },
            Coordinate {
                x: 5.338304810491028,
                y: 40.827338286334054,
            },
            Coordinate {
                x: 4.620550644285577,
                y: 42.290590807651434,
            },
            Coordinate {
                x: 4.2272248334521025,
                y: 43.05065078387458,
            },
            Coordinate {
                x: 3.0952544545454552,
                y: 43.856669696969696,
            },
            Coordinate {
                x: 2.4151309047213045,
                y: 44.36484906213364,
            },
            Coordinate {
                x: 1.3799379064969368,
                y: 44.57816889035667,
            },
            Coordinate {
                x: 0.2030001087504256,
                y: 44.807329026217225,
            },
            Coordinate {
                x: -1.014351625231912,
                y: 44.62444416909621,
            },
            Coordinate {
                x: -1.8792688937329702,
                y: 44.47033828337875,
            },
            Coordinate {
                x: -2.865053340354768,
                y: 43.91641189024391,
            },
            Coordinate {
                x: -3.3367251252144086,
                y: 43.648459433962266,
            },
            Coordinate {
                x: -4.163782016583749,
                y: 42.74171782752902,
            },
            Coordinate {
                x: -4.387177230293664,
                y: 42.48452557959815,
            },
            Coordinate {
                x: -4.750206533980583,
                y: 41.464914563106795,
            },
            Coordinate {
                x: -4.831281000000001,
                y: 41.275040393700785,
            },
            Coordinate {
                x: -4.874867413342054,
                y: 39.75972913669065,
            },
            Coordinate {
                x: -4.983139000000002,
                y: 39.779250000000005,
            },
            Coordinate {
                x: -5.6213299400386845,
                y: 39.61569893617022,
            },
            Coordinate {
                x: -5.640739397653959,
                y: 41.365090645161295,
            },
            Coordinate {
                x: -5.54123770967742,
                y: 41.69300419354839,
            },
            Coordinate {
                x: -5.2052024690019465,
                y: 42.99475733944954,
            },
            Coordinate {
                x: -4.936675558271237,
                y: 43.355789016393445,
            },
            Coordinate {
                x: -4.133007236051502,
                y: 44.43872639484979,
            },
            Coordinate {
                x: -3.5600852883869187,
                y: 44.86291890243902,
            },
            Coordinate {
                x: -2.5856937695818427,
                y: 45.581317571234734,
            },
            Coordinate {
                x: -1.2322711190909104,
                y: 45.93006,
            },
            Coordinate {
                x: 0.1608385382727273,
                y: 46.259575,
            },
            Coordinate {
                x: 1.7365781648429088,
                y: 46.00393449408673,
            },
            Coordinate {
                x: 2.8468431018518516,
                y: 45.86612743055555,
            },
            Coordinate {
                x: 4.256296545562899,
                y: 44.89123009535161,
            },
            Coordinate {
                x: 4.995079966769783,
                y: 44.38911180021954,
            },
            Coordinate {
                x: 6.380004119331006,
                y: 42.54066106094808,
            },
            Coordinate {
                x: 6.952215063811189,
                y: 41.74905192307692,
            },
            Coordinate {
                x: 7.349461253825382,
                y: 39.13993118811881,
            },
            Coordinate {
                x: 7.490684559925788,
                y: 38.2612581632653,
            },
            Coordinate {
                x: 7.423452432835821,
                y: 37.77020880597015,
            },
            Coordinate {
                x: 7.202182780487805,
                y: 36.0074131097561,
            },
            Coordinate {
                x: 6.960150369047618,
                y: 35.57710565476191,
            },
            Coordinate {
                x: 6.101460821862348,
                y: 33.83537813765182,
            },
            Coordinate {
                x: 5.62992931724461,
                y: 33.371372164948454,
            },
            Coordinate {
                x: 3.8474323207649777,
                y: 31.76261410767697,
            },
            Coordinate {
                x: 2.056944251000573,
                y: 30.584706603773586,
            },
            Coordinate {
                x: 1.6163872697599135,
                y: 30.26859866468843,
            },
            Coordinate {
                x: 1.3550732860903671,
                y: 29.94790892857143,
            },
            Coordinate {
                x: 0.4001953975927917,
                y: 28.697926627218937,
            },
            Coordinate {
                x: -0.36884680624752175,
                y: 27.03094937694704,
            },
            Coordinate {
                x: -0.5169385745180364,
                y: 26.724205607917057,
            },
            Coordinate {
                x: -0.5741684070001553,
                y: 26.454760562180578,
            },
            Coordinate {
                x: -0.8981726969152933,
                y: 24.49638052064632,
            },
            Coordinate {
                x: -0.8786989969234652,
                y: 24.347400762829405,
            },
            Coordinate {
                x: -0.44431434975902834,
                y: 22.374085657225855,
            },
            Coordinate {
                x: -0.06148776779719839,
                y: 21.08726538461537,
            },
        ])]);

        let _collated = layer.collate().unwrap();
    }
}
