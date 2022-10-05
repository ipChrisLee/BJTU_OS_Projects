use crossbeam::scope;
use log::info;
use std::{fs, vec};
use std::{
    io::stdin,
    sync::{Arc, Mutex},
    thread::spawn,
};
fn check_sudoku(filepath: &str) -> bool {
    let mut mat = [[0i32; 9]; 9];
    let contents = fs::read_to_string(filepath).unwrap();

    let mut nums = contents.split(['\n', ' ']);

    for i in 0..9 {
        for j in 0..9 {
            let s = nums.next().unwrap();
            // dbg!(s);
            mat[i][j] = s.parse::<i32>().unwrap();
        }
    }
    let mat = &mat;
    let ans = Arc::new(Mutex::new(true));
    scope(|s| {
        s.spawn(|_| {
            let ans = ans.clone();
            for i in 0..9 {
                let mut ocr = [false; 10];
                for j in 0..9 {
                    ocr[mat[i][j] as usize] = true;
                }
                for j in 1..10 {
                    *ans.lock().unwrap() &= ocr[j];
                }
            }
        });
        s.spawn(|_| {
            let ans = ans.clone();
            for j in 0..9 {
                let mut ocr = [false; 10];
                for i in 0..9 {
                    ocr[mat[i][j] as usize] = true;
                }
                for i in 1..10 {
                    *ans.lock().unwrap() &= ocr[i];
                }
            }
        });
        for r in 0..9 {
            if r % 3 == 0 {
                for c in 0..9 {
                    if c % 3 == 0 {
                        let ans = ans.clone();
                        s.spawn(move |_| {
                            let ans = ans.clone();
                            let mut ocr = [false; 10];
                            for i in 0..3 {
                                for j in 0..3 {
                                    ocr[mat[r + i][c + j] as usize] = true;
                                }
                            }
                            for i in 1..10 {
                                *ans.lock().unwrap() &= ocr[i];
                            }
                        });
                    }
                }
            }
        }
    })
    .unwrap();
    let b = *(ans.lock().unwrap());
    b
}

fn m_sort(vec: &Vec<i32>, st: usize, ed: usize) -> Vec<i32> {
    info!("{} {}", st, ed);
    if st == ed {
        vec![vec[st]]
    } else {
        let md = (st + ed) / 2;
        let mut vec_l = Vec::new();
        let mut vec_r = Vec::new();
        scope(|s| {
            s.spawn(|_| {
                vec_l = m_sort(vec, st, md);
            });
            s.spawn(|_| {
                vec_r = m_sort(vec, md + 1, ed);
            });
        })
        .unwrap();
        let mut vec_a = Vec::<i32>::new();
        let n_vec_l = vec_l.len();
        let n_vec_r = vec_r.len();
        let mut i = 0;
        let mut j = 0;
        while i < n_vec_l && j < n_vec_r {
            if vec_l[i] < vec_r[j] {
                vec_a.push(vec_l[i]);
                i = i + 1;
            } else {
                vec_a.push(vec_r[j]);
                j = j + 1;
            }
        }
        while i < n_vec_l {
            vec_a.push(vec_l[i]);
            i = i + 1;
        }
        while j < n_vec_r {
            vec_a.push(vec_r[j]);
            j = j + 1;
        }
        vec_a
    }
}
fn check_sort(filepath: &str) {
    let contents = fs::read_to_string(filepath).unwrap();
    let nums = contents.split([' ', '\n']);
    let nums: Vec<_> = nums
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    dbg!("{}", nums.clone());
    let n = nums.len() - 1;
    let ans = m_sort(&nums, 0, n);
    dbg!("{}", ans);
}
pub const LOG4RS_MTP_PATH: &str = "log4rs_mtp.yaml";
fn main() {
    log4rs::init_file(LOG4RS_MTP_PATH, Default::default()).unwrap();
    let c = check_sudoku("demo/sudoku.txt");
    println!("{}", c);
    check_sort("demo/array.txt");
}
