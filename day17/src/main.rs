
// integer suvat
// s = sum[i=0->t] u + a*t_i
// v = u + at


fn falls_in_bounds(u: i32, min: i32, max: i32) -> bool {
    assert!(max < 0);
    let a = -1;
    let mut t = 0;
    let mut s = 0;
    while s >= max {
        s += u + a*t;
        if (max..=min).contains(&s) {
            return true;
        }
        t += 1;
    }
    return false;
}

fn falls_in_bounds_2d(u_x: i32, u_y: i32, min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> bool {
    let mut s_x = 0;
    let mut s_y = 0;
    let mut v_x = u_x;
    let mut v_y = u_y;
    while s_y >= max_y && s_x <= max_x {
        s_x += v_x;
        s_y += v_y;
        if (min_x..=max_x).contains(&s_x) && (max_y..=min_y).contains(&s_y) {
            return true;
        }
        v_x = std::cmp::max(v_x - 1, 0);
        v_y -= 1;
    }
    return false;
}

fn get_distance(u : i32, t: i32) -> i32 {
    (0..t).fold(0, |s, t_i| s + u - t_i)
}

fn find_max_vertical_velocity(min: i32, max: i32) -> i32 {
    let mut max_u = 0;
    for u in 0..10000 {
        if falls_in_bounds(u, min, max) {
            max_u = u;
        }
    }
    return max_u;
}

fn find_all_trajectories(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Vec<(i32,i32)> {
    let mut trajectories = Vec::new();
    for u_x in -2000..2000 {
        if u_x % 100 == 0 {
            println!("u_x: {}", u_x);
        }
        for u_y in -2000..2000 {
            if falls_in_bounds_2d(u_x, u_y, min_x, max_x, min_y, max_y) {
                trajectories.push((u_x, u_y));
            }
        }
    }
    return trajectories;
}

fn main() {
    //let max_vel = find_max_vertical_velocity(-69, -126);
    //println!("Max velocity: {}", max_vel);
    //println!("Max height: {}", get_distance(max_vel, max_vel));
    println!("{:?}", find_all_trajectories(217, 240, -69, -126).len());
}
