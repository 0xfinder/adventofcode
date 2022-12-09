use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let lines: Vec<&str> = input.split("\r\n").collect();

    let mut count = 0;

    for line in lines {
        let v: Vec<_> = line.split(&[',', '-'][..]).collect();
        println!("{:?}", v);
        let v: Vec<i32> = v.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
        // if (v[0] <= v[2] && v[1] >= v[3]) || (v[0] >= v[2] && v[1] <= v[3]) {
        //     count += 1;
        // }
        // if (v[0] <= v[3] && v[2] <= v[1]) || (v[2] <= v[1] && v[0] <= v[3]) {
        //     count += 1;
        // }
        let range1 = v[0]..v[1];
        let range2 = v[2]..v[3];

        if range1.all(|x| range2.contains(&x)) || range2.all(|x| range2.contains(&x)) {
            count += 1;
        }
    }

    println!("{}", count);
}
