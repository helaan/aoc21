pub(crate) fn run(data: &[u8]) -> String {
    let mut p1_cnt: [u16; 12] = [0; 12];
    let mut i = 0;
    let mut n = 0;

    let mut el: u16 = 0;

    let mut nums = Vec::new();
    nums.reserve(1000);

    for c in data {
        if *c == '\n' as u8 {
            nums.push(el);
            el = 0;
            i = 0;
            n += 1;
        } else {
            let v = *c as u16 - '0' as u16;
            p1_cnt[i] += v;
            i += 1;
            el <<= 1;
            el += v;
        }
    }

    n /= 2;

    let mut eps = 0;
    let mut gamma = 0;

    //dbg!(p1_cnt);

    for i in 0..12 {
        eps <<= 1;
        gamma <<= 1;
        if p1_cnt[i] >= n {
            eps += 1;
        } else {
            gamma += 1;
        }
        //dbg!(eps, gamma);
    }

    let p1 = eps * gamma;

    // p2
    nums.sort();
    let p2_oxy = select(&nums, 11, true);
    let p2_co2 = select(&nums, 11, false);
    //dbg!(p2_oxy, p2_co2);
    let p2 = p2_oxy as u32 * p2_co2 as u32;

    format!("{} {}\n", p1, p2)
}

fn select(data: &[u16], bitnum: u16, max: bool) -> u16 {
    //dbg!(data.len(), bitnum);
    if data.len() == 1 {
        return data[0];
    }
    // Position of midpoint
    //let mid_idx = (data.len() + if max { 1 } else { 0 }) / 2;
    let mid_idx = data.len() / 2;
    // Position of pivot.
    let pp_idx = data.partition_point(|x| (x & ((1 << bitnum + 1) - 1)) < (1 << bitnum));
    //dbg!(mid_idx, pp_idx);
    if max {
        // Get the largest slice
        if pp_idx <= mid_idx {
            return select(&data[pp_idx..], bitnum - 1, max);
        } else {
            return select(&data[..pp_idx], bitnum - 1, max);
        }
    } else {
        if pp_idx <= mid_idx {
            return select(&data[..pp_idx], bitnum - 1, max);
        } else {
            return select(&data[pp_idx..], bitnum - 1, max);
        }
    }
}
