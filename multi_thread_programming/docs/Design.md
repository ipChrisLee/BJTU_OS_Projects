# 个人信息

学号：19211332

姓名：李远铄



# Thead control methods.

Linux提供了以下函数

```c
int pthread_create(pthread_t * thread, 					//	pointer of pthread controler
                   const pthread_attr_t * attr,			//	pointer to pthread config
                   void * (*start_routine)(void *), 	//	pointer to function
                   void *arg);							//	pointer to function arguments
void pthread_exit(void *retval);						//	pointer to save return value
int pthread_join(pthread_t thread, 						//	thread
                 void **value_ptr);						//	where to save value pointer

```





# 我的设计

## 基础信息

使用Rust实现了两个作业题。由于Rust特殊的机制，且不想使用`unsafe`，所以和C语言相比会复杂一些。



## 数独检测

因为Rust不允许一个变量传个多个线程（有race condition的风险），所以每一次都是创造一个不可变引用来避免Rust的检查。

对于答案，因为可能需要多个线程同时更改，所以需要使用Rust提供的`Arc`（atomic reference count）和`Mutex`（mutex）来避免对答案更改时的race condition。

此外，需要使用`crossbeam.scope`来允许在线程的lambda函数里创造变量。

```rust
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
```



## 并行归并排序

```rust
use crossbeam::scope;
use log::info;
use std::{fs, vec};
use std::{
    io::stdin,
    sync::{Arc, Mutex},
    thread::spawn,
};
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
```

这里就是很简单的逻辑。

一个需要注意的点是，这里不能使用C语言里常用的全局变量，或者`Arc<Mutex>`，前者是因为如此做就绕不开Rust的`unsafe`了，后者是因为，如果每一次访问都是保证互斥的话，并行的意义就会下降很多——因为访问数组的时间本来就是排序中的大头，这部分不能并行的话效率自然会下降。

这里是和前一个一样使用不可变引用规避race condition。



# 结果

## 完整程序

```rust
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
    dbg!(nums.clone());
    let n = nums.len() - 1;
    let ans = m_sort(&nums, 0, n);
    dbg!(ans);
}
pub const LOG4RS_MTP_PATH: &str = "log4rs_mtp.yaml";
fn main() {
    log4rs::init_file(LOG4RS_MTP_PATH, Default::default()).unwrap();
    let c = check_sudoku("demo/sudoku.txt");
    dbg!(c);
    check_sort("demo/array.txt");
}

```



## 输入内容

输入都是文件。



### demo/array.txt

```
10 1 4 5 7 6 3 8 0 2 9
```



### demo/sudoku.txt

```
6 2 4 5 3 9 1 8 7
5 1 9 7 2 8 6 3 4
8 3 7 6 1 4 2 9 5
1 4 3 8 6 5 7 2 9
9 5 8 2 4 7 3 6 1
7 6 2 3 9 1 4 5 8
3 7 1 9 5 6 8 4 2
4 9 6 1 8 2 5 7 3
2 8 5 4 7 3 9 1 6
```



## 输出

```
[multi_thread_programming/src/main.rs:134] c = true
[multi_thread_programming/src/main.rs:125] nums.clone() = [
    10,
    1,
    4,
    5,
    7,
    6,
    3,
    8,
    0,
    2,
    9,
]
[multi_thread_programming/src/main.rs:128] ans = [
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
]
```

可以看出程序运行正确。

