// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/rusoto/rusoto/master/assets/logo-square.png"
)]
//! <p>AWS X-Ray provides APIs for managing debug traces and retrieving service maps and other data created by processing those traces.</p>
//!
//! If you're using the service, you're probably looking for [XRayClient](struct.XRayClient.html) and [XRay](trait.XRay.html).

extern crate bytes;
extern crate futures;
extern crate rusoto_core;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod custom;
mod generated;

pub use crate::custom::*;
pub use crate::generated::*;
