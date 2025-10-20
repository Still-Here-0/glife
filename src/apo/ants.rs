
use std::rc::Rc;

use super::ant::Ant;
use super::ant_map_traits::AntMap;
use super::ant_maps::_2d::map_2d::Map2D;
use super::ant_maps::types::node::NodeType;
use super::ant_maps::types::map::direcional::DirType;
use super::ant_maps::types::map::initialization::Initialized;


pub struct Ants {
    slots: Vec<Vec<Ant>>, // Make ant group, that has Vec<Ant>, total_distance, etc (all parameters of the group)
    quantity: u32,
    siblens_greed: Vec<f64>,

    // APO decision parameters
    distance_coef: f64,
    pheromone_coef: f64,
}

impl Ants {
    pub fn new(quantity: u32, siblens: Vec<f64>, distance_coef: Option<f64>, pheromone_coef: Option<f64>) -> Ants {
        let distance_coef = distance_coef.unwrap_or(2.0);
        let pheromone_coef = pheromone_coef.unwrap_or(2.0);
        
        Ants { 
            slots: Vec::new(), 
            quantity, 
            siblens_greed: siblens,
            distance_coef,
            pheromone_coef,
        }
    }

    pub fn path_all(&mut self, ref_map: &dyn AntMap, start_node_index: usize) {

        for slot in self.slots.iter_mut() {
            let mut unreachable_nodes = Vec::new();
            let total_greed = self.siblens_greed.iter().sum::<f64>();

            for (ant, &greed) in slot.iter_mut().zip(self.siblens_greed.iter()) {
                let norm_greed = greed/total_greed;

                ant.resolve_pathing(
                    ref_map, 
                    start_node_index, 
                    norm_greed, 
                    &self.distance_coef,
                    &self.pheromone_coef, 
                    &unreachable_nodes
                );

                unreachable_nodes.extend(ant.path.clone());
            }
        }
    }

    pub fn setup_ants(&mut self) {
        self.slots.clear();

        for _ in 0..self.quantity {
            let mut slot = Vec::new();
            
            for greed in self.siblens_greed.iter() {
                let ant = Ant::new();
                slot.push(ant);
            }

            self.slots.push(slot);
        }
    }

    pub fn get_quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        self.quantity = quantity;
    }

    pub fn get_sliblens(&self) -> &Vec<f64> {
        &self.siblens_greed
    }

    pub fn set_siblens(&mut self, siblens: Vec<f64>) {
        self.siblens_greed = siblens;
    }

    pub fn get_ants(&self) -> &Vec<Vec<Ant>> {
        &self.slots
    }

    pub fn deactivate_ants(&mut self, indx: usize) {
        let mut ants = &mut self.slots[indx];

        for ant in ants.iter_mut() {
            ant.active = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; 
}