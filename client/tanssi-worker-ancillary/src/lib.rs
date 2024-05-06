// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Contains the logic for executing PVFs. Used by the polkadot-execute-worker binary.

#![allow(unsafe_code)]

use std::ffi::CStr;
use std::path::Path;
//use std::sys::net::Socket;
use std::io::{self, IoSlice, IoSliceMut};
use std::marker::PhantomData;
use std::mem::size_of;
use std::mem::zeroed;
use std::os::fd::{AsRawFd, OwnedFd, RawFd};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::net::{SocketAddr, UnixStream};
use std::ptr;
use std::ptr::{eq, read_unaligned};
use std::slice::from_raw_parts;

//use std::os::unix::net::SocketAncillary;

#[derive(Debug)]
pub struct SocketAncillary<'a> {
    buffer: &'a mut [u8],
    length: usize,
    truncated: bool,
}

pub struct Socket(pub OwnedFd);

impl Socket {
    #[cfg(any(target_os = "android", target_os = "linux"))]
    pub fn send_msg(&self, msg: &mut libc::msghdr) -> io::Result<usize> {
        let n = cvt(unsafe { libc::sendmsg(self.as_raw_fd(), msg, 0) })?;
        Ok(n as usize)
    }

    #[cfg(any(target_os = "android", target_os = "linux"))]
    pub fn recv_msg(&self, msg: &mut libc::msghdr) -> io::Result<usize> {
        let n = cvt(unsafe { libc::recvmsg(self.as_raw_fd(), msg, libc::MSG_CMSG_CLOEXEC) })?;
        Ok(n as usize)
    }
}

impl AsRawFd for Socket {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

pub fn cvt(t: libc::ssize_t) -> std::io::Result<libc::ssize_t> {
    if t == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(t)
    }
}

