// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use std::mem;
use libc::*;
use std::option::Some;
use std::kinds::marker::NoSend;

use internal::bitmap_like::*;
use internal::color::*;

use ffi::*;

pub mod external
{
	pub use super::{Bitmap, SubBitmap, MemoryBitmap};
}

pub struct Bitmap
{
	allegro_bitmap: *mut ALLEGRO_BITMAP,
	is_ref: bool,
	no_send_marker: NoSend
}

impl Bitmap
{
	fn new(w: i32, h: i32) -> Result<Bitmap, ()>
	{
		unsafe
		{
			let b =	al_create_bitmap(w as c_int, h as c_int);
			if b.is_null()
			{
				Err(())
			}
			else
			{
				Ok(Bitmap{ allegro_bitmap: b, is_ref: false, no_send_marker: NoSend })
			}
		}
	}

	fn load(filename: &str) -> Result<Bitmap, ()>
	{
		let b = filename.with_c_str(|s|
		{
			unsafe
			{
				al_load_bitmap(s)
			}
		});
		if b.is_null()
		{
			Err(())
		}
		else
		{
			Ok(Bitmap{ allegro_bitmap: b, is_ref: false, no_send_marker: NoSend })
		}
	}

	pub fn create_sub_bitmap<'l>(&'l self, x: i32, y: i32, w: i32, h: i32) -> Result<SubBitmap<'l>, ()>
	{
		unsafe
		{
			let b = al_create_sub_bitmap(self.allegro_bitmap, x as c_int, y as c_int, w as c_int, h as c_int);
			if b.is_null()
			{
				Err(())
			}
			else
			{
				Ok(SubBitmap{ allegro_bitmap: b, parent: self })
			}
		}
	}

	pub fn maybe_clone(&self) -> Result<Bitmap, ()>
	{
		clone_bitmap(self.allegro_bitmap)
	}

	pub fn into_memory_bitmap(self) -> Result<MemoryBitmap, Bitmap>
	{
		if self.get_flags() & MEMORY_BITMAP && !self.is_ref
		{
			let bmp = self.allegro_bitmap;
			unsafe
			{
				// Don't run Bitmap's destructor
				mem::forget(self);
			}
			Ok(MemoryBitmap{ allegro_bitmap: bmp })
		}
		else
		{
			Err(self)
		}
	}
}

impl BitmapLike for Bitmap
{
	fn get_bitmap(&self) -> *mut ALLEGRO_BITMAP
	{
		self.allegro_bitmap
	}
}

impl Clone for Bitmap
{
	fn clone(&self) -> Bitmap
	{
		self.maybe_clone().unwrap()
	}
}

// Not Send just because of the marker
#[unsafe_destructor]
impl Drop for Bitmap
{
	fn drop(&mut self)
	{
		if !self.is_ref
		{
			unsafe
			{
				handle_bitmap_destruction(self.allegro_bitmap, false);
			}
		}
	}
}

pub struct MemoryBitmap
{
	allegro_bitmap: *mut ALLEGRO_BITMAP,
}

impl MemoryBitmap
{
	pub fn into_bitmap(self) -> Bitmap
	{
		let bmp = self.allegro_bitmap;
		unsafe
		{
			// Don't run MemoryBitmap's destructor
			mem::forget(self);
		}
		Bitmap{ allegro_bitmap: bmp, is_ref: false, no_send_marker: NoSend }
	}
}

impl Drop for MemoryBitmap
{
	fn drop(&mut self)
	{
		unsafe
		{
			handle_bitmap_destruction(self.allegro_bitmap, false);
		}
	}
}

pub struct SubBitmap<'m>
{
	allegro_bitmap: *mut ALLEGRO_BITMAP,
	parent: &'m Bitmap
}

impl<'m> SubBitmap<'m>
{
	pub fn create_sub_bitmap<'l>(&'l self, x: i32, y: i32, w: i32, h: i32) -> Result<SubBitmap<'l>, ()>
	{
		unsafe
		{
			let b = al_create_sub_bitmap(self.allegro_bitmap, x as c_int, y as c_int, w as c_int, h as c_int);
			if b.is_null()
			{
				Err(())
			}
			else
			{
				Ok(SubBitmap{ allegro_bitmap: b, parent: self.parent })
			}
		}
	}

	pub fn get_parent<'l>(&'l self) -> &'l Bitmap
	{
		self.parent
	}

	pub fn to_bitmap(&self) -> Result<Bitmap, ()>
	{
		clone_bitmap(self.allegro_bitmap)
	}
}

impl<'m> BitmapLike for SubBitmap<'m>
{
	fn get_bitmap(&self) -> *mut ALLEGRO_BITMAP
	{
		self.allegro_bitmap
	}
}

/* Should be safe, as the parent doesn't reference the child */
#[unsafe_destructor]
impl<'l> Drop for SubBitmap<'l>
{
	fn drop(&mut self)
	{
		unsafe
		{
			handle_bitmap_destruction(self.allegro_bitmap, true);
		}
	}
}

unsafe fn handle_bitmap_destruction(bmp: *mut ALLEGRO_BITMAP, is_sub_bitmap: bool)
{
	use internal::core::dummy_target;

	/* If this bitmap is the target or the parent of the target
	 * then the target becomes invalid. Set it to the dummy target. */
	let target = al_get_target_bitmap();
	if target.is_null()
	{
		fail!("Null target bitmap!");
	}
	if target == bmp || (!is_sub_bitmap && al_get_parent_bitmap(target) == bmp)
	{
		al_set_target_bitmap(dummy_target);
	}
	al_destroy_bitmap(bmp);
}

pub fn new_bitmap_ref(bmp: *mut ALLEGRO_BITMAP) -> Bitmap
{
	Bitmap{ allegro_bitmap: bmp, is_ref: true, no_send_marker: NoSend }
}

pub fn clone_bitmap(bmp: *mut ALLEGRO_BITMAP) -> Result<Bitmap, ()>
{
	unsafe
	{
		let b = al_clone_bitmap(bmp);
		if b.is_null()
		{
			Err(())
		}
		else
		{
			Ok(Bitmap{ allegro_bitmap: b, is_ref: false, no_send_marker: NoSend })
		}
	}
}

impl ::internal::core::Core
{
	pub fn set_new_bitmap_flags(&self, flags: BitmapFlags)
	{
		unsafe
		{
			al_set_new_bitmap_flags(flags.get() as c_int);
		}
	}

	pub fn get_new_bitmap_flags(&self) -> BitmapFlags
	{
		unsafe
		{
			mem::transmute(al_get_new_bitmap_flags() as u32)
		}
	}

	pub fn set_new_bitmap_format(&self, format: PixelFormat)
	{
		unsafe
		{
			al_set_new_bitmap_format(format as c_int);
		}
	}

	pub fn get_new_bitmap_format(&self) -> PixelFormat
	{
		unsafe
		{
			mem::transmute(al_get_new_bitmap_format() as u32)
		}
	}

	pub fn create_bitmap(&self, w: i32, h: i32) -> Result<Bitmap, ()>
	{
		Bitmap::new(w, h)
	}

	pub fn load_bitmap(&self, filename: &str) -> Result<Bitmap, ()>
	{
		Bitmap::load(filename)
	}
}
