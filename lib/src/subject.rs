// Copyright © 2025 Matei Pralea <matei@machtentfaltung.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::account::{Student, Teacher};
use crate::grade::Grade;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject {
    name: String,
    teacher: Teacher,
    grades: HashMap<Student, Vec<Grade>>,
}

