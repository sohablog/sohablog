use crate::{
	utils::{Page, TemplateContext, StaticFile},
	interfaces::models::{Content, Author}
};
use std::{
	io,
	any::Any,
};
#[cfg(feature = "main")]
use std::{
	collections::HashMap,
	ffi::OsStr,
};

#[cfg(feature = "main")]
use libloading::{Library, Symbol};

#[repr(u8)]
pub enum PluginType {
	Theme,
}
pub trait PluginMetadata: Any + Send + Sync {
	/// Plugin trait version, must be equal with PLUGIN_TRAIT_VERSION.
	/// Or the plugin will not be loaded.
	fn plugin_version(&self) -> u32;
	/// Plugin name
	fn name(&self) -> &'static str;
	/// Plugin description
	fn description(&self) -> &'static str;
	/// Plugin version
	fn version(&self) -> &'static str;
	/// Plugin type
	fn r#type(&self) -> PluginType;
}

pub const THEME_TRAIT_VERSION: u32 = 1;
/// Although theme is also a dynamically loaded plugin, it needs a special interface
pub trait Theme: PluginMetadata {
	/// Theme identity string, should be unique
	fn identity(&self) -> &'static str;
	/// This function should write the render result for post list page to `out`
	fn post_list(&self, out: &mut io::Write, ctx: &TemplateContext, title: &str, page: Page, posts: Vec<Box<Content>>) -> io::Result<()>;
	/// This function should write the render result for post detail page to `out`
	fn post_show(&self, out: &mut io::Write, ctx: &TemplateContext, title: &str, post: Box<Content>, previous_author: Option<Box<Author>>) -> io::Result<()>;
	/// This function should return `StaticFile` struct for server to serve static files
	fn static_file(&self, name: &str) -> Option<Box<StaticFile>>;
}

pub const PLUGIN_TRAIT_VERSION: u32 = 0;
pub trait Plugin: PluginMetadata {
	// TODO: Finish this.
}

#[macro_export]
macro_rules! declare_plugin_metadata {
	($plugin:ty, $constructor:path) => {
		#[no_mangle]
		pub extern "C" fn _plugin_metadata() -> *mut $crate::plugin::PluginMetadata {
			let constructor: fn() -> $plugin = $constructor;
			let object = constructor();
			let boxed: Box<$crate::plugin::PluginMetadata> = Box::new(object);
			Box::into_raw(boxed)
		}
	};
}
#[macro_export]
macro_rules! declare_plugin {
	($plugin:ty, $constructor:path, $plugin_type:path) => {
		#[no_mangle]
		pub extern "C" fn _plugin_create() -> *mut $plugin_type {
			let constructor: fn() -> $plugin = $constructor;
			let object = constructor();
			let boxed: Box<$plugin_type> = Box::new(object);
			Box::into_raw(boxed)
		}
	};
}

#[cfg(feature = "main")]
pub struct PluginManager {
	plugins: Vec<Box<Plugin>>,
	themes: HashMap<String, Box<Theme>>,
	loaded_libraries: Vec<Library>,
}
#[allow(unreachable_patterns)]
#[cfg(feature = "main")]
impl PluginManager {
	pub fn new() -> Self {
		Self {
			plugins: Vec::new(),
			themes: HashMap::new(),
			loaded_libraries: Vec::new(),
		}
	}

	pub fn get_theme(&self, name: &String) -> Option<&Box<Theme>> {
		self.themes.get(name)
	}

	pub unsafe fn load<T: AsRef<OsStr>>(&mut self, filename: T) -> Result<(), &str> {
		let lib = Library::new(filename.as_ref()).map_err(|_| "Unable to load the theme")?;
		self.loaded_libraries.push(lib);
		let lib = self.loaded_libraries.last().unwrap();

		let constructor: Symbol<unsafe extern fn () -> *mut PluginMetadata> = lib.get(b"_plugin_metadata").map_err(|_| "Not a valid plugin library")?;
		let raw_box = constructor();
		let metadata: Box<PluginMetadata> = Box::from_raw(raw_box);

		match metadata.r#type() {
			PluginType::Theme => {
				let constructor: Symbol<unsafe extern fn () -> *mut Theme> = lib.get(b"_plugin_create").map_err(|_| "Not a valid plugin library")?;
				let raw_box = constructor();
				let theme: Box<Theme> = Box::from_raw(raw_box);
				if theme.plugin_version() != THEME_TRAIT_VERSION {
					return Err("Theme version is not compatible.")
				}
				self.themes.insert(String::from(theme.identity()), theme);
			},
			_ => {
				let constructor: Symbol<unsafe extern fn () -> *mut Plugin> = lib.get(b"_plugin_create").map_err(|_| "Not a valid plugin library")?;
				let raw_box = constructor();
				let plugin: Box<Plugin> = Box::from_raw(raw_box);
				if plugin.plugin_version() != PLUGIN_TRAIT_VERSION {
					return Err("Plugin version is not compatible.")
				}
				self.plugins.push(plugin);
			}
		}

		Ok(())
	}

	pub fn load_from_dir(&mut self, path: &String) -> std::io::Result<()> {
		use std::{path::{Path, PathBuf}, fs::read_dir};
		let path = Path::new(path);
		if path.is_dir() {
			for file in read_dir(path)? {
				let file = file?;
				let path: PathBuf = file.path();
				if path.is_file() {
					// TODO: LOG
					unsafe {
						if let Err(e) = self.load(path.as_os_str()) {
							dbg!(e);
						} else {
							dbg!(format!("library loaded from {:?}", path));
						}
					}
				}
			}
		}
		Ok(())
	}
}
