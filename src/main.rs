use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

// Custom error type
#[derive(Debug)]
enum TaskError {
    TaskNotFound,
    InvalidInput,
    DuplicateTask,
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskError::TaskNotFound => write!(f, "Task not found"),
            TaskError::InvalidInput => write!(f, "Invalid input provided"),
            TaskError::DuplicateTask => write!(f, "Task with this title already exists"),
        }
    }
}

impl std::error::Error for TaskError {}

// Task priority levels
#[derive(Debug, Clone, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
            Priority::Critical => write!(f, "Critical"),
        }
    }
}

impl Priority {
    fn from_str(s: &str) -> Result<Priority, TaskError> {
        match s.to_lowercase().as_str() {
            "low" | "l" => Ok(Priority::Low),
            "medium" | "m" => Ok(Priority::Medium),
            "high" | "h" => Ok(Priority::High),
            "critical" | "c" => Ok(Priority::Critical),
            _ => Err(TaskError::InvalidInput),
        }
    }
}

// Task status
#[derive(Debug, Clone, PartialEq)]
enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::Completed => write!(f, "Completed"),
        }
    }
}

// Task struct
#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    description: String,
    priority: Priority,
    status: TaskStatus,
    tags: Vec<String>,
}

impl Task {
    fn new(id: u32, title: String, description: String, priority: Priority) -> Self {
        Task {
            id,
            title,
            description,
            priority,
            status: TaskStatus::Pending,
            tags: Vec::new(),
        }
    }

    fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    fn update_status(&mut self, status: TaskStatus) {
        self.status = status;
    }

    fn matches_filter(&self, filter: &str) -> bool {
        self.title.to_lowercase().contains(&filter.to_lowercase()) ||
        self.description.to_lowercase().contains(&filter.to_lowercase()) ||
        self.tags.iter().any(|tag| tag.to_lowercase().contains(&filter.to_lowercase()))
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, 
            "ID: {} | {} | Priority: {} | Status: {}\nDescription: {}\nTags: [{}]\n",
            self.id,
            self.title,
            self.priority,
            self.status,
            self.description,
            self.tags.join(", ")
        )
    }
}

// Task Manager struct
struct TaskManager {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String, description: String, priority: Priority) -> Result<u32, TaskError> {
        // Check for duplicate titles
        if self.tasks.values().any(|task| task.title == title) {
            return Err(TaskError::DuplicateTask);
        }

        let task = Task::new(self.next_id, title, description, priority);
        let id = self.next_id;
        self.tasks.insert(id, task);
        self.next_id += 1;
        Ok(id)
    }

    fn get_task(&self, id: u32) -> Result<&Task, TaskError> {
        self.tasks.get(&id).ok_or(TaskError::TaskNotFound)
    }

    fn get_task_mut(&mut self, id: u32) -> Result<&mut Task, TaskError> {
        self.tasks.get_mut(&id).ok_or(TaskError::TaskNotFound)
    }

    fn update_task_status(&mut self, id: u32, status: TaskStatus) -> Result<(), TaskError> {
        let task = self.get_task_mut(id)?;
        task.update_status(status);
        Ok(())
    }

    fn add_tag_to_task(&mut self, id: u32, tag: String) -> Result<(), TaskError> {
        let task = self.get_task_mut(id)?;
        task.add_tag(tag);
        Ok(())
    }

    fn delete_task(&mut self, id: u32) -> Result<(), TaskError> {
        self.tasks.remove(&id).ok_or(TaskError::TaskNotFound)?;
        Ok(())
    }

    fn list_tasks(&self) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.tasks.values().collect();
        tasks.sort_by(|a, b| a.id.cmp(&b.id));
        tasks
    }

    fn filter_tasks(&self, filter: &str) -> Vec<&Task> {
        self.tasks.values()
            .filter(|task| task.matches_filter(filter))
            .collect()
    }

    fn get_tasks_by_priority(&self, priority: Priority) -> Vec<&Task> {
        self.tasks.values()
            .filter(|task| task.priority == priority)
            .collect()
    }

    fn get_tasks_by_status(&self, status: TaskStatus) -> Vec<&Task> {
        self.tasks.values()
            .filter(|task| task.status == status)
            .collect()
    }

    fn get_statistics(&self) -> (usize, usize, usize, usize) {
        let total = self.tasks.len();
        let completed = self.tasks.values().filter(|t| t.status == TaskStatus::Completed).count();
        let in_progress = self.tasks.values().filter(|t| t.status == TaskStatus::InProgress).count();
        let pending = self.tasks.values().filter(|t| t.status == TaskStatus::Pending).count();
        (total, completed, in_progress, pending)
    }
}

