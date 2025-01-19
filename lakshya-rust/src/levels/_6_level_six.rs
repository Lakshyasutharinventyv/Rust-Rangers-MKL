use std::time::Duration;
use tokio::time::sleep;

trait VisitCave {
    fn cave_details(&self);
    fn visit_cave(&self);
}

struct Cave {
    height: u32,
    width: u32,
    visitors_allowed: bool,
    under_water: bool,
}

impl VisitCave for Cave {
    fn cave_details(&self) {
        println!(
            "height: {}m\nwidth: {}m.\nVisitors are {} and it is {}.",
            self.height, self.width, match self.visitors_allowed {
                true =>"allowed",
                _=>"not allowed"
            },match self.under_water {
                true => "under water",
                _=> "not under water"
            } 
        );
    }
    fn visit_cave(&self) {
        match self.visitors_allowed{
            true => println!("Visit cave because visitors are allowed"),
            _=>println!("Do not visit cave because visitors are not allowed")
        }
    }
}


pub async fn level6() {
    let cave = Cave {
        height: 10,
        width: 15,
        visitors_allowed: true,
        under_water: false,
    };
    println!("Loading level 6");
    sleep(Duration::from_millis(3000)).await;
    println!("\nYou are visitor of cave. Check details of cave before visiting.");
    sleep(Duration::from_millis(3000)).await;
    println!("\nDetails of Cave are as follow:");
    sleep(Duration::from_millis(3000)).await;
    Cave::cave_details(&cave);
    sleep(Duration::from_millis(3000)).await;
    Cave::visit_cave(&cave);
    super::_7_level_seven::level7().await;
}
