//! Certificate types which are passed to `CertificateCheck` in
//! `RemoteCallbacks`.

use std::marker;
use std::mem;
use std::slice;

use raw;
use util::Binding;

/// A certificate for a remote connection, viewable as one of `CertHostkey` or
/// `CertX509` currently.
pub struct Cert<'a> {
    raw: *mut raw::git_cert,
    marker: marker::ContravariantLifetime<'a>,
}

/// Hostkey information taken from libssh2
pub struct CertHostkey<'a> {
    raw: *mut raw::git_cert_hostkey,
    marker: marker::ContravariantLifetime<'a>,
}

/// X.509 certificate information
pub struct CertX509<'a> {
    raw: *mut raw::git_cert_x509,
    marker: marker::ContravariantLifetime<'a>,
}

impl<'a> Cert<'a> {
    /// Attempt to view this certificate as an SSH hostkey.
    ///
    /// Returns `None` if this is not actually an SSH hostkey.
    pub fn as_hostkey(&self) -> Option<&CertHostkey<'a>> {
        self.cast(raw::GIT_CERT_HOSTKEY_LIBSSH2)
    }

    /// Attempt to view this certificate as an X.509 certificate.
    ///
    /// Returns `None` if this is not actually an X.509 certificate.
    pub fn as_x509(&self) -> Option<&CertX509<'a>> {
        self.cast(raw::GIT_CERT_X509)
    }

    fn cast<T>(&self, kind: raw::git_cert_t) -> Option<&T> {
        assert_eq!(mem::size_of::<Cert<'a>>(), mem::size_of::<T>());
        unsafe {
            if kind == (*self.raw).cert_type {
                Some(&*(self as *const Cert<'a> as *const T))
            } else {
                None
            }
        }
    }
}

impl<'a> CertHostkey<'a> {
    /// Returns the md5 hash of the hostkey, if available.
    pub fn hash_md5(&self) -> Option<&[u8; 16]> {
        unsafe {
            if (*self.raw).kind as u32 & raw::GIT_CERT_SSH_MD5 as u32 == 0 {
                None
            } else {
                Some(&(*self.raw).hash_md5)
            }
        }
    }

    /// Returns the SHA-1 hash of the hostkey, if available.
    pub fn hash_sha1(&self) -> Option<&[u8; 20]> {
        unsafe {
            if (*self.raw).kind as u32 & raw::GIT_CERT_SSH_SHA1 as u32 == 0 {
                None
            } else {
                Some(&(*self.raw).hash_sha1)
            }
        }
    }
}

impl<'a> CertX509<'a> {
    /// Return the X.509 certificate data as a byte slice
    pub fn data(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts((*self.raw).data as *const u8,
                                  (*self.raw).len as usize)
        }
    }
}

impl<'a> Binding for Cert<'a> {
    type Raw = *mut raw::git_cert;
    fn from_raw(raw: *mut raw::git_cert) -> Cert<'a> {
        Cert { raw: raw, marker: marker::ContravariantLifetime }
    }
    fn raw(&self) -> *mut raw::git_cert { self.raw }
}
