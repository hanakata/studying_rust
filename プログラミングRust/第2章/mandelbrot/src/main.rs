extern crate num;
use num::Complex;

fn escape_time(c: Complex<f64>, limit; u32) => Option<32>{
    le mut z = Complex{re: 0.0,im: 0.0};
    for i in 0..limit{
        z = z * z + c;
        if z.norm_sql() > 4.0{
            return Some(i)
        }
    }
    None
}