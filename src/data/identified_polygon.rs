use geo::{BoundingRect, Contains, EuclideanDistance, Point, Polygon};
use rstar::{PointDistance, RTreeObject, AABB};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IdentifiedPolygonType {
    Country,
    Capital,
}

#[derive(Debug)]
pub struct IdentifiedPolygon {
    id: String,
    priority: bool,
    polygon: Polygon<f32>,
    polygon_type: IdentifiedPolygonType,
}

impl IdentifiedPolygon {
    pub fn new(
        id: String,
        polygon: Polygon<f32>,
        priority: bool,
        polygon_type: IdentifiedPolygonType,
    ) -> Self {
        Self {
            id,
            priority,
            polygon,
            polygon_type,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn priority(&self) -> bool {
        self.priority
    }

    pub fn polygon_type(&self) -> IdentifiedPolygonType {
        self.polygon_type
    }
}

impl RTreeObject for IdentifiedPolygon {
    type Envelope = AABB<[f32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        let bounds = self.polygon.bounding_rect().unwrap();
        AABB::from_corners(
            [bounds.min().x, bounds.min().y],
            [bounds.max().x, bounds.max().y],
        )
    }
}

impl PointDistance for IdentifiedPolygon {
    fn distance_2(&self, point: &[f32; 2]) -> f32 {
        let geo_point = Point::new(point[0], point[1]);
        let distance = self.polygon.euclidean_distance(&geo_point);
        distance * distance
    }

    fn contains_point(&self, point: &[f32; 2]) -> bool {
        let geo_point = Point::new(point[0], point[1]);
        self.polygon.contains(&geo_point)
    }
}
