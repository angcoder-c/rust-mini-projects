use std::collections::{HashMap, BTreeMap};

// estructura de una tarea
struct Task {
    id: usize,
    titulo: String,
    prioridad: u8,
    completado: bool,
}

struct TaskManager {
    tareas: HashMap<usize, Task>,
    next_id: usize,
}

impl TaskManager {
    // constructor
    fn new() -> Self {
        Self {
            tareas: HashMap::new(),
            next_id: 1,
        }
    }

    // agrega una nueva tarea
    fn add_task(&mut self, titulo: &str, prioridad: u8) {
        let task = Task {
            id: self.next_id,
            titulo: titulo.to_string(),
            prioridad,
            completado: false,
        };
        self.tareas.insert(self.next_id, task);
        self.next_id += 1;
    }

    // marca una tarea como completada
    fn complete_task(&mut self, id: usize) {
        if let Some(task) = self.tareas.get_mut(&id) {
            task.completado = true;
        }
    }

    // leer las tareas guardadas
    fn list_tasks(&self) -> BTreeMap<u8, Vec<&Task>> {
        let mut organized: BTreeMap<u8, Vec<&Task>> = BTreeMap::new();
        for task in self.tareas.values() {
            organized.entry(task.prioridad).or_default().push(task);
        }
        organized
    }

    // estado de las tareas
    fn show_summary(&self) {
        let total = self.tareas.len();
        let completed = self.tareas.values().filter(|t| t.completado).count();
        println!("Tareas completadas: {}/{}", completed, total);
    }
}

fn main() {
    let mut manager = TaskManager::new();

    manager.add_task("Tarea 1", 1);
    manager.add_task("Tarea 2", 2);
    manager.add_task("Tarea 3", 1);

    manager.complete_task(2);

    println!("\n====== TAREAS ======");
    for (prioridad, tareas) in manager.list_tasks() {
        println!("Prioridad {}:", prioridad);
        for task in tareas {
            println!("  [{}] {} - {}", if task.completado {"x"} else {" "}, task.id, task.titulo);
        }
    }
    println!("\nResumen:");
    manager.show_summary();
}