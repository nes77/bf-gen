use std::io::{stdin, Read};
use std::process::abort;
#[macro_export]
macro_rules! bf {
    ($v:vis $func_name:ident => $($btok:tt)+) => {
        $v fn $func_name(ctx: &mut ::bf::panicking::StaticContext8) {

            $($crate::bf_tok_expand!(ctx, $btok);)+
        }
    };
}

#[macro_export]
macro_rules! bf_tok_expand {
    ($ctx:ident, +) => {
        $ctx.adj_val(1);
    };

    ($ctx:ident, ->) => {
        $ctx.adj_val(-1);
        $ctx.adj_pos(1);
    };

    ($ctx:ident, <-) => {
        $ctx.adj_pos(-1);
        $ctx.adj_val(-1);
    };

    ($ctx:ident, -) => {
        $ctx.adj_val(-1);
    };

    ($ctx:ident, >) => {
        $ctx.adj_pos(1);
    };

    ($ctx:ident, <) => {
        $ctx.adj_pos(-1);
    };

    ($ctx:ident, ,) => {
        $ctx.inp();
    };

    ($ctx:ident, .) => {
        $ctx.out();
    };

    ($ctx:ident, ..) => {
        $ctx.out();
        $ctx.out();
    };

    ($ctx:ident, [-]) => {
        $ctx.clear();
    };

    ($ctx:ident, <<) => {
        $ctx.adj_pos(-2);
    };

    ($ctx:ident, >>) => {
        $ctx.adj_pos(2);
    };

    ($ctx:ident, [$($body:tt)*]) => {
        while $ctx.cur() != 0 {
            $($crate::bf_tok_expand!($ctx, $body);)*
        };
    };

    ($ctx:ident, $body:tt) => {
    };
}


pub fn write_char(i: i8) {
    print!("{}", i as u8 as char);
}

pub fn read_char() -> i8 {
    let mut buf = [0u8];
    stdin().read_exact(&mut buf).unwrap();
    buf[0] as i8
}

#[macro_export]
macro_rules! bf_simple {
    ($v:vis $func_name:ident => $($btok:tt)+) => {
        $v fn $func_name() {
            use std::time::Instant;
            let start_time = Instant::now();
            let mut ctx = [0i8; (64 * 1024)];
            let mut pos = 0usize;
            $($crate::bf_tok_expand_simp!(ctx, pos, $btok);)+

            let end_time = Instant::now();
            println!("Execution took {}μs.", end_time.duration_since(start_time).as_micros());
        }
    };
}

#[macro_export]
macro_rules! bf_tok_expand_simp {
    ($ctx:ident, $pos:ident, +) => {
        $ctx[$pos] = $ctx[$pos].wrapping_add(1);
    };

    ($ctx:ident, $pos:ident, ->) => {
        $crate::bf_tok_expand_simp!($ctx, $pos, -);
        $crate::bf_tok_expand_simp!($ctx, $pos, >);
    };

    ($ctx:ident, $pos:ident, <-) => {
        $crate::bf_tok_expand_simp!($ctx, $pos, <);
        $crate::bf_tok_expand_simp!($ctx, $pos, -);
    };

    ($ctx:ident, $pos:ident, -) => {
        $ctx[$pos] = $ctx[$pos].wrapping_sub(1);
    };

    ($ctx:ident, $pos:ident, >) => {
        $pos = $pos.wrapping_add(1);
    };

    ($ctx:ident, $pos:ident, <) => {
        $pos = $pos.wrapping_sub(1);
    };

    ($ctx:ident, $pos:ident, ,) => {
        $ctx[$pos] = $crate::read_char();
    };

    ($ctx:ident, $pos:ident, .) => {
        $crate::write_char($ctx[$pos]);
    };

    ($ctx:ident, $pos:ident, ..) => {
        $crate::write_char($ctx[$pos]);
        $crate::write_char($ctx[$pos]);
    };

    ($ctx:ident, $pos:ident, [-]) => {
        $ctx[$pos] = 0;
    };

    ($ctx:ident, $pos:ident, <<) => {
        $pos = $pos.wrapping_sub(2);
    };

    ($ctx:ident, $pos:ident, >>) => {
        $pos = $pos.wrapping_add(2);
    };

    ($ctx:ident, $pos:ident, [$($body:tt)*]) => {
        while $ctx[$pos] != 0 {
            $($crate::bf_tok_expand_simp!($ctx, $pos, $body);)*
        };
    };

    ($ctx:ident, $pos:ident, $body:tt) => {
    };
}

#[cfg(test)]
mod tests {
    use bf::panicking::StaticContext8;

    #[test]
     pub fn test_parse() {
         bf! {
            tester => [->+<]>.
         }

        let mut ctx = StaticContext8::with_state(vec![10, 20]);
        tester(&mut ctx);
        assert_eq!(ctx.data()[1], 30);

     }
}