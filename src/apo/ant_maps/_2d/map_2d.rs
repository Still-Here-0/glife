use std::cmp::Ordering;
use rand::rand_core::le;
use std::marker::PhantomData;


use super::super::types::map::direcional::{Bidirecional, DirType, Unidirecional};
use super::super::types::map::initialization::{InitType, Initialized, Uninitialized};
use super::super::types::node::{NodeType, Priced, Unpriced};
use super::node_2d::Node2D;
use crate::apo::ant_map_traits::{ AntMapDistances, AntMapGeneric, AntMapInfo, AntMapNodes, AntMapPheromones, AntMapHelper };

#[derive(Debug)]
pub struct Map2D<DirecT: DirType, NodeT: NodeType, InitT: InitType = Uninitialized> {
    nodes: Vec<Node2D<NodeT>>,
    distances: Vec<Vec<f64>>,
    pheromones: Vec<Vec<f64>>,
    direcional: PhantomData<DirecT>,
    initialization: PhantomData<InitT>,
}

impl<DT: DirType, NT: NodeType, IT: InitType> Map2D<DT, NT, IT> {
    pub fn get_nodes(&self) -> &Vec<Node2D<NT>> { &self.nodes }

    pub fn get_empty_map_info(&self) -> Vec<Vec<f64>> {
        
        let mut empty_map_info = Vec::<Vec<f64>>::new();

        for line in self.distances.iter() {
            empty_map_info.push(vec![0.0; line.len()]);
        }
        
        empty_map_info
    }
}

/* #region UNINITIALIZED */

/* #region DIRECTION TYPE SPECIFIC */
impl<NT: NodeType> Map2D<Bidirecional, NT, Uninitialized> {
    // How do I make it possible for the user to define different distances from one node to another?
    pub fn new(nodes: Vec<Node2D<NT>>) -> Map2D<Bidirecional, NT, Uninitialized> { todo!() }
}

impl<NT: NodeType> Map2D<Unidirecional, NT, Uninitialized> {
    pub fn new(nodes: Vec<Node2D<NT>>) -> Map2D<Unidirecional, NT, Uninitialized> {
        Map2D {
            nodes,
            distances: Vec::new(),
            pheromones: Vec::new(),
            direcional: PhantomData,
            initialization: PhantomData,
        }
    }
}
/* #endregion */

/* #region NODE TYPE SPECIFIC */
impl<DT: DirType> Map2D<DT, Unpriced, Uninitialized> {
    pub fn add(&mut self, x: f64, y: f64, name: Option<String>) {
        let node = Node2D::<Unpriced>::new(x, y, name);
        self.nodes.push(node);
    }
}

impl<DT: DirType> Map2D<DT, Priced, Uninitialized> {
    pub fn add(&mut self, x: f64, y: f64, price: f64, name: Option<String>) {
        let node = Node2D::<Priced>::new(x, y, price, name);
        self.nodes.push(node);
    }
}
/* #endregion */

impl<DT: DirType, NT: NodeType> Map2D<DT, NT, Uninitialized> {
    pub fn init_map(mut self, phero_initializer: Option<f64>) -> Map2D<DT, NT, Initialized> {
        let phero_initializer = phero_initializer.unwrap_or(1.0);

        for (line, main_node) in self.nodes.iter().enumerate() {
            let mut vec = Vec::new();
            for (column, other_node) in self.nodes.iter().enumerate() {
                if line == column {
                    break;
                }

                vec.push(main_node.get_distance(other_node));
            }

            self.pheromones.push(vec![phero_initializer; vec.len()]);
            self.distances.push(vec);
        }

        Map2D {
            nodes: self.nodes,
            distances: self.distances,
            pheromones: self.pheromones,
            direcional: PhantomData,
            initialization: PhantomData,
        }
    }
}

/* #endregion */

impl<DT: DirType, NT: NodeType> Map2D<DT, NT, Initialized> {
    pub fn get_distances(&self) -> &Vec<Vec<f64>> {
        &self.distances
    }

    pub fn get_pheromones(&self) -> &Vec<Vec<f64>> {
        &self.pheromones
    }

    pub fn update_pheromones(&mut self, pheromones: Vec<Vec<f64>>) -> Result<(), &'static str> {
        if self.pheromones.len() != pheromones.len() {
            return Err("Row count mismatch");
        }

        for (new_row, self_row) in pheromones.iter().zip(self.pheromones.iter_mut()) {
            if new_row.len() != self_row.len() {
                return Err("Column count mismatch");
            }

            for (&new_value, self_value) in new_row.iter().zip(self_row.iter_mut()) {
                *self_value = new_value;
            }
        }

        Result::Ok(())
    }
}

