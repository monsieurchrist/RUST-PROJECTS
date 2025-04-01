use std::thread;
use std::time::Duration;
fn main()
{
    let man = thread::spawn
    (||
        {
            for i in 1..3 {
                  println!("je suis la nouvelle tache {}",i);
                  thread::sleep(Duration::from_millis(3));
            }
        }
    );
    for i in 1..20 {
                println!("je suis la tache initiale {}",i);
                thread::sleep(Duration::from_millis(3));
    }
    man.join().unwrap();
}
