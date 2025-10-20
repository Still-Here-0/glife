use rand::seq;


#[derive(Clone)]
pub enum AntMapInfo {
    Distance,
    Pheromone
}

pub trait AntMapHelper {
    fn helper_add_to_info(info: &mut Vec<Vec<f64>>, value: f64, start_node: usize, end_node: usize);
}

pub trait AntMapGeneric {
    fn fetch_number_of_nodes(&self) -> usize;
}

pub trait AntMapPheromones {
    fn update_pheromone(&mut self, start_node: usize, end_node: usize, new_value: f64);
}

pub trait AntMapDistances {
    fn fetch_info(&self, start_node: usize, info: AntMapInfo) -> Vec<f64>;
    fn fetch_value_from(&self, info: AntMapInfo, start_node: usize, end_node: usize) -> Result<f64, String>;
}

pub trait AntMapNodes {
    fn fetch_total_node_gain(&self) -> f64;
    fn fetch_node_gain(&self, idx: usize) -> f64;
}

pub trait AntMap: AntMapDistances + AntMapNodes + AntMapPheromones + AntMapGeneric {
}

impl<T: AntMapDistances + AntMapNodes  + AntMapPheromones + AntMapGeneric> AntMap for T {}