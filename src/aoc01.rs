pub(crate) fn run(data: &[u8]) -> String {
    let mut p1: u32 = 0;
    let mut p1_last = 9999;

    let mut p2: i32 = -3;
    let mut p2_acc = 0;
    let mut p2_wnd = [0, 0, 0];
    let mut p2_wnd_i = 0;

    let mut acc: u16 = 0;

    data.iter().for_each(|b| {
        if *b == '\n' as u8 {
            if acc > p1_last {
                //p1 += 1;
                p1 = p1.wrapping_add(1);
            }
            p1_last = acc;

            //println!("{}, {}", p2_acc, acc);
            let p2_new_acc: u16 = p2_acc - p2_wnd[p2_wnd_i] + acc;
            if p2_new_acc > p2_acc {
                //p2 += 1;
                p2 = p2.wrapping_add(1);
                //println!("+{}", p2);
            }
            p2_acc = p2_new_acc;
            p2_wnd[p2_wnd_i] = acc;
            p2_wnd_i += 1;
            p2_wnd_i %= 3;

            acc = 0;
        } else {
            acc *= 10;
            acc += (b - '0' as u8) as u16;
        }
    });

    format!("{} {}\n", p1, p2)
}
