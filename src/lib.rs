/*
 * Copyright (C) 2026 Tavish Wille <tavish.wille42@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

/// List of available colors to be used with `cprint!` and `cprintln!`
/// Only here for book-keeping.
///
///  In macro, use all lowercase and spaces for colors
/// e.g. BrightRed -> `$crate::cprintln!(bright red; "red");`
pub enum Colors {
    Black,
    White,
    Red,
    Green,
    Blue,
    DarkRed,
    DarkGreen,
    DarkBlue,
    LightRed,
    LightGreen,
    LightBlue,
    BrightRed,
    BrightGreen,
    BrightBlue,
    DarkYellow,
    Yellow,
    LightYellow,
    BrightYellow,
    DarkMagenta,
    Magenta,
    LightMagenta,
    BrightMagenta,
    DarkCyan,
    Cyan,
    LightCyan,
    BrightCyan,
    DarkPink,
    Pink,
    LightPink,
    DarkOrange,
    Orange,
    LightOrange,
    DarkGray,
    Gray,
    LightGray,
    Lime,
    Peach,
    Sky,
    Lavender,
    Teal,
    Maroon,
    Purple,
    Brown,
    LightBrown,
}

/// Print all of the available colors and names
/// can be called with arguments `bold`, `udnerline`, or `dim`
#[macro_export]
macro_rules! cprint_palette {
    () => {
        $crate::cprint_palette!(@all)
    };

    ($tt:tt) => {
        $crate::cprint_palette!(@all, $tt)
    };

    (@all $(, $tt:tt)? $(,)?) => {
        {
        $crate::cprint_palette!($($tt)?;white);
        $crate::cprint_palette!($($tt)?;black);
        println!("");
        $crate::cprint_palette!($($tt)?;dark red);
        $crate::cprint_palette!($($tt)?;red);
        $crate::cprint_palette!($($tt)?;light red);
        $crate::cprint_palette!($($tt)?;bright red);
        println!("");
        $crate::cprint_palette!($($tt)?;dark green);
        $crate::cprint_palette!($($tt)?;green);
        $crate::cprint_palette!($($tt)?;light green);
        $crate::cprint_palette!($($tt)?;bright green);
        println!("");
        $crate::cprint_palette!($($tt)?;dark blue);
        $crate::cprint_palette!($($tt)?;blue);
        $crate::cprint_palette!($($tt)?;light blue);
        $crate::cprint_palette!($($tt)?;bright blue);
        println!("");
        println!("");
        println!("");
        $crate::cprint_palette!($($tt)?;dark yellow);
        $crate::cprint_palette!($($tt)?;yellow);
        $crate::cprint_palette!($($tt)?;light yellow);
        $crate::cprint_palette!($($tt)?;bright yellow);
        println!("");
        $crate::cprint_palette!($($tt)?;dark magenta);
        $crate::cprint_palette!($($tt)?;magenta);
        $crate::cprint_palette!($($tt)?;light magenta);
        $crate::cprint_palette!($($tt)?;bright magenta);
        println!("");
        $crate::cprint_palette!($($tt)?;dark cyan);
        $crate::cprint_palette!($($tt)?;cyan);
        $crate::cprint_palette!($($tt)?;light cyan);
        $crate::cprint_palette!($($tt)?;bright cyan);
        println!("");
        println!("");
        println!("");
        $crate::cprint_palette!($($tt)?;dark gray);
        $crate::cprint_palette!($($tt)?;gray);
        $crate::cprint_palette!($($tt)?;light gray);
        println!("");
        $crate::cprint_palette!($($tt)?;dark pink);
        $crate::cprint_palette!($($tt)?;pink);
        $crate::cprint_palette!($($tt)?;light pink);
        println!("");
        $crate::cprint_palette!($($tt)?;dark orange);
        $crate::cprint_palette!($($tt)?;orange);
        $crate::cprint_palette!($($tt)?;light orange);
        println!("");
        $crate::cprint_palette!($($tt)?;lime);
        $crate::cprint_palette!($($tt)?;peach);
        $crate::cprint_palette!($($tt)?;sky);
        $crate::cprint_palette!($($tt)?;lavender);
        $crate::cprint_palette!($($tt)?;teal);
        $crate::cprint_palette!($($tt)?;maroon);
        $crate::cprint_palette!($($tt)?;purple);
        $crate::cprint_palette!($($tt)?;brown);
        $crate::cprint_palette!($($tt)?;light brown);
        }

    };


    ($($cmod:ident)?; $($tt:tt)*) => {
        $(
            print!("{}", $crate::_cmod!($cmod));
        )?
        println!("\x1b[38;5;{}m{}\x1b[0m", $crate::_color!($($tt)*), stringify!($($tt)*));
    };
}

pub mod internal {
    #[doc(hidden)]
    #[macro_export]
    macro_rules! _cmod {
        (bold) => {
            "\x1b[1m"
        };
        (dim) => {
            "\x1b[2m"
        };
        (italic) => {
            "\x1b[3m"
        };
        (underline) => {
            "\x1b[4m"
        };
        (reverse) => {
            $crate::_cmod!(invert)
        };
        (invert) => {
            "\x1b[7m"
        };
        (hidden) => {
            "\x1b[8m"
        };
        (strikethrough) => {
            "\x1b[9m"
        };
        (x) => {
            $crate::_cmod!(strikethrough)
        };
        (dashed) => {
            $crate::_cmod!(strikethrough)
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! _color {
        // ===== Ctl =====
    (reset) => { "\x1b[0m" };

        // ===== Base Colors =====
    (black) =>          {  232 };
    (white) =>          { 255 };

    (pure red) =>       { $crate::_color!(@code, 5, 0, 0) };
    (pure green) =>     { $crate::_color!(@code, 0, 5, 0) };
    (pure blue) =>      { $crate::_color!(@code, 0, 0, 5) };

    (red) =>            { $crate::_color!(@code, 5, 1, 1) };
    (green) =>          { $crate::_color!(@code, 2, 4, 1) };
    (blue) =>           { $crate::_color!(@code, 2, 3, 5) };

    (light red) =>      { $crate::_color!(@code, 5, 3, 3) };
    (light green) =>    { $crate::_color!(@code, 3, 5, 4) };
    (light blue) =>     { $crate::_color!(@code, 3, 4, 5) };

    (dark red) =>       { $crate::_color!(@code, 4, 0, 0) };
    (dark green) =>     { $crate::_color!(@code, 0, 3, 0) };
    (dark blue) =>      { $crate::_color!(@code, 0, 1, 4) };

    (bright red) =>     { $crate::_color!(pure red)      };
    (bright green) =>   { $crate::_color!(pure green)    };
    (bright blue) =>    { $crate::_color!(pure blue)     };


        // ===== Secondary Colors =====
    (yellow) =>         { $crate::_color!(@code, 4, 4, 0) };
    (dark yellow) =>    { $crate::_color!(@code, 2, 2, 0) };
    (light yellow) =>   { $crate::_color!(@code, 4, 4, 2) };
    (bright yellow) =>  { $crate::_color!(@code, 5, 5, 0) };

    (magenta) =>         { $crate::_color!(@code, 4, 1, 4) };
    (dark magenta) =>    { $crate::_color!(@code, 3, 0, 3) };
    (light magenta) =>   { $crate::_color!(@code, 4, 2, 4) };
    (bright magenta) =>  { $crate::_color!(@code, 5, 0, 5) };

    (cyan) =>            { $crate::_color!(@code, 0, 4, 4) };
    (dark cyan) =>       { $crate::_color!(@code, 0, 2, 2) };
    (light cyan) =>      { $crate::_color!(@code, 2, 4, 4) };
    (bright cyan) =>     { $crate::_color!(@code, 0, 5, 5) };

        // ===== Alternative Colors =====
    (dark pink) =>       { $crate::_color!(@code, 3, 1, 2) };
    (pink) =>            { $crate::_color!(@code, 5, 2, 4) };
    (light pink) =>      { $crate::_color!(@code, 5, 4, 5) };

    (dark orange) =>     { $crate::_color!(@code, 4, 2, 0) };
    (orange) =>          { $crate::_color!(@code, 5, 1, 0) };
    (light orange) =>    { $crate::_color!(@code, 5, 3, 1) };

    (gray) =>            { 246 };
    (light gray) =>      { 252 };
    (dark gray) =>       { 242 };

    (grey) =>            { gray };
    (light grey) =>      { light gray };
    (dark grey) =>       { dark gray };

    (lime) =>            { $crate::_color!(@code, 3, 5, 0) };
    (peach) =>           { $crate::_color!(@code, 5, 3, 1) };
    (sky) =>             { $crate::_color!(@code, 2, 4, 5) };
    (lavender) =>        { $crate::_color!(@code, 3, 3, 5) };
    (teal) =>            { $crate::_color!(@code, 2, 5, 4) };
    (maroon) =>          { $crate::_color!(@code, 5, 2, 1) };
    (purple) =>          { $crate::_color!(@code, 4, 0, 4) };
    (brown) =>           { $crate::_color!(@code, 2, 1, 0) };
    (light brown) =>     { $crate::_color!(@code, 3, 2, 0) };


    (@code, $r:literal, $g:literal, $b:literal) => {
        (16 + 36*$r + 6*$g + $b)
    };
}
}

/// #### println! with colors
/// Behaves almost identically to the standard `println!` macro.
///
/// In fact it can act as a drop in replacement.
///
/// # Arguments
/// a `cprintln!` call requires a prepend of a color, other optional params, then `;`
/// to the fmt and arguments passed to `println!`
///
/// ## Colors
/// A full list of colors can be seen by looking at the Color enum, or by using
/// the `cprint_palette!` macro. Additionally, `_` can be used as a placeholder
/// for terminal default color.
///
/// ## Backgrounds
/// `cprintln!` supports the same color list being used as text background, this can be
/// done by following a `color` keyword with the `on` `<color>` `;`
///
/// ## Modifiers
/// Text can be further modified after a color with an optional parameter
///     - `bold`
///     - `italic`
///     - `underline`
///     - `x` | `strikethrough` | `dashed`
///     - `invert` | `reverse`
///     - `hidden`
///
/// # Examples
///
/// ```
/// use cprint::cprintln;
///
/// // println!("hello world");  ==
/// // cprintln!("hello world"); ==
/// cprintln!(_; "hello world");
///
/// cprintln!(red; "red text");
/// cprintln!(light red; "light red text");
/// cprintln!(green; bold; "bold green text");
/// cprintln!(dark blue; italic; "italic dark blue text");
/// cprintln!(_; invert; "inverted color text");
/// cprintln!(_; italic; "italic default text");
/// cprintln!(_; x; "strikethrough text with args: 1=={} and 2=={}", 1, 2);
/// cprintln!(red on black; "red text on black background");
/// cprintln!(light red on black; "light red text on black background");
/// cprintln!(bright red on dark gray; "bright red text on dark gray background");
/// cprintln!(bright red on dark gray; invert; "dark gray text on bright red background");
/// ```
#[macro_export]
macro_rules! cprintln {
    (_ on $alt2:ident $color2:ident; $cmod:ident; $($tt:tt)*) => {
        {
            print!("{}\x1b[48;5;{}m", cprint::$crate::_cmod!($cmod), $crate::_color!($alt2 $color2));
            print!($($tt)*);
            println!("{}", $crate::_color!(reset));
        }
    };

    (_ on $color2:ident; $cmod:ident; $($tt:tt)*) => {
        {
            print!("{}\x1b[48;5;{}m", $crate::_cmod!($cmod), $crate::_color!($color2));
            print!($($tt)*);
            println!("{}", $crate::_color!(reset));
        }
    };

    (_; $cmod:ident; $($tt:tt)*) => {
        {
            print!("{}", $crate::_cmod!($cmod));
            print!($($tt)*);
            println!("{}", $crate::_color!(reset));
        }
    };

    (_ on $alt2:ident $color2:ident; $($tt:tt)*) => {
        {
            print!("{}\x1b[48;5;{}m", $crate::_color!($alt2 $color2));
            print!($($tt)*);
            println!("{}", $crate::_color!(reset));
        }
    };

    (_ on $color2:ident; $($tt:tt)*) => {
        {
            print!("\x1b[48;5;{}m", $crate::_color!($color2));
            print!($($tt)*);
            println!("{}", $crate::_color!(reset));
        }
    };

    (_; $($tt:tt)*) => {
        {
            println!($($tt)*);
        }
    };


    ($alt:ident $color:ident on $alt2:ident $color2:ident; $cmod:ident; $($tt:tt)*) => {
        {
            print!("{}\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($alt2 $color2), $crate::_color!($alt $color));
            print!($($tt)*);
            println!("{}", $crate::_color!(reset));
        }
    };

    ($color:ident on $alt2:ident $color2:ident; $cmod:ident; $($tt:tt)*) => {
        {
            print!("{}\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($alt2 $color2), $crate::_color!($color));
            print!($($tt)*);
            println!("{}", $crate::_color!(reset));
        }
    };

    ($alt:ident $color:ident on $color2:ident; $cmod:ident; $($tt:tt)*) => {
        {
            print!("{}\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($color2), $crate::_color!($alt $color));
            print!($($tt)*);
            println!("{}", $crate::_color!(reset));
        }
    };

    ($color:ident on $color2:ident; $cmod:ident; $($tt:tt)*) => {
        {
            print!("{}\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($color2), $crate::_color!($color));
            print!($($tt)*);
            println!("{}", $crate::_color!(reset));
        }
    };

    ($alt:ident $color:ident on $alt2:ident $color2:ident; $($tt:tt)*) => {
        {
            print!("\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_color!($alt2 $color2), $crate::_color!($alt $color));
            print!($($tt)*);
            println!("{}", $crate::_color!(reset));
        }
    };

    ($color:ident on $alt2:ident $color2:ident; $($tt:tt)*) => {
        {
            print!("\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_color!($alt2 $color2), $crate::_color!($color));
            print!($($tt)*);
            println!("{}", $crate::_color!(reset));
        }
    };

    ($alt:ident $color:ident on $color2:ident; $($tt:tt)*) => {
        {
            print!("\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_color!($color2), $crate::_color!($alt $color));
            print!($($tt)*);
            println!("{}", $crate::_color!(reset));
        }
    };

    ($color:ident on $color2:ident; $($tt:tt)*) => {
        {
            print!("\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_color!($color2), $crate::_color!($color));
            print!($($tt)*);
            println!("{}", $crate::_color!(reset));
        }
    };

    ($alt:ident $color:ident; $cmod:ident; $($tt:tt)*) => {
        {
            print!("{}\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($alt $color));
            print!($($tt)*);
            println!("{}", $crate::_color!(reset));
        }
    };


    ($alt:ident $color:ident; $($tt:tt)*) => {
        {
            print!("\x1b[38;5;{}m", $crate::_color!($alt $color));
            print!($($tt)*);
            println!("{}", $crate::_color!(reset));
        }
    };

    ($color:ident; $cmod:ident; $($tt:tt)*) => {
        {
            print!("{}\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($color));
            print!($($tt)*);
            println!("{}", $crate::_color!(reset));
        }
    };


    ($color:ident; $($tt:tt)*) => {
        {
            print!("\x1b[38;5;{}m", $crate::_color!($color));
            print!($($tt)*);
            println!("{}", $crate::_color!(reset));
        }
    };

    ($($tt:tt)*) => {
        {
            println!($($tt)*);
        }
    };
}

/// #### print! with colors
/// Behaves almost identically to the standard `print!` macro.
///
/// In fact it can act as a drop in replacement.
///
/// # Arguments
/// a `cprint!` call requires a prepend of a color, other optional params, then `;`
/// to the fmt and arguments passed to `print!`
///
/// ## Colors
/// A full list of colors can be seen by looking at the Color enum, or by using
/// the `cprint_palette!` macro. Additionally, `_` can be used as a placeholder
/// for terminal default color.
///
/// ## Backgrounds
/// `cprint!` supports the same color list being used as text background, this can be
/// done by following a `color` keyword with the `on` `<color>` `;`
///
/// ## Modifiers
/// Text can be further modified after a color with an optional parameter
///     - `bold`
///     - `italic`
///     - `underline`
///     - `x` | `strikethrough` | `dashed`
///     - `invert` | `reverse`
///     - `hidden`
///
/// # Examples
///
/// ```
/// use cprint::cprint;
///
/// // print!("hello world");  ==
/// // cprint!("hello world"); ==
/// cprint!(_; "hello world");
///
/// cprint!(red; "red text");
/// cprint!(light red; "light red text");
/// cprint!(green; bold; "bold green text");
/// cprint!(dark blue; italic; "italic dark blue text");
/// cprint!(_; invert; "inverted color text");
/// cprint!(_; italic; "italic default text");
/// cprint!(_; x; "strikethrough text with args: 1=={} and 2=={}", 1, 2);
/// cprint!(red on black; "red text on black background");
/// cprint!(light red on black; "light red text on black background");
/// cprint!(bright red on dark gray; "bright red text on dark gray background");
/// cprint!(bright red on dark gray; invert; "dark gray text on bright red background");
/// ```
#[macro_export]
macro_rules! cprint {
    (_ on $alt2:ident $color2:ident; $cmod:ident; $($tt:tt)*) => {
        {
            print!("{}\x1b[48;5;{}m", $crate::_cmod!($cmod), $crate::_color!($alt2 $color2));
            print!($($tt)*);
            print!("{}", $crate::_color!(reset));
        }
    };

    (_ on $color2:ident; $cmod:ident; $($tt:tt)*) => {
        {
            print!("{}\x1b[48;5;{}m", $crate::_cmod!($cmod), $crate::_color!($color2));
            print!($($tt)*);
            print!("{}", $crate::_color!(reset));
        }
    };

    (_; $cmod:ident; $($tt:tt)*) => {
        {
            print!("{}", $crate::_cmod!($cmod));
            print!($($tt)*);
            print!("{}", $crate::_color!(reset));
        }
    };

    (_ on $alt2:ident $color2:ident; $($tt:tt)*) => {
        {
            print!("{}\x1b[48;5;{}m", $crate::_color!($alt2 $color2));
            print!($($tt)*);
            print!("{}", $crate::_color!(reset));
        }
    };

    (_ on $color2:ident; $($tt:tt)*) => {
        {
            print!("\x1b[48;5;{}m", $crate::_color!($color2));
            print!($($tt)*);
            print!("{}", $crate::_color!(reset));
        }
    };

    (_; $($tt:tt)*) => {
        {
            print!($($tt)*);
        }
    };

    ($alt:ident $color:ident on $alt2:ident $color2:ident; $cmod:ident; $($tt:tt)*) => {
        {
            print!("{}\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($alt2 $color2), $crate::_color!($alt $color));
            print!($($tt)*);
            print!("{}", $crate::_color!(reset));
        }
    };

    ($color:ident on $alt2:ident $color2:ident; $cmod:ident; $($tt:tt)*) => {
        {
            print!("{}\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($alt2 $color2), $crate::_color!($color));
            print!($($tt)*);
            print!("{}", $crate::_color!(reset));
        }
    };

    ($alt:ident $color:ident on $color2:ident; $cmod:ident; $($tt:tt)*) => {
        {
            print!("{}\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($color2), $crate::_color!($alt $color));
            print!($($tt)*);
            print!("{}", $crate::_color!(reset));
        }
    };

    ($color:ident on $color2:ident; $cmod:ident; $($tt:tt)*) => {
        {
            print!("{}\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($color2), $crate::_color!($color));
            print!($($tt)*);
            print!("{}", $crate::_color!(reset));
        }
    };

    ($alt:ident $color:ident on $alt2:ident $color2:ident; $($tt:tt)*) => {
        {
            print!("\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_color!($alt2 $color2), $crate::_color!($alt $color));
            print!($($tt)*);
            print!("{}", $crate::_color!(reset));
        }
    };

    ($color:ident on $alt2:ident $color2:ident; $($tt:tt)*) => {
        {
            print!("\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_color!($alt2 $color2), $crate::_color!($color));
            print!($($tt)*);
            print!("{}", $crate::_color!(reset));
        }
    };

    ($alt:ident $color:ident on $color2:ident; $($tt:tt)*) => {
        {
            print!("\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_color!($color2), $crate::_color!($alt $color));
            print!($($tt)*);
            print!("{}", $crate::_color!(reset));
        }
    };

    ($color:ident on $color2:ident; $($tt:tt)*) => {
        {
            print!("\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_color!($color2), $crate::_color!($color));
            print!($($tt)*);
            print!("{}", $crate::_color!(reset));
        }
    };

    ($alt:ident $color:ident; $cmod:ident; $($tt:tt)*) => {
        {
            print!("{}\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($alt $color));
            print!($($tt)*);
            print!("{}", $crate::_color!(reset));
        }
    };


    ($alt:ident $color:ident; $($tt:tt)*) => {
        {
            print!("\x1b[38;5;{}m", $crate::_color!($alt $color));
            print!($($tt)*);
            print!("{}", $crate::_color!(reset));
        }
    };

    ($color:ident; $cmod:ident; $($tt:tt)*) => {
        {
            print!("{}\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($color));
            print!($($tt)*);
            print!("{}", $crate::_color!(reset));
        }
    };


    ($color:ident; $($tt:tt)*) => {
        {
            print!("\x1b[38;5;{}m", $crate::_color!($color));
            print!($($tt)*);
            print!("{}", $crate::_color!(reset));
        }
    };

    ($($tt:tt)*) => {
        {
            print!($($tt)*);
        }
    };
}

#[cfg(test)]
mod tests {

    macro_rules! section {
        ($name:expr, $s:expr) => {
            {
                $crate::cprintln!(_ ; bold; " ===== {} ===== ", $name.to_uppercase());
                $s;
                $crate::cprintln!("");
                $crate::cprintln!("");
            }
        };
    }

    fn full_palette() {
        cprint_palette!();
    }

    fn color() {
        cprintln!(lime; "Hello world");
        cprintln!(white; "Hello world");
        cprintln!(black; "Hello world");
        cprintln!(red; "Hello world");
        cprintln!(light red; "Hello world");
        cprintln!(teal; "Hello world");
        cprintln!(dark orange; "Hello world");
        cprintln!(light pink; "Hello world, with args {}, {}", 1, 2);
    }

    fn bold() {
        cprintln!(lime; bold; "Hello world");
        cprintln!(white; bold; "Hello world");
        cprintln!(black; bold; "Hello world");
    }

    fn strikethrough() {
        cprintln!(lime; x; "Hello world");
        cprintln!(white; strikethrough; "Hello world");
        cprintln!(black; x; "Hello world");
    }

    fn with_background() {
        cprintln!(lime on black; "Hello world");
        cprintln!(lime on dark gray; "Hello world");
        cprintln!(black on white; "Hello world");
        cprintln!(_ on black; "Hello world");
    }

    fn reverse() {
        cprintln!(lime on black; reverse; "Hello world");
        cprintln!(lime on dark gray; reverse; "Hello world");
        cprintln!(white; reverse; "Hello world");
        cprintln!(black; invert; "Hello world");
        cprintln!(_ on black; invert; "Hello world");
    }

    #[test]
    fn demo() {
        section!("pallete", full_palette());
        section!("color", color());
        section!("bold", bold());
        section!("x (strikethrough)", strikethrough());
        section!("with background", with_background());
        section!("inverted (reverse)", reverse());
    }
}
