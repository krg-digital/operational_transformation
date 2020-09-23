fn main() {
  println!("Hello World!");
}

fn isValid(stale: &str, latest: &str, otjson: OperationalTransform) -> bool {

}

struct OperationalTransform {
  op: Operation,
  count: int,
}

enum Operation {
  Insert,
  Delete,
  Skip,
}