/* #region ANT-MAP-DISTANCES */
impl<NT: NodeType> AntMapDistances for Map2D<Unidirecional, NT, Initialized> {
    fn fetch_info(&self, start_node: usize, info: AntMapInfo) -> Vec<f64> {
        let mut distances = Vec::new();

        for node_index in 0..self.nodes.len() {
            let distance = self
                .fetch_value_from(info.clone(), start_node, node_index)
                .unwrap();

            distances.push(distance);
        }

        distances
    }

    fn fetch_value_from(
        &self,
        info: AntMapInfo,
        mut start_node: usize,
        mut end_node: usize,
    ) -> Result<f64, String> {
        let map = match info {
            AntMapInfo::Distance => &self.distances,
            AntMapInfo::Pheromone => &self.pheromones,
        };

        if start_node == end_node {
            return Ok(0.0);
        }

        if start_node < end_node {
            (start_node, end_node) = (end_node, start_node);
        }

        match map.get(start_node) {
            Some(line) => match line.get(end_node) {
                Some(dist) => Ok(dist.clone()),
                None => Err("No column".to_string()),
            },
            None => Err("No line".to_string()),
        }
    }
}

impl<NT: NodeType> AntMapDistances for Map2D<Bidirecional, NT, Initialized> {
    fn fetch_info(&self, start_node: usize, info: AntMapInfo) -> Vec<f64> { todo!() }

    fn fetch_value_from(&self, info: AntMapInfo, start_node: usize, end_node: usize) -> Result<f64, String> { todo!() }
}
/* #endregion */

/* #region ANT-MAP-NODES */
impl<DT: DirType> AntMapNodes for Map2D<DT, Priced, Initialized> {
    fn fetch_total_node_gain(&self) -> f64 {
        self.nodes.iter().map(|node| node.get_goods()).sum::<f64>()
    }

    fn fetch_node_gain(&self, idx: usize) -> f64 {
        self.nodes[idx].get_goods()
    }
}

impl<DT: DirType> AntMapNodes for Map2D<DT, Unpriced, Initialized> {
    fn fetch_total_node_gain(&self) -> f64 {
        self.nodes.len() as f64
    }

    fn fetch_node_gain(&self, idx: usize) -> f64 {
        1.0
    }
}
/* #endregion */

/* #region ANT-MAP-PHEROMONES */
impl<NT: NodeType> AntMapPheromones for Map2D<Unidirecional, NT, Initialized> {
    fn update_pheromone(&mut self, mut start_node: usize, mut end_node: usize, new_value: f64) {
        if start_node == end_node {
            return;
        }

        if start_node < end_node {
            (start_node, end_node) = (end_node, start_node);
        }

        self.pheromones[start_node][end_node] = new_value;
    }
}

impl<NT: NodeType> AntMapPheromones for Map2D<Bidirecional, NT, Initialized> {
    fn update_pheromone(&mut self, start_node: usize, end_node: usize, new_value: f64) {
        todo!()
    }
}
/* #endregion */

/* #region ANT-MAP-HELPER */
impl<NT: NodeType, IT: InitType> AntMapHelper for Map2D<Unidirecional, NT, IT> {
    fn helper_add_to_info(info: &mut Vec<Vec<f64>>, value: f64, mut start_node: usize, mut end_node: usize) {
        
        if start_node == end_node { return; }

        if start_node < end_node { (start_node, end_node) = (end_node, start_node); }

        info[start_node][end_node] += value;
    }
}

impl<NT: NodeType, IT: InitType> AntMapHelper for Map2D<Bidirecional, NT, IT>{
    fn helper_add_to_info(info: &mut Vec<Vec<f64>>, value: f64, mut start_node: usize, mut end_node: usize) {
        
        if start_node == end_node { return; }

        info[start_node][end_node] += value;
    }
}
/* #endregion */

