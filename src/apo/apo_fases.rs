

pub enum APOFases {
    AllSurvive(f64),
    BestSurvive { prec: f64, best: f64 },
}