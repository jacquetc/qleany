use anyhow::Result;
pub trait UndoRedoCommand : Send {
    fn undo(&mut self) -> Result<()>;
    fn redo(&mut self) -> Result<()>;
}

pub struct UndoRedoManager {
    undo_stack: Vec<Box<dyn UndoRedoCommand>>,
    redo_stack: Vec<Box<dyn UndoRedoCommand>>,
}

impl UndoRedoManager {
    pub fn new() -> Self {
        UndoRedoManager {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn undo(&mut self) -> Result<()> {
        if let Some(mut command) = self.undo_stack.pop() {
            command.undo()?;
            self.redo_stack.push(command);
        }
        Ok(())
    }

    pub fn redo(&mut self) -> Result<()> {
        if let Some(mut command) = self.redo_stack.pop() {
            command.redo()?;
            self.undo_stack.push(command);
        }
        Ok(())
    }

    pub fn add_command(&mut self, command: Box<dyn UndoRedoCommand>) {
        self.undo_stack.push(command);
        self.redo_stack.clear();
    }
}