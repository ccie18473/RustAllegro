use std::cast;
use std::libc::*;
use std::ptr;
use std::num::Zero;
use std::i32;

use core_drawing::*;
use color::*;
use bitmap::*;
use bitmap::private::*;

use ffi::*;

pub use display::display_flag::*;

flag_type!(
	mod display_flag
	{
		DisplayFlags
		{
			WINDOWED                  = ALLEGRO_WINDOWED,
			FULLSCREEN                = ALLEGRO_FULLSCREEN,
			OPENGL                    = ALLEGRO_OPENGL,
			DIRECT3D                  = ALLEGRO_DIRECT3D_INTERNAL,
			RESIZABLE                 = ALLEGRO_RESIZABLE,
			FRAMELESS                 = ALLEGRO_FRAMELESS,
			GENERATE_EXPOSE_EVENTS    = ALLEGRO_GENERATE_EXPOSE_EVENTS,
			OPENGL_3_0                = ALLEGRO_OPENGL_3_0,
			OPENGL_FORWARD_COMPATIBLE = ALLEGRO_OPENGL_FORWARD_COMPATIBLE,
			FULLSCREEN_WINDOW         = ALLEGRO_FULLSCREEN_WINDOW,
			MINIMIZED                 = ALLEGRO_MINIMIZED
		}
	}
)

enum DisplayOption
{
	RedSize = ALLEGRO_RED_SIZE,
	GreenSize = ALLEGRO_GREEN_SIZE,
	BlueSize = ALLEGRO_BLUE_SIZE,
	AlphaSize = ALLEGRO_ALPHA_SIZE,
	RedShift = ALLEGRO_RED_SHIFT,
	GreenShift = ALLEGRO_GREEN_SHIFT,
	BlueShift = ALLEGRO_BLUE_SHIFT,
	AlphaShift = ALLEGRO_ALPHA_SHIFT,
	AccRedSize = ALLEGRO_ACC_RED_SIZE,
	AccGreenSize = ALLEGRO_ACC_GREEN_SIZE,
	AccBlueSize = ALLEGRO_ACC_BLUE_SIZE,
	AccAlphaSize = ALLEGRO_ACC_ALPHA_SIZE,
	Stereo = ALLEGRO_STEREO,
	AuxBuffers = ALLEGRO_AUX_BUFFERS,
	ColorSize = ALLEGRO_COLOR_SIZE,
	DepthSize = ALLEGRO_DEPTH_SIZE,
	StencilSize = ALLEGRO_STENCIL_SIZE,
	SampleBuffers = ALLEGRO_SAMPLE_BUFFERS,
	Samples = ALLEGRO_SAMPLES,
	RenderMethod = ALLEGRO_RENDER_METHOD,
	FloatColor = ALLEGRO_FLOAT_COLOR,
	FloatDepth = ALLEGRO_FLOAT_DEPTH,
	SingleBuffer = ALLEGRO_SINGLE_BUFFER,
	SwapMethod = ALLEGRO_SWAP_METHOD,
	CompatibleDisplay = ALLEGRO_COMPATIBLE_DISPLAY,
	UpdateDisplayRegion = ALLEGRO_UPDATE_DISPLAY_REGION,
	Vsync = ALLEGRO_VSYNC,
	MaxBitmapSize = ALLEGRO_MAX_BITMAP_SIZE,
	SupportNpotBitmap = ALLEGRO_SUPPORT_NPOT_BITMAP,
	CanDrawIntoBitmap = ALLEGRO_CAN_DRAW_INTO_BITMAP,
	SupportSeparateAlpha = ALLEGRO_SUPPORT_SEPARATE_ALPHA,
}

enum DisplayOptionImportance
{
	DontCare = ALLEGRO_DONTCARE,
	Require = ALLEGRO_REQUIRE,
	Suggest = ALLEGRO_SUGGEST,
}

enum DisplayOrientation
{
	DisplayOrientation0Degrees = ALLEGRO_DISPLAY_ORIENTATION_0_DEGREES,
	DisplayOrientation90Degrees = ALLEGRO_DISPLAY_ORIENTATION_90_DEGREES,
	DisplayOrientation180Degrees = ALLEGRO_DISPLAY_ORIENTATION_180_DEGREES,
	DisplayOrientation270Degrees = ALLEGRO_DISPLAY_ORIENTATION_270_DEGREES,
	DisplayOrientationFaceUp = ALLEGRO_DISPLAY_ORIENTATION_FACE_UP,
	DisplayOrientationFaceDown = ALLEGRO_DISPLAY_ORIENTATION_FACE_DOWN,
}

