/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};

use super::auth::runners::Password;
use crate::errors::*;
use crate::AppData;

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.account.delete",
    wrap = "crate::api::v1::get_middleware()"
)]
pub async fn delete_account(
    id: Identity,
    payload: web::Json<Password>,
    data: AppData,
) -> ServiceResult<impl Responder> {
    use argon2_creds::Config;

    let username = id.identity().unwrap();

    let hash = data
        .dblib
        .get_password(&db_core::Login::Username(&username))
        .await?;

    if Config::verify(&hash.hash, &payload.password)? {
        runners::delete_user(&username, &data).await?;
        id.forget();
        Ok(HttpResponse::Ok())
    } else {
        Err(ServiceError::WrongPassword)
    }
}

pub mod runners {

    use super::*;

    pub async fn delete_user(name: &str, data: &AppData) -> ServiceResult<()> {
        data.dblib.delete_user(name).await?;
        Ok(())
    }
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(delete_account);
}
