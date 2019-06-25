use crate::{
	util::GlobalContext,
	utils::Page,
	models::{
		content::Content,
		comment::Author
	},
};
use std::{
	io,
	any::Any,
	collections::HashMap,
	ffi::OsStr,
};
use libloading::{Library, Symbol};

pub const THEME_TRAIT_VERSION: u32 = 1;
/// Although theme is also a dynamically loaded plugin, it needs a special interface
pub trait Theme: Any + Send + Sync {
	/// Plugin trait version, must be equal with THEME_TRAIT_VERSION.
	/// Or the plugin will not be loaded.
	fn plugin_version(&self) -> u32;
	/// Theme identity string, should be unique
	fn identity(&self) -> &'static str;
	/// Theme name
	fn name(&self) -> &'static str;
	/// Theme description, where you can place author info
	fn description(&self) -> &'static str;
	/// Theme version
	fn version(&self) -> &'static str;
	/// This function should write the render result for post list page to `out`
	fn post_list(&self, out: &mut io::Write, ctx: &GlobalContext, title: &str, page: Page, posts: Vec<Content>) -> io::Result<()>;
	/// This function should write the render result for post detail page to `out`
	fn post_show(&self, out: &mut io::Write, ctx: &GlobalContext, title: &str, post: Content, previous_author: Option<Author>) -> io::Result<()>;
}

pub const PLUGIN_TRAIT_VERSION: u32 = 0;
pub trait Plugin: Any + Send + Sync {
	/// Plugin trait version, must be equal with PLUGIN_TRAIT_VERSION.
	/// Or the plugin will not be loaded.
	fn plugin_version(&self) -> u32;
	/// Plugin name
	fn name(&self) -> &'static str;
	/// Plugin description
	fn description(&self) -> &'static str;
	/// Plugin version
	fn version(&self) -> &'static str;
	// TODO: Finish this.
}

#[macro_export]
macro_rules! declare_plugin {
	($plugin_type:ty, $constructor:path) => {
		#[no_mangle]
		pub extern "C" fn _plugin_create() -> *mut $crate::Plugin {
			let constructor: fn() -> $plugin_type = $constructor;
			let object = constructor();
			let boxed: Box<$crate::Plugin> = Box::new(object);
			Box::into_raw(boxed)
		}
	};
}

pub struct PluginManager {
	plugins: Vec<Box<Plugin>>,
	themes: HashMap<&'static str, Box<Theme>>,
	loaded_libraries: Vec<Library>,
}
impl PluginManager {
	pub fn new() -> Self {
		Self {
			plugins: Vec::new(),
			themes: HashMap::new(),
			loaded_libraries: Vec::new(),
		}
	}

	pub unsafe fn load_theme<T: AsRef<OsStr>>(&mut self, filename: T) -> Result<(), &str> {
		type PluginConstructor = unsafe fn () -> *mut Theme;
		let lib = Library::new(filename.as_ref()).map_err(|_| "Unable to load the theme")?;
		self.loaded_libraries.push(lib);
		let lib = self.loaded_libraries.last().unwrap();
		let constructor: Symbol<PluginConstructor> = lib.get(b"_plugin_create").map_err(|_| "Not a valid plugin library")?;
		let raw_box = constructor();
		let theme: Box<Theme> = Box::from_raw(raw_box);
		if theme.plugin_version() != THEME_TRAIT_VERSION {
			return Err("Plugin version is not compatible.")
		}
		self.themes.insert(&theme.identity(), theme);

		Ok(())
	}
}
