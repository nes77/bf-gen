
#[macro_export]
macro_rules! bf {
    ($v:vis $func_name:ident => $($btok:tt)+) => {
        $v fn $func_name(ctx: &mut ::bf::Context) -> Result<(), ::bf::Error> {

            $($crate::bf_tok_expand!(ctx, $btok);)+

            Ok(())
        }
    };
}

#[macro_export]
macro_rules! bf_tok_expand {
    ($ctx:ident, +) => {
        $ctx.inc()?;
    };

    ($ctx:ident, ->) => {
        $ctx.dec()?;
        $ctx.next();
    };

    ($ctx:ident, <-) => {
        $ctx.prev();
        $ctx.dec()?;
    };

    ($ctx:ident, -) => {
        $ctx.dec()?;
    };

    ($ctx:ident, >) => {
        $ctx.next();
    };

    ($ctx:ident, <) => {
        $ctx.prev();
    };

    ($ctx:ident, ,) => {
        $ctx.inp()?;
    };

    ($ctx:ident, .) => {
        $ctx.out()?;
    };

    ($ctx:ident, ..) => {
        $ctx.out()?;
        $ctx.out()?;
    };

    ($ctx:ident, <<) => {
        $ctx.prev();
        $ctx.prev();
    };

    ($ctx:ident, >>) => {
        $ctx.next();
        $ctx.next();
    };

    ($ctx:ident, [$($body:tt)*]) => {
        while $ctx.cur()? != 0 {
            $($crate::bf_tok_expand!($ctx, $body);)*
        };
    };

    ($ctx:ident, $body:tt) => {
    };
}



#[cfg(test)]
mod tests {
    use bf::Context;

    #[test]
     pub fn test_parse() {
         bf! {
            tester => [->+<]>.
         }

        let mut ctx = Context::with_state(vec![10, 20]);
        tester(&mut ctx).unwrap();
        assert_eq!(ctx.data()[1], 30)

     }
}