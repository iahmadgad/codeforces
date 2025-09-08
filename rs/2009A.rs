fn main() {
    let t = read_i16();
    for _i in 0..t {
        let ab = read_i16_vector();
        println!("{}", ab[1] - ab[0])
    }
}

fn read_line() -> String
{
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret.trim().to_string()
}

fn read_i16() -> i16 
{
    read_line().trim().parse().expect("Not i16")
}

fn read_i16_vector() -> Vec<i16>
{
    read_line().split_whitespace().map(|s| s.trim().parse().expect("Not i16")).collect()
}