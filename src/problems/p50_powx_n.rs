
pub fn my_pow(x: f64, n: i32) -> f64 {

    pub struct I32Bits {
        value:i64,
        bit_no:u8,
    };

    impl I32Bits {

        fn new(v:i32) -> Self {
            Self {
                value:v as i64,
                bit_no : 0,
            }
        }

        fn next_bit(&mut self) -> Option<bool> {


            if self.bit_no > 63 {
                return None;
            }

            let mask = 1 << (63- self.bit_no) ;
            self.bit_no  += 1;
            // println!("value={:#066b}",self.value);
            // println!("mask={:#066b}",mask);

            if mask & self.value != 0 {
                Some(true)
            } else {
                Some(false)
            }

        }

        fn toggle(&mut self) -> bool{
            println!("value={:#066b}",self.value);
            if self.value < 0 {
                self.value = -self.value;
                true
            } else {
                false
            }
        }

    }

    let mut result = 1.0;

    let v:i32 = i32::MIN;
    let mut i  = I32Bits::new(n);
    let minus = i.toggle();

    while let Some(f) = i.next_bit() {
        result *= result;
        if f {
            result *= x;
        } 
    }

    if minus {
        result = 1.0/result;
    }

    result

}

#[test]
fn test() {

    println!("{}",my_pow(2.00000,-2)); // 0.2500
    println!("{}",my_pow(2.10000,3)); // 9.26100
    println!("{}",my_pow(2.00000,10)); // 1024.000000
    println!("{}",my_pow(0.00001, 2147483647));

    /*
    let i:i32 = 1;
    let j = i<<31 ;
    println!("{}",j);
    println!("{}",j >>31);
    println!("{}",j >>30);
    */




    // println!("{}",23431 & 1);
}