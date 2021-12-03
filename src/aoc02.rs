pub(crate) fn run(data: &[u8]) -> String {
    let mut p1_dep: u32 = 0;
    let mut p1_pos: u32 = 0;
    let mut p2_aim: u32 = 0;
    let mut p2_dep: u32 = 0;

    let mut i = 0;
    while i < data.len() {
        let op = data[i] as char;
        i += match op {
            'f' => 8,
            'd' => 5,
            'u' => 3,
            _ => unreachable!(),
        };

        let val = data[i] as u32 - '0' as u32;
        match op {
            'f' => {
                p1_pos += val;
                p2_dep += val * p2_aim;
            }
            'd' => {
                p1_dep += val;
                p2_aim += val;
            }
            'u' => {
                p1_dep -= val;
                p2_aim -= val;
            }

            _ => unreachable!(),
        }
        i += 2;
        //
    }

    let p1 = p1_dep * p1_pos;
    let p2 = p2_dep * p1_pos;

    format!("{} {}\n", p1, p2)
}
