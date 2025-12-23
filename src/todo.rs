pub enum TodoKind {
    Marked,
    Regular,
    Completed,
}

pub struct Todo {
    pub value: String,
    pub kind: TodoKind,
}

impl Todo {
    pub fn from_file_text(file_text: &str) -> Vec<Todo> {
        file_text
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() {
                    return None;
                }
                Some(Todo::from(line))
            })
            .collect()
    }
}

impl From<&str> for Todo {
    fn from(value: &str) -> Self {
        let kind = if let Some(prefix) = value.get(0..2) {
            match prefix {
                ". " => TodoKind::Marked,
                "x " => TodoKind::Completed,
                _ => TodoKind::Regular,
            }
        } else {
            TodoKind::Regular
        };

        Self {
            value: value.to_string(),
            kind,
        }
    }
}
