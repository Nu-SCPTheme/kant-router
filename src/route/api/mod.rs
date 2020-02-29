/*
 * route/api/mod.rs
 *
 * thaumiel - Wikidot-like web server to provide pages, forums, and other services
 * Copyright (C) 2019-2020 Ammon Smith
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

mod prelude {
    pub use super::super::prelude::*;
    pub use super::types::*;
    pub use crate::remote::*;
    pub use actix_identity::Identity;
    pub use deepwell_core::prelude::*;
    pub use deepwell_rpc::Api as _;
    pub use ftml_rpc::Api as _;
}

#[macro_use]
mod macros;

mod auth;
mod misc;
mod page;
mod types;
mod user;

pub use self::auth::*;
pub use self::misc::*;
pub use self::page::*;
pub use self::user::*;
