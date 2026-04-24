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

pub mod animations;

/// List of available colors to be used with `cprint!` and `cprintln!`
///
///  In macro, use all lowercase and spaces for colors
/// e.g. BrightRed -> `$crate::cprintln!(bright red; "red");`
#[derive(Debug, Clone, Copy)]
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
    Foreground,
    Background,
    Ok,
    Error,
    Warn,
    Info,
    Verbose,
}
impl Colors {
    pub fn to_color(&self) -> u32 {
        match self {
            Colors::Black => _color!(black),
            Colors::White => _color!(white),
            Colors::Red => _color!(red),
            Colors::Green => _color!(green),
            Colors::Blue => _color!(blue),
            Colors::DarkRed => _color!(dark red),
            Colors::DarkGreen => _color!(dark green),
            Colors::DarkBlue => _color!(dark blue),
            Colors::LightRed => _color!(light red),
            Colors::LightGreen => _color!(light green),
            Colors::LightBlue => _color!(light blue),
            Colors::BrightRed => _color!(bright red),
            Colors::BrightGreen => _color!(bright green),
            Colors::BrightBlue => _color!(bright blue),
            Colors::DarkYellow => _color!(dark yellow),
            Colors::Yellow => _color!(yellow),
            Colors::LightYellow => _color!(light yellow),
            Colors::BrightYellow => _color!(bright yellow),
            Colors::DarkMagenta => _color!(dark magenta),
            Colors::Magenta => _color!(magenta),
            Colors::LightMagenta => _color!(light magenta),
            Colors::BrightMagenta => _color!(bright magenta),
            Colors::DarkCyan => _color!(dark cyan),
            Colors::Cyan => _color!(cyan),
            Colors::LightCyan => _color!(light cyan),
            Colors::BrightCyan => _color!(bright cyan),
            Colors::DarkPink => _color!(dark pink),
            Colors::Pink => _color!(pink),
            Colors::LightPink => _color!(light pink),
            Colors::DarkOrange => _color!(dark orange),
            Colors::Orange => _color!(orange),
            Colors::LightOrange => _color!(light orange),
            Colors::DarkGray => _color!(dark gray),
            Colors::Gray => _color!(gray),
            Colors::LightGray => _color!(light gray),
            Colors::Lime => _color!(lime),
            Colors::Peach => _color!(peach),
            Colors::Sky => _color!(sky),
            Colors::Lavender => _color!(lavender),
            Colors::Teal => _color!(teal),
            Colors::Maroon => _color!(maroon),
            Colors::Purple => _color!(purple),
            Colors::Brown => _color!(brown),
            Colors::LightBrown => _color!(light brown),

            Colors::Foreground => _color!(fg),
            Colors::Background => _color!(bg),

            Colors::Ok => _color!(ok),
            Colors::Warn => _color!(warn),
            Colors::Error => _color!(error),
            Colors::Info => _color!(info),
            Colors::Verbose => _color!(verbose),
        }
    }
}

