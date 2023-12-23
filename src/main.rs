use std::{fs, collections::HashMap, env::args};
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
        let prefix_vector: Vec<(String,Vec<usize>)> = index_map.into_iter().collect();
        let thread_return:Vec<(String,&str)> = prefix_vector.into_par_iter().map(|(key,value)| {
            let mut thread_best_prob = 0.0;
            let mut word = "";
            let counter_pre = value.len() as f32;
            let mut counter_full:f32 = 0.0;
            for e in value.iter() {
                let full = Ngram::convert_list_to_string(&data[*e..(*e+self.n)]);
                for d in value.iter() {
                    if full == Ngram::convert_list_to_string(&data[*d..(*d+self.n)]) {
                        counter_full += 1.0;
                    }
                }
                let prob = counter_full/counter_pre;
                if thread_best_prob < prob {
                    thread_best_prob = prob;
                    word = data[e+self.n-1];
                }
            }    
            (key,word)    
        }).collect();
        for (key,val) in thread_return.iter() {
            self.prefix.insert(key.clone(), String::from(*val));
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
    let args: Vec<String> = args().collect();
    let mut arg = String::new();
    for i in 1..args.len() {
        arg.push_str(&args[i]);
        arg.push_str(" ");
    }
    if let Some(x) = arg.strip_suffix(" ") {
        let start_seq = String::from(x);
        let n = Ngram::split_into_words(&start_seq).len()+1;
        let mut ng = Ngram::new(n);
        if let Ok(x) = fs::read_to_string("./data/bible.txt") {
            let data = x.to_lowercase();
            if data.contains(&start_seq) {
                println!("start sequence found");
                ng.train(Ngram::split_into_words(&data));
                println!("training finished");
                let out = ng.create_new_sequenze(&String::from(start_seq), 1000);
                fs::write("./data/output.txt", out).expect("Unable to write file");
                print!("done")
            } else {
                println!("start sequence not found");
                return;
            }
        } else {
            println!("Error: file not found");
            return;
        }
    } else {
        println!("Error: no input");
        return;
    }
}
