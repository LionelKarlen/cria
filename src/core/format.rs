use crate::core::todo::{Todo, TodoKind};

pub fn format_todo_lines(todos: &str) -> String {
    let todos = Todo::from_lines(todos);
    let mut total = Vec::new();
    let mut marked = Vec::new();
    let mut regular = Vec::new();
    let mut completed = Vec::new();
    for todo in todos {
        match todo.kind {
            TodoKind::Marked => marked.push(todo.value),
            TodoKind::Regular => regular.push(todo.value),
            TodoKind::Completed => completed.push(todo.value),
        }
    }

    if !marked.is_empty() {
        let m = marked.join("\n");
        total.push(m);
        total.push("\n".into());
    }
    if !regular.is_empty() {
        let r = regular.join("\n");
        total.push(r);
    }
    if !completed.is_empty() {
        let c = completed.join("\n");
        total.push("\n\n".into());
        total.push(c);
    }

    total.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_1() {
        let input = "x completed
        . dotted
        regular";
        let output = ". dotted
regular

x completed";
        assert_eq!(format_todo_lines(input), output);
    }

    #[test]
    fn test_format_dotted() {
        let input = "x completed
        . dotted
        . second dotted
        regular";
        let output = ". dotted
. second dotted
regular

x completed";
        assert_eq!(format_todo_lines(input), output);
    }

    #[test]
    fn test_format_no_dots() {
        let input = "x completed
        regular 2
        regular";
        let output = "regular 2
regular

x completed";
        assert_eq!(format_todo_lines(input), output);
    }
}
