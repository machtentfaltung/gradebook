// Copyright © 2025 Matei Pralea <matei@machtentfaltung.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use serde::{Deserialize, Serialize};
use crate::account::Student;
use crate::subject::Subject;

#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    name: String,
    students: Vec<Student>,
    subjects: Vec<Subject>,
}

