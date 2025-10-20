use std::marker::PhantomData;

use crate::utils::vec_2d::Vec2D;
use super::super::types::node::{Priced, Unpriced, NodeType};

#[derive(Debug)]
pub struct Node2D<NType: NodeType> {
    coord: Vec2D,
    name: Option<String>,
    goods: f64,
    n_type: PhantomData<NType>
}

impl Node2D<Priced> {
    pub fn new(x: f64, y: f64, price: f64, name: Option<String>) -> Node2D<Priced> {
        let coord = Vec2D { x, y };
        Node2D { coord, name, goods: price, n_type: PhantomData }
    }
}

impl Node2D<Unpriced> {
    pub fn new(x: f64, y: f64, name: Option<String>) -> Node2D<Unpriced> {
        let coord = Vec2D { x, y };
        Node2D { coord, name, goods: 0.0, n_type: PhantomData }
    }
}

impl<T: NodeType> Node2D<T> {
    pub fn get_distance<U: NodeType>(&self, other: &Node2D<U>) -> f64 { self.coord.get_distance(&other.coord) }

    pub fn get_goods(&self) -> f64 { self.goods }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_priced_node() {
        let node = Node2D::<Priced>::new(0.0, 0.0, 0.0, Some("R".to_string()));
        assert!(node.coord.x == 0.0);
        assert!(node.coord.y == 0.0);
        assert!(node.goods == 0.0);
        assert!(node.name == Option::Some("R".to_string()));
    }

    #[test]
    fn create_unpriced_node() {
        let node = Node2D::<Unpriced>::new(0.0, 0.0, Some("R".to_string()));
        assert!(node.coord.x == 0.0);
        assert!(node.coord.y == 0.0);
        assert!(node.goods == 0.0);
        assert!(node.name == Option::Some("R".to_string()));
    }
}