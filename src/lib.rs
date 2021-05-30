use std::fs;

/// A structure that represents a metro station.
///
/// The type `Station` is made up of the followings:
/// * an `id` that is unique to each station,
/// * a `line` which indicates the metro line passing by,
/// * a `state` which is true if the station is a terminus and false otherwise,
/// * a `name` to print when giving the itinerary.
///
/// The `Station` type is one of the building blocks of the type [`Metro`].
///
/// [`Metro`]: Metro
pub struct Station {
    pub id: usize,
    pub line: String,
    pub state: bool,
    pub name: String,
}

impl Station {
    /// Constructs a new `Station` using a given `&str`.
    ///
    /// # Arguments
    /// * `description` - a string slice describing the `Station` to construct.
    ///
    /// # Example
    /// Constructing a station for "Les Halles":
    /// ```
    /// use paris_metro::Station;
    ///
    /// let description = "V 0042 4 0 Les Halles";
    ///
    /// let station = Station::new(description);
    ///
    /// assert_eq!(station.id, 42);
    /// assert_eq!(station.line, String::from("4"));
    /// assert_eq!(station.state, false);
    /// assert_eq!(station.name, String::from("Les Halles"));
    /// ```
    pub fn new(description: &str) -> Self {
        let parsed: Vec<&str> = description
            .clone()
            .trim_start_matches("V ")
            .splitn(4, " ")
            .collect();

        Station {
            id: parsed[0]
                .parse::<usize>()
                .expect("Could not cast as u32"),
            line: parsed[1]
                .trim_start_matches("0")
                .to_string(),
            state: parsed[2]
                .eq("1"),
            name: parsed[3]
                .to_string(),
        }
    }
}


/// A structure that represents a trip between two metro stations.
///
/// The `Trip` type is used to link two [`Station`]s together using their unique
/// identifiers.
/// It is made up of the followings:
/// * `first` is the `id` of one of the `Station`s,
/// * `second` is the `id` of the other,
/// * `time` is the time it takes (in seconds) to travel between the two
/// `Station`s.
///
/// The `Trip` type is the other building block of the [`Metro`] type.
///
/// [`Station`]: Station
/// [`Metro`]: Metro
pub struct Trip {
    pub first: usize,
    pub second: usize,
    pub time: usize,
}

impl Trip {
    /// Constructs a new `Trip` using a given `&str`.
    ///
    /// # Arguments
    /// * `description` - a string slice describing the `Trip` to construct.
    ///
    /// # Example
    /// ```
    /// use paris_metro::Trip;
    ///
    /// let description = "E 0042 0069 420";
    ///
    /// let trip = Trip::new(description);
    ///
    /// assert_eq!(trip.first, 42);
    /// assert_eq!(trip.second, 69);
    /// assert_eq!(trip.time, 420);
    /// ```
    pub fn new(config: &str) -> Self {
        let parsed: Vec<&str> = config
            .clone()
            .trim_start_matches("E ")
            .splitn(3, " ")
            .collect();

        Trip {
            first: parsed[0]
                .parse::<usize>()
                .expect("Could not cast as u32"),
            second: parsed[1]
                .parse::<usize>()
                .expect("Could not cast as u32"),
            time: parsed[2]
                .parse::<usize>()
                .expect("Could not cast as u32"),
        }
    }
}

/// A structure that represents a metro system.
///
/// The `Metro` type is built around the two aforementionned types: [`Station`]
/// and [`Trip`].
/// It is made up of the followings:
/// * `stations` is a list (`Vec`) of all the `Station`s part of the network,
/// * `trips` is a list (`Vec`) of all the possible trips between
/// the `Station`s of the network.
///
/// [`Station`]: Station
/// [`Trip`]: Trip
pub struct Metro {
    pub stations: Vec<Station>,
    pub trips: Vec<Trip>,
}

impl Metro {
    /// Constructs a new Metro using a `&str`.
    ///
    /// # Arguments
    /// * `filename` - a string slice referencing the configuration file name.
    pub fn new(filename: &String) -> Self {
        let mut stations: Vec<Station> = Vec::new();
        let mut trips: Vec<Trip> = Vec::new();

        let contents = fs::read_to_string(filename)
            .expect("Could not read file");

        for line in contents.lines() {
            if line.starts_with("V") {
                stations.push(Station::new(line));
            } else if line.starts_with("E") {
                trips.push(Trip::new(line));
            } else {
                continue;
            }
        }

        Metro {
            stations,
            trips,
        }
    }

