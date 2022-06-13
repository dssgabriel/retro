use retro::*;

fn main() {
    let filename = String::from("metro.txt");
    let metro = Metro::new(&filename);

    println!("\n\x1b[1mDeparture:\x1b[0m ");
    let departures = Metro::get_station(&metro);
    println!("\n\x1b[1mArrival:\x1b[0m ");
    let arrivals = Metro::get_station(&metro);
    println!();

    let start = std::time::Instant::now();
    let mut results = Vec::new();
    for departure in departures {
        for arrival in &arrivals {
            results.push(Metro::dijkstra(&metro, departure.id, arrival.id));
        }
    }

    let mut best = 0;
    for i in 1..results.len() {
        if results[i].time < results[best].time {
            best = i;
        }
    }
    let end = std::time::Instant::now();

    Metro::print_travel(&metro, &results[best]);

    let time = end - start;
    println!("Executed in {time:#?}");
}
