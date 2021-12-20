use aoc::map::{FreeCoord, Map};
use aoc::prelude::*;

type Input = Vec<(Cave, Cave)>;

#[derive(Clone, Debug)]
struct Cave {
    name: String,
    ctype: CaveType,
}

#[derive(Copy, Clone, Eq, Debug, PartialEq)]
enum CaveType {
    //Start,
    //End,
    Small,
    Big,
}

impl CaveType {
    fn from_name(s: &str) -> CaveType {
        match s {
            //"start" => CaveType::Start,
            //"end" => CaveType::End,
            s if s.chars().all(|c| c.is_ascii_uppercase()) => CaveType::Big,
            _ => CaveType::Small,
        }
    }
}

fn parser(path: &Path) -> anyhow::Result<Input> {
    let reader = BufReader::new(File::open(path)?);
    let mut input = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let (a, b) = line.trim_end().split_once('-').unwrap();
        let a = Cave {
            name: a.to_owned(),
            ctype: CaveType::from_name(a),
        };
        let b = Cave {
            name: b.to_owned(),
            ctype: CaveType::from_name(b),
        };
        input.push((a, b));
    }
    Ok(input)
}

fn into_connections(input: &Input) -> HashMap<String, Vec<Cave>> {
    let mut result = HashMap::new();
    for (a, b) in input.iter() {
        let entry = result.entry(a.name.clone()).or_insert_with(Vec::new);
        entry.push(b.clone());
        let entry = result.entry(b.name.clone()).or_insert_with(Vec::new);
        entry.push(a.clone());
    }
    result
}

fn small_caves(input: &Input) -> HashSet<String> {
    let mut result = HashSet::new();
    for (a, b) in input.iter() {
        if a.ctype == CaveType::Small {
            result.insert(a.name.clone());
        }
        if b.ctype == CaveType::Small {
            result.insert(b.name.clone());
        }
    }
    result
}

#[derive(Clone, Debug)]
struct CPath {
    path: Vec<String>,
    visited: HashSet<String>,
    double_used: bool,
}

impl CPath {
    fn new() -> Self {
        CPath {
            path: Vec::new(),
            visited: HashSet::new(),
            double_used: false,
        }
    }

    fn at(&self) -> &str {
        &self.path.last().unwrap()
    }

    fn add(&mut self, s: &str) {
        self.path.push(s.to_owned());
        self.visited.insert(s.to_owned());
    }

    fn visits(&self, s: &str) -> bool {
        self.visited.contains(s)
    }
}

fn part_a(input: &Input) -> i64 {
    let need_to_visit = small_caves(input);
    let connections = into_connections(input);
    let mut path = CPath::new();
    path.add("start");
    let mut paths = Vec::<CPath>::new();
    let mut search_queue = VecDeque::new();
    search_queue.push_back(path);
    while let Some(path) = search_queue.pop_front() {
        let at = path.at();
        let can_go_to = connections.get(at).map(|v| v.as_slice()).unwrap_or(&[]);
        //println!("Thonk: {:?}", path.path);
        for cave in can_go_to {
            if at == "c" {
                //println!("At c. Thinking about going to {:?}", cave);
            }
            if cave.ctype == CaveType::Small {
                if path.visits(&cave.name) {
                    continue;
                }
            }
            //if at == "c" { println!("going to {}", cave.name);}
            let mut new_path = path.clone();
            new_path.add(&cave.name);
            if cave.name == "end" {
                //println!("Found path: {:?}", new_path);
                paths.push(new_path);
            } else {
                //println!("adding to queue: {:?}", new_path);
                search_queue.push_back(new_path);
            }
        }
    }
    paths.len() as i64
}

fn part_b(input: &Input) -> i64 {
    let need_to_visit = small_caves(input);
    let connections = into_connections(input);
    let mut path = CPath::new();
    path.add("start");
    let mut paths = Vec::<CPath>::new();
    let mut search_queue = VecDeque::new();
    search_queue.push_back(path);
    while let Some(path) = search_queue.pop_front() {
        let at = path.at();
        let can_go_to = connections.get(at).map(|v| v.as_slice()).unwrap_or(&[]);
        //println!("Thonk: {:?}", path.path);
        for cave in can_go_to {
            if at == "c" {
                //println!("At c. Thinking about going to {:?}", cave);
            }
            if cave.ctype == CaveType::Small {
                if path.visits(&cave.name) && path.double_used || cave.name == "start" {
                    continue;
                }
            }
            //if at == "c" { println!("going to {}", cave.name);}
            let mut new_path = path.clone();
            if path.visits(&cave.name) && cave.ctype == CaveType::Small {
                new_path.double_used = true;
            }
            new_path.add(&cave.name);
            if cave.name == "end" {
                //println!("Found path: {:?}", new_path);
                paths.push(new_path);
            } else {
                //println!("adding to queue: {:?}", new_path);
                search_queue.push_back(new_path);
            }
        }
    }
    /*
    println!("Paths found:");
    for path in paths.iter() {
        for p in path.path.iter() {
            print!("{},", p);
        }
        println!();
    }
    */
    paths.len() as i64
}

aoc::aoc!(parser, part_a, part_b, Some(10), Some(36));
