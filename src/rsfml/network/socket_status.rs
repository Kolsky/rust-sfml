/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
* 
* 3. This notice may not be removed or altered from any source distribution.
*/

/*!
* Status codes that may be returned by socket functions.
*
*
*
*/

use std::libc::c_int;

pub mod ffi {

	use std::libc::c_int;

	pub type SocketStatus = c_int;
	pub static SOCKETNONE: 			SocketStatus = 0;
	pub static SOCKETNOTREADY: 		SocketStatus = 1;
	pub static SOCKETDISCONNECTED: 	SocketStatus = 2;
	pub static SOCKETERROR: 		SocketStatus = 3;
}

pub enum SocketStatus {
    SocketNone = 			ffi::SOCKETNONE as c_int,
    SocketNotReady = 		ffi::SOCKETNOTREADY as c_int,
    SocketDisconnected = 	ffi::SOCKETDISCONNECTED as c_int,
    SocketError = 			ffi::SOCKETERROR as c_int
}