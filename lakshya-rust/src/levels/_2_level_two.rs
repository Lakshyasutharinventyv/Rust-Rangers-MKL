use std::time::Duration;
use tokio::time::sleep;

#[allow(dead_code)]
#[tokio::main]
pub async fn level2() {
    println!("Loading..");
    sleep(Duration::from_millis(3000)).await;
    println!("game started.");
    sleep(Duration::from_millis(3000)).await;
    println!("\n\nTask 1: While Loop - Monitoring Water Level");
    sleep(Duration::from_millis(3000)).await;
    let mut water_level = 5;
    let max_safe_level = 10;
    while water_level <= max_safe_level {
        sleep(Duration::from_millis(500)).await;
        println!("Water level: {}", water_level);
        water_level += 1;
    }
    println!("Warning! Water level exceeded safe limits.");
    sleep(Duration::from_millis(3000)).await;
    println!("\n\nTask 2: Infinite Loop - Escaping the Whirlpool");
    let mut attempts = 0;
    loop {
        attempts += 1;
        sleep(Duration::from_millis(500)).await;
        println!("Attempt {} to escape the whirlpool!", attempts);

        if attempts == 3 {
            println!("Whirlpool escaped!");
            break;
        }
    }
    sleep(Duration::from_millis(3000)).await;
    println!("\n\nTask 3: Using Continue - Catching Big Fish");
    let mut large_fishes = Vec::new();
    let fish_sizes = vec![2, 5, 10, 3, 7];
    println!("caught following large fishes!");
    for size in fish_sizes {
        if size < 5 {
            continue;
        }
        large_fishes.push(format!("fish of size {}", size));
    }
    for fish in large_fishes{
        sleep(Duration::from_millis(500)).await;
        println!("{fish}");
    }
}
