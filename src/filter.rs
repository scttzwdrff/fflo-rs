use rand::random;
use rand::Rng;
use libm::tanh;
use crate::global;
use crate::field::*;
use crate::fflo::*;

pub struct Filter {
	pub mask : Vec<f64>,
    pub targets : Vec<(i64,i64)>,
}
impl Filter {
    pub fn field_from_filter(&self, field : &Field) -> Field {
        let mut cells = vec![0.0f64; field.rows*field.cols];
        for r in 0..field.rows {
            for c in 0..field.cols {
                cells[(r*field.cols +c) as usize] = self.of_cell(field, r as i64, c as i64);
            }
        }
        Field {
            rows : field.rows,
            cols : field.cols,
            cells
        }
    }
    pub fn of_cell(&self, field : &Field, row : i64, col: i64) -> f64 {
        let mut sum = 0.0f64;
        for (i, t) in self.targets.iter().enumerate() {
            sum += field.get(row + t.0,col + t.1)*self.mask[i];
        }
        tanh(sum)
    }
}

impl Fflo {
    pub fn filter(&self, kind : &str) -> Filter {
        let mut targets = vec![];
        let mut rng = rand::thread_rng();
        let row_span = rng.gen_range(0..self.row_span) as i64 + 1;
        let col_span = rng.gen_range(0..self.col_span) as i64 + 1;
        match kind {
            "rect" => targets = rect_target_set(row_span, col_span),
            "jagged" => targets = jagged_target_set(self.targets, row_span, col_span),
            _ => targets = rect_target_set(row_span, col_span),
        }
        let mut mask = vec![0.0;targets.len()];
        for x in mask.iter_mut() {
            *x = self.mask_intensity*(1.0 - 2.0*rand::random::<f64>());
        }
        Filter {
            mask,
            targets,
        }
    }
    pub fn random_filters(&self, kind : &str, num : usize) -> Vec<Filter> {
        let mut filters = vec![];
        for _ in 0..num {
            filters.push(self.filter(kind));
        }
        filters
    }
}

fn jagged_target_set(num_targets :  usize, row_span :  i64, col_span : i64) -> Vec<(i64,i64)> {
    let mut rng = rand::thread_rng();
    let mut targets = vec![];
    for i in 0..num_targets {
        targets.push((
            rng.gen_range(0..row_span as usize) as i64,
            rng.gen_range(0..col_span as usize) as i64
        ));
    }
    targets
}

fn rect_target_set(row_span :  i64, col_span : i64) -> Vec<(i64,i64)> {
    let counter = 0usize;
    let mut targets : Vec<(i64,i64)> = vec![]; 
    for row in -row_span..=row_span {
        for col in -col_span..=col_span {
            targets.push((row,col));
        }
    } 
    targets
}


