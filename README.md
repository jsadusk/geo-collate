# geo-collate
Polygon collation for the Rust [geo](https://github.com/georust/geo) types.

## What is polygon collation?

Polygon collation is converting an unorganized set of closed [LineStrings](https://docs.rs/geo-types/0.6.0/geo_types/struct.LineString.html), and organizing them into a set of [Polygons](https://docs.rs/geo-types/0.6.0/geo_types/struct.Polygon.html). This isn't trivial, because you have to identify which LineStrings are holes, and which are exteriors, and which holes belong to which exteriors. 

## Example

```rust
let exterior1: LineString<i64> = vec![(0, 0), (0, 30), (30, 30), (30, 0), (0, 0)].into();
let hole1: LineString<i64> = vec![(10, 10), (20, 10), (20, 20), (10, 20), (10, 10)].into();
let exterior2: LineString<i64> = exterior1.translate(40, 0);

let hole2: LineString<i64> = hole1.translate(40, 0);

let uncollated: MultiLineString<i64> = (vec![exterior1, hole1, exterior2, hole2])
    .into_iter()
    .collect();
let collated: MultiPolygon<i64> = uncollated.collate().unwrap();
```
