/// Creates the path to a file relative to the project directory
#[macro_export]
macro_rules! project_path {
    ($path:expr) => { concat!(env!("CARGO_MANIFEST_DIR"), $path) };
}

/// Create a new HashMap as you would make a Vec with vec![]
///
/// ## Create an empty HashMap
///
/// ```rs
/// let mut map = map!{};
/// ```
///
/// ## Create an empty HashMap<&str, Vec<usize>>
///
/// ```rs
/// let mut map = map!{ &str => Vec<usize> }
/// ```
///
/// ## Create a HashMap with some stuff in it
///
/// ```rs
/// let scores = map!{
///     "Aidan" => 3,
///     "Emily" => 2,
///     "Rory" => 5,
///     "Stephanie" => 4,
/// }
/// ```
#[macro_export]
macro_rules! map {
    ($K:ty => $V:ty) => { map!{ $K => $V;} };

    () => { std::collections::HashMap::new() };

    ($( $k:expr => $v:expr ),* $(,)*) => {
        {
            #[allow(unused_mut)]
            let mut m = std::collections::HashMap::new();
            $( m.insert($k, $v); )*
            m
        }
    };

    ($K:ty => $V:ty ; $( $k:expr => $v:expr ),* $(,)*) => {
        {
            #[allow(unused_mut)]
            let mut m: std::collections::HashMap<$K, $V> = std::collections::HashMap::new();
            $( m.insert($k, $v); )*
            m
        }
    };
}

#[macro_export]
macro_rules! render_template {
    ($tmpl:expr, { $( $key:expr => $value:expr ),* $(,)* }) => {
        {
            let mut ctx = ::rocket_contrib::templates::tera::Context::new();
            $( ctx.insert($key, &$value); )*
            render_template!($tmpl, ctx)
        }
    };

    ($tmpl:expr, $context:expr) => {
        ::rocket_contrib::templates::Template::render($tmpl, $context)
    };

    ($tmpl:expr) => { render_template!($tmpl, map!{ String => String }) };
}



