use std::fs;
use std::collections::HashMap;

pub struct Object {
    pub name: String,
    pub children: Vec<Object>,
    pub distance: i32
}

pub struct System {
    root: Object
}

impl System {
    pub fn from_map(filename: &str) -> System {
        let mut objects: HashMap<&str, Vec<&str>> = HashMap::new();

        let raw = fs::read_to_string(filename).expect("Failed to read input file");
        for val in raw.split('\n') {
            let object_names: Vec<&str> = val.trim().split(')').collect();
            let obj1_name = object_names.first().expect("Invalid input format");
            let obj2_name = object_names.last().expect("Invalid input format");

            if objects.contains_key(obj1_name) {
                let children = objects.get_mut(obj1_name).expect("Failed to retrieve object");
                children.push(obj2_name);
            } else {
                let mut children: Vec<&str> = Vec::new();
                children.push(obj2_name);
                objects.insert(obj1_name, children);
            }
        }

        fn create_object(objects: &HashMap<&str, Vec<&str>>, name: &str, distance: i32) -> Object {
            let children = match objects.get(name) {
                Some(children) => children.iter()
                    .map(|child| {
                        create_object(objects, child, distance + 1)
                    })
                    .collect(),
                _ => Vec::new()
            };

            Object {
                name: String::from(name),
                children,
                distance
            }
        }

        System {
            root: create_object(&objects, "COM", 0)
        }
    }

    pub fn total_distance(&self) -> i32 {
        fn recursively_calc(object: &Object) -> i32 {
            let mut children_distance = 0;
            for child in object.children.iter() {
                children_distance += recursively_calc(child);
            }

            object.distance + children_distance
        }

        recursively_calc(&self.root)
    }

    pub fn total_transfers(&self, from: &str, to: &str) -> Option<i32> {
        enum Ancestry {
            Unrelated,
            Ancestor(i32),
            CommonAncestor(i32)
        }

        fn walk(object: &Object, from: &str, to: &str) -> Ancestry {
            if (object.name == from) || (object.name == to) {
                return Ancestry::Ancestor(0);
            }

            object.children.iter()
                .map(|child| walk(child, from, to))
                .fold(Ancestry::Unrelated, |acc, ancestry| {
                    match ancestry {
                        Ancestry::CommonAncestor(_) => ancestry,
                        Ancestry::Ancestor(dist) => {
                            match acc {
                                Ancestry::Ancestor(dist2) => Ancestry::CommonAncestor(dist + dist2 + 1),
                                _ => Ancestry::Ancestor(dist + 1)
                            }
                        },
                        Ancestry::Unrelated => acc
                    }
                })
        }

        match walk(&self.root, from, to) {
            Ancestry::CommonAncestor(distance) => Some(distance - 2),
            _ => None
        }
    }
}

fn main() {
    let system = System::from_map("input.txt");

    let from = "YOU";
    let to = "SAN";

    println!("total direct and indirect orbits: {}", system.total_distance());
    println!("total number of transfers from {} to {}: {}", from, to, system.total_transfers(from, to)
        .expect("No path found between source and target"));
}
