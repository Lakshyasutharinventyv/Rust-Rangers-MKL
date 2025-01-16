#[allow(dead_code)]
use std::{io::stdin, time::Duration};
use tokio::time::sleep;

#[tokio::main]
pub async fn level1(){
    
    println!("Loading ...");
    sleep(Duration::from_millis(3000)).await;
    println!("Game Started.");
    let cur_no_of_trees = loop {
        println!("Enter the current number of trees:");
        
        let mut input = String::new();
        stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    match input.trim().parse::<i32>() {
        Ok(r) => break r, 
        Err(_) => println!("Please enter a valid tree count!"),
    }
};
    let pollution = cur_no_of_trees > 20;
    sleep(Duration::from_millis(5000)).await;
    println!("There is magical tree which shrinks or increases height by 5 meter in 1 sec and it will disappear or reaches its max height of 50 depending on numbers of trees left.");
    let mut tree_height = 30;
    println!("Height of tree is {}",tree_height);
    println!("No of leaves in tree is {}",count_leaves(&tree_height));
    sleep(Duration::from_millis(3000)).await;
    
    loop {
        if tree_height <= 0{
            println!("Tree disappeared. Whoooo!");
            break
        }
        if tree_height >=50{
            println!("Tree reached max height of 50 and pollution is reduced. Yay! we saved earth.");
            break
        }
        if pollution{
            println!("Oh no pollution is less so magical tree's height is shrinking down!");
            shrink(&mut tree_height);
        }else{
            println!("Oh pollution is increasing so in order to reduce pollution tree's height is increasing!");
            increase(&mut tree_height);
        }
        
        sleep(Duration::from_secs(1)).await
    }
}

fn shrink(height:&mut i32){
    println!("{height}");
    *height -= 5;
}
fn increase(height:&mut i32){
    println!("{height}");
    *height += 5;
}
fn count_leaves(height: &i32) -> f32 {
    (*height as f32) * 111.11
}