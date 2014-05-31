// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use ffi::*;
use rust_util::Flag;

pub mod key
{
	use libc::*;
	use std::mem;

	#[deriving(PartialEq)]
	#[repr(u32)]
	pub enum KeyCode
	{
		A = 1,
		B = 2,
		C = 3,
		D = 4,
		E = 5,
		F = 6,
		G = 7,
		H = 8,
		I = 9,
		J = 10,
		K = 11,
		L = 12,
		M = 13,
		N = 14,
		O = 15,
		P = 16,
		Q = 17,
		R = 18,
		S = 19,
		T = 20,
		U = 21,
		V = 22,
		W = 23,
		X = 24,
		Y = 25,
		Z = 26,
		_0 = 27,
		_1 = 28,
		_2 = 29,
		_3 = 30,
		_4 = 31,
		_5 = 32,
		_6 = 33,
		_7 = 34,
		_8 = 35,
		_9 = 36,
		Pad0 = 37,
		Pad1 = 38,
		Pad2 = 39,
		Pad3 = 40,
		Pad4 = 41,
		Pad5 = 42,
		Pad6 = 43,
		Pad7 = 44,
		Pad8 = 45,
		Pad9 = 46,
		F1 = 47,
		F2 = 48,
		F3 = 49,
		F4 = 50,
		F5 = 51,
		F6 = 52,
		F7 = 53,
		F8 = 54,
		F9 = 55,
		F10 = 56,
		F11 = 57,
		F12 = 58,
		Escape = 59,
		Tilde = 60,
		Minus = 61,
		Equals = 62,
		Backspace = 63,
		Tab = 64,
		Openbrace = 65,
		Closebrace = 66,
		Enter = 67,
		Semicolon = 68,
		Quote = 69,
		Backslash = 70,
		Backslash2 = 71,
		Comma = 72,
		Fullstop = 73,
		Slash = 74,
		Space = 75,
		Insert = 76,
		Delete = 77,
		Home = 78,
		End = 79,
		PgUp = 80,
		PgDn = 81,
		Left = 82,
		Right = 83,
		Up = 84,
		Down = 85,
		PadSlash = 86,
		PadAsterisk = 87,
		PadMinus = 88,
		PadPlus = 89,
		PadDelete = 90,
		PadEnter = 91,
		PrintScreen = 92,
		Pause = 93,
		AbntC1 = 94,
		Yen = 95,
		Kana = 96,
		Convert = 97,
		NoConvert = 98,
		At = 99,
		Circumflex = 100,
		Colon2 = 101,
		Kanji = 102,
		PadEquals = 103,
		Backquote = 104,
		Semicolon2 = 105,
		Command = 106,
		Unknown = 107,

		LShift = 215,
		RShift = 216,
		LCtrl = 217,
		RCtrl = 218,
		Alt = 219,
		AltGr = 220,
		LWin = 221,
		RWin = 222,
		Menu = 223,
		ScrollLock = 224,
		NumLock = 225,
		CapsLock = 226,
	}

	impl KeyCode
	{
		pub unsafe fn from_allegro_key(k: c_int) -> KeyCode
		{
			mem::transmute(k as u32)
		}
	}
}

flag_type!(
	KeyModifier
	{
		SHIFT = ALLEGRO_KEYMOD_SHIFT,
		CTRL = ALLEGRO_KEYMOD_CTRL,
		ALT = ALLEGRO_KEYMOD_ALT,
		LWIN = ALLEGRO_KEYMOD_LWIN,
		RWIN = ALLEGRO_KEYMOD_RWIN,
		MENU = ALLEGRO_KEYMOD_MENU,
		ALTGR = ALLEGRO_KEYMOD_ALTGR,
		COMMAND = ALLEGRO_KEYMOD_COMMAND,
		SCROLLLOCK = ALLEGRO_KEYMOD_SCROLLLOCK,
		NUMLOCK = ALLEGRO_KEYMOD_NUMLOCK,
		CAPSLOCK = ALLEGRO_KEYMOD_CAPSLOCK,
		INALTSEQ = ALLEGRO_KEYMOD_INALTSEQ,
		ACCENT1 = ALLEGRO_KEYMOD_ACCENT1,
		ACCENT2 = ALLEGRO_KEYMOD_ACCENT2,
		ACCENT3 = ALLEGRO_KEYMOD_ACCENT3,
		ACCENT4 = ALLEGRO_KEYMOD_ACCENT4
	}
)