pub fn recv_vectored_with_ancillary_from(
    socket: &Socket,
    bufs: &mut [IoSliceMut<'_>],
    ancillary: &mut SocketAncillary<'_>,
) -> io::Result<(usize, bool, io::Result<SocketAddr>)> {
    unsafe {
        let mut msg_name: libc::sockaddr_un = zeroed();
        let mut msg: libc::msghdr = zeroed();
        msg.msg_name = core::ptr::addr_of_mut!(msg_name) as *mut _;
        msg.msg_namelen = size_of::<libc::sockaddr_un>() as libc::socklen_t;
        msg.msg_iov = bufs.as_mut_ptr().cast();
        msg.msg_iovlen = bufs.len() as _;
        msg.msg_controllen = ancillary.buffer.len() as _;
        // macos requires that the control pointer is null when the len is 0.
        if msg.msg_controllen > 0 {
            msg.msg_control = ancillary.buffer.as_mut_ptr().cast();
        }

        let count = socket.recv_msg(&mut msg)?;

        ancillary.length = msg.msg_controllen as usize;
        ancillary.truncated = msg.msg_flags & libc::MSG_CTRUNC == libc::MSG_CTRUNC;

        let truncated = msg.msg_flags & libc::MSG_TRUNC == libc::MSG_TRUNC;
        //let addr = SocketAddr::from_parts(msg_name, msg.msg_namelen);
        //let sun_path: [libc::c_char; 108] = msg_name.sun_path;
        let sun_path_cstr = CStr::from_ptr(&msg_name.sun_path as *const i8);
        let sun_path_str_slice = sun_path_cstr
            .to_str()
            .expect("Failed to convert CStr to str");
        let addr = SocketAddr::from_pathname(&sun_path_str_slice);

        Ok((count, truncated, addr))
    }
}

pub fn send_vectored_with_ancillary_to(
    socket: &Socket,
    path: Option<&Path>,
    bufs: &[IoSlice<'_>],
    ancillary: &mut SocketAncillary<'_>,
) -> io::Result<usize> {
    unsafe {
        let (mut msg_name, msg_namelen) = if let Some(path) = path {
            sockaddr_un(path)?
        } else {
            (zeroed(), 0)
        };

        let mut msg: libc::msghdr = zeroed();
        msg.msg_name = core::ptr::addr_of_mut!(msg_name) as *mut _;
        msg.msg_namelen = msg_namelen;
        msg.msg_iov = bufs.as_ptr() as *mut _;
        msg.msg_iovlen = bufs.len() as _;
        msg.msg_controllen = ancillary.length as _;
        // macos requires that the control pointer is null when the len is 0.
        if msg.msg_controllen > 0 {
            msg.msg_control = ancillary.buffer.as_mut_ptr().cast();
        }

        ancillary.truncated = false;

        socket.send_msg(&mut msg)
    }
}

fn sun_path_offset(addr: &libc::sockaddr_un) -> usize {
    // Work with an actual instance of the type since using a null pointer is UB
    let base = (addr as *const libc::sockaddr_un) as usize;
    let path = core::ptr::addr_of!(addr.sun_path) as usize;
    path - base
}

pub fn sockaddr_un(path: &Path) -> io::Result<(libc::sockaddr_un, libc::socklen_t)> {
    // SAFETY: All zeros is a valid representation for `sockaddr_un`.
    let mut addr: libc::sockaddr_un = unsafe { zeroed() };
    addr.sun_family = libc::AF_UNIX as libc::sa_family_t;

    let bytes = path.as_os_str().as_bytes();

    if bytes.contains(&0) {
        /*
        return Err(io::const_io_error!(
            io::ErrorKind::InvalidInput,
            "paths must not contain interior null bytes",
        ));
        */
        panic!("todo: return error")
    }

    if bytes.len() >= addr.sun_path.len() {
        /*
        return Err(io::const_io_error!(
            io::ErrorKind::InvalidInput,
            "path must be shorter than SUN_LEN",
        ));
        */
        panic!("todo: return error")
    }
    // SAFETY: `bytes` and `addr.sun_path` are not overlapping and
    // both point to valid memory.
    // NOTE: We zeroed the memory above, so the path is already null
    // terminated.
    unsafe {
        ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            addr.sun_path.as_mut_ptr().cast(),
            bytes.len(),
        )
    };

    let mut len = sun_path_offset(&addr) + bytes.len();
    match bytes.get(0) {
        Some(&0) | None => {}
        Some(_) => len += 1,
    }
    Ok((addr, len as libc::socklen_t))
}

impl<'a> SocketAncillary<'a> {
    /// Create an ancillary data with the given buffer.
    pub fn new(buffer: &'a mut [u8]) -> Self {
        SocketAncillary {
            buffer,
            length: 0,
            truncated: false,
        }
    }

    /// Returns the capacity of the buffer.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    /// Returns `true` if the ancillary data is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the number of used bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.length
    }

    /// Is `true` if during a recv operation the ancillary was truncated.
    #[must_use]
    pub fn truncated(&self) -> bool {
        self.truncated
    }

    /// Add file descriptors to the ancillary data.
    ///
    /// The function returns `true` if there was enough space in the buffer.
    /// If there was not enough space then no file descriptors was appended.
    /// Technically, that means this operation adds a control message with the level `SOL_SOCKET`
    /// and type `SCM_RIGHTS`.
    pub fn add_fds(&mut self, fds: &[RawFd]) -> bool {
        self.truncated = false;
        add_to_ancillary_data(
            &mut self.buffer,
            &mut self.length,
            fds,
            libc::SOL_SOCKET,
            libc::SCM_RIGHTS,
        )
    }

    /// Clears the ancillary data, removing all values.
    pub fn clear(&mut self) {
        self.length = 0;
        self.truncated = false;
    }

    /// Returns the iterator of the control messages.
    pub fn messages(&self) -> Messages<'_> {
        Messages {
            buffer: &self.buffer[..self.length],
            current: None,
        }
    }
}