    /// Returns a `Vec` of references of `Station`s with a matching name.
    ///
    /// This method asks a station name for the user to input
    /// from the command-line.
    /// If the input matches station's name in the list, references to those
    /// are pushed to the to-be returned `Vec`.
    ///
    /// # Argument
    /// * `&self` - a reference to self.
    pub fn get_station(&self) -> Vec<&Station> {
        let mut matches = Vec::new();

        loop {
            let mut input = String::new();

            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line.");

            input = input.trim().to_string();

            for station in &self.stations {
                if input.eq(&station.name) {
                    matches.push(station);
                }
            }

            match matches.as_slice() {
                [] => {
                    println!(
                        "Unknown station, please check that you have typed correctly."
                    );
                }
                _ => break,
            }
        }

        matches
    }

    /// Returns a `Vec` of references of `Trip`s for the available routes from
    /// a given `Station` identifier.
    ///
    /// This methods looks for all the possible trips for a given station
    /// identifier.
    /// TODO: It also removes trips that are only one way.
    ///
    /// # Arguments
    /// * `&self` - a reference to self.
    /// * `current` - the identifier of the current station.
    fn get_paths_to_neighboors(&self, current: usize) -> Vec<&Trip> {
        let mut paths = Vec::new();

        for path in &self.trips {
            if (current == path.first) | (current == path.second) {
                paths.push(path);
            }
        }

        paths
    }

    /// Returns the `identifier` to a terminus `Station`.
    ///
    /// From a previous and current station, determines the `Station`
    /// (terminus) at the end of the metro line.
    ///
    /// # Arguments
    /// * `&self` - a reference to self.
    /// * `prev` - the identifier of the previous station.
    /// * `curr` - the identifier of the current station.
    fn get_terminus(&self, mut prev: usize, mut curr: usize) -> usize {
        while self.stations[curr].state == false {
            let trips = Self::get_paths_to_neighboors(self, curr);

            for trip in trips {
                if curr == trip.first {
                    if (&self.stations[curr].line == &self.stations[trip.second].line) &&
                        (prev != trip.second)
                    {
                        prev = curr;
                        curr = trip.second;
                    }
                } else if curr == trip.second {
                    if (&self.stations[curr].line == &self.stations[trip.first].line) &&
                       (prev != trip.first)
                    {
                        prev = curr;
                        curr = trip.first;
                    }
                }
            }
        }

        self.stations[curr].id
    }

    /// Returns a tuple of `Vec`s, one holding references to `Station`s and
    /// the other holding `usize`.
    ///
    /// From the list `prevs`, gets the full path from the `end` `Station` to
    /// the `end`.
    /// Then reverse this path and gets every metro line changes in it.
    ///
    /// # Argument
    /// * `&self` - a reference to self.
    /// * `start` - the identifier of the starting `Station`.
    /// * `end` - the identifier of the ending `Station`.
    /// * `prevs` - the `Vec` of previous `Station`s identifiers.
    fn get_changes(
        &self,
        start: usize,
        end: usize,
        prevs: Vec<usize>
    ) -> (Vec<&Station>, Vec<usize>) {
        let mut path = Vec::new();
        let mut changes = Vec::new();
        let mut directions = Vec::new();
        let mut current = end;

        path.push(&self.stations[end]);
        while current != start {
            let next = prevs[current];
            path.push(&self.stations[next]);
            current = next;
        }
        path.reverse();

        directions.push(Self::get_terminus(self, path[0].id, path[1].id));
        for i in 1..path.len() {
            if path[i-1].line != path[i].line {
                changes.push(path[i]);
                if i+1 < path.len() {
                    directions.push(Self::get_terminus(
                        self,
                        path[i].id,
                        path[i+1].id
                    ));
                }
            }
        }

        (changes, directions)
    }

