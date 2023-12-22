use std::{fs, collections::HashMap};
use rayon::prelude::*;
struct Ngram {
    n: usize,
    prefix: HashMap<String,String>
}

impl Ngram {
    fn new(n: usize) -> Ngram {
        return Ngram {
            n,
            prefix: HashMap::new(),
        };
    }


    fn train(&mut self, data: Vec<&str>) {
        let mut index_map: HashMap<String,Vec<usize>> = HashMap::new();
        let data_len = data.len()-self.n;
        for i in 0..(data_len) {            
            let prefix = Ngram::convert_list_to_string(&data[i..(i+self.n-1)]);
            match index_map.get(&prefix) {
                Some(x) => {
                    let mut tmp = x.clone();
                    tmp.push(i);
                    index_map.insert(prefix, tmp);
                },
                None => {index_map.insert(prefix, vec![i]);},
            }
        }
        // let prefix_vector: Vec<(String,Vec<usize>)> = index_map.into_iter().collect();
        let thread_return:Vec<HashMap<String,String>> = (0..10).into_par_iter().map(|t| {
            let start_index = data_len/10*t;
            let mut thread_best_prob:HashMap<String,f32> = HashMap::new();
            let mut thread_prefix:HashMap<String,String>  = HashMap::new();
            for i in start_index..(data_len/10+start_index) {
                let word = &data[i+self.n-1];
                let prefix = Ngram::convert_list_to_string(&data[i..(i+self.n-1)]);
                let full = Ngram::convert_list_to_string(&data[i..(i+self.n)]);
                if let Some(index_vector) = index_map.get(&prefix) {
                    let counter_pre = index_vector.len() as f32;
                    let mut counter_full = 0.0;
                    for j in index_vector {
                        if full == Ngram::convert_list_to_string(&data[*j..(*j+self.n)]) {
                            counter_full += 1.0;
                        }
                    }
                    let prob:f32 = counter_full / counter_pre;
                    match thread_best_prob.get(&prefix) {
                        Some(x) => {
                            if x < &prob {
                                thread_best_prob.insert(prefix.clone(), prob);
                                thread_prefix.insert(prefix.clone(), String::from(*word));
                            }
                        }
                        None => {
                            thread_best_prob.insert(prefix.clone(), prob);
                            thread_prefix.insert(prefix.clone(), String::from(*word));
                        }
                    }
                    
                }
            }
            thread_prefix
        }).collect();
        for map in thread_return.iter() {
            for (key,value) in map.iter() {
                self.prefix.insert(key.clone(), value.clone());
            }
        }
                
    }

    fn create_new_sequenze(&self, start_seq: &String, lenght: usize) -> String {
        let mut start_seq = start_seq.clone();
        start_seq.push_str(" ");
        let mut term = start_seq.clone();
        let mut out = start_seq.clone();
        for _ in 0..lenght {
            if let Some(next) = self.prefix.get(&term) {
                
                term = Ngram::convert_list_to_string(&Ngram::split_into_words(&term)[1..]);
                term.push_str(next);
                term.push_str(" ");
                out.push_str(next);
                out.push_str(" ");
            }
        }
        return out;
    }

    fn convert_list_to_string(data: &[&str]) -> String {
        let mut out:String = String::new(); 
        for i in data.iter() {
            out.push_str(i);
            out.push(' ');
        }
        return out;
    }

    fn split_into_words(data: &str) -> Vec<&str> {
        data.split_whitespace().collect()
    }
}


fn main() {
    let mut ng = Ngram::new(3);
    let data = fs::read_to_string("./data/bible.txt").unwrap();
    let data = data.to_lowercase();
    ng.train(Ngram::split_into_words(&data));
    println!("train finished");
    let out = ng.create_new_sequenze(&String::from("gott ist"), 1000);
    
    fs::write("./data/output.txt", out).unwrap();
    print!("done")
}
