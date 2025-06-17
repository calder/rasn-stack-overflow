#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
mod generated {
    #[allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        unused,
        clippy::too_many_arguments
    )]
    pub mod main {
        extern crate alloc;
        use core::borrow::Borrow;
        use lazy_static::lazy_static;
        use rasn::prelude::*;
        /// Inner type
        #[rasn(choice, tag(explicit(context, 123)))]
        pub enum FizzBuzz {
            #[rasn(tag(context, 456))]
            foo(()),
            #[rasn(tag(context, 789))]
            bar(()),
        }
        impl rasn::AsnType for FizzBuzz {
            const TAG: rasn::types::Tag = {
                rasn::types::Tag::new(rasn::types::Class::Context, 123)
            };
            const TAG_TREE: rasn::types::TagTree = const {
                let list: &'static [rasn::types::TagTree] = const {
                    &[
                        {
                            let variant_list: &'static [rasn::types::TagTree] = const {
                                &[
                                    rasn::types::TagTree::Leaf(
                                        rasn::types::Tag::new(rasn::types::Class::Context, 456),
                                    ),
                                    rasn::types::TagTree::Leaf(
                                        rasn::types::Tag::new(rasn::types::Class::Context, 789),
                                    ),
                                ]
                            };
                            let variant_tag_tree: rasn::types::TagTree = rasn::types::TagTree::Choice(
                                variant_list,
                            );
                            if !variant_tag_tree.is_unique() {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "FizzBuzz\'s variants is not unique, ensure that your variants\' tags are correct.",
                                        ),
                                    );
                                }
                            }
                            variant_tag_tree
                        },
                    ]
                };
                let tag_tree: rasn::types::TagTree = rasn::types::TagTree::Choice(list);
                if !tag_tree.is_unique() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "FizzBuzz\'s variants is not unique, ensure that your variants\' tags are correct.",
                            ),
                        );
                    }
                }
                rasn::types::TagTree::Leaf(Self::TAG)
            };
            const IDENTIFIER: rasn::types::Identifier = rasn::types::Identifier(
                Some("FizzBuzz"),
            );
        }
        impl rasn::types::Choice for FizzBuzz {
            const VARIANTS: &'static [rasn::types::TagTree] = &[
                rasn::types::TagTree::Leaf(
                    rasn::types::Tag::new(rasn::types::Class::Context, 456),
                ),
                rasn::types::TagTree::Leaf(
                    rasn::types::Tag::new(rasn::types::Class::Context, 789),
                ),
            ];
            const VARIANCE_CONSTRAINT: rasn::types::Constraints = rasn::types::Constraints::new(
                &[
                    rasn::types::Constraint::Value(
                        rasn::types::constraints::Extensible::new(
                                rasn::types::constraints::Value::new(
                                    rasn::types::constraints::Bounded::const_new(
                                        0i128 as i128,
                                        1i128 as i128,
                                    ),
                                ),
                            )
                            .set_extensible(false),
                    ),
                ],
            );
            const EXTENDED_VARIANTS: Option<&'static [rasn::types::TagTree]> = None;
            const IDENTIFIERS: &'static [&'static str] = &["foo", "bar"];
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for FizzBuzz {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    FizzBuzz::foo(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "foo",
                            &__self_0,
                        )
                    }
                    FizzBuzz::bar(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "bar",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for FizzBuzz {
            #[inline]
            fn clone(&self) -> FizzBuzz {
                match self {
                    FizzBuzz::foo(__self_0) => {
                        FizzBuzz::foo(::core::clone::Clone::clone(__self_0))
                    }
                    FizzBuzz::bar(__self_0) => {
                        FizzBuzz::bar(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl rasn::types::DecodeChoice for FizzBuzz {
            fn from_tag<D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::types::Tag,
            ) -> core::result::Result<Self, D::Error> {
                use rasn::de::Decode;
                if rasn::types::TagTree::tag_contains(
                    &tag,
                    &[
                        rasn::types::TagTree::Leaf(
                            rasn::types::Tag::new(rasn::types::Class::Context, 456),
                        ),
                    ],
                ) {
                    return <_>::decode_with_tag(decoder, tag).map(Self::foo);
                }
                if rasn::types::TagTree::tag_contains(
                    &tag,
                    &[
                        rasn::types::TagTree::Leaf(
                            rasn::types::Tag::new(rasn::types::Class::Context, 789),
                        ),
                    ],
                ) {
                    return <_>::decode_with_tag(decoder, tag).map(Self::bar);
                }
                Err(rasn::de::Error::no_valid_choice("FizzBuzz", decoder.codec()))
            }
        }
        #[automatically_derived]
        impl rasn::Decode for FizzBuzz {
            fn decode_with_tag_and_constraints<D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::types::Tag,
                constraints: rasn::types::Constraints,
            ) -> core::result::Result<Self, D::Error> {
                decoder.decode_explicit_prefix(tag)
            }
            fn decode<D: rasn::Decoder>(
                decoder: &mut D,
            ) -> core::result::Result<Self, D::Error> {
                #[rasn(choice)]
                enum InnerFizzBuzz {
                    #[rasn(tag(context, 456))]
                    foo(()),
                    #[rasn(tag(context, 789))]
                    bar(()),
                }
                impl rasn::AsnType for InnerFizzBuzz {
                    const TAG: rasn::types::Tag = { rasn::types::Tag::EOC };
                    const TAG_TREE: rasn::types::TagTree = const {
                        let list: &'static [rasn::types::TagTree] = const {
                            &[
                                {
                                    let variant_list: &'static [rasn::types::TagTree] = const {
                                        &[
                                            rasn::types::TagTree::Leaf(
                                                rasn::types::Tag::new(rasn::types::Class::Context, 456),
                                            ),
                                            rasn::types::TagTree::Leaf(
                                                rasn::types::Tag::new(rasn::types::Class::Context, 789),
                                            ),
                                        ]
                                    };
                                    let variant_tag_tree: rasn::types::TagTree = rasn::types::TagTree::Choice(
                                        variant_list,
                                    );
                                    if !variant_tag_tree.is_unique() {
                                        {
                                            ::core::panicking::panic_fmt(
                                                format_args!(
                                                    "InnerFizzBuzz\'s variants is not unique, ensure that your variants\' tags are correct.",
                                                ),
                                            );
                                        }
                                    }
                                    variant_tag_tree
                                },
                            ]
                        };
                        let tag_tree: rasn::types::TagTree = rasn::types::TagTree::Choice(
                            list,
                        );
                        if !tag_tree.is_unique() {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "InnerFizzBuzz\'s variants is not unique, ensure that your variants\' tags are correct.",
                                    ),
                                );
                            }
                        }
                        tag_tree
                    };
                    const IDENTIFIER: rasn::types::Identifier = rasn::types::Identifier(
                        Some("InnerFizzBuzz"),
                    );
                }
                impl rasn::types::Choice for InnerFizzBuzz {
                    const VARIANTS: &'static [rasn::types::TagTree] = &[
                        rasn::types::TagTree::Leaf(
                            rasn::types::Tag::new(rasn::types::Class::Context, 456),
                        ),
                        rasn::types::TagTree::Leaf(
                            rasn::types::Tag::new(rasn::types::Class::Context, 789),
                        ),
                    ];
                    const VARIANCE_CONSTRAINT: rasn::types::Constraints = rasn::types::Constraints::new(
                        &[
                            rasn::types::Constraint::Value(
                                rasn::types::constraints::Extensible::new(
                                        rasn::types::constraints::Value::new(
                                            rasn::types::constraints::Bounded::const_new(
                                                0i128 as i128,
                                                1i128 as i128,
                                            ),
                                        ),
                                    )
                                    .set_extensible(false),
                            ),
                        ],
                    );
                    const EXTENDED_VARIANTS: Option<&'static [rasn::types::TagTree]> = None;
                    const IDENTIFIERS: &'static [&'static str] = &["foo", "bar"];
                }
                #[automatically_derived]
                impl rasn::types::DecodeChoice for InnerFizzBuzz {
                    fn from_tag<D: rasn::Decoder>(
                        decoder: &mut D,
                        tag: rasn::types::Tag,
                    ) -> core::result::Result<Self, D::Error> {
                        use rasn::de::Decode;
                        if rasn::types::TagTree::tag_contains(
                            &tag,
                            &[
                                rasn::types::TagTree::Leaf(
                                    rasn::types::Tag::new(rasn::types::Class::Context, 456),
                                ),
                            ],
                        ) {
                            return <_>::decode_with_tag(decoder, tag).map(Self::foo);
                        }
                        if rasn::types::TagTree::tag_contains(
                            &tag,
                            &[
                                rasn::types::TagTree::Leaf(
                                    rasn::types::Tag::new(rasn::types::Class::Context, 789),
                                ),
                            ],
                        ) {
                            return <_>::decode_with_tag(decoder, tag).map(Self::bar);
                        }
                        Err(
                            rasn::de::Error::no_valid_choice(
                                "InnerFizzBuzz",
                                decoder.codec(),
                            ),
                        )
                    }
                }
                #[automatically_derived]
                impl rasn::Decode for InnerFizzBuzz {
                    fn decode_with_tag_and_constraints<D: rasn::Decoder>(
                        decoder: &mut D,
                        tag: rasn::types::Tag,
                        constraints: rasn::types::Constraints,
                    ) -> core::result::Result<Self, D::Error> {
                        decoder.decode_explicit_prefix(tag)
                    }
                    fn decode<D: rasn::Decoder>(
                        decoder: &mut D,
                    ) -> core::result::Result<Self, D::Error> {
                        decoder.decode_choice(Self::CONSTRAINTS)
                    }
                }
                let value = decoder
                    .decode_explicit_prefix::<
                        InnerFizzBuzz,
                    >(<Self as rasn::AsnType>::TAG)?;
                Ok(
                    match value {
                        InnerFizzBuzz::foo(i0) => FizzBuzz::foo(i0),
                        InnerFizzBuzz::bar(i0) => FizzBuzz::bar(i0),
                    },
                )
            }
        }
        #[automatically_derived]
        impl rasn::Encode for FizzBuzz {
            fn encode<'encoder, E: rasn::Encoder<'encoder>>(
                &self,
                encoder: &mut E,
            ) -> core::result::Result<(), E::Error> {
                #[rasn(choice)]
                enum InnerFizzBuzz<'inner> {
                    #[rasn(tag(context, 456))]
                    foo(&'inner ()),
                    #[rasn(tag(context, 789))]
                    bar(&'inner ()),
                }
                impl<'inner> rasn::AsnType for InnerFizzBuzz<'inner> {
                    const TAG: rasn::types::Tag = { rasn::types::Tag::EOC };
                    const TAG_TREE: rasn::types::TagTree = const {
                        let list: &'static [rasn::types::TagTree] = const {
                            &[
                                {
                                    let variant_list: &'static [rasn::types::TagTree] = const {
                                        &[
                                            rasn::types::TagTree::Leaf(
                                                rasn::types::Tag::new(rasn::types::Class::Context, 456),
                                            ),
                                            rasn::types::TagTree::Leaf(
                                                rasn::types::Tag::new(rasn::types::Class::Context, 789),
                                            ),
                                        ]
                                    };
                                    let variant_tag_tree: rasn::types::TagTree = rasn::types::TagTree::Choice(
                                        variant_list,
                                    );
                                    if !variant_tag_tree.is_unique() {
                                        {
                                            ::core::panicking::panic_fmt(
                                                format_args!(
                                                    "InnerFizzBuzz\'s variants is not unique, ensure that your variants\' tags are correct.",
                                                ),
                                            );
                                        }
                                    }
                                    variant_tag_tree
                                },
                            ]
                        };
                        let tag_tree: rasn::types::TagTree = rasn::types::TagTree::Choice(
                            list,
                        );
                        if !tag_tree.is_unique() {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "InnerFizzBuzz\'s variants is not unique, ensure that your variants\' tags are correct.",
                                    ),
                                );
                            }
                        }
                        tag_tree
                    };
                    const IDENTIFIER: rasn::types::Identifier = rasn::types::Identifier(
                        Some("InnerFizzBuzz"),
                    );
                }
                impl<'inner> rasn::types::Choice for InnerFizzBuzz<'inner> {
                    const VARIANTS: &'static [rasn::types::TagTree] = &[
                        rasn::types::TagTree::Leaf(
                            rasn::types::Tag::new(rasn::types::Class::Context, 456),
                        ),
                        rasn::types::TagTree::Leaf(
                            rasn::types::Tag::new(rasn::types::Class::Context, 789),
                        ),
                    ];
                    const VARIANCE_CONSTRAINT: rasn::types::Constraints = rasn::types::Constraints::new(
                        &[
                            rasn::types::Constraint::Value(
                                rasn::types::constraints::Extensible::new(
                                        rasn::types::constraints::Value::new(
                                            rasn::types::constraints::Bounded::const_new(
                                                0i128 as i128,
                                                1i128 as i128,
                                            ),
                                        ),
                                    )
                                    .set_extensible(false),
                            ),
                        ],
                    );
                    const EXTENDED_VARIANTS: Option<&'static [rasn::types::TagTree]> = None;
                    const IDENTIFIERS: &'static [&'static str] = &["foo", "bar"];
                }
                #[automatically_derived]
                impl<'inner> rasn::Encode for InnerFizzBuzz<'inner> {
                    fn encode<'encoder, E: rasn::Encoder<'encoder>>(
                        &self,
                        encoder: &mut E,
                    ) -> core::result::Result<(), E::Error> {
                        encoder
                            .encode_choice::<
                                Self,
                            >(
                                Self::CONSTRAINTS,
                                match self {
                                    InnerFizzBuzz::foo(_) => {
                                        rasn::types::Tag::new(rasn::types::Class::Context, 456)
                                    }
                                    InnerFizzBuzz::bar(_) => {
                                        rasn::types::Tag::new(rasn::types::Class::Context, 789)
                                    }
                                },
                                |encoder| match self {
                                    InnerFizzBuzz::foo(value) => {
                                        rasn::Encode::encode_with_tag_and_identifier(
                                                value,
                                                encoder,
                                                rasn::types::Tag::new(rasn::types::Class::Context, 456),
                                                rasn::types::Identifier(Some("foo")),
                                            )
                                            .map(|_| rasn::types::Tag::new(
                                                rasn::types::Class::Context,
                                                456,
                                            ))
                                    }
                                    InnerFizzBuzz::bar(value) => {
                                        rasn::Encode::encode_with_tag_and_identifier(
                                                value,
                                                encoder,
                                                rasn::types::Tag::new(rasn::types::Class::Context, 789),
                                                rasn::types::Identifier(Some("bar")),
                                            )
                                            .map(|_| rasn::types::Tag::new(
                                                rasn::types::Class::Context,
                                                789,
                                            ))
                                    }
                                },
                                Self::IDENTIFIER,
                            )
                            .map(drop)
                    }
                    fn encode_with_identifier<'encoder, E: rasn::Encoder<'encoder>>(
                        &self,
                        encoder: &mut E,
                        identifier: rasn::types::Identifier,
                    ) -> core::result::Result<(), E::Error> {
                        encoder
                            .encode_choice::<
                                Self,
                            >(
                                Self::CONSTRAINTS,
                                match self {
                                    InnerFizzBuzz::foo(_) => {
                                        rasn::types::Tag::new(rasn::types::Class::Context, 456)
                                    }
                                    InnerFizzBuzz::bar(_) => {
                                        rasn::types::Tag::new(rasn::types::Class::Context, 789)
                                    }
                                },
                                |encoder| match self {
                                    InnerFizzBuzz::foo(value) => {
                                        rasn::Encode::encode_with_tag_and_identifier(
                                                value,
                                                encoder,
                                                rasn::types::Tag::new(rasn::types::Class::Context, 456),
                                                rasn::types::Identifier(Some("foo")),
                                            )
                                            .map(|_| rasn::types::Tag::new(
                                                rasn::types::Class::Context,
                                                456,
                                            ))
                                    }
                                    InnerFizzBuzz::bar(value) => {
                                        rasn::Encode::encode_with_tag_and_identifier(
                                                value,
                                                encoder,
                                                rasn::types::Tag::new(rasn::types::Class::Context, 789),
                                                rasn::types::Identifier(Some("bar")),
                                            )
                                            .map(|_| rasn::types::Tag::new(
                                                rasn::types::Class::Context,
                                                789,
                                            ))
                                    }
                                },
                                identifier,
                            )
                            .map(drop)
                    }
                    fn encode_with_tag_and_constraints<
                        'encoder,
                        EN: rasn::Encoder<'encoder>,
                    >(
                        &self,
                        encoder: &mut EN,
                        tag: rasn::types::Tag,
                        constraints: rasn::types::Constraints,
                        identifier: rasn::types::Identifier,
                    ) -> core::result::Result<(), EN::Error> {
                        encoder.encode_explicit_prefix(tag, self, identifier).map(drop)
                    }
                }
                let value = match &self {
                    Self::foo { 0: ref i0 } => InnerFizzBuzz::foo { 0: i0 },
                    Self::bar { 0: ref i0 } => InnerFizzBuzz::bar { 0: i0 },
                };
                encoder
                    .encode_explicit_prefix::<
                        InnerFizzBuzz,
                    >(
                        <FizzBuzz as rasn::AsnType>::TAG,
                        &value,
                        rasn::types::Identifier(Some("FizzBuzz")),
                    )
                    .map(drop)
            }
            fn encode_with_identifier<'encoder, E: rasn::Encoder<'encoder>>(
                &self,
                encoder: &mut E,
                identifier: rasn::types::Identifier,
            ) -> core::result::Result<(), E::Error> {
                encoder
                    .encode_choice::<
                        Self,
                    >(
                        Self::CONSTRAINTS,
                        match self {
                            FizzBuzz::foo(_) => {
                                rasn::types::Tag::new(rasn::types::Class::Context, 456)
                            }
                            FizzBuzz::bar(_) => {
                                rasn::types::Tag::new(rasn::types::Class::Context, 789)
                            }
                        },
                        |encoder| match self {
                            FizzBuzz::foo(value) => {
                                rasn::Encode::encode_with_tag_and_identifier(
                                        value,
                                        encoder,
                                        rasn::types::Tag::new(rasn::types::Class::Context, 456),
                                        rasn::types::Identifier(Some("foo")),
                                    )
                                    .map(|_| rasn::types::Tag::new(
                                        rasn::types::Class::Context,
                                        456,
                                    ))
                            }
                            FizzBuzz::bar(value) => {
                                rasn::Encode::encode_with_tag_and_identifier(
                                        value,
                                        encoder,
                                        rasn::types::Tag::new(rasn::types::Class::Context, 789),
                                        rasn::types::Identifier(Some("bar")),
                                    )
                                    .map(|_| rasn::types::Tag::new(
                                        rasn::types::Class::Context,
                                        789,
                                    ))
                            }
                        },
                        identifier,
                    )
                    .map(drop)
            }
            fn encode_with_tag_and_constraints<'encoder, EN: rasn::Encoder<'encoder>>(
                &self,
                encoder: &mut EN,
                tag: rasn::types::Tag,
                constraints: rasn::types::Constraints,
                identifier: rasn::types::Identifier,
            ) -> core::result::Result<(), EN::Error> {
                encoder.encode_explicit_prefix(tag, self, identifier).map(drop)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for FizzBuzz {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for FizzBuzz {
            #[inline]
            fn eq(&self, other: &FizzBuzz) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (FizzBuzz::foo(__self_0), FizzBuzz::foo(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (FizzBuzz::bar(__self_0), FizzBuzz::bar(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for FizzBuzz {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<()>;
                let _: ::core::cmp::AssertParamIsEq<()>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for FizzBuzz {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state);
                match self {
                    FizzBuzz::foo(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                    FizzBuzz::bar(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                }
            }
        }
        pub struct Fizz {
            #[rasn(tag(explicit(context, 123)))]
            pub buzz: FizzBuzz,
        }
        #[automatically_derived]
        impl rasn::types::Constructed<1usize, 0usize> for Fizz {
            const FIELDS: rasn::types::fields::Fields<1usize> = rasn::types::fields::Fields::from_static([
                {
                    rasn::types::fields::Field::new_required(
                        0usize,
                        rasn::types::Tag::new(rasn::types::Class::Context, 123),
                        rasn::types::TagTree::Leaf(
                            rasn::types::Tag::new(rasn::types::Class::Context, 123),
                        ),
                        "buzz",
                    )
                },
            ]);
            const IS_EXTENSIBLE: bool = false;
            const EXTENDED_FIELDS: Option<rasn::types::fields::Fields<0usize>> = None;
        }
        #[automatically_derived]
        impl rasn::AsnType for Fizz {
            const TAG: rasn::types::Tag = { rasn::types::Tag::SEQUENCE };
            const IDENTIFIER: rasn::types::Identifier = rasn::types::Identifier(
                Some("Fizz"),
            );
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Fizz {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Fizz",
                    "buzz",
                    &&self.buzz,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Fizz {
            #[inline]
            fn clone(&self) -> Fizz {
                Fizz {
                    buzz: ::core::clone::Clone::clone(&self.buzz),
                }
            }
        }
        impl rasn::Decode for Fizz {
            fn decode_with_tag_and_constraints<D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::types::Tag,
                constraints: rasn::types::Constraints,
            ) -> core::result::Result<Self, D::Error> {
                decoder
                    .decode_sequence::<
                        1usize,
                        0usize,
                        _,
                        _,
                        _,
                    >(
                        tag,
                        None::<fn() -> Self>,
                        |decoder| {
                            Ok(Self {
                                buzz: {
                                    decoder
                                        .decode_explicit_prefix(
                                            rasn::types::Tag::new(rasn::types::Class::Context, 123),
                                        )
                                        .map_err(|error| rasn::de::Error::field_error(
                                            "Fizz.buzz",
                                            error.into(),
                                            decoder.codec(),
                                        ))?
                                },
                            })
                        },
                    )
            }
        }
        #[allow(clippy::mutable_key_type)]
        impl rasn::Encode for Fizz {
            fn encode_with_tag_and_constraints<'encoder, EN: rasn::Encoder<'encoder>>(
                &self,
                encoder: &mut EN,
                tag: rasn::types::Tag,
                constraints: rasn::types::Constraints,
                identifier: rasn::types::Identifier,
            ) -> core::result::Result<(), EN::Error> {
                #[allow(unused)]
                let __rasn_field_buzz = &self.buzz;
                encoder
                    .encode_sequence::<
                        1usize,
                        0usize,
                        Self,
                        _,
                    >(
                        tag,
                        |encoder| {
                            encoder
                                .encode_explicit_prefix(
                                    rasn::types::Tag::new(rasn::types::Class::Context, 123),
                                    &self.buzz,
                                    rasn::types::Identifier(Some("buzz")),
                                )?;
                            Ok(())
                        },
                        identifier,
                    )
                    .map(drop)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Fizz {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Fizz {
            #[inline]
            fn eq(&self, other: &Fizz) -> bool {
                self.buzz == other.buzz
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Fizz {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<FizzBuzz>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Fizz {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.buzz, state)
            }
        }
        impl Fizz {
            pub fn new(buzz: FizzBuzz) -> Self {
                Self { buzz }
            }
        }
    }
}
pub use generated::main::*;
