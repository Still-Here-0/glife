
use super::ants::Ants;
use super::apo_fases::APOFases;
use super::ant_map_traits::AntMap;
use super::ant_maps::_2d::map_2d::Map2D;
use super::ant_maps::_2d::node_2d::Node2D;
use crate::apo::ant_map_traits::{ AntMapDistances, AntMapInfo, AntMapHelper };
use super::ant_maps::types::node::{ Priced, Unpriced, NodeType };
use super::ant_maps::types::map::direcional::{ Unidirecional, Bidirecional, DirType };
use super::ant_maps::types::map::initialization::{ Initialized, Uninitialized, InitType };

#[derive(Clone, Debug)]
pub struct ApoResult {
    path: Vec<Vec<usize>>,
    distance: f64
}

pub struct Apo2D<DT: DirType, NT: NodeType, IT: InitType> {
    owned_map: Map2D<DT, NT, IT>,
    ants: Ants,
    global_best: ApoResult,
    
    // APO meta parameters
    min_phero: f64,
    max_phero: Option<f64>,
    max_iteration: u32,

    // APO direct parameters
    phero_distribution: u32,
    evaporation_rate: f64,

    // APO extra parameters
    fases: Vec<APOFases>,
}

/* #region UNINITIALIZED APO */

/* #region DIRECTION TYPE AND NODE TYPE SPECIFIC  */
impl Apo2D<Unidirecional, Priced, Uninitialized> {
    pub fn new_unidi_priced(
        nodes: Vec<Node2D<Priced>>, 
        ant_quantity: Option<u32>, 
        ant_siblens_greed: Option<Vec<f64>>,
        distance_coef: Option<f64>, 
        pheromone_coef: Option<f64>,
    ) -> Self {
        let map = Map2D::<Unidirecional, Priced>::new(nodes);
        
        Apo2D::set_default(map, ant_quantity, ant_siblens_greed, distance_coef, pheromone_coef)
    }
}

impl Apo2D<Bidirecional, Priced, Uninitialized> {
    pub fn new_bidi_priced(nodes: Vec<Node2D<Priced>>) -> Self { todo!() }
}

impl Apo2D<Unidirecional, Unpriced, Uninitialized> {
    pub fn new_unidi_unpriced(
        nodes: Vec<Node2D<Unpriced>>, 
        ant_quantity: Option<u32>, 
        ant_siblens_greed: Option<Vec<f64>>,
        distance_coef: Option<f64>, 
        pheromone_coef: Option<f64>,
    ) -> Apo2D<Unidirecional, Unpriced, Uninitialized> {
        let map = Map2D::<Unidirecional, Unpriced>::new(nodes);
        
        Apo2D::set_default(map, ant_quantity, ant_siblens_greed, distance_coef, pheromone_coef)
    }
}

impl Apo2D<Bidirecional, Unpriced, Uninitialized> {
    pub fn new_bidi_unpriced(nodes: Vec<Node2D<Unpriced>>) -> Self { todo!() }
}
/* #endregion */

/* #region NODE SPECIFIC */
impl<DT: DirType> Apo2D<DT, Priced, Uninitialized> {
    pub fn add_node(&mut self, x: f64, y: f64, price: f64, name: Option<String>) {
        self.owned_map.add(x, y, price, name);
    }
}

impl<DT: DirType> Apo2D<DT, Unpriced, Uninitialized> {
    pub fn add_node(&mut self, x: f64, y: f64, name: Option<String>) {
        self.owned_map.add(x, y, name);
    }
}
/* #endregion */

/* #region GENERIC */
impl<DT: DirType, NT: NodeType> Apo2D<DT, NT, Uninitialized> {
    fn set_default(
        map: Map2D<DT, NT>, 
        ant_quantity: Option<u32>, 
        ant_siblens_greed: Option<Vec<f64>>, 
        distance_coef: Option<f64>, 
        pheromone_coef: Option<f64>,
    ) -> Self {
        let ant_quantity = ant_quantity.unwrap_or(100);
        let ant_siblens_greed = ant_siblens_greed.unwrap_or(vec![1.0]);
        
        let mut ants = Ants::new(ant_quantity, ant_siblens_greed, distance_coef, pheromone_coef);

        let fases = vec![
            APOFases::BestSurvive { prec: 0.2, best: 0.15 },
            APOFases::AllSurvive(0.9),
            APOFases::BestSurvive { prec: 1.0, best: 0.15 }
        ];

        Apo2D { 
            owned_map: map,
            ants,
            global_best: ApoResult { path: Vec::new(), distance: f64::MAX },
            min_phero: 0.1,
            max_phero: Some(1.0e15), 
            max_iteration: 1_000,
            phero_distribution: 1,
            evaporation_rate: 0.15,
            fases,
        }
    }

    pub fn set_ant_quantity(&mut self, quantity: u32) {
        self.ants.set_quantity(quantity);
    }

    pub fn set_ant_siblens(&mut self, siblens: Vec<f64>) {
        self.ants.set_siblens(siblens);
    }

    pub fn init_map(self, phero_initializer: Option<f64>) -> Apo2D<DT, NT, Initialized> {
        let map = self.owned_map.init_map(phero_initializer);

        Apo2D {
            owned_map: map,
            ants: self.ants,
            global_best: self.global_best,
            min_phero: self.min_phero,
            max_phero: self.max_phero,
            max_iteration: self.max_iteration,
            phero_distribution: self.phero_distribution,
            evaporation_rate: self.evaporation_rate,
            fases: self.fases,
        }
    }
}
/* #endregion */

