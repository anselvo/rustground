use std::fmt::Debug;

pub mod binary_search {
    pub mod e_35;
    pub mod e_69;
    pub mod m_33;
    pub mod m_34;
    pub mod m_74;
    pub mod m_81;
    pub mod h_4;
}

pub mod bfs {
    pub mod e_101;
    pub mod e_102;
    pub mod e_103;
    pub mod e_104;
    pub mod e_222;
}

pub mod dfs {
    pub mod e_21;
    pub mod e_94;
    pub mod e_100;
    pub mod m_79;
    pub mod m_98;
    pub mod m_99;
    pub mod e_101;
    pub mod e_104;
    pub mod e_222;
}

pub mod dp {
    pub mod e_70;
    pub mod m_718;
}

pub mod math {
    pub mod e_9;
    pub mod e_13;
    pub mod e_66;
    pub mod e_67;
    pub mod e_3110;
}

pub mod string {
    pub mod e_14;
    pub mod e_20;
    pub mod e_28;
    pub mod e_58;
    pub mod m_187;
    pub mod e_3042;
}

pub mod structure {
    pub mod m_307;
    pub mod e_705;
    pub mod e_706;
}

pub mod x {
    pub mod e_26;
    pub mod e_27;
}

/* TEMPLATE */
pub fn task_name() -> String {
    return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(task_name(), "".to_string());
    }
}

/* UTILS */

fn print_2d_vec<T : Debug>(vec: Vec<Vec<T>>) {
    println!();
    print!("  ");
    for j in 0..vec[0].len() {
        print!("{} ", j);
    }
    println!();
    for i in 0..vec.len() {
        print!("{} ", i);
        for j in 0..vec[i].len() {
            print!("{:?} ", vec[i][j]);
        }
        println!();
    }
}
