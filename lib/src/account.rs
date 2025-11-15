// Copyright © 2025 Matei Pralea <matei@machtentfaltung.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AccountKind {
    Teacher,
    Student,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct AccountId(u64);

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct AccountDetails {
    id: AccountId,
    name: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Student {
    details: AccountDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Teacher {
    details: AccountDetails,
}