fn add_to_ancillary_data<T>(
    buffer: &mut [u8],
    length: &mut usize,
    source: &[T],
    cmsg_level: libc::c_int,
    cmsg_type: libc::c_int,
) -> bool {
    #[cfg(not(target_os = "freebsd"))]
    let cmsg_size = source.len().checked_mul(size_of::<T>());
    #[cfg(target_os = "freebsd")]
    let cmsg_size = Some(unsafe { libc::SOCKCRED2SIZE(1) });

    let source_len = if let Some(source_len) = cmsg_size {
        if let Ok(source_len) = u32::try_from(source_len) {
            source_len
        } else {
            return false;
        }
    } else {
        return false;
    };

    unsafe {
        let additional_space = libc::CMSG_SPACE(source_len) as usize;

        let new_length = if let Some(new_length) = additional_space.checked_add(*length) {
            new_length
        } else {
            return false;
        };

        if new_length > buffer.len() {
            return false;
        }

        buffer[*length..new_length].fill(0);

        *length = new_length;

        let mut msg: libc::msghdr = zeroed();
        msg.msg_control = buffer.as_mut_ptr().cast();
        msg.msg_controllen = *length as _;

        let mut cmsg = libc::CMSG_FIRSTHDR(&msg);
        let mut previous_cmsg = cmsg;
        while !cmsg.is_null() {
            previous_cmsg = cmsg;
            cmsg = libc::CMSG_NXTHDR(&msg, cmsg);

            // Most operating systems, but not Linux or emscripten, return the previous pointer
            // when its length is zero. Therefore, check if the previous pointer is the same as
            // the current one.
            if eq(cmsg, previous_cmsg) {
                break;
            }
        }

        if previous_cmsg.is_null() {
            return false;
        }

        (*previous_cmsg).cmsg_level = cmsg_level;
        (*previous_cmsg).cmsg_type = cmsg_type;
        (*previous_cmsg).cmsg_len = libc::CMSG_LEN(source_len) as _;

        let data = libc::CMSG_DATA(previous_cmsg).cast();

        libc::memcpy(data, source.as_ptr().cast(), source_len as usize);
    }
    true
}

/// This struct is used to iterate through the control messages.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Messages<'a> {
    buffer: &'a [u8],
    current: Option<&'a libc::cmsghdr>,
}

impl<'a> Iterator for Messages<'a> {
    type Item = Result<AncillaryData<'a>, AncillaryError>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut msg: libc::msghdr = zeroed();
            msg.msg_control = self.buffer.as_ptr() as *mut _;
            msg.msg_controllen = self.buffer.len() as _;

            let cmsg = if let Some(current) = self.current {
                libc::CMSG_NXTHDR(&msg, current)
            } else {
                libc::CMSG_FIRSTHDR(&msg)
            };

            let cmsg = cmsg.as_ref()?;

            // Most operating systems, but not Linux or emscripten, return the previous pointer
            // when its length is zero. Therefore, check if the previous pointer is the same as
            // the current one.
            if let Some(current) = self.current {
                if eq(current, cmsg) {
                    return None;
                }
            }

            self.current = Some(cmsg);
            let ancillary_result = AncillaryData::try_from_cmsghdr(cmsg);
            Some(ancillary_result)
        }
    }
}

/// The error type which is returned from parsing the type a control message.
#[non_exhaustive]
#[derive(Debug)]
pub enum AncillaryError {
    Unknown { cmsg_level: i32, cmsg_type: i32 },
}

/// This control message contains file descriptors.
///
/// The level is equal to `SOL_SOCKET` and the type is equal to `SCM_RIGHTS`.
pub struct ScmRights<'a>(AncillaryDataIter<'a, RawFd>);

impl<'a> Iterator for ScmRights<'a> {
    type Item = RawFd;

    fn next(&mut self) -> Option<RawFd> {
        self.0.next()
    }
}

/// This control message contains unix credentials.
///
/// The level is equal to `SOL_SOCKET` and the type is equal to `SCM_CREDENTIALS` or `SCM_CREDS`.
#[cfg(any(target_os = "android", target_os = "linux",))]
pub struct ScmCredentials<'a>(AncillaryDataIter<'a, libc::ucred>);

