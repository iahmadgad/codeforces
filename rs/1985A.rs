fn main() {
    let t = read_i16();
    for _i in 0..t {
        let string_vector = read_string_vector();
        let (a, b) = (string_vector[0].clone(), string_vector[1].clone());
        println!("{}{} {}{}", &b[..1], &a[1..], &a[..1], &b[1..]);
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

fn read_string_vector() -> Vec<String>
{
    read_line().split_whitespace().map(|s| String::from(s.trim())).collect()
}