struct DisplayOptions<'self>
{
	flags: DisplayFlags,
	refresh_rate: Option<int>,
	adapter: Option<int>,
	window_position: Option<[int, ..2]>,
	options: Option<&'self [(DisplayOption, i32, DisplayOptionImportance)]>
}

impl<'self> DisplayOptions<'self>
{
	fn new() -> DisplayOptions
	{
		DisplayOptions{ flags: Zero::zero(), refresh_rate: None, adapter: None, window_position: None, options: None }
	}
}

struct Display
{
	priv allegro_display: *mut ALLEGRO_DISPLAY,
	priv backbuffer: Bitmap
}

impl Display
{
	pub fn new(w: int, h: int) -> Option<Display>
	{
		unsafe
		{
			let d = al_create_display(w as c_int, h as c_int);
			if ptr::is_null(d)
			{
				None
			}
			else
			{
				Some(Display{ allegro_display: d, backbuffer: bitmap_ref(al_get_backbuffer(d)) })
			}
		}
	}

	pub fn new_with_options(w: int, h: int, opt: &DisplayOptions) -> Option<Display>
	{
		unsafe
		{
			al_set_new_display_flags(opt.flags.get() as c_int);

			match opt.refresh_rate
			{
				Some(r) => al_set_new_display_refresh_rate(r as c_int),
				None => al_set_new_display_refresh_rate(0)
			}

			match opt.adapter
			{
				Some(a) => al_set_new_display_adapter(a as c_int),
				None => al_set_new_display_adapter(ALLEGRO_DEFAULT_DISPLAY_ADAPTER),
			}

			match opt.window_position
			{
				Some([x, y]) =>	al_set_new_window_position(x as c_int, y as c_int),
				None =>	al_set_new_window_position(i32::max_value, i32::max_value)
			}

			al_reset_new_display_options();

			match opt.options
			{
				Some(options) =>
				{
					for &(option, value, importance) in options.iter()
					{
						al_set_new_display_option(option as c_int, value as c_int, importance as c_int);
					}
				},
				None => ()
			}
		}

		Display::new(w, h)
	}

	pub fn get_width(&self) -> float
	{
		unsafe
		{
			al_get_display_width(self.allegro_display) as float
		}
	}

	pub fn get_height(&self) -> float
	{
		unsafe
		{
			al_get_display_height(self.allegro_display) as float
		}
	}

	pub fn get_format(&self) -> PixelFormat
	{
		unsafe
		{
			cast::transmute(al_get_display_format(self.allegro_display) as int)
		}
	}

	pub fn get_refresh_rate(&self) -> int
	{
		unsafe
		{
			al_get_display_refresh_rate(self.allegro_display) as int
		}
	}

	pub fn get_flags(&self) -> DisplayFlags
	{
		unsafe
		{
			cast::transmute(al_get_display_flags(self.allegro_display))
		}
	}

	pub fn set_flag(&self, flag: DisplayFlags, onoff: bool) -> bool
	{
		unsafe
		{
			al_set_display_flag(self.allegro_display, flag.get(), onoff as u8) != 0
		}
	}

	pub fn get_backbuffer<'l>(&'l self) -> &'l Bitmap
	{
		&self.backbuffer
	}

	pub fn acknowledge_resize(&self) -> bool
	{
		unsafe
		{
			al_acknowledge_resize(self.allegro_display) != 0
		}
	}

	pub fn resize(&self, w: int, h: int) -> bool
	{
		unsafe
		{
			al_resize_display(self.allegro_display, w as c_int, h as c_int) != 0
		}
	}

	pub fn flip(&self)
	{
		unsafe
		{
			al_flip_display();
		}
	}
}

impl Drop for Display
{
	fn drop(&self)
	{
		unsafe
		{
			al_destroy_display(self.allegro_display);
		}
	}
}

impl DrawTarget for Display
{
	fn get_target_bitmap(&self) -> *mut ALLEGRO_BITMAP
	{
		unsafe
		{
			al_get_backbuffer(self.allegro_display)
		}
	}
}

impl CoreDrawing for Display;
