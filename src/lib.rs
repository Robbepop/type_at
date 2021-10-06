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
/// # use tuple_type::TypeAt;
/// let _: i8  = <(i8, i16, i32, i64) as TypeAt<0>>::Type::default();
/// let _: i16 = <(i8, i16, i32, i64) as TypeAt<1>>::Type::default();
/// let _: i32 = <(i8, i16, i32, i64) as TypeAt<2>>::Type::default();
/// let _: i64 = <(i8, i16, i32, i64) as TypeAt<3>>::Type::default();
/// ```
pub trait TypeAt<const N: usize> {
    /// The N-th tuple type.
    type Type;
}

macro_rules! impl_tuple_type {
    ( $( $t:ident ),* $(,)? ) => {
        impl_tuple_type! {
            @rec
            head = [],
            rest = [ $( $t ),* ],
        }
    };
    (
        @rec
        head = [ $first_head:ident $( , $head:ident )* $(,)? ],
        rest = [ $first_rest:ident $( , $rest:ident )* $(,)? ] $(,)?
    ) => {
        impl_tuple_type! {
            @rec
            head = [ $first_head $( ,$head )* , $first_rest ],
            rest = [ $( $rest ),* ],
        }
        impl_tuple_type! {
            @imp
            head = [ ],
            current = $first_head,
            rest = [ $first_rest $( , $head )* ],
            acc = 0,
        }
    };
    (
        @rec
        head = [ ],
        rest = [ $first_rest:ident $( , $rest:ident )* $(,)? ] $(,)?
    ) => {
        impl_tuple_type! {
            @rec
            head = [ $first_rest ],
            rest = [ $( $rest ),* ],
        }
        impl_tuple_type! {
            @imp
            head = [ ],
            current = $first_rest,
            rest = [ ],
            acc = 0,
        }
    };
    (
        @rec
        head = [ $( $head:ident ),* $(,)? ],
        rest = [ ] $(,)?
    ) => {};
    (
        @imp
        head = [ $( $head:ident ),* $(,)? ],
        current = $current:ident,
        rest = [ $rest_head:ident $( , $rest:ident )* $(,)? ],
        acc = $acc:expr $(,)?
    ) => {
        impl<
            $($head,)* $current, $rest_head $(, $rest)*
        > $crate::TypeAt<{$acc}> for (
            $($head,)* $current, $rest_head $(, $rest)* ,
        ) {
            type Type = $current;
        }

        impl_tuple_type! {
            @imp
            head = [ $( $head ,)* $current ],
            current = $rest_head,
            rest = [ $( $rest ),* ],
            acc = ($acc + 1),
        }
    };
    (
        @imp
        head = [ $( $head:ident ),* $(,)? ],
        current = $current:ident,
        rest = [ $(,)? ],
        acc = $acc:expr $(,)?
    ) => {
        impl<
            $($head,)* $current
        > $crate::TypeAt<{$acc}> for (
            $($head,)* $current ,
        ) {
            type Type = $current;
        }
    }
}
impl_tuple_type! {
    A, B, C, D, E, F,
    G, H, I, J, K, L,
}
