use std::io::Stdout;

use exitfailure::ExitFailure;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::file_ops::{self, DirectoryItem};

pub struct App<'a> {
    pub terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    pub selection_index: Option<usize>,
    pub directory_contents: Vec<DirectoryItem>,
    pub search_buffer: Vec<char>,
    pub error: Option<String>,
    pub window_height: u16,

    max_file_selection: usize,
}

impl<'a> App<'a> {
    pub fn new(
        terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<App<'a>, ExitFailure> {
        let window_height = terminal.size().unwrap().height - 2;

        let mut app = App {
            terminal,
            selection_index: None,
            directory_contents: Vec::new(),
            search_buffer: Vec::new(),
            error: None,
            window_height,
            max_file_selection: 0,
        };

        app.populate_files()?;

        Ok(app)
    }

    pub fn update_window_height(&mut self) {
        //borders window height add up to 2
        self.window_height = self.terminal.size().unwrap().height - 2;
    }

    pub fn populate_files(&mut self) -> Result<(), ExitFailure> {
        let mut dir_items = file_ops::get_files_for_current_directory()?;
        // Sort: folder > file
        dir_items.sort_by(|a, b| b.cmp(a));

        self.directory_contents = dir_items;
        self.max_file_selection = self.directory_contents.len();

        if self.max_file_selection == 0 {
            self.selection_index = None;
        } else {
            self.selection_index = Some(0);
        }

        Ok(())
    }

    pub fn get_search_string(&mut self) -> String {
        let mut search_string = String::new();
        for c in &self.search_buffer {
            search_string.push(*c);
        }

        search_string
    }
}
