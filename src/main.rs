mod time_data;

use time_data::TimeData;

fn main() {
    let t_d = TimeData::default();
    let t_d1 = TimeData::new(0,1,31,99);
    println!("{}", t_d);
    println!("{}", t_d1);
    println!("{}", t_d1 + t_d1);
}

#[cfg(test)]
mod tests {
    use crate::time_data::TimeData;    

    #[test]
    fn test_add1() {
        let t1 = TimeData::new(0,1,31,99);
        assert_eq!(t1+t1, TimeData::new(0,3,2,198));
    }

    #[test]
    fn test_add2() {
        let t1 = TimeData::new(0,0,0,500);
        assert_eq!(t1+t1, TimeData::new(0,0,1,0));
    }

    #[test]
    fn test_add3() {
        let t1 = TimeData::new(0,1,20,256);
        let t2 = TimeData::new(0,2,26,812);
        assert_eq!(t1+t2, TimeData::new(0,3,47,68));
    }
    
    #[test]
    fn test_add4() {
        let t1 = TimeData::new(1,51,29,500);
        let t2 = TimeData::new(0,9,30,501);
        assert_eq!(t1+t2, TimeData::new(2,1,0,1));
    }
}
