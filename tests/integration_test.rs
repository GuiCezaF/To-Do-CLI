use todo_cli::TodoList;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_todo() {
        let todo = TodoList::new();
        assert!(todo.items.is_empty()); // Verifica se a lista está vazia ao inicializar
    }
}
