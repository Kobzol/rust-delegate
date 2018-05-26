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
                action: delegate__expand
            }
        }
    };

}

#[macro_export]
#[doc(hidden)]
macro_rules! delegate__expand {
    { $($tokens:tt)* } => { $($tokens)* };
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
            action: $action:tt
        }
    } => {
        $action ! { $($items)* }
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
        buffer: { target self . $field:tt { $($methods:tt)* } $($rest:tt)* },
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
                signature: { #[default_target_method] #[inline] },
                body: { $target },
                target: $target,
                $($stack)*
            }
        }
    };

    // state: parse_method_attributes

    {
        state: parse_method_attributes,
        buffer: { #[target_method($target_method:ident)] $($rest:tt)* },
        stack: {
            signature: { #[default_target_method] $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_attributes,
            buffer: { $($rest)* },
            stack: {
                signature: { #[target_method($target_method)] $($signature)* },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_attributes,
        buffer: { #[inline $($inline:tt)*] $($rest:tt)* },
        stack: {
            signature: { #[$($target_method:tt)*] #[inline] $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_attributes,
            buffer: { $($rest)* },
            stack: {
                signature: { #[$($target_method)*] $($signature)* #[inline $($inline)*] },
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
            state: parse_method_visibility_safety,
            buffer: $buffer,
            stack: $stack
        }
    };

    // state: parse_method_visibility_safety

    {
        state: parse_method_visibility_safety,
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

    {
        state: parse_method_visibility_safety,
        buffer: { pub ( $($pub_mod:tt)* ) $($rest:tt)* },
        stack: {
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_visibility_safety,
            buffer: { $($rest)* },
            stack: {
                signature: { $($signature)* pub ($($pub_mod)*) },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_visibility_safety,
        buffer: { pub $($rest:tt)* },
        stack: {
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_visibility_safety,
            buffer: { $($rest)* },
            stack: {
                signature: { $($signature)* pub },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_visibility_safety,
        buffer: { unsafe $($rest:tt)* },
        stack: {
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_visibility_safety,
            buffer: { $($rest)* },
            stack: {
                signature: { $($signature)* unsafe },
                $($stack)*
            }
        }
    };

    // state: parse_method_name

    {
        state: parse_method_name,
        buffer: { $name:ident $($rest:tt)* },
        stack: {
            signature: { #[default_target_method] $($signature:tt)* },
            body: { $($body:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_generics,
            buffer: { $($rest)* },
            stack: {
                depth: {},
                signature: { $($signature)* $name },
                body: { $($body)* . $name },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_name,
        buffer: { $name:ident $($rest:tt)* },
        stack: {
            signature: { #[target_method($target_method:ident)] $($signature:tt)* },
            body: { $($body:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_generics,
            buffer: { $($rest)* },
            stack: {
                depth: {},
                signature: { $($signature)* $name },
                body: { $($body)* . $target_method },
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
            depth: { { $($depth:tt)+ } },
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
            depth: { {} },
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
            buffer: { > $($rest)* },
            stack: {
                depth: { { $($depth)* } },
                signature: { $($signature)* > },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_generics,
        buffer: $buffer:tt,
        stack: {
            depth: {},
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_args,
            buffer: $buffer,
            stack: {
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
        buffer: { & $lifetime:tt mut $self:tt, $($rest:tt)+ },
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
            state: parse_method_args_rest,
            buffer: { $($rest)* },
            stack: {
                signature_args: { & $lifetime mut $self , },
                invoke_args: {},
                signature: $signature,
                body: { $self . $($body)* },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_args_self,
        buffer: { & $lifetime:tt mut $self:tt },
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
            state: parse_method_args_rest,
            buffer: {},
            stack: {
                signature_args: { & $lifetime mut $self },
                invoke_args: {},
                signature: $signature,
                body: { $self . $($body)* },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_args_self,
        buffer: { & mut $self:tt, $($rest:tt)+ },
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
            state: parse_method_args_rest,
            buffer: { $($rest)* },
            stack: {
                signature_args: { & mut $self , },
                invoke_args: {},
                signature: $signature,
                body: { $self . $($body)* },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_args_self,
        buffer: { & mut $self:tt },
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
            state: parse_method_args_rest,
            buffer: {},
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
        buffer: { & $lifetime:tt $self:tt, $($rest:tt)+ },
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
            state: parse_method_args_rest,
            buffer: { $($rest)* },
            stack: {
                signature_args: { & $lifetime $self , },
                invoke_args: {},
                signature: $signature,
                body: { $self . $($body)* },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_args_self,
        buffer: { & $lifetime:tt $self:tt },
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
            state: parse_method_args_rest,
            buffer: {},
            stack: {
                signature_args: { & $lifetime $self },
                invoke_args: {},
                signature: $signature,
                body: { $self . $($body)* },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_args_self,
        buffer: { & $self:tt, $($rest:tt)+ },
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
            state: parse_method_args_rest,
            buffer: { $($rest)* },
            stack: {
                signature_args: { & $self , },
                invoke_args: {},
                signature: $signature,
                body: { $self . $($body)* },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_args_self,
        buffer: { & $self:tt },
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
            state: parse_method_args_rest,
            buffer: {},
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
        buffer: { $self:tt, $($rest:tt)+ },
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
            state: parse_method_args_rest,
            buffer: { $($rest)* },
            stack: {
                signature_args: { $self , },
                invoke_args: {},
                signature: $signature,
                body: { $self . $($body)* },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_args_self,
        buffer: { $self:tt },
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
            state: parse_method_args_rest,
            buffer: {},
            stack: {
                signature_args: { $self },
                invoke_args: {},
                signature: $signature,
                body: { $self . $($body)* },
                $($stack)*
            }
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
            buffer: { ; $($rest)* },
            stack: {
                signature: { $($signature)* -> $ret },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_return,
        buffer: { -> $ret:ty where $($rest:tt)* },
        stack: {
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_where_clause,
            buffer: { where $($rest)* },
            stack: {
                signature: { $($signature)* -> $ret },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_return,
        buffer: $buffer:tt,
        stack: $stack:tt
    } => {
        delegate__parse! {
            state: parse_method_where_clause,
            buffer: $buffer,
            stack: $stack
        }
    };

    // state: parse_method_where_clause

    {
        state: parse_method_where_clause,
        buffer: { where $($rest:tt)* },
        stack: {
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_where_clause,
            buffer: { $($rest)* },
            stack: {
                where: { where },
                signature: { $($signature)* },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_where_clause,
        buffer: { ; $($rest:tt)* },
        stack: {
            where: { $($where:tt)* },
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_end,
            buffer: { ; $($rest)* },
            stack: {
                signature: { $($signature)* $($where)* },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_where_clause,
        buffer: { $token:tt $($rest:tt)* },
        stack: {
            where: { $($where:tt)* },
            signature: { $($signature:tt)* },
            $($stack:tt)*
        }
    } => {
        delegate__parse! {
            state: parse_method_where_clause,
            buffer: { $($rest)* },
            stack: {
                where: { $($where)* $token },
                signature: { $($signature)* },
                $($stack)*
            }
        }
    };

    {
        state: parse_method_where_clause,
        buffer: $buffer:tt,
        stack: $stack:tt
    } => {
        delegate__parse! {
            state: parse_method_end,
            buffer: $buffer,
            stack: $stack
        }
    };

    // state: parse_method_end

    {
        state: parse_method_end,
        buffer: { ; $($rest:tt)* },
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
            buffer: { $($rest)* },
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
            { ( & $lifetime:tt mut self $(, $name:ident : $type:ty)* ) $($rest:tt)* }
            { $($tokens:tt)* }
        } => {
            stringify_tokens! {
                { $($rest)* }
                { $($tokens)* ( & $lifetime mut self $(, $name : $type)* ) }
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
            { ( & $lifetime:tt self $(, $name:ident : $type:ty)* ) $($rest:tt)* }
            { $($tokens:tt)* }
        } => {
            stringify_tokens! {
                { $($rest)* }
                { $($tokens)* ( & $lifetime self $(, $name : $type)* ) }
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
    fn test_target_method() {
        assert_delegation! {
            {
                target self.inner {
                    #[target_method(something_else)]
                    fn test_target_method(self);

                    #[before]
                    #[target_method(something_else)]
                    #[after]
                    fn test_target_method_with_attributes(self);
                }
            },

            {
                #[inline]
                fn test_target_method(self) {
                    self.inner.something_else()
                }

                #[inline]
                #[before]
                #[after]
                fn test_target_method_with_attributes(self) {
                    self.inner.something_else()
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
    fn test_unsafe() {
        assert_delegation! {
            {
                target self.inner {
                    unsafe fn test_unsafe(self);
                    pub unsafe fn test_pub_unsafe(self);
                    pub(crate) fn test_pub_crate_unsafe(self);
                }
            },

            {
                #[inline]
                unsafe fn test_unsafe(self) {
                    self.inner.test_unsafe()
                }

                #[inline]
                pub unsafe fn test_pub_unsafe(self) {
                    self.inner.test_pub_unsafe()
                }

                #[inline]
                pub(crate) unsafe fn test_pub_crate_unsafe(self) {
                    self.inner.test_pub_crate_unsafe()
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

                    fn test_borrow_self_with_lifetime<'a>(&'a self);

                    fn test_borrow_self_with_lifetime_multi_args<'a, 'b, 'c, 'd>(&'a self, first: &'b str, second: &'c str, third: &'d str);
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

                #[inline]
                fn test_borrow_self_with_lifetime<'a>(&'a self) {
                    self.inner.test_borrow_self_with_lifetime()
                }

                #[inline]
                fn test_borrow_self_with_lifetime_multi_args<'a, 'b, 'c, 'd>(&'a self, first: &'b str, second: &'c str, third: &'d str) {
                    self.inner.test_borrow_self_with_lifetime_multi_args(first, second, third)
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

                    fn test_borrow_mut_self_with_multi_args(&mut self, first: i32, second: &mut str, third: bool);

                    fn test_borrow_mut_self_with_lifetime<'a>(&'a mut self);

                    fn test_borrow_mut_self_with_lifetime_multi_args<'a, 'b, 'c, 'd>(&'a mut self, first: &'b mut str, second: &'c mut str, third: &'d mut str);
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
                fn test_borrow_mut_self_with_multi_args(&mut self, first: i32, second: &mut str, third: bool) {
                    self.inner.test_borrow_mut_self_with_multi_args(first, second, third)
                }

                #[inline]
                fn test_borrow_mut_self_with_lifetime<'a>(&'a mut self) {
                    self.inner.test_borrow_mut_self_with_lifetime()
                }

                #[inline]
                fn test_borrow_mut_self_with_lifetime_multi_args<'a, 'b, 'c, 'd>(&'a mut self, first: &'b mut str, second: &'c mut str, third: &'d mut str) {
                    self.inner.test_borrow_mut_self_with_lifetime_multi_args(first, second, third)
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
    fn test_where_clause() {
        assert_delegation! {
            {
                target self.inner {
                    fn test_simple_where_clause<T>(self, first: T) where T: Display;

                    fn test_simple_where_clause_with_return<T>(self) -> T where T: Display;

                    fn test_complex_where_clause<T, U, V>(self, first: T, second: U, third: V)
                        where T: SomeTrait + AnotherTrait,
                              U: Yet<T> + AnotherTrait,
                              V: StillAnotherTrait<T, U> + ?Sized;

                    fn test_complex_where_clause_with_return<T, U, V>(self) -> V
                        where T: SomeTrait + AnotherTrait,
                              U: Yet<T> + AnotherTrait,
                              V: StillAnotherTrait<T, U> + ?Sized;
                }
            },

            {
                #[inline]
                fn test_simple_where_clause<T>(self, first: T) where T: Display {
                    self.inner.test_simple_where_clause(first)
                }

                #[inline]
                fn test_simple_where_clause_with_return<T>(self) -> T where T: Display {
                    self.inner.test_simple_where_clause_with_return()
                }

                #[inline]
                fn test_complex_where_clause<T, U, V>(self, first: T, second: U, third: V)
                        where T: SomeTrait + AnotherTrait,
                              U: Yet<T> + AnotherTrait,
                              V: StillAnotherTrait<T, U> + ?Sized {
                    self.inner.test_complex_where_clause(first, second, third)
                }

                #[inline]
                fn test_complex_where_clause_with_return<T, U, V>(self) -> V
                    where T: SomeTrait + AnotherTrait,
                          U: Yet<T> + AnotherTrait,
                          V: StillAnotherTrait<T, U> + ?Sized {
                    self.inner.test_complex_where_clause_with_return()
                }
            }
        }
    }

    #[test]
    fn test_comments() {
        assert_delegation! {
            {
                target self.inner {
                    // simple comment
                    fn test_simple_comment(self);

                    // multiple
                    // simple
                    // comments
                    fn test_multiple_simple_comments(self);

                    /// doc comment
                    fn test_doc_comment(self);

                    /// multiple
                    /// doc
                    /// comments
                    fn test_multiple_doc_comments(self);

                    //! bang comment
                    fn test_bang_comment(self);

                    //! multiple
                    //! bang
                    //! comments
                    fn test_multiple_bang_comments(self);
                }
            },

            {
                #[inline]
                // simple comment
                fn test_simple_comment(self) {
                    self.inner.test_simple_comment()
                }

                #[inline]
                // multiple
                // simple
                // comments
                fn test_multiple_simple_comments(self) {
                    self.inner.test_multiple_simple_comments()
                }

                #[inline]
                /// doc comment
                fn test_doc_comment(self) {
                    self.inner.test_doc_comment()
                }

                #[inline]
                /// multiple
                /// doc
                /// comments
                fn test_multiple_doc_comments(self) {
                    self.inner.test_multiple_doc_comments()
                }

                #[inline]
                //! bang comment
                fn test_bang_comment(self) {
                    self.inner.test_bang_comment()
                }

                #[inline]
                //! multiple
                //! bang
                //! comments
                fn test_multiple_bang_comments(self) {
                    self.inner.test_multiple_bang_comments()
                }
            }
        }
    }

    #[test]
    fn test_multiple_targets() {
        assert_delegation! {
            {
                target self.0 {
                    fn test_zero(self);
                }

                target self.999 {
                    fn test_nine_nine_nine(self);
                }

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
                fn test_zero(self) {
                    self.0.test_zero()
                }

                #[inline]
                fn test_nine_nine_nine(self) {
                    self.999.test_nine_nine_nine()
                }

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