#[macro_export]
macro_rules! cprint_term {
    () => {
        $crate::cprint_term!(@all)
    };

    ($tt:tt) => {
        $crate::cprint_term!(@all, $tt)
    };

    (@all $(, $tt:tt)? $(,)?) => {
        $crate::cprint_term!($($tt)?;fg);
        $crate::cprint_term!($($tt)?;bg);
        println!("");
        $crate::cprint_term!($($tt)?;base0);
        $crate::cprint_term!($($tt)?;base1);
        $crate::cprint_term!($($tt)?;base2);
        $crate::cprint_term!($($tt)?;base3);
        $crate::cprint_term!($($tt)?;base4);
        $crate::cprint_term!($($tt)?;base5);
        $crate::cprint_term!($($tt)?;base6);
        $crate::cprint_term!($($tt)?;base7);
        println!("");
        $crate::cprint_term!($($tt)?;strong0);
        $crate::cprint_term!($($tt)?;strong1);
        $crate::cprint_term!($($tt)?;strong2);
        $crate::cprint_term!($($tt)?;strong3);
        $crate::cprint_term!($($tt)?;strong4);
        $crate::cprint_term!($($tt)?;strong5);
        $crate::cprint_term!($($tt)?;strong6);
        $crate::cprint_term!($($tt)?;strong7);
    };

    ($($cmod:ident)?; $($tt:tt)*) => {
        $(
            print!("{}", $crate::_cmod!($cmod));
        )?
        println!("\x1b[38;5;{}m{}\x1b[0m", $crate::_color!($($tt)*), stringify!($($tt)*));
    };
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

    (grey) =>            { $crate::_color!(gray) };
    (light grey) =>      { $crate::_color!(light gray) };
    (dark grey) =>       { $crate::_color!(dark gray) };

    (lime) =>            { $crate::_color!(@code, 3, 5, 0) };
    (peach) =>           { $crate::_color!(@code, 5, 3, 1) };
    (sky) =>             { $crate::_color!(@code, 2, 4, 5) };
    (lavender) =>        { $crate::_color!(@code, 3, 3, 5) };
    (teal) =>            { $crate::_color!(@code, 2, 5, 4) };
    (maroon) =>          { $crate::_color!(@code, 5, 2, 1) };
    (purple) =>          { $crate::_color!(@code, 4, 0, 4) };
    (brown) =>           { $crate::_color!(@code, 2, 1, 0) };
    (light brown) =>     { $crate::_color!(@code, 3, 2, 0) };

    (base0) =>            { 0 };
    (base1) =>            { 1 };
    (base2) =>            { 2 };
    (base3) =>            { 3 };
    (base4) =>            { 4 };
    (base5) =>            { 5 };
    (base6) =>            { 6 };
    (base7) =>            { 7 };

    (strong0) =>          { 8  };
    (strong1) =>          { 9  };
    (strong2) =>          { 10 };
    (strong3) =>          { 11 };
    (strong4) =>          { 12 };
    (strong5) =>          { 13 };
    (strong6) =>          { 14 };
    (strong7) =>          { 15 };

    (bg)       =>          { 0 };
    (lower)    =>          { 1 };
    (low)      =>          { 2 };
    (mid_low)  =>          { 3 };
    (mid_high) =>          { 4 };
    (high)     =>          { 5 };
    (higher)   =>          { 6 };
    (fg_w)       =>        { 7 };

    (bg_s)       =>          { 8  };
    (lower_s)    =>          { 9  };
    (low_s)      =>          { 10 };
    (mid_low_s)  =>          { 11 };
    (mid_high_s) =>          { 12 };
    (high_s)     =>          { 13 };
    (higher_s)   =>          { 14 };
    (fg)         =>          { 15 };


    (error)        =>           { 9  };
    (ok)           =>           { 10 };
    (warn)         =>           { 11 };
    (info)         =>           { 12 };
    (verbose)      =>           { 13 };

    // ===== Foreground ANSI (literal) =====
    (@fg black)         => { "\x1b[38;5;232m" };
    (@fg white)         => { "\x1b[38;5;255m" };

    (@fg pure red)      => { "\x1b[38;5;196m" };
    (@fg pure green)    => { "\x1b[38;5;46m" };
    (@fg pure blue)     => { "\x1b[38;5;21m" };

    (@fg red)           => { "\x1b[38;5;203m" };
    (@fg green)         => { "\x1b[38;5;113m" };
    (@fg blue)          => { "\x1b[38;5;111m" };

    (@fg light red)     => { "\x1b[38;5;217m" };
    (@fg light green)   => { "\x1b[38;5;157m" };
    (@fg light blue)    => { "\x1b[38;5;153m" };

    (@fg dark red)      => { "\x1b[38;5;160m" };
    (@fg dark green)    => { "\x1b[38;5;34m" };
    (@fg dark blue)     => { "\x1b[38;5;18m" };

    (@fg bright red)    => { "\x1b[38;5;196m" };
    (@fg bright green)  => { "\x1b[38;5;46m" };
    (@fg bright blue)   => { "\x1b[38;5;21m" };


    // ===== Secondary =====
    (@fg yellow)        => { "\x1b[38;5;184m" };
    (@fg dark yellow)   => { "\x1b[38;5;100m" };
    (@fg light yellow)  => { "\x1b[38;5;186m" };
    (@fg bright yellow) => { "\x1b[38;5;226m" };

    (@fg magenta)       => { "\x1b[38;5;169m" };
    (@fg dark magenta)  => { "\x1b[38;5;127m" };
    (@fg light magenta) => { "\x1b[38;5;176m" };
    (@fg bright magenta)=> { "\x1b[38;5;201m" };

    (@fg cyan)          => { "\x1b[38;5;44m" };
    (@fg dark cyan)     => { "\x1b[38;5;30m" };
    (@fg light cyan)    => { "\x1b[38;5;116m" };
    (@fg bright cyan)   => { "\x1b[38;5;51m" };


    // ===== Alternatives =====
    (@fg dark pink)     => { "\x1b[38;5;132m" };
    (@fg pink)          => { "\x1b[38;5;212m" };
    (@fg light pink)    => { "\x1b[38;5;225m" };

    (@fg dark orange)   => { "\x1b[38;5;166m" };
    (@fg orange)        => { "\x1b[38;5;202m" };
    (@fg light orange)  => { "\x1b[38;5;215m" };

    (@fg gray)          => { "\x1b[38;5;246m" };
    (@fg light gray)    => { "\x1b[38;5;252m" };
    (@fg dark gray)     => { "\x1b[38;5;242m" };

    (@fg grey)          => { "\x1b[38;5;246m" };
    (@fg light grey)    => { "\x1b[38;5;252m" };
    (@fg dark grey)     => { "\x1b[38;5;242m" };

    (@fg lime)          => { "\x1b[38;5;154m" };
    (@fg peach)         => { "\x1b[38;5;215m" };
    (@fg sky)           => { "\x1b[38;5;111m" };
    (@fg lavender)      => { "\x1b[38;5;147m" };
    (@fg teal)          => { "\x1b[38;5;121m" };
    (@fg maroon)        => { "\x1b[38;5;209m" };
    (@fg purple)        => { "\x1b[38;5;129m" };
    (@fg brown)         => { "\x1b[38;5;94m" };
    (@fg light brown)   => { "\x1b[38;5;136m" };


    // ===== UI =====
    (@fg error)   => { "\x1b[38;5;9m" };
    (@fg ok)      => { "\x1b[38;5;10m" };
    (@fg warn)    => { "\x1b[38;5;11m" };
    (@fg info)    => { "\x1b[38;5;12m" };


    // ===== Background ANSI (literal) =====
    (@bg black)         => { "\x1b[48;5;232m" };
    (@bg white)         => { "\x1b[48;5;255m" };

    (@bg pure red)      => { "\x1b[48;5;196m" };
    (@bg pure green)    => { "\x1b[48;5;46m" };
    (@bg pure blue)     => { "\x1b[48;5;21m" };

    (@bg red)           => { "\x1b[48;5;203m" };
    (@bg green)         => { "\x1b[48;5;113m" };
    (@bg blue)          => { "\x1b[48;5;111m" };

    (@bg light red)     => { "\x1b[48;5;217m" };
    (@bg light green)   => { "\x1b[48;5;157m" };
    (@bg light blue)    => { "\x1b[48;5;153m" };

    (@bg dark red)      => { "\x1b[48;5;160m" };
    (@bg dark green)    => { "\x1b[48;5;34m" };
    (@bg dark blue)     => { "\x1b[48;5;18m" };

    (@bg bright red)    => { "\x1b[48;5;196m" };
    (@bg bright green)  => { "\x1b[48;5;46m" };
    (@bg bright blue)   => { "\x1b[48;5;21m" };


    // ===== Secondary =====
    (@bg yellow)        => { "\x1b[48;5;184m" };
    (@bg dark yellow)   => { "\x1b[48;5;100m" };
    (@bg light yellow)  => { "\x1b[48;5;186m" };
    (@bg bright yellow) => { "\x1b[48;5;226m" };

    (@bg magenta)       => { "\x1b[48;5;169m" };
    (@bg dark magenta)  => { "\x1b[48;5;127m" };
    (@bg light magenta) => { "\x1b[48;5;176m" };
    (@bg bright magenta)=> { "\x1b[48;5;201m" };

    (@bg cyan)          => { "\x1b[48;5;44m" };
    (@bg dark cyan)     => { "\x1b[48;5;30m" };
    (@bg light cyan)    => { "\x1b[48;5;116m" };
    (@bg bright cyan)   => { "\x1b[48;5;51m" };


    // ===== Alternatives =====
    (@bg dark pink)     => { "\x1b[48;5;132m" };
    (@bg pink)          => { "\x1b[48;5;212m" };
    (@bg light pink)    => { "\x1b[48;5;225m" };

    (@bg dark orange)   => { "\x1b[48;5;166m" };
    (@bg orange)        => { "\x1b[48;5;202m" };
    (@bg light orange)  => { "\x1b[48;5;215m" };

    (@bg gray)          => { "\x1b[48;5;246m" };
    (@bg light gray)    => { "\x1b[48;5;252m" };
    (@bg dark gray)     => { "\x1b[48;5;242m" };

    (@bg grey)          => { "\x1b[48;5;246m" };
    (@bg light grey)    => { "\x1b[48;5;252m" };
    (@bg dark grey)     => { "\x1b[48;5;242m" };

    (@bg lime)          => { "\x1b[48;5;154m" };
    (@bg peach)         => { "\x1b[48;5;215m" };
    (@bg sky)           => { "\x1b[48;5;111m" };
    (@bg lavender)      => { "\x1b[48;5;147m" };
    (@bg teal)          => { "\x1b[48;5;121m" };
    (@bg maroon)        => { "\x1b[48;5;209m" };
    (@bg purple)        => { "\x1b[48;5;129m" };
    (@bg brown)         => { "\x1b[48;5;94m" };
    (@bg light brown)   => { "\x1b[48;5;136m" };


    // ===== UI =====
    (@bg error)   => { "\x1b[48;5;9m" };
    (@bg ok)      => { "\x1b[48;5;10m" };
    (@bg warn)    => { "\x1b[48;5;11m" };
    (@bg info)    => { "\x1b[48;5;12m" };
    (@bg verbose) => { "\x1b[48;5;13m" };

    (@code, $r:literal, $g:literal, $b:literal) => {
        (16 + 36*$r + 6*$g + $b)
    };
}
}

