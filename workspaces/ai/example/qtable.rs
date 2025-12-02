#[derive(Resource, Debug)]
pub struct QTable {
    pub qtable: Vec<[f32; 6]>,
}

impl QTable {
    fn state_index(x: usize, y: usize, vx: usize, vy: usize) -> usize;
    fn index_to_state(mut idx: usize) -> (usize, usize, usize, usize);
    
    pub fn save_to_csv(&self, path: &str) -> std::io::Result<()>;
    pub fn load_from_csv(path: &str) -> io::Result<Self>;

    pub fn get(&self, x: usize, y: usize, vx: usize, vy: usize, a: Action) -> f32;
    pub fn set(&mut self, x: usize, y: usize, vx: usize, vy: usize, a: Action, value: f32);
    
    pub fn best_a(&self, x: usize, y: usize, vx: usize, vy: usize) -> Action;
    pub fn max_q(&self, x: usize, y: usize, vx: usize, vy: usize) -> f32;
}
