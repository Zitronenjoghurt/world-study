use geo::{BoundingRect, Contains, EuclideanDistance, Point, Polygon};
use rstar::{PointDistance, RTreeObject, AABB};

#[derive(Debug)]
pub struct IdentifiedPolygon {
    id: String,
    priority: bool,
    polygon: Polygon<f64>,
}

impl IdentifiedPolygon {
    pub fn new(id: String, polygon: Polygon<f64>, priority: bool) -> Self {
        Self {
            id,
            priority,
            polygon,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn priority(&self) -> bool {
        self.priority
    }
}

impl RTreeObject for IdentifiedPolygon {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        let bounds = self.polygon.bounding_rect().unwrap();
        AABB::from_corners(
            [bounds.min().x, bounds.min().y],
            [bounds.max().x, bounds.max().y],
        )
    }
}

impl PointDistance for IdentifiedPolygon {
    fn distance_2(&self, point: &[f64; 2]) -> f64 {
        let geo_point = Point::new(point[0], point[1]);
        let distance = self.polygon.euclidean_distance(&geo_point);
        distance * distance
    }

    fn contains_point(&self, point: &[f64; 2]) -> bool {
        let geo_point = Point::new(point[0], point[1]);
        self.polygon.contains(&geo_point)
    }
}