/// Internal emitter used by `cprint!` and `cprintln!`.
///
/// This keeps the public macros as close to `print!` / `println!` as possible while
/// making each color branch perform a single output macro call.
#[doc(hidden)]
#[macro_export]
macro_rules! _cprint_emit {
    (@print $style:expr; $($tt:tt)*) => {{
        print!("{}{}{}", $style, format_args!($($tt)*), $crate::_color!(reset));
    }};

    (@println $style:expr; $($tt:tt)*) => {{
        println!("{}{}{}", $style, format_args!($($tt)*), $crate::_color!(reset));
    }};

    (@print_plain; $($tt:tt)*) => {{
        print!($($tt)*);
    }};

    (@println_plain; $($tt:tt)*) => {{
        println!($($tt)*);
    }};
}

/// Shared parser for `cprint!` and `cprintln!`.
#[doc(hidden)]
#[macro_export]
macro_rules! _cprint_parse {
    // ===== Option<Colors> forms =====
    (@$mode:ident; [$cf:expr] on [$cb:expr]; $cmod:ident; $($tt:tt)*) => {{
        match ($cf, $cb) {
            (Some(f), Some(b)) => $crate::_cprint_parse!(@$mode; {f} on {b}; $cmod; $($tt)*),
            (Some(f), None) => $crate::_cprint_parse!(@$mode; {f}; $cmod; $($tt)*),
            (None, Some(b)) => $crate::_cprint_parse!(@$mode; _ on {b}; $cmod; $($tt)*),
            (None, None) => $crate::_cprint_parse!(@$mode; _; $cmod; $($tt)*),
        }
    }};

    (@$mode:ident; [$cf:expr] on [$cb:expr]; $($tt:tt)*) => {{
        match ($cf, $cb) {
            (Some(f), Some(b)) => $crate::_cprint_parse!(@$mode; {f} on {b}; $($tt)*),
            (Some(f), None) => $crate::_cprint_parse!(@$mode; {f}; $($tt)*),
            (None, Some(b)) => $crate::_cprint_parse!(@$mode; _ on {b}; $($tt)*),
            (None, None) => $crate::_cprint_parse!(@$mode; _; $($tt)*),
        }
    }};

    (@$mode:ident; [$cf:expr]; $cmod:ident; $($tt:tt)*) => {{
        match $cf {
            Some(c) => $crate::_cprint_parse!(@$mode; {c}; $cmod; $($tt)*),
            None => $crate::_cprint_parse!(@$mode; _; $cmod; $($tt)*),
        }
    }};

    (@$mode:ident; [$cf:expr]; $($tt:tt)*) => {{
        match $cf {
            Some(c) => $crate::_cprint_parse!(@$mode; {c}; $($tt)*),
            None => $crate::_cprint_parse!(@$mode; _; $($tt)*),
        }
    }};

    (@$mode:ident; _ on [$cb:expr]; $cmod:ident; $($tt:tt)*) => {{
        match $cb {
            Some(c) => $crate::_cprint_parse!(@$mode; _ on {c}; $cmod; $($tt)*),
            None => $crate::_cprint_parse!(@$mode; _; $cmod; $($tt)*),
        }
    }};

    (@$mode:ident; _ on [$cb:expr]; $($tt:tt)*) => {{
        match $cb {
            Some(c) => $crate::_cprint_parse!(@$mode; _ on {c}; $($tt)*),
            None => $crate::_cprint_parse!(@$mode; _; $($tt)*),
        }
    }};


    // ===== Default foreground/background forms =====
    (@$mode:ident; _ on {$cb:expr}; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("{}\x1b[48;5;{}m", $crate::_cmod!($cmod), $cb.to_color()); $($tt)*)
    };

    (@$mode:ident; _ on {$cb:expr}; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("\x1b[48;5;{}m", $cb.to_color()); $($tt)*)
    };

    (@$mode:ident; _ on $alt2:ident $color2:ident; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("{}\x1b[48;5;{}m", $crate::_cmod!($cmod), $crate::_color!($alt2 $color2)); $($tt)*)
    };

    (@$mode:ident; _ on $alt2:ident $color2:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("\x1b[48;5;{}m", $crate::_color!($alt2 $color2)); $($tt)*)
    };

    (@$mode:ident; _ on $color2:ident; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("{}\x1b[48;5;{}m", $crate::_cmod!($cmod), $crate::_color!($color2)); $($tt)*)
    };

    (@$mode:ident; _ on $color2:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("\x1b[48;5;{}m", $crate::_color!($color2)); $($tt)*)
    };

    (@$mode:ident; _; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode $crate::_cmod!($cmod); $($tt)*)
    };

    (@$mode:ident; _; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode ""; $($tt)*)
    };

    // ===== Enum foreground/background forms =====
    (@$mode:ident; {$cf:expr} on {$cb:expr}; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("{}\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_cmod!($cmod), $cb.to_color(), $cf.to_color()); $($tt)*)
    };

    (@$mode:ident; {$cf:expr} on {$cb:expr}; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("\x1b[48;5;{}m\x1b[38;5;{}m", $cb.to_color(), $cf.to_color()); $($tt)*)
    };

    (@$mode:ident; {$cf:expr} on $alt2:ident $color2:ident; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("{}\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($alt2 $color2), $cf.to_color()); $($tt)*)
    };

    (@$mode:ident; {$cf:expr} on $alt2:ident $color2:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_color!($alt2 $color2), $cf.to_color()); $($tt)*)
    };

    (@$mode:ident; {$cf:expr} on $color2:ident; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("{}\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($color2), $cf.to_color()); $($tt)*)
    };

    (@$mode:ident; {$cf:expr} on $color2:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_color!($color2), $cf.to_color()); $($tt)*)
    };

    (@$mode:ident; $alt:ident $color:ident on {$cb:expr}; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("{}\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_cmod!($cmod), $cb.to_color(), $crate::_color!($alt $color)); $($tt)*)
    };

    (@$mode:ident; $alt:ident $color:ident on {$cb:expr}; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("\x1b[48;5;{}m\x1b[38;5;{}m", $cb.to_color(), $crate::_color!($alt $color)); $($tt)*)
    };

    (@$mode:ident; $color:ident on {$cb:expr}; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("{}\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_cmod!($cmod), $cb.to_color(), $crate::_color!($color)); $($tt)*)
    };

    (@$mode:ident; $color:ident on {$cb:expr}; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("\x1b[48;5;{}m\x1b[38;5;{}m", $cb.to_color(), $crate::_color!($color)); $($tt)*)
    };

    (@$mode:ident; _ on {$cb:expr}; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("{}\x1b[48;5;{}m", $crate::_cmod!($cmod), $cb.to_color()); $($tt)*)
    };

    (@$mode:ident; _ on {$cb:expr}; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("\x1b[48;5;{}m", $cb.to_color()); $($tt)*)
    };

    (@$mode:ident; {$cf:expr}; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("{}\x1b[38;5;{}m", $crate::_cmod!($cmod), $cf.to_color()); $($tt)*)
    };

    (@$mode:ident; {$cf:expr}; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("\x1b[38;5;{}m", $cf.to_color()); $($tt)*)
    };

    // ===== Named foreground/background forms =====
    (@$mode:ident; $alt:ident $color:ident on $alt2:ident $color2:ident; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("{}\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($alt2 $color2), $crate::_color!($alt $color)); $($tt)*)
    };

    (@$mode:ident; $alt:ident $color:ident on $alt2:ident $color2:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_color!($alt2 $color2), $crate::_color!($alt $color)); $($tt)*)
    };

    (@$mode:ident; $alt:ident $color:ident on $color2:ident; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("{}\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($color2), $crate::_color!($alt $color)); $($tt)*)
    };

    (@$mode:ident; $alt:ident $color:ident on $color2:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_color!($color2), $crate::_color!($alt $color)); $($tt)*)
    };

    (@$mode:ident; $color:ident on $alt2:ident $color2:ident; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("{}\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($alt2 $color2), $crate::_color!($color)); $($tt)*)
    };

    (@$mode:ident; $color:ident on $alt2:ident $color2:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_color!($alt2 $color2), $crate::_color!($color)); $($tt)*)
    };

    (@$mode:ident; $color:ident on $color2:ident; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("{}\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($color2), $crate::_color!($color)); $($tt)*)
    };

    (@$mode:ident; $color:ident on $color2:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("\x1b[48;5;{}m\x1b[38;5;{}m", $crate::_color!($color2), $crate::_color!($color)); $($tt)*)
    };

    (@$mode:ident; _ on $alt2:ident $color2:ident; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("{}\x1b[48;5;{}m", $crate::_cmod!($cmod), $crate::_color!($alt2 $color2)); $($tt)*)
    };

    (@$mode:ident; _ on $alt2:ident $color2:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("\x1b[48;5;{}m", $crate::_color!($alt2 $color2)); $($tt)*)
    };

    (@$mode:ident; _ on $color2:ident; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("{}\x1b[48;5;{}m", $crate::_cmod!($cmod), $crate::_color!($color2)); $($tt)*)
    };

    (@$mode:ident; _ on $color2:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("\x1b[48;5;{}m", $crate::_color!($color2)); $($tt)*)
    };

    (@$mode:ident; $alt:ident $color:ident; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("{}\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($alt $color)); $($tt)*)
    };

    (@$mode:ident; $alt:ident $color:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("\x1b[38;5;{}m", $crate::_color!($alt $color)); $($tt)*)
    };

    (@$mode:ident; $color:ident; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("{}\x1b[38;5;{}m", $crate::_cmod!($cmod), $crate::_color!($color)); $($tt)*)
    };

    (@$mode:ident; $color:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode format_args!("\x1b[38;5;{}m", $crate::_color!($color)); $($tt)*)
    };

    // ===== Default foreground forms =====
    (@$mode:ident; _; $cmod:ident; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode $crate::_cmod!($cmod); $($tt)*)
    };

    (@$mode:ident; _; $($tt:tt)*) => {
        $crate::_cprint_emit!(@$mode ""; $($tt)*)
    };

    // ===== Plain drop-in forms =====
    (@print; $($tt:tt)*) => {
        $crate::_cprint_emit!(@print_plain; $($tt)*)
    };

    (@println; $($tt:tt)*) => {
        $crate::_cprint_emit!(@println_plain; $($tt)*)
    };
}

