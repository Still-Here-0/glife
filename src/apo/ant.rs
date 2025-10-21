use rand::Rng;

use crate::utils::vec_func;
use super::ant_map_traits::{ AntMapInfo, AntMap };

pub struct Ant {
    pub path: Vec<usize>,
    pub gain: f64,
    pub total_dist: f64,
    pub active: bool
}

impl Ant {
    pub fn new() -> Self {
        Ant { path: Vec::new(), gain: 0.0, total_dist: 0.0, active: true }
    }

    pub fn resolve_pathing(
        &mut self, 
        ref_map: &dyn AntMap,
        start_node_index: usize, 
        norm_greed: f64,
        dist_coef: &f64,
        phero_coef: &f64,
        unreachable: &Vec<usize>,
    ) {
        self.gain = ref_map.fetch_node_gain(start_node_index);
        let mut current_node_index = start_node_index;
        let num_nodes = ref_map.fetch_number_of_nodes();
        let total_gain = ref_map.fetch_total_node_gain();
        
        self.path.push(current_node_index);

        let mut unique_unreachable: Vec<usize> = unreachable.clone();   // All nodes pathed by the slot
        unique_unreachable.extend(self.path.iter());
        unique_unreachable = vec_func::remove_duplicates(unique_unreachable);

        while self.gain/total_gain < norm_greed && unique_unreachable.len() < num_nodes {
            let next_node_index = self.move_from(current_node_index, ref_map, dist_coef, phero_coef, &unique_unreachable);
            
            self.total_dist += ref_map.fetch_value_from(AntMapInfo::Distance, current_node_index, next_node_index).unwrap();
            self.gain += ref_map.fetch_node_gain(next_node_index);
            
            unique_unreachable.push(next_node_index);
            self.path.push(next_node_index);

            current_node_index = next_node_index;
        };

        // Get back to the start node
        self.total_dist += ref_map.fetch_value_from(AntMapInfo::Distance, current_node_index, start_node_index).unwrap();
        self.path.push(start_node_index); 
    }

    fn move_from(
        &self,
        node_index: usize, 
        ref_map: &dyn AntMap,
        dist_coef: &f64, 
        phero_coef: &f64,
        unreachable: &Vec<usize>,
    ) -> usize {

        let distances = ref_map.fetch_info(node_index, AntMapInfo::Distance);
        let pheromones = ref_map.fetch_info(node_index, AntMapInfo::Pheromone);
        
        let (advanceability, desirability) = self.get_meta_parameters(distances, pheromones, dist_coef, phero_coef, unreachable);
        
        let spaced_prob = self.get_move_spaced_probabilities(advanceability, desirability);

        let move_to = self.chose_move(spaced_prob).unwrap();

        move_to
    }
    
    fn get_meta_parameters(
        &self, 
        distances: Vec<f64>, 
        pheromones: Vec<f64>, 
        &dist_coef: &f64, 
        &phero_coef: &f64,
        unreachable: &Vec<usize>,
    ) -> (Vec<f64>, Vec<f64>) {
        let mut advanceability = Vec::new();
        let mut desirability = Vec::new();

        for (idx, (&dist, &phero)) in distances.iter().zip(pheromones.iter()).enumerate() {
            
            if unreachable.contains(&idx) || self.path.contains(&idx) { 
                advanceability.push(0.0);
                desirability.push(0.0);
                continue;
            }
            
            advanceability.push((if dist != 0.0 { 1.0/dist } else { 0.0 }).powf(dist_coef));
            desirability.push(phero.powf(phero_coef));
        }

        (advanceability, desirability)
    }

    fn get_move_spaced_probabilities(&self, advanceability: Vec<f64>, desirability: Vec<f64>) -> Vec<f64> {
        let combined: Vec<f64> = advanceability.iter().zip(desirability.iter()).map(|(adv, des)| adv + des).collect();
        let comb_sum: f64 = combined.iter().sum();
        let mov_prob: Vec<f64> = combined.iter().map(|x| x / comb_sum).collect();
        let sum_prob: f64 = mov_prob.iter().sum();
        
        let mut mov_spaced_prob = Vec::with_capacity(mov_prob.len());
        let mut acc = 0.0;

        for &prob in mov_prob.iter() {
            acc += prob/sum_prob;
            mov_spaced_prob.push(acc);
        }

        mov_spaced_prob.sort_by(|a, b| a.total_cmp(b)); // ASC

        mov_spaced_prob
    }

    fn chose_move(&self, spaced_prob: Vec<f64> ) -> Result<usize, &'static str> {

        let choice = rand::rng().random_range(0.0..1.0);

        for (idx, &prob) in spaced_prob.iter().enumerate() {
            
            if choice <= prob {
                return Ok(idx);
            }

        }

        Err("No result found")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
}