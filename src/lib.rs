#![recursion_limit="256"]

#[macro_export]
macro_rules! delegate {

    // entry point

    { $($rest:tt)* } => {
        delegate__parse! {
            state: top_level,
            buffer: { $($rest)* },
            stack: {
                items: [],
                action: expand
            }
        }
    };

}

#[macro_export]
#[doc(hidden)]
macro_rules! delegate__parse {

    // state: top_level

    {
        state: top_level,
        buffer: {},
        stack: {
            items: [ $($items:tt)* ],
            action: expand
        }
    } => {
        $($items)*
    };

    {
        state: top_level,
        buffer: {},
        stack: {
            items: [ $($items:tt)* ],
            action: stringify
        }
    } => {
        stringify!($($items)*)
    };

    {
        state: top_level,
        buffer: $buffer:tt,
        stack: $stack:tt
    } => {
        delegate__parse! {
            state: parse_target,
            buffer: $buffer,
            stack: $stack
        }
    };

    // state: parse target

    {
        state: parse_target,
        buffer: { target self . $field:ident { $($methods:tt)* } $($rest:tt)* },
        stack: {
            items: $items:tt,
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_methods,
            buffer: { $($methods)* },
            stack: {
                target: $field,
                items: $items,
                rest: { $($rest)* },
                $($stack)*
            }
        }
    };

    // state: parse_methods

    {
        state: parse_methods,
        buffer: {},
        stack: {
            target: $target:tt,
            items: $items:tt,
            rest: { $($rest:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: top_level,
            buffer: { $($rest)* },
            stack: {
                items: $items,
                $($stack)*
            }
        }
    };

    {
        state: parse_methods,
        buffer: $buffer:tt,
        stack: {
            target: $target:tt,
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_attributes,
            buffer: $buffer,
            stack: {
                signature: { #[inline] },
                body: { $target },
                target: $target,
                $($stack)*
            }
        }
    };

    // state: parse_method_attributes

    {
        state: parse_method_attributes,
        buffer: { #[inline $($inline:tt)*] $($rest:tt)* },
        stack: {
            signature: { #[inline] $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_attributes,
            buffer: { $($rest)* },
            stack: {
                signature: { $($signature)* #[inline $($inline)*] },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_attributes,
        buffer: { #[$($attribute:tt)*] $($rest:tt)* },
        stack: {
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_attributes,
            buffer: { $($rest)* },
            stack: {
                signature: { $($signature)* #[$($attribute)*] },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_attributes,
        buffer: { #![$($attribute:tt)*] $($rest:tt)* },
        stack: {
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_attributes,
            buffer: { $($rest)* },
            stack: {
                signature: { $($signature)* #![$($attribute)*] },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_attributes,
        buffer: $buffer:tt,
        stack: $stack:tt
    } => {
        delegate__parse! {
            state: parse_method_visibility,
            buffer: $buffer,
            stack: $stack
        }
    };

    // state: parse_method_visibility

    {
        state: parse_method_visibility,
        buffer: { pub fn $($rest:tt)* },
        stack: {
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_name,
            buffer: { $($rest)* },
            stack: {
                signature: { $($signature)* pub fn },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_visibility,
        buffer: { pub $pub_mod:tt fn $($rest:tt)* },
        stack: {
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_name,
            buffer: { $($rest)* },
            stack: {
                signature: { $($signature)* pub $pub_mod fn },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_visibility,
        buffer: { fn $($rest:tt)* },
        stack: {
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_name,
            buffer: { $($rest)* },
            stack: {
                signature: { $($signature)* fn },
                $($stack)*
            }
        }
    };

    // state: parse_method_name

    {
        state: parse_method_name,
        buffer: { $name:ident < $($rest:tt)* },
        stack: {
            signature: { $($signature:tt)* },
            body: { $($body:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_generics,
            buffer: { $($rest)* },
            stack: {
                depth: {},
                signature: { $($signature)* $name < },
                body: { $($body)* . $name },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_name,
        buffer: { $name:ident $($rest:tt)* },
        stack: {
            signature: { $($signature:tt)* },
            body: { $($body:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_args,
            buffer: { $($rest)* },
            stack: {
                signature: { $($signature)* $name },
                body: { $($body)* . $name },
                $($stack)*
            }
        }
    };

    // state: parse_method_generics

    {
        state: parse_method_generics,
        buffer: { < $($rest:tt)* },
        stack: {
            depth: { $($depth:tt)* },
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_generics,
            buffer: { $($rest)* },
            stack: {
                depth: { { $($depth)* } },
                signature: { $($signature)* < },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_generics,
        buffer: { > $($rest:tt)* },
        stack: {
            depth: { { $($depth:tt)* } },
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_generics,
            buffer: { $($rest)* },
            stack: {
                depth: { $($depth)* },
                signature: { $($signature)* > },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_generics,
        buffer: { > $($rest:tt)* },
        stack: {
            depth: {},
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_args,
            buffer: { $($rest)* },
            stack: {
                signature: { $($signature)* > },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_generics,
        buffer: { >> $($rest:tt)* },
        stack: {
            depth: { { { $($depth:tt)* } } },
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_generics,
            buffer: { $($rest)* },
            stack: {
                depth: { $($depth)* },
                signature: { $($signature)* > > },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_generics,
        buffer: { >> $($rest:tt)* },
        stack: {
            depth: { {} },
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_args,
            buffer: { $($rest)* },
            stack: {
                signature: { $($signature)* > > },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_generics,
        buffer: { $next:tt $($rest:tt)* },
        stack: {
            depth: $depth:tt,
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_generics,
            buffer: { $($rest)* },
            stack: {
                depth: $depth,
                signature: { $($signature)* $next },
                $($stack)*
            }
        }
    };

    // state: parse_method_args

    {
        state: parse_method_args,
        buffer: { ( $($args:tt)* ) $($rest:tt)* },
        stack: {
            signature: $signature:tt,
            body: $body:tt,
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_args_self,
            buffer: { $($args)* },
            stack: {
                signature_args: {},
                invoke_args: {},
                signature: $signature,
                body: $body,
                rest: { $($rest)* },
                $($stack)*
            }
        }
    };

    // state: parse_method_args_self

    {
        state: parse_method_args_self,
        buffer: { & mut $self:tt $($rest:tt)* },
        stack: {
            signature_args: {},
            invoke_args: {},
            signature: $signature:tt,
            body: { $($body:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__ensure_self!($self);

        delegate__parse! {
            state: parse_method_args_consume_possible_comma,
            buffer: { $($rest)* },
            stack: {
                signature_args: { & mut $self },
                invoke_args: {},
                signature: $signature,
                body: { $self . $($body)* },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_args_self,
        buffer: { & $self:tt $($rest:tt)* },
        stack: {
            signature_args: {},
            invoke_args: {},
            signature: $signature:tt,
            body: { $($body:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__ensure_self!($self);

        delegate__parse! {
            state: parse_method_args_consume_possible_comma,
            buffer: { $($rest)* },
            stack: {
                signature_args: { & $self },
                invoke_args: {},
                signature: $signature,
                body: { $self . $($body)* },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_args_self,
        buffer: { $self:tt $($rest:tt)* },
        stack: {
            signature_args: {},
            invoke_args: {},
            signature: $signature:tt,
            body: { $($body:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__ensure_self!($self);

        delegate__parse! {
            state: parse_method_args_consume_possible_comma,
            buffer: { $($rest)* },
            stack: {
                signature_args: { $self },
                invoke_args: {},
                signature: $signature,
                body: { $self . $($body)* },
                $($stack)*
            }
        }
    };

    // state: parse_method_args_consume_possible_comma

    {
        state: parse_method_args_consume_possible_comma,
        buffer: { , $($rest:tt)+ },
        stack: {
            signature_args: { $($signature_args:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_args_rest,
            buffer: { $($rest)* },
            stack: {
                signature_args: { $($signature_args)* , },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_args_consume_possible_comma,
        buffer: {},
        stack: $stack:tt
    } => {
        delegate__parse! {
            state: parse_method_args_rest,
            buffer: {},
            stack: $stack
        }
    };

    // state: parse_method_args_rest

    {
        state: parse_method_args_rest,
        buffer: { $name:ident : $type:ty , $($rest:tt)+ },
        stack: {
            signature_args: { $($signature_args:tt)* },
            invoke_args: { $($invoke_args:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_args_rest,
            buffer: { $($rest)* },
            stack: {
                signature_args: { $($signature_args)* $name : $type , },
                invoke_args: { $($invoke_args)* $name , },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_args_rest,
        buffer: { $name:ident : $type:ty },
        stack: {
            signature_args: { $($signature_args:tt)* },
            invoke_args: { $($invoke_args:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_args_rest,
            buffer: {},
            stack: {
                signature_args: { $($signature_args)* $name : $type },
                invoke_args: { $($invoke_args)* $name },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_args_rest,
        buffer: {},
        stack: {
            signature_args: { $($signature_args:tt)* },
            invoke_args: { $($invoke_args:tt)* },
            signature: { $($signature:tt)* },
            body: { $($body:tt)* },
            rest: { $($rest:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_return,
            buffer: { $($rest)* },
            stack: {
                signature: { $($signature)* ( $($signature_args)* ) },
                body: { $($body)* ( $($invoke_args)* ) },
                $($stack)*
            }
        }
    };

    // state: parse_method_return

    {
        state: parse_method_return,
        buffer: { -> $ret:ty ; $($rest:tt)* },
        stack: {
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_end,
            buffer: { $($rest)* },
            stack: {
                signature: { $($signature)* -> $ret },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_return,
        buffer: { ; $($rest:tt)* },
        stack: $stack:tt
    } => {
        delegate__parse! {
            state: parse_method_end,
            buffer: { $($rest)* },
            stack: $stack
        }
    };

    // state: parse_method_end

    {
        state: parse_method_end,
        buffer: $buffer:tt,
        stack: {
            signature: { $($signature:tt)* },
            body: { $($body:tt)* },
            target: $target:tt,
            items: [ $($items:tt)* ],
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_methods,
            buffer: $buffer,
            stack: {
                target: $target,
                items: [
                    $($items)*
                    $($signature)* { $($body)* }
                ],
                $($stack)*
            }
        }
    };

    // Catch all

    {
        state: $state:tt,
        buffer: { $token:tt $($rest:tt)* },
        stack: $stack:tt
    } => {
        delegate__parse_fail!($token);
        compile_error!(concat!(
            "ParseError! ",
            "Unexpected token `", stringify!($token), "` in `", stringify!($state), "`. ",
            "This is possibly a bug in the delegate library itself. ",
            "Please file an issue with the following debug information:\n\n",
            stringify!($stack),
            "\n\n"
        ));
    };

    {
        $($state:tt)*
    } => {
        compile_error!(concat!(
            "ParseError! ",
            "Unexpected parser state. ",
            "This is most likely a bug in the delegate library itself. ",
            "Please file an issue with the following debug information:\n\n",
            stringify!($($state)*),
            "\n\n"
        ));
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! delegate__parse_fail {
    {} => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! delegate__ensure_self {
    { self } => {};

    { $token:tt } => {
        compile_error!(concat!(
            "ParseError! ",
            "Unexpected token `", stringify!($token), "` in `parse_method_args_self`. ",
            "The first argument in a method must be one of `self`, `&self` or `&mut self`."
        ));
    }
}

#[cfg(test)]
mod tests {
    macro_rules! stringify_tokens {
        {
            {}
            { $($tokens:tt)* }
        } => {
            stringify! { $($tokens)* }
        };

        {
            { ( self $(, $name:ident : $type:ty)* ) $($rest:tt)* }
            { $($tokens:tt)* }
        } => {
            stringify_tokens! {
                { $($rest)* }
                { $($tokens)* ( self $(, $name : $type)* ) }
            }
        };

        {
            { ( & self $(, $name:ident : $type:ty)* ) $($rest:tt)* }
            { $($tokens:tt)* }
        } => {
            stringify_tokens! {
                { $($rest)* }
                { $($tokens)* ( & self $(, $name : $type)* ) }
            }
        };

        {
            { ( & mut self $(, $name:ident : $type:ty)* ) $($rest:tt)* }
            { $($tokens:tt)* }
        } => {
            stringify_tokens! {
                { $($rest)* }
                { $($tokens)* ( & mut self $(, $name : $type)* ) }
            }
        };

        {
            { -> $ret:ty { $($body:tt)* } $($rest:tt)* }
            { $($tokens:tt)* }
        } => {
            stringify_tokens! {
                { $($rest)* }
                { $($tokens)* -> $ret { $($body)* } }
            }
        };

        {
            { >> $($rest:tt)* }
            { $($tokens:tt)* }
        } => {
            stringify_tokens! {
                { $($rest)* }
                { $($tokens)* > > }
            }
        };

        {
            { $next:tt $($rest:tt)* }
            { $($tokens:tt)* }
        } => {
            stringify_tokens! {
                { $($rest)* }
                { $($tokens)* $next }
            }
        };
    }

    macro_rules! assert_delegation {
        { { $($actual:tt)* }, { $($expected:tt)* } } => {
            let actual = {
                delegate__parse! {
                    state: top_level,
                    buffer: { $($actual)* },
                    stack: {
                        items: [],
                        action: stringify
                    }
                }
            };

            let expected = stringify_tokens!( { $($expected)* } {} );

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_attributes() {
        assert_delegation! {
            {
                target self.inner {
                    #[foo]
                    fn test_simple_attribute(self);

                    #![foo]
                    fn test_simple_bang_attribute(self);

                    #[foo="bar"]
                    fn test_key_value_attribute(self);

                    #![foo="bar"]
                    fn test_key_value_bang_attribute(self);

                    #[foo(bar, baz="bat")]
                    fn test_complex_attribute(self);

                    #![foo(bar, baz="bat")]
                    fn test_complex_bang_attribute(self);

                    #[foo]
                    #![bar]
                    #[baz="wow"]
                    #![bat(omg="wat")]
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    fn testAllTheAttributes(self);
                }
            },

            {
                #[inline]
                #[foo]
                fn test_simple_attribute(self) {
                    self.inner.test_simple_attribute()
                }

                #[inline]
                #![foo]
                fn test_simple_bang_attribute(self) {
                    self.inner.test_simple_bang_attribute()
                }

                #[inline]
                #[foo="bar"]
                fn test_key_value_attribute(self) {
                    self.inner.test_key_value_attribute()
                }

                #[inline]
                #![foo="bar"]
                fn test_key_value_bang_attribute(self) {
                    self.inner.test_key_value_bang_attribute()
                }

                #[inline]
                #[foo(bar, baz="bat")]
                fn test_complex_attribute(self) {
                    self.inner.test_complex_attribute()
                }

                #[inline]
                #![foo(bar, baz="bat")]
                fn test_complex_bang_attribute(self) {
                    self.inner.test_complex_bang_attribute()
                }

                #[inline]
                #[foo]
                #![bar]
                #[baz="wow"]
                #![bat(omg="wat")]
                #[doc(hidden)]
                #[allow(non_snake_case)]
                fn testAllTheAttributes(self) {
                    self.inner.testAllTheAttributes()
                }
            }
        }
    }

    #[test]
    fn test_inline() {
        assert_delegation! {
            {
                target self.inner {
                    #[inline]
                    fn test_inline(self);

                    #[inline(always)]
                    fn test_inline_always(self);

                    #[inline(never)]
                    fn test_inline_never(self);

                    #[before]
                    #[inline]
                    #[after]
                    fn test_inline_with_attributes(self);

                    #[before]
                    #[inline(always)]
                    #[after]
                    fn test_inline_always_with_attributes(self);

                    #[before]
                    #[inline(never)]
                    #[after]
                    fn test_inline_never_with_attributes(self);
                }
            },

            {
                #[inline]
                fn test_inline(self) {
                    self.inner.test_inline()
                }

                #[inline(always)]
                fn test_inline_always(self) {
                    self.inner.test_inline_always()
                }

                #[inline(never)]
                fn test_inline_never(self) {
                    self.inner.test_inline_never()
                }

                #[before]
                #[inline]
                #[after]
                fn test_inline_with_attributes(self) {
                    self.inner.test_inline_with_attributes()
                }

                #[before]
                #[inline(always)]
                #[after]
                fn test_inline_always_with_attributes(self) {
                    self.inner.test_inline_always_with_attributes()
                }

                #[before]
                #[inline(never)]
                #[after]
                fn test_inline_never_with_attributes(self) {
                    self.inner.test_inline_never_with_attributes()
                }
            }
        }
    }

    #[test]
    fn test_pub() {
        assert_delegation! {
            {
                target self.inner {
                    pub fn test_pub(self);

                    pub(crate) fn test_pub_crate(self);

                    pub(super) fn test_pub_super(self);

                    pub(self) fn test_pub_self(self);

                    pub(in some::other::mod) fn test_pub_in(self);
                }
            },

            {
                #[inline]
                pub fn test_pub(self) {
                    self.inner.test_pub()
                }

                #[inline]
                pub(crate) fn test_pub_crate(self) {
                    self.inner.test_pub_crate()
                }

                #[inline]
                pub(super) fn test_pub_super(self) {
                    self.inner.test_pub_super()
                }

                #[inline]
                pub(self) fn test_pub_self(self) {
                    self.inner.test_pub_self()
                }

                #[inline]
                pub(in some::other::mod) fn test_pub_in(self) {
                    self.inner.test_pub_in()
                }
            }
        }
    }

    #[test]
    fn test_self() {
        assert_delegation! {
            {
                target self.inner {
                    fn test_self_no_args(self);

                    fn test_self_with_one_arg(self, first: i32);

                    fn test_self_with_multi_args(self, first: i32, second: &str, third: bool);
                }
            },

            {
                #[inline]
                fn test_self_no_args(self) {
                    self.inner.test_self_no_args()
                }

                #[inline]
                fn test_self_with_one_arg(self, first: i32) {
                    self.inner.test_self_with_one_arg(first)
                }

                #[inline]
                fn test_self_with_multi_args(self, first: i32, second: &str, third: bool) {
                    self.inner.test_self_with_multi_args(first, second, third)
                }
            }
        }
    }

    #[test]
    fn test_borrow_self() {
        assert_delegation! {
            {
                target self.inner {
                    fn test_borrow_self_no_args(&self);

                    fn test_borrow_self_with_one_arg(&self, first: i32);

                    fn test_borrow_self_with_multi_args(&self, first: i32, second: &str, third: bool);
                }
            },

            {
                #[inline]
                fn test_borrow_self_no_args(&self) {
                    self.inner.test_borrow_self_no_args()
                }

                #[inline]
                fn test_borrow_self_with_one_arg(&self, first: i32) {
                    self.inner.test_borrow_self_with_one_arg(first)
                }

                #[inline]
                fn test_borrow_self_with_multi_args(&self, first: i32, second: &str, third: bool) {
                    self.inner.test_borrow_self_with_multi_args(first, second, third)
                }
            }
        }
    }

    #[test]
    fn test_borrow_mut_self() {
        assert_delegation! {
            {
                target self.inner {
                    fn test_borrow_mut_self_no_args(&mut self);

                    fn test_borrow_mut_self_with_one_arg(&mut self, first: i32);

                    fn test_borrow_mut_self_with_multi_args(&mut self, first: i32, second: &str, third: bool);
                }
            },

            {
                #[inline]
                fn test_borrow_mut_self_no_args(&mut self) {
                    self.inner.test_borrow_mut_self_no_args()
                }

                #[inline]
                fn test_borrow_mut_self_with_one_arg(&mut self, first: i32) {
                    self.inner.test_borrow_mut_self_with_one_arg(first)
                }

                #[inline]
                fn test_borrow_mut_self_with_multi_args(&mut self, first: i32, second: &str, third: bool) {
                    self.inner.test_borrow_mut_self_with_multi_args(first, second, third)
                }
            }
        }
    }

    #[test]
    fn test_generics() {
        assert_delegation! {
            {
                target self.inner {
                    fn test_empty_generic<>(self);

                    fn test_single_generic<T>(self, first: T);

                    fn test_multiple_generics<T, U, V>(self, first: T, second: U, third: V);

                    fn test_single_generic_with_bound<T: Display>(self, first: T);

                    fn test_multiple_generics_with_bounds<T: Display, U, V: Debug>(self, first: T, second: U, third: V);

                    fn test_single_lifetime_generic<'a>(&self, first: &'a str);

                    fn test_multiple_lifetime_generics<'a, 'b, 'c, T, U, V>(&self, first: &'a T, second: &'b U, third: &'c V);

                    fn test_single_nested_generic<T: Some<Generic<Trait<()>>>>(self, first: T);

                    fn test_multiple_nested_generics<T, U: Some<Generic<Trait<T>>>, V: Some<Other<Trait<T>>>>(self, first: T, second: U, third: V);

                    fn test_complex_generics<T, U: ?Sized + Clone + Copy + for<'a> From<&'a U>>(self, first: T, second: U);
                }
            },

            {
                #[inline]
                fn test_empty_generic<>(self) {
                    self.inner.test_empty_generic()
                }

                #[inline]
                fn test_single_generic<T>(self, first: T) {
                    self.inner.test_single_generic(first)
                }

                #[inline]
                fn test_multiple_generics<T, U, V>(self, first: T, second: U, third: V) {
                    self.inner.test_multiple_generics(first, second, third)
                }

                #[inline]
                fn test_single_generic_with_bound<T: Display>(self, first: T) {
                    self.inner.test_single_generic_with_bound(first)
                }

                #[inline]
                fn test_multiple_generics_with_bounds<T: Display, U, V: Debug>(self, first: T, second: U, third: V) {
                    self.inner.test_multiple_generics_with_bounds(first, second, third)
                }

                #[inline]
                fn test_single_lifetime_generic<'a>(&self, first: &'a str) {
                    self.inner.test_single_lifetime_generic(first)
                }

                #[inline]
                fn test_multiple_lifetime_generics<'a, 'b, 'c, T, U, V>(&self, first: &'a T, second: &'b U, third: &'c V) {
                    self.inner.test_multiple_lifetime_generics(first, second, third)
                }

                #[inline]
                fn test_single_nested_generic<T: Some<Generic<Trait<()>>>>(self, first: T) {
                    self.inner.test_single_nested_generic(first)
                }

                #[inline]
                fn test_multiple_nested_generics<T, U: Some<Generic<Trait<T>>>, V: Some<Other<Trait<T>>>>(self, first: T, second: U, third: V) {
                    self.inner.test_multiple_nested_generics(first, second, third)
                }

                #[inline]
                fn test_complex_generics<T, U: ?Sized + Clone + Copy + for<'a> From<&'a U>>(self, first: T, second: U) {
                    self.inner.test_complex_generics(first, second)
                }
            }
        }
    }

    #[test]
    fn test_return() {
        assert_delegation! {
            {
                target self.inner {
                    fn test_implicit_return(self);

                    fn test_simple_return(self) -> ();

                    fn test_complex_return(&self) -> &Vec<i32>;

                    fn test_generic_return<T>(self) -> T;

                    fn test_generic_lifetime_return<'a, T>(&self, first: &'a Vec<T>) -> &'a T;
                }
            },

            {
                #[inline]
                fn test_implicit_return(self) {
                    self.inner.test_implicit_return()
                }

                #[inline]
                fn test_simple_return(self) -> () {
                    self.inner.test_simple_return()
                }

                #[inline]
                fn test_complex_return(&self) -> &Vec<i32> {
                    self.inner.test_complex_return()
                }

                #[inline]
                fn test_generic_return<T>(self) -> T {
                    self.inner.test_generic_return()
                }

                #[inline]
                fn test_generic_lifetime_return<'a, T>(&self, first: &'a Vec<T>) -> &'a T {
                    self.inner.test_generic_lifetime_return(first)
                }
            }
        }
    }

    #[test]
    fn test_multiple_targets() {
        assert_delegation! {
            {
                target self.first {
                    fn test_first_a(self);
                    fn test_first_b(self);
                }

                target self.second {
                    fn test_second_a(self);
                    fn test_second_b(self);
                }

                target self.third {
                    fn test_third_a(self);
                    fn test_third_b(self);
                }
            },

            {
                #[inline]
                fn test_first_a(self) {
                    self.first.test_first_a()
                }

                #[inline]
                fn test_first_b(self) {
                    self.first.test_first_b()
                }

                #[inline]
                fn test_second_a(self) {
                    self.second.test_second_a()
                }

                #[inline]
                fn test_second_b(self) {
                    self.second.test_second_b()
                }

                #[inline]
                fn test_third_a(self) {
                    self.third.test_third_a()
                }

                #[inline]
                fn test_third_b(self) {
                    self.third.test_third_b()
                }
            }
        }
    }
}