/// #### println! with colors
/// Behaves almost identically to the standard `println!` macro.
#[macro_export]
macro_rules! cprintln {
    ($($tt:tt)*) => {
        $crate::_cprint_parse!(@println; $($tt)*)
    };
}

/// #### print! with colors
/// Behaves almost identically to the standard `print!` macro.
#[macro_export]
macro_rules! cprint {
    ($($tt:tt)*) => {
        $crate::_cprint_parse!(@print; $($tt)*)
    };
}

pub fn print_reset() {
    print!("{}", _color!(reset))
}

pub fn ansi_256_color_to_index(r: u8, g: u8, b: u8) -> u8 {
    16 + 36*r + 6*g + b
}

pub fn ansi_256_index_to_color(index: u8) -> Option<(u8, u8, u8)> {
    if index < 16 || index > 231 {
        return None; // not in the color cube
    }

    let i = index - 16;

    let r = i / 36;
    let g = (i % 36) / 6;
    let b = i % 6;

    Some((r, g, b))
}

#[cfg(test)]
mod tests {
    use crate::Colors;

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

    pub fn full_palette() {
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

    fn formatter() {
        cprintln!(lime on black; "{:<10}|{}", "hello", "world"); // works

        cprint!(lime on black; "{:<10}|", "hello");
        cprintln!(red; bold; "{}", "world");
    }

    fn enums() {
        cprintln!({Colors::Orange}; "{{Colors::Orange}}");
        cprintln!({Colors::Pink}; "{{Colors::Pink}}");
        cprintln!(_ on {Colors::DarkBlue}; "_ on {{Colors::DarkBlue}}");
        cprintln!({Colors::Black} on {Colors::Pink}; "{{Colors::Black}} on {{Colors::Pink}}");
        cprintln!({Colors::Pink}; x; "X {{Colors::Pink}}");
        cprintln!({Colors::Black} on {Colors::Pink}; bold; "bold {{Colors::Black}} on {{Colors::Pink}}");
    }

    fn options() {
        let mut fg = Some(Colors::Black);
        let mut bg = Some(Colors::Red);
        cprintln!([fg] on [bg]; "Some(Black) on Some(Red)");
        bg = None;
        fg = Some(Colors::Red);
        cprintln!([fg] on [bg]; "Some(Red) on None");

        fg = Some(Colors::Black);
        bg = Some(Colors::LightBlue);
        cprintln!([fg] on [bg]; x; "X Some(Black) on Some(LightBlue)");
        bg = None;
        fg = Some(Colors::LightBlue);
        cprintln!([fg] on [bg]; bold; "bold Some(LightBlue) on None");
    }

    fn terminal() {
        cprint_term!();
    }

    fn utility() {
        cprintln!(ok; "ok");
        cprintln!(warn; "warn");
        cprintln!(error; "error");
        cprintln!(info; "info");
        cprintln!(verbose; "verbose");
    }

    #[test]
    pub fn demo() {
        cprint!({Colors::Lavender}; "====== ");
        cprint!(red; "C");
        cprint!(blue; "P");
        cprint!(green; "R");
        cprint!(cyan; "I");
        cprint!(yellow; "N");
        cprint!(teal; "T");
        cprint!({Colors::BrightMagenta}; "!");
        cprintln!({Colors::Lavender}; " ======");
        println!("");
        println!("");

        section!("pallete", full_palette());
        section!("color", color());
        section!("bold", bold());
        section!("x (strikethrough)", strikethrough());
        section!("with background", with_background());
        section!("inverted (reverse)", reverse());
        section!("formatters", formatter());
        section!("using Colors enum", enums());
        section!("using Option<Colors>", options());
        section!("Terminal Colors", terminal());
        section!("Utility Colors", utility());
    }
}

#[cfg(test)]
pub use crate::tests::demo as demo;