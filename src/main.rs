fn main() {
    let stale_1 = "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.";    
    let latest_1 = "Repl.it uses operational transformations.";    
    let ops_1 = vec![Operation::Skip(40), Operation::Delete(47)];

    let stale_2 = "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.";
    let latest_2 = "We use operational transformations to keep everyone in a multiplayer repl in sync."; 
    let ops_2 = vec![Operation::Delete(7), Operation::Insert(String::from("We")), Operation::Skip(4), Operation::Delete(1)];

    
    let test_1 = is_valid(
        stale_1,
        latest_1,
        ops_1
    );
    println!("{}", test_1);

    let test_2 = is_valid(
        stale_2,
        latest_2,
        ops_2  
    );
    println!("{}", test_2);
}

fn is_valid(stale: &str, latest: &str, otjson: Vec<Operation>) -> bool {
    let mut ot = OperationalTransformation::new(String::from(stale), otjson);

    ot.transform() == String::from(latest)
}

fn insert(s: String, ins: String) -> String {
    let mut res = ins.clone();
    res.push_str(&s);

    res
}

#[derive(Debug)]
struct OperationalTransformation {
    s: String,
    position: usize,
    ops: Vec<Operation>,
}

impl OperationalTransformation {
    fn new(s: String, ops: Vec<Operation>) -> OperationalTransformation {
        OperationalTransformation { s, position: 0, ops }    
    }
    
    fn transform(&mut self) -> String {
        let mut res = self.s.clone();

        for op in &self.ops {
            match op {
                Operation::Insert(ins) => res.insert_str(self.position, &ins),
                Operation::Delete(n) => res.replace_range(self.position..(self.position + *n), ""),
                Operation::Skip(n) => self.position += n,
            }
        }

        res
    }
}

#[derive(Debug)]
enum Operation {
    Insert(String),
    Delete(usize),
    Skip(usize),
}