impl<DT: DirType, NT: NodeType> AntMapGeneric for Map2D<DT, NT, Initialized> {
    fn fetch_number_of_nodes(&self) -> usize { self.nodes.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_unid_map() {
        let nodes: Vec<Node2D<Priced>> = Vec::new();
        let map = Map2D::<Unidirecional, _>::new(nodes);

        assert!(map.nodes.len() == 0);
    }

    #[test]
    fn create_loaded_unid_map() {
        let mut nodes: Vec<Node2D<Priced>> = Vec::new();
        nodes.push(Node2D::<Priced>::new(0.0, 0.0, 1.0, None));
        nodes.push(Node2D::<Priced>::new(0.0, 0.0, 1.0, None));
        nodes.push(Node2D::<Priced>::new(0.0, 0.0, 1.0, None));
        let map = Map2D::<Unidirecional, _>::new(nodes);

        assert!(map.nodes.len() == 3);
    }

    #[test]
    fn init_empty_unid_map() {
        let nodes: Vec<Node2D<Priced>> = Vec::new();
        let map = Map2D::<Unidirecional, _>::new(nodes);
        let map = map.init_map(None);

        assert!(map.distances.len() == 0);
        assert!(map.pheromones.len() == 0);
    }

    #[test]
    fn init_loaded_unid_map() {
        let mut nodes: Vec<Node2D<Unpriced>> = Vec::new();
        nodes.push(Node2D::<Unpriced>::new(0.0, 0.0, None));
        nodes.push(Node2D::<Unpriced>::new(0.0, 0.0, None));
        nodes.push(Node2D::<Unpriced>::new(0.0, 0.0, None));
        let map = Map2D::<Unidirecional, _>::new(nodes);
        let map = map.init_map(None);

        assert!(map.distances.len() == 3);
        assert!(map.distances[0].len() == 0);
        assert!(map.distances[1].len() == 1);
        assert!(map.distances[2].len() == 2);

        assert!(map.pheromones.len() == 3);
        assert!(map.pheromones[0] == vec![]);
        assert!(map.pheromones[1] == vec![1.0]);
        assert!(map.pheromones[2] == vec![1.0, 1.0]);

        let mut nodes: Vec<Node2D<Unpriced>> = Vec::new();
        nodes.push(Node2D::<Unpriced>::new(0.0, 0.0, None));
        nodes.push(Node2D::<Unpriced>::new(0.0, 0.0, None));
        nodes.push(Node2D::<Unpriced>::new(0.0, 0.0, None));
        let map = Map2D::<Unidirecional, _>::new(nodes);
        let map = map.init_map(Some(5.0));

        assert!(map.distances.len() == 3);
        assert!(map.distances[0].len() == 0);
        assert!(map.distances[1].len() == 1);
        assert!(map.distances[2].len() == 2);

        assert!(map.pheromones.len() == 3);
        assert!(map.pheromones[0] == vec![]);
        assert!(map.pheromones[1] == vec![5.0]);
        assert!(map.pheromones[2] == vec![5.0, 5.0]);
    }

    #[test]
    fn set_pheromones_ok() {
        let mut nodes: Vec<Node2D<Priced>> = Vec::new();
        nodes.push(Node2D::<Priced>::new(0.0, 0.0, 1.0, None));
        nodes.push(Node2D::<Priced>::new(0.0, 0.0, 1.0, None));
        nodes.push(Node2D::<Priced>::new(0.0, 0.0, 1.0, None));
        let map = Map2D::<Unidirecional, _>::new(nodes);
        let mut map = map.init_map(None);

        let l1: Vec<f64> = vec![];
        let l2 = vec![0.0];
        let l3 = vec![0.0, 1.0];
        let pheromones = vec![l1.clone(), l2.clone(), l3.clone()];

        let r = map.update_pheromones(pheromones);

        assert!(r == Result::Ok(()));

        assert!(map.pheromones.len() == 3);
        assert!(map.pheromones[0] == l1);
        assert!(map.pheromones[1] == l2);
        assert!(map.pheromones[2] == l3);
    }

    #[test]
    fn set_pheromones_err() {
        let mut nodes: Vec<Node2D<Priced>> = Vec::new();
        nodes.push(Node2D::<Priced>::new(0.0, 0.0, 1.0, None));
        nodes.push(Node2D::<Priced>::new(0.0, 0.0, 1.0, None));
        nodes.push(Node2D::<Priced>::new(0.0, 0.0, 1.0, None));
        let map = Map2D::<Unidirecional, _>::new(nodes);
        let mut map = map.init_map(None);

        let l1: Vec<f64> = vec![0.0];
        let l2 = vec![0.0];
        let l3 = vec![0.0, 1.0];
        let pheromones = vec![l1, l2, l3];

        let r = map.update_pheromones(pheromones);

        assert!(r == Result::Err("Column count mismatch"));

        let l1: Vec<f64> = vec![];
        let l2 = vec![0.0];
        let l3 = vec![0.0, 1.0];
        let l4 = vec![0.0, 1.0, 2.0];
        let pheromones = vec![l1, l2, l3, l4];

        let r = map.update_pheromones(pheromones);

        assert!(r == Result::Err("Row count mismatch"));
    }

    #[test]
    fn add_individual_nodes() {
        let mut nodes: Vec<Node2D<Priced>> = Vec::new();
        let mut map = Map2D::<Unidirecional, _>::new(nodes);
        map.add(0.0, 0.0, 1.0, None);
        map.add(1.0, 0.0, 1.0, None);
        map.add(0.0, 1.0, 1.0, None);

        assert!(map.nodes.len() == 3);
    }
}
