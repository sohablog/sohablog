use crate::{
	utils::{Page, TemplateContext},
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
}

pub const PLUGIN_TRAIT_VERSION: u32 = 0;
pub trait Plugin: PluginMetadata {
	// TODO: Finish this.
}

#[macro_export]
macro_rules! declare_plugin_metadata {
	($plugin:ty, $constructor:path) => {
		#[no_mangle]
		pub extern "C" fn _plugin_metadata() -> *mut $crate::PluginMetadata {
			let constructor: fn() -> $plugin = $constructor;
			let object = constructor();
			let boxed: Box<$crate::PluginMetadata> = Box::new(object);
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
	themes: HashMap<&'static str, Box<Theme>>,
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

	pub unsafe fn load<T: AsRef<OsStr>>(&mut self, filename: T) -> Result<(), &str> {
		type PluginMetadataConstructor = unsafe fn () -> *mut PluginMetadata;
		let lib = Library::new(filename.as_ref()).map_err(|_| "Unable to load the theme")?;
		self.loaded_libraries.push(lib);
		let lib = self.loaded_libraries.last().unwrap();
		let constructor: Symbol<PluginMetadataConstructor> = lib.get(b"_plugin_metadata").map_err(|_| "Not a valid plugin library")?;
		let raw_box = constructor();
		let metadata: Box<PluginMetadata> = Box::from_raw(raw_box);

		match metadata.r#type() {
			PluginType::Theme => {
				type PluginConstructor = unsafe fn () -> *mut Theme;
				let constructor: Symbol<PluginConstructor> = lib.get(b"_plugin_create").map_err(|_| "Not a valid plugin library")?;
				let raw_box = constructor();
				let theme: Box<Theme> = Box::from_raw(raw_box);
				if theme.plugin_version() != THEME_TRAIT_VERSION {
					return Err("Plugin version is not compatible.")
				}
				self.themes.insert(&theme.identity(), theme);
			},
			_ => {
				type PluginConstructor = unsafe fn () -> *mut Plugin;
				let constructor: Symbol<PluginConstructor> = lib.get(b"_plugin_create").map_err(|_| "Not a valid plugin library")?;
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
}