/* #endregion */

/* #region INITIALIZED APO */

impl<DT: DirType + 'static, NT: NodeType + 'static> Apo2D<DT, NT, Initialized> 
where
    Map2D<DT, NT, Initialized>: AntMap + AntMapHelper
{
    pub fn find_best_path(mut self, start_node: usize) -> ApoResult {
        
        for it in 0..self.max_iteration {
            let ref_map = &self.owned_map as &dyn AntMap;

            self.ants.setup_ants();
            self.ants.path_all(ref_map, start_node);
            self.update_global_best();
            self.parse_ants(it);
            self.update_pheromones();
        }

        self.global_best
    }

    fn get_solted_results(&self) -> Vec<ApoResult> {
        let slots = self.ants.get_ants();

        let mut slot_results = Vec::<ApoResult>::new();
        for slot in slots.iter() {
            let mut distance = 0.0;
            let mut path = Vec::new();
            for ant in slot.iter() {
                distance += ant.total_dist;
                path.push(ant.path.clone());
            }
            slot_results.push(ApoResult { path, distance });
        }

        slot_results.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap()); // ASC

        slot_results
    }

    fn update_global_best(&mut self) {
        let sorted_results = self.get_solted_results();
        let local_best = &sorted_results[0];

        if local_best.distance < self.global_best.distance {
            self.global_best = (*local_best).clone();
        }
    }

    fn parse_ants(&mut self, iteration: u32) {
        let current_perc = iteration as f64 / self.max_iteration as f64;

        let fase_to_run: APOFases;
        for fase in self.fases.iter() {
            match fase {
                APOFases::BestSurvive { prec, best } => {
                    if current_perc <= *prec { self.parse_best_ants(*best); return; }
                },
                APOFases::AllSurvive(prec) => {
                    if current_perc <= *prec { return; }

                }
            }
        }
    }

    fn parse_best_ants(&mut self, best_perc: f64) {
        let slots = self.ants.get_ants();

        let sorted_results = self.get_solted_results();

        let worst_of_the_best_idx = ((sorted_results.len() as f64)*best_perc).ceil() as usize;
        let max_distance = sorted_results[worst_of_the_best_idx].distance;

        let mut deactivate_list = Vec::new();
        for (indx, slot) in slots.iter().enumerate() {
            
            let mut distance = 0.0;
            for ant in slot.iter() {
                distance += ant.total_dist;
            }
            
            if distance > max_distance {
                deactivate_list.push(indx);
            }
        }

        for indx in deactivate_list {
            self.ants.deactivate_ants(indx);
        }
    }

    fn update_pheromones(&mut self) {
        
        let mut new_pheromones = self.owned_map.get_empty_map_info();
        
        // Add new pheromone
        for slots in self.ants.get_ants() {
            if !slots[0].active { continue; }

            for ant in slots {
                let mut start_node = ant.path[0];
                for &next_node in ant.path[1..].iter() {
                    let distance = self.owned_map.fetch_value_from(AntMapInfo::Pheromone, start_node, next_node).unwrap();

                    let pheromone_placed = (self.phero_distribution as f64)*distance/ant.total_dist; 
                    Map2D::helper_add_to_info(&mut new_pheromones, pheromone_placed, start_node, next_node);

                    start_node = next_node;
                }
            }
        }

        // Add old pheromone
        for (start_node, slot) in self.owned_map.get_pheromones().iter().enumerate() {
            for (end_node, old_pheromone) in slot.iter().enumerate() {
                let remaining_pheromone = (1.0 - self.evaporation_rate)*old_pheromone;
                Map2D::helper_add_to_info(&mut new_pheromones, remaining_pheromone, start_node, end_node);
            }
        }

        // Check bounds
        for paths in new_pheromones.iter_mut() {
            for trail in paths.iter_mut() {
                *trail = (*trail).max(self.min_phero);

                match self.max_phero {
                    Some(max_value) => *trail = (*trail).min(max_value),
                    None => {},
                }
            }
        }

        self.owned_map.update_pheromones(new_pheromones);
    }

}

/* #endregion */

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn priced_full_use_case() {
        let nodes = vec![
            Node2D::<Priced>::new(0.0, 0.0, 0.0, None),
            Node2D::<Priced>::new(0.0, 1.0, 1.0, None),
            Node2D::<Priced>::new(0.0, 2.0, 1.0, None),
            Node2D::<Priced>::new(1.0, 2.0, 1.0, None),
            Node2D::<Priced>::new(2.0, 2.0, 1.0, None),
            Node2D::<Priced>::new(2.0, 1.0, 1.0, None),
            Node2D::<Priced>::new(2.0, 0.0, 1.0, None),
            Node2D::<Priced>::new(1.0, 0.0, 1.0, None),
        ];
        let apo = Apo2D::new_unidi_priced(
            nodes,
            None, 
            None, 
            None, 
            None
        );

        let apo = apo.init_map(None);
        
        let result = apo.find_best_path(0);

        assert_eq!(result.distance, 8.0);
        assert_eq!(result.path[0].len(), 9);
    }

}