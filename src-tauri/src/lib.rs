use tauri::App;

#[cfg(mobile)]
mod mobile;
#[cfg(mobile)]
pub use mobile::*;

pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn get_config() -> String {
    let contents = std::fs::read_to_string("/data/data/com.emstone.moview/files/nvr-viewer.conf")
        .expect("Should have been able to read the file");

    contents
}

#[derive(Default)]
pub struct AppBuilder {
    setup: Option<SetupHook>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn setup<F>(mut self, setup: F) -> Self
    where
        F: FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        self.setup.replace(Box::new(setup));
        self
    }

    pub fn run(self) {
        let setup = self.setup;
        tauri::Builder::default()
            .setup(move |app| {
                if let Some(setup) = setup {
                    (setup)(app)?;
                }
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![greet, get_config])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}

#[cfg(mobile)]
fn do_something() {
    println!("Hello from Mobile!");
}

#[cfg(desktop)]
fn do_something() {
    println!("Hello from Desktop!");
}

fn run() {
    if cfg!(mobile) {
        println!("Hello from Mobile!");
    } else {
        println!("Hello from Desktop!");
    }
}
