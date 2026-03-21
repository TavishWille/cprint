# cprint

This library crate gives you a simple
macro around println! and print! for terminal colors
and select modifiers.


 #### print! with colors
 Behaves almost identically to the standard `print!` macro.

 In fact it can act as a drop in replacement.

 # Arguments
 a `cprint!` call requires a prepend of a color, other optional params, then `;`
 to the fmt and arguments passed to `print!`

 ## Colors
 A full list of colors can be seen by looking at the Color enum, or by using
 the `cprint_palette!` macro. Additionally, `_` can be used as a placeholder
 for terminal default color.

 ## Backgrounds
 `cprint!` supports the same color list being used as text background, this can be
 done by following a `color` keyword with the `on` `<color>` `;`

 ## Modifiers
 Text can be further modified after a color with an optional parameter
     - `bold`
     - `italic`
     - `underline`
     - `x` | `strikethrough` | `dashed`
     - `invert` | `reverse`
     - `hidden`

 # Examples

 ```rust
use cprint::cprint;
// print!("hello world");  ==
// cprint!("hello world"); ==
cprint!(_; "hello world");
cprint!(red; "red text");
cprint!(light red; "light red text");
cprint!(green; bold; "bold green text");
cprint!(dark blue; italic; "italic dark blue text");
cprint!(_; invert; "inverted color text");
cprint!(_; italic; "italic default text");
cprint!(_; x; "strikethrough text with args: 1=={} and 2=={}", 1, 2);
cprint!(red on black; "red text on black background");
cprint!(light red on black; "light red text on black background");
cprint!(bright red on dark gray; "bright red text on dark gray background");
cprint!(bright red on dark gray; invert; "dark gray text on bright red background");
```