// CLI Interface
struct CLI {
    task_manager: TaskManager,
}

impl CLI {
    fn new() -> Self {
        CLI {
            task_manager: TaskManager::new(),
        }
    }

    fn run(&mut self) {
        println!("=== Personal Task Manager ===");
        println!("Welcome! Type 'help' for available commands.\n");

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Error reading input. Please try again.");
                continue;
            }

            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            if input == "quit" || input == "exit" {
                println!("Goodbye!");
                break;
            }

            self.handle_command(input);
        }
    }

    fn handle_command(&mut self, input: &str) {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }

        match parts[0] {
            "help" => self.show_help(),
            "add" => self.add_task_interactive(),
            "list" => self.list_tasks(),
            "show" => self.show_task(&parts[1..]),
            "update" => self.update_task_status(&parts[1..]),
            "tag" => self.add_tag(&parts[1..]),
            "delete" => self.delete_task(&parts[1..]),
            "filter" => self.filter_tasks(&parts[1..]),
            "priority" => self.filter_by_priority(&parts[1..]),
            "status" => self.filter_by_status(&parts[1..]),
            "stats" => self.show_statistics(),
            _ => println!("Unknown command. Type 'help' for available commands."),
        }
    }

    fn show_help(&self) {
        println!("Available commands:");
        println!("  add                    - Add a new task (interactive)");
        println!("  list                   - List all tasks");
        println!("  show <id>              - Show details of a specific task");
        println!("  update <id> <status>   - Update task status (pending/progress/completed)");
        println!("  tag <id> <tag>         - Add a tag to a task");
        println!("  delete <id>            - Delete a task");
        println!("  filter <keyword>       - Filter tasks by keyword");
        println!("  priority <level>       - Filter tasks by priority (low/medium/high/critical)");
        println!("  status <status>        - Filter tasks by status (pending/progress/completed)");
        println!("  stats                  - Show task statistics");
        println!("  help                   - Show this help message");
        println!("  quit/exit              - Exit the application");
    }

    fn add_task_interactive(&mut self) {
        println!("=== Add New Task ===");
        
        let title = self.get_input("Enter task title: ");
        let description = self.get_input("Enter task description: ");
        
        println!("Select priority (low/medium/high/critical): ");
        let priority_input = self.get_input("Priority: ");
        
        let priority = match Priority::from_str(&priority_input) {
            Ok(p) => p,
            Err(_) => {
                println!("Invalid priority. Using 'Medium' as default.");
                Priority::Medium
            }
        };

        match self.task_manager.add_task(title, description, priority) {
            Ok(id) => println!("Task added successfully with ID: {}", id),
            Err(e) => println!("Error adding task: {}", e),
        }
    }

    fn get_input(&self, prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    fn list_tasks(&self) {
        let tasks = self.task_manager.list_tasks();
        if tasks.is_empty() {
            println!("No tasks found.");
            return;
        }

        println!("=== All Tasks ===");
        for task in tasks {
            println!("{}", task);
            println!("---");
        }
    }

    fn show_task(&self, args: &[&str]) {
        if args.is_empty() {
            println!("Usage: show <task_id>");
            return;
        }

        let id = match args[0].parse::<u32>() {
            Ok(id) => id,
            Err(_) => {
                println!("Invalid task ID. Please provide a number.");
                return;
            }
        };

        match self.task_manager.get_task(id) {
            Ok(task) => {
                println!("=== Task Details ===");
                println!("{}", task);
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    fn update_task_status(&mut self, args: &[&str]) {
        if args.len() < 2 {
            println!("Usage: update <task_id> <status>");
            println!("Status options: pending, progress, completed");
            return;
        }

        let id = match args[0].parse::<u32>() {
            Ok(id) => id,
            Err(_) => {
                println!("Invalid task ID. Please provide a number.");
                return;
            }
        };

        let status = match args[1] {
            "pending" => TaskStatus::Pending,
            "progress" => TaskStatus::InProgress,
            "completed" => TaskStatus::Completed,
            _ => {
                println!("Invalid status. Use: pending, progress, or completed");
                return;
            }
        };

        match self.task_manager.update_task_status(id, status) {
            Ok(_) => println!("Task status updated successfully."),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn add_tag(&mut self, args: &[&str]) {
        if args.len() < 2 {
            println!("Usage: tag <task_id> <tag>");
            return;
        }

        let id = match args[0].parse::<u32>() {
            Ok(id) => id,
            Err(_) => {
                println!("Invalid task ID. Please provide a number.");
                return;
            }
        };

        let tag = args[1..].join(" ");
        
        match self.task_manager.add_tag_to_task(id, tag) {
            Ok(_) => println!("Tag added successfully."),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn delete_task(&mut self, args: &[&str]) {
        if args.is_empty() {
            println!("Usage: delete <task_id>");
            return;
        }

        let id = match args[0].parse::<u32>() {
            Ok(id) => id,
            Err(_) => {
                println!("Invalid task ID. Please provide a number.");
                return;
            }
        };

        match self.task_manager.delete_task(id) {
            Ok(_) => println!("Task deleted successfully."),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn filter_tasks(&self, args: &[&str]) {
        if args.is_empty() {
            println!("Usage: filter <keyword>");
            return;
        }

        let filter = args.join(" ");
        let tasks = self.task_manager.filter_tasks(&filter);
        
        if tasks.is_empty() {
            println!("No tasks found matching '{}'.", filter);
            return;
        }

        println!("=== Filtered Tasks ===");
        for task in tasks {
            println!("{}", task);
            println!("---");
        }
    }

    fn filter_by_priority(&self, args: &[&str]) {
        if args.is_empty() {
            println!("Usage: priority <level>");
            println!("Levels: low, medium, high, critical");
            return;
        }

        let priority = match Priority::from_str(args[0]) {
            Ok(p) => p,
            Err(_) => {
                println!("Invalid priority. Use: low, medium, high, or critical");
                return;
            }
        };

        let tasks = self.task_manager.get_tasks_by_priority(priority);
        
        if tasks.is_empty() {
            println!("No tasks found with {} priority.", args[0]);
            return;
        }

        println!("=== {} Priority Tasks ===", args[0].to_uppercase());
        for task in tasks {
            println!("{}", task);
            println!("---");
        }
    }

    fn filter_by_status(&self, args: &[&str]) {
        if args.is_empty() {
            println!("Usage: status <status>");
            println!("Status options: pending, progress, completed");
            return;
        }

        let status = match args[0] {
            "pending" => TaskStatus::Pending,
            "progress" => TaskStatus::InProgress,
            "completed" => TaskStatus::Completed,
            _ => {
                println!("Invalid status. Use: pending, progress, or completed");
                return;
            }
        };

        let tasks = self.task_manager.get_tasks_by_status(status);
        
        if tasks.is_empty() {
            println!("No tasks found with {} status.", args[0]);
            return;
        }

        println!("=== {} Tasks ===", args[0].to_uppercase());
        for task in tasks {
            println!("{}", task);
            println!("---");
        }
    }

    fn show_statistics(&self) {
        let (total, completed, in_progress, pending) = self.task_manager.get_statistics();
        
        println!("=== Task Statistics ===");
        println!("Total tasks: {}", total);
        println!("Completed: {}", completed);
        println!("In progress: {}", in_progress);
        println!("Pending: {}", pending);
        
        if total > 0 {
            let completion_rate = (completed as f64 / total as f64) * 100.0;
            println!("Completion rate: {:.1}%", completion_rate);
        }
    }
}

fn main() {
    let mut cli = CLI::new();
    cli.run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new(1, "Test Task".to_string(), "Description".to_string(), Priority::High);
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.priority, Priority::High);
        assert_eq!(task.status, TaskStatus::Pending);
    }

    #[test]
    fn test_task_manager_add_task() {
        let mut manager = TaskManager::new();
        let result = manager.add_task("Test".to_string(), "Description".to_string(), Priority::Low);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_duplicate_task_error() {
        let mut manager = TaskManager::new();
        manager.add_task("Test".to_string(), "Description".to_string(), Priority::Low).unwrap();
        let result = manager.add_task("Test".to_string(), "Another Description".to_string(), Priority::High);
        assert!(result.is_err());
    }

    #[test]
    fn test_task_filtering() {
        let mut manager = TaskManager::new();
        manager.add_task("Buy groceries".to_string(), "Milk and bread".to_string(), Priority::Medium).unwrap();
        manager.add_task("Walk dog".to_string(), "Morning walk".to_string(), Priority::Low).unwrap();
        
        let filtered = manager.filter_tasks("dog");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].title, "Walk dog");
    }
}