    /// Computes the shortest path between two `Station`s and returns
    /// a `Results` structure.
    ///
    /// # Arguments
    /// * `&self` - a reference to self.
    /// * `start` - the identifier of the starting `Station`.
    /// * `end` - the identifier of the ending `Station`.
    pub fn dijkstra(&self, start: usize, end: usize) -> Results {
        let mut distance = vec![usize::MAX; self.stations.len()];
        let mut prevs = vec![usize::MAX; self.stations.len()];
        let mut unvisited = vec![0; self.stations.len()];
        let mut visited = 0;
        let stop_time = 30;

        for i in 0..unvisited.capacity() {
            unvisited[i] = i;
        }
        distance[start] = 0;

        while visited < self.stations.len() {
            let current = get_next(&mut distance, &mut unvisited, &mut visited);
            let paths = Self::get_paths_to_neighboors(self, current);

            for path in paths {
                if current == path.first {
                    if distance[current] + path.time < distance[path.second] {
                        distance[path.second] = distance[current] + path.time + stop_time;
                        prevs[path.second] = current;
                    }
                } else if current == path.second {
                    if distance[current] + path.time < distance[path.first] {
                        distance[path.first] = distance[current] + path.time + stop_time;
                        prevs[path.first] = current;
                    }
                }
            }
        }

        let time: (usize, usize) = get_time(distance[end]);
        let (changes, directions) = Self::get_changes(self, start, end, prevs);

        Results { start, time, changes, directions, end }
    }

    /// Prints the travel to terminal.
    ///
    /// # Arguments
    /// * `&self` - a reference to self.
    /// * `results` - the structure holding the results of the dijkstra
    /// algorithm.
    pub fn print_travel(&self, results: &Results) {
        println!("\nTrip time: \x1b[1m{} mins, {} secs\x1b[0m",
            results.time.0,
            results.time.1
        );

        print!("\n\x1b[1m{}\x1b[0m", &self.stations[results.start].name);
        println!("\n|\n|");
        print!("\x1b[1m\x1b[32m{}\x1b[0m - \x1b[1m{}\x1b[0m\n|\tTowards {}",
            &self.stations[results.start].line,
            &self.stations[results.start].name,
            &self.stations[results.directions[0]].name
        );
        println!("\n|");
        for i in 0..results.changes.len() {
            print!("\x1b[1m\x1b[32m{}\x1b[0m - \x1b[1m{}\x1b[0m\n|\tTowards {}",
                results.changes[i].line,
                results.changes[i].name,
                &self.stations[results.directions[i+1]].name
            );
            println!("\n|");
        }
        println!("\x1b[1m{}\x1b[0m\n", &self.stations[results.end].name);
    }
}

/// A structure that holds the results of a run of the dijkstra algorithm.
///
/// The `Result` type is made up of:
/// * `start` is the identifier of the starting `Station`,
/// * `changes` is the `Vec` holding the `Station`s where the user
/// has to change metro lines,
/// * `directions` is the `Vec` holding the terminus `Station`s identifiers,
/// * `time` is a tuple holding the time taken in minutes and seconds,
/// * `end` - the identifier of the ending `Station`.
pub struct Results<'a> {
    pub start: usize,
    pub changes: Vec<&'a Station>,
    pub directions: Vec<usize>,
    pub time: (usize, usize),
    pub end: usize,
}

/// Returns a `usize` corresponding to the identifier of the next `Station`.
///
/// # Arguments
/// * `distance` - a mutable reference on a `Vec` holding the distance
/// in seconds to the other `Station`s.
/// * `unvisited` - a mutable reference on a `Vec` holding the identifiers
/// of the unvisited `Station`s.
/// * `visited` - a mutable reference on the number of visited `Station`s.
fn get_next(
    distance: &mut Vec<usize>,
    unvisited: &mut Vec<usize>,
    visited: &mut usize
) -> usize {
    let mut min = usize::MAX;
    let mut next = 0;

    for station in &*unvisited {
        if distance[*station] < min {
            min = distance[*station];
            next = *station;
        }
    }

    for i in 0..unvisited.len() {
        if unvisited[i] == next {
            unvisited.remove(i);
            break;
        }
    }
    *visited += 1;

    next
}

/// Returns a tuple of `usize`s holding the time taken in minutes and seconds.
///
/// # Argument
/// * `time` - the time taken in seconds.
fn get_time(time: usize) -> (usize, usize) {
    let minutes = time / 60;
    let seconds = time - (minutes * 60);

    (minutes, seconds)
}
