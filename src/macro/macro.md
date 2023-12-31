
# Macros

## Fragment Types
```ignore
+----------------+-----------------------------------------------------+-----------------------------+
|fragement type  |matches                                              | can be followed by          |
+----------------+-----------------------------------------------------+-----------------------------+
|expr            |an expression: 2 + 2, "udo", x.len()                 |  => , ;                     |
+----------------+-----------------------------------------------------+-----------------------------+
|stmt            |An expression or declaration, no trailing semicolon: | => , ;                      |
|                |(hard to use, try expr or block instead)             |                             |
+----------------+---------------------------------------- ------------+-----------------------------+
|ty              |A type: String, Vec<u8>, (&str, bool)                | => , ; = | { [ : > as where |
+----------------+---------------------------------------- ------------+-----------------------------+
|path            |A path: ferns, std::sync::mpsc                       | => , ; = | { [ : > as where |
+----------------+---------------------------------------- ------------+-----------------------------+
|pat             |A pattern: _, Some(ref x)                            | => , = | if in              |
+----------------+-----------------------------------------------------+-----------------------------+
|item            | An item:                                            | Anything                    |
|                | struct Point {x: f64, y: f64}, mod ferns            |                             |
+----------------+-----------------------------------------------------+-----------------------------+
|block           | A block: { s += "ok\n"; true; }                     | Anything                    |
+----------------+-----------------------------------------------------+-----------------------------+
|meta            | The body of an attribute:                           | Anything                    |
|                |   inline, derive(Copy, Clone), doc="3D model"       |                             |
+----------------+-----------------------------------------------------+-----------------------------+
|ident           | An identifier: std, Json, var_name                  | Anything                    |
+----------------+-----------------------------------------------------+-----------------------------+
|tt              | A token tree: ; >=, {}, [0 1 (+ 0 1)]               | Anything                    |
+----------------+-----------------------------------------------------+-----------------------------+
```

## Patterns
```ignore
+--------------+-----------------------------------+
| `$( ... )*`  | 匹配0次到多次，不包含分隔符       |
+------------- +-----------------------------------+
| `$( ... ),*` | 用来匹配一列参数，以逗号来分隔    |
+--------------+-----------------------------------+
| `$( ... );*` | 匹配0次到多次，用分号作为分隔     |
+--------------+-----------------------------------+
| `$( ... )+   | 匹配1次到多次，没有分隔符         |
+--------------+-----------------------------------+
| `$( ... ),+` | 匹配1次到多次，用逗号作为分隔     |
+--------------+-----------------------------------+
| `$( ... );+` | 匹配1次到多次，用分号作为分隔     |
+--------------+-----------------------------------+
```

## Misc
可以在函数内部定义宏并调用之:

```rust
fn write_color(
    &mut self,
    fg: bool,
    c: &Color,
    intense: bool,
) -> io::Result<()> {
    macro_rules! write_intense {
        ($clr:expr) => {
            if fg {
                self.write_str(concat!("\x1B[38;5;", $clr, "m"))
            } else {
                self.write_str(concat!("\x1B[48;5;", $clr, "m"))
            }
        };
    }
    macro_rules! write_normal {
        ($clr:expr) => {
            if fg {
                self.write_str(concat!("\x1B[3", $clr, "m"))
            } else {
                self.write_str(concat!("\x1B[4", $clr, "m"))
            }
        };
    }
    if intense {
        match *c {
            Color::Black => write_intense!("8"),
            Color::Blue => write_intense!("12"),
            Color::Green => write_intense!("10"),
            Color::Red => write_intense!("9"),
            Color::Cyan => write_intense!("14"),
            Color::Magenta => write_intense!("13"),
            Color::Yellow => write_intense!("11"),
            Color::White => write_intense!("15"),
            Color::__Nonexhaustive => unreachable!(),
        }
    }
}
```

## libcore

* panic
* assert_eq
* assert_ne
* debug_assert
* debug_assert_eq
* debug_assert_ne
* try
* write
* write_nl
* unreachable
* unimiplemented
* todo

* compile_error
* format_args
* format_args_nl
* env
* option_env
* concat
* line
* file
* column
* stringify
* include_str
* include
* include_bytes
* module_path
* cfg
* assert
* test
* global_allocator

* Copy
* Debug
* Default
* Eq
* Hash
* Ord
* PartialEq
* PartialOrd
* RustcDecodable
* RustcEncodable

* asm
* global_asm
* log_syntax
* trace_macros
* bench
* test_case


## References
* [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/README.html)