/// This enum represent one control message of variable type.
pub enum AncillaryData<'a> {
    ScmRights(ScmRights<'a>),
    #[cfg(any(
        doc,
        target_os = "android",
        target_os = "linux",
        target_os = "netbsd",
        target_os = "freebsd"
    ))]
    ScmCredentials(ScmCredentials<'a>),
}

impl<'a> AncillaryData<'a> {
    /// Create an `AncillaryData::ScmRights` variant.
    ///
    /// # Safety
    ///
    /// `data` must contain a valid control message and the control message must be type of
    /// `SOL_SOCKET` and level of `SCM_RIGHTS`.
    unsafe fn as_rights(data: &'a [u8]) -> Self {
        let ancillary_data_iter = AncillaryDataIter::new(data);
        let scm_rights = ScmRights(ancillary_data_iter);
        AncillaryData::ScmRights(scm_rights)
    }

    /// Create an `AncillaryData::ScmCredentials` variant.
    ///
    /// # Safety
    ///
    /// `data` must contain a valid control message and the control message must be type of
    /// `SOL_SOCKET` and level of `SCM_CREDENTIALS` or `SCM_CREDS`.
    #[cfg(any(
        doc,
        target_os = "android",
        target_os = "linux",
        target_os = "netbsd",
        target_os = "freebsd"
    ))]
    unsafe fn as_credentials(data: &'a [u8]) -> Self {
        let ancillary_data_iter = AncillaryDataIter::new(data);
        let scm_credentials = ScmCredentials(ancillary_data_iter);
        AncillaryData::ScmCredentials(scm_credentials)
    }

    fn try_from_cmsghdr(cmsg: &'a libc::cmsghdr) -> Result<Self, AncillaryError> {
        unsafe {
            let cmsg_len_zero = libc::CMSG_LEN(0) as usize;
            let data_len = (*cmsg).cmsg_len as usize - cmsg_len_zero;
            let data = libc::CMSG_DATA(cmsg).cast();
            let data = from_raw_parts(data, data_len);

            match (*cmsg).cmsg_level {
                libc::SOL_SOCKET => match (*cmsg).cmsg_type {
                    libc::SCM_RIGHTS => Ok(AncillaryData::as_rights(data)),
                    #[cfg(any(target_os = "android", target_os = "linux",))]
                    libc::SCM_CREDENTIALS => Ok(AncillaryData::as_credentials(data)),
                    #[cfg(target_os = "freebsd")]
                    libc::SCM_CREDS2 => Ok(AncillaryData::as_credentials(data)),
                    #[cfg(target_os = "netbsd")]
                    libc::SCM_CREDS => Ok(AncillaryData::as_credentials(data)),
                    cmsg_type => Err(AncillaryError::Unknown {
                        cmsg_level: libc::SOL_SOCKET,
                        cmsg_type,
                    }),
                },
                cmsg_level => Err(AncillaryError::Unknown {
                    cmsg_level,
                    cmsg_type: (*cmsg).cmsg_type,
                }),
            }
        }
    }
}

struct AncillaryDataIter<'a, T> {
    data: &'a [u8],
    phantom: PhantomData<T>,
}

impl<'a, T> AncillaryDataIter<'a, T> {
    /// Create `AncillaryDataIter` struct to iterate through the data unit in the control message.
    ///
    /// # Safety
    ///
    /// `data` must contain a valid control message.
    unsafe fn new(data: &'a [u8]) -> AncillaryDataIter<'a, T> {
        AncillaryDataIter {
            data,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Iterator for AncillaryDataIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if size_of::<T>() <= self.data.len() {
            unsafe {
                let unit = read_unaligned(self.data.as_ptr().cast());
                self.data = &self.data[size_of::<T>()..];
                Some(unit)
            }
        } else {
            None
        }
    }
}
