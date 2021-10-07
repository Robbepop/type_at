// Copyright 2021 Robin Freyler
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_std]

/// Reflects the N-th type of a tuple.
///
/// # Usage
///
/// ```
/// # use type_at::TypeAt;
/// let _: i8  = <(i8, i16, i32, i64) as TypeAt<0>>::Type::default();
/// let _: i16 = <(i8, i16, i32, i64) as TypeAt<1>>::Type::default();
/// let _: i32 = <(i8, i16, i32, i64) as TypeAt<2>>::Type::default();
/// let _: i64 = <(i8, i16, i32, i64) as TypeAt<3>>::Type::default();
/// ```
pub trait TypeAt<const N: usize> {
    /// The N-th type of the tuple.
    type Type;
}
type_at_derive::impl_type_at_for_tuples!();
