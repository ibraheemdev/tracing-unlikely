#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub use tracing::*;

/// Constructs an event at the error level.
#[macro_export]
macro_rules! error {
    // Name / target / parent.
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::ERROR, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::ERROR, { $($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::ERROR, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::ERROR, { %$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::ERROR, {}, $($arg)+)
    );

    // Name / target.
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::ERROR, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::ERROR, { $($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::ERROR, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::ERROR, { %$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::ERROR, {}, $($arg)+)
    );

    // Target / parent.
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::ERROR, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::ERROR, { $($k).+ $($field)* })
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::ERROR, { ?$($k).+ $($field)* })
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::ERROR, { %$($k).+ $($field)* })
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::ERROR, {}, $($arg)+)
    );

    // Name / parent.
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::ERROR, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::ERROR, { $($k).+ $($field)* })
    );
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::ERROR, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::ERROR, { %$($k).+ $($field)* })
    );
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::ERROR, {}, $($arg)+)
    );

    // Name.
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::ERROR, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::ERROR, { $($k).+ $($field)* })
    );
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::ERROR, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::ERROR, { %$($k).+ $($field)* })
    );
    (name: $name:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, $crate::Level::ERROR, {}, $($arg)+)
    );

    // Target.
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, { $($k).+ $($field)* })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, { ?$($k).+ $($field)* })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, { %$($k).+ $($field)* })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, {}, $($arg)+)
    );

    // Parent.
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { $($field)+ },
            $($arg)+
        )
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { $($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { ?$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { %$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { $($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { ?$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { %$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            {},
            $($arg)+
        )
    );

    // ...
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { $($k).+ = $($field)*}
        )
    );
    (?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { ?$($k).+ = $($field)*}
        )
    );
    (%$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { %$($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            $($arg)+
        )
    );
}

/// Constructs a span at the error level.
#[macro_export]
macro_rules! error_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            parent: $parent,
            $crate::Level::ERROR,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, parent: $parent:expr, $name:expr) => {
        $crate::error_span!(target: $target, parent: $parent, $name,)
    };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            $name,
            $($field)*
        )
    };
    (parent: $parent:expr, $name:expr) => {
        $crate::error_span!(parent: $parent, $name,)
    };
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::ERROR,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::error_span!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::ERROR,
            $name,
            $($field)*
        )
    };
    ($name:expr) => {$crate::error_span!($name,)};
}

/// Constructs an event at the warn level.
#[macro_export]
macro_rules! warn {
    // Name / target / parent.
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::WARN, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::WARN, { $($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::WARN, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::WARN, { %$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::WARN, {}, $($arg)+)
    );

    // Name / target.
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::WARN, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::WARN, { $($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::WARN, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::WARN, { %$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::WARN, {}, $($arg)+)
    );

    // Target / parent.
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, { $($k).+ $($field)* })
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, { ?$($k).+ $($field)* })
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, { %$($k).+ $($field)* })
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, {}, $($arg)+)
    );

    // Name / parent.
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::WARN, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::WARN, { $($k).+ $($field)* })
    );
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::WARN, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::WARN, { %$($k).+ $($field)* })
    );
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::WARN, {}, $($arg)+)
    );

    // Name.
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::WARN, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::WARN, { $($k).+ $($field)* })
    );
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::WARN, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::WARN, { %$($k).+ $($field)* })
    );
    (name: $name:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, $crate::Level::WARN, {}, $($arg)+)
    );

    // Target.
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::WARN, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::WARN, { $($k).+ $($field)* })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::WARN, { ?$($k).+ $($field)* })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::WARN, { %$($k).+ $($field)* })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::WARN, {}, $($arg)+)
    );

    // Parent.
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { $($field)+ },
            $($arg)+
        )
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { $($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { ?$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { %$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { $($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { ?$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { %$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            {},
            $($arg)+
        )
    );

    // ...
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { $($k).+ = $($field)*}
        )
    );
    (?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { ?$($k).+ = $($field)*}
        )
    );
    (%$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { %$($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            $($arg)+
        )
    );
}

/// Constructs a span at the warn level.
#[macro_export]
macro_rules! warn_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            parent: $parent,
            $crate::Level::WARN,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, parent: $parent:expr, $name:expr) => {
        $crate::warn_span!(target: $target, parent: $parent, $name,)
    };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            $name,
            $($field)*
        )
    };
    (parent: $parent:expr, $name:expr) => {
        $crate::warn_span!(parent: $parent, $name,)
    };
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::WARN,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::warn_span!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::WARN,
            $name,
            $($field)*
        )
    };
    ($name:expr) => {$crate::warn_span!($name,)};
}

/// Constructs an event at the info level.
#[macro_export]
macro_rules! info {
    // Name / target / parent.
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::INFO, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::INFO, { $($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::INFO, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::INFO, { %$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::INFO, {}, $($arg)+)
    );

    // Name / target.
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::INFO, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::INFO, { $($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::INFO, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::INFO, { %$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::INFO, {}, $($arg)+)
    );

    // Target / parent.
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, { $($k).+ $($field)* })
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, { ?$($k).+ $($field)* })
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, { %$($k).+ $($field)* })
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, {}, $($arg)+)
    );

    // Name / parent.
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::INFO, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::INFO, { $($k).+ $($field)* })
    );
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::INFO, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::INFO, { %$($k).+ $($field)* })
    );
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::INFO, {}, $($arg)+)
    );

    // Name.
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::INFO, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::INFO, { $($k).+ $($field)* })
    );
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::INFO, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::INFO, { %$($k).+ $($field)* })
    );
    (name: $name:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, $crate::Level::INFO, {}, $($arg)+)
    );

    // Target.
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::INFO, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::INFO, { $($k).+ $($field)* })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::INFO, { ?$($k).+ $($field)* })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::INFO, { %$($k).+ $($field)* })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::INFO, {}, $($arg)+)
    );

    // Parent.
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { $($field)+ },
            $($arg)+
        )
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { $($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { ?$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { %$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { $($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { ?$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { %$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            {},
            $($arg)+
        )
    );

    // ...
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { $($k).+ = $($field)*}
        )
    );
    (?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { ?$($k).+ = $($field)*}
        )
    );
    (%$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { %$($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            $($arg)+
        )
    );
}

/// Constructs a span at the info level.
#[macro_export]
macro_rules! info_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            parent: $parent,
            $crate::Level::INFO,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, parent: $parent:expr, $name:expr) => {
        $crate::info_span!(target: $target, parent: $parent, $name,)
    };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            $name,
            $($field)*
        )
    };
    (parent: $parent:expr, $name:expr) => {
        $crate::info_span!(parent: $parent, $name,)
    };
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::INFO,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::info_span!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::INFO,
            $name,
            $($field)*
        )
    };
    ($name:expr) => {$crate::info_span!($name,)};
}

/// Constructs an event at the debug level.
#[macro_export]
macro_rules! debug {
    // Name / target / parent.
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::DEBUG, { $($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::DEBUG, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::DEBUG, { %$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::DEBUG, {}, $($arg)+)
    );

    // Name / target.
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::DEBUG, { $($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::DEBUG, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::DEBUG, { %$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::DEBUG, {}, $($arg)+)
    );

    // Target / parent.
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { $($k).+ $($field)* })
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { ?$($k).+ $($field)* })
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { %$($k).+ $($field)* })
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, {}, $($arg)+)
    );

    // Name / parent.
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::DEBUG, { $($k).+ $($field)* })
    );
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::DEBUG, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::DEBUG, { %$($k).+ $($field)* })
    );
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::DEBUG, {}, $($arg)+)
    );

    // Name.
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::DEBUG, { $($k).+ $($field)* })
    );
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::DEBUG, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::DEBUG, { %$($k).+ $($field)* })
    );
    (name: $name:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, $crate::Level::DEBUG, {}, $($arg)+)
    );

    // Target.
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, { $($k).+ $($field)* })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, { ?$($k).+ $($field)* })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, { %$($k).+ $($field)* })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, {}, $($arg)+)
    );

    // Parent.
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { $($field)+ },
            $($arg)+
        )
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { $($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { ?$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { %$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { $($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { ?$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { %$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            {},
            $($arg)+
        )
    );

    // ...
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { $($k).+ = $($field)*}
        )
    );
    (?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { ?$($k).+ = $($field)*}
        )
    );
    (%$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { %$($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            $($arg)+
        )
    );
}

/// Constructs a span at the debug level.
#[macro_export]
macro_rules! debug_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            parent: $parent,
            $crate::Level::DEBUG,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, parent: $parent:expr, $name:expr) => {
        $crate::debug_span!(target: $target, parent: $parent, $name,)
    };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            $name,
            $($field)*
        )
    };
    (parent: $parent:expr, $name:expr) => {
        $crate::debug_span!(parent: $parent, $name,)
    };
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::DEBUG,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::debug_span!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::DEBUG,
            $name,
            $($field)*
        )
    };
    ($name:expr) => {$crate::debug_span!($name,)};
}

/// Constructs an event at the trace level.
#[macro_export]
macro_rules! trace {
    // Name / target / parent.
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::TRACE, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::TRACE, { $($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::TRACE, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::TRACE, { %$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $crate::Level::TRACE, {}, $($arg)+)
    );

    // Name / target.
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::TRACE, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::TRACE, { $($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::TRACE, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::TRACE, { %$($k).+ $($field)* })
    );
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, target: $target, $crate::Level::TRACE, {}, $($arg)+)
    );

    // Target / parent.
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE, { $($k).+ $($field)* })
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE, { ?$($k).+ $($field)* })
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE, { %$($k).+ $($field)* })
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE, {}, $($arg)+)
    );

    // Name / parent.
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::TRACE, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::TRACE, { $($k).+ $($field)* })
    );
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::TRACE, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::TRACE, { %$($k).+ $($field)* })
    );
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, parent: $parent, $crate::Level::TRACE, {}, $($arg)+)
    );

    // Name.
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::TRACE, { $($field)* }, $($arg)*)
    );
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::TRACE, { $($k).+ $($field)* })
    );
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::TRACE, { ?$($k).+ $($field)* })
    );
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(name: $name, $crate::Level::TRACE, { %$($k).+ $($field)* })
    );
    (name: $name:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, $crate::Level::TRACE, {}, $($arg)+)
    );

    // Target.
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, { $($k).+ $($field)* })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, { ?$($k).+ $($field)* })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, { %$($k).+ $($field)* })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, {}, $($arg)+)
    );

    // Parent.
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { $($field)+ },
            $($arg)+
        )
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { $($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { ?$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { %$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { $($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { ?$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { %$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            {},
            $($arg)+
        )
    );

    // ...
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { $($k).+ = $($field)*}
        )
    );
    (?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { ?$($k).+ = $($field)*}
        )
    );
    (%$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { %$($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            $($arg)+
        )
    );
}

/// Constructs a span at the trace level.
#[macro_export]
macro_rules! trace_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            parent: $parent,
            $crate::Level::TRACE,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, parent: $parent:expr, $name:expr) => {
        $crate::trace_span!(target: $target, parent: $parent, $name,)
    };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            $name,
            $($field)*
        )
    };
    (parent: $parent:expr, $name:expr) => {
        $crate::trace_span!(parent: $parent, $name,)
    };
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::TRACE,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::trace_span!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::TRACE,
            $name,
            $($field)*
        )
    };
    ($name:expr) => { $crate::trace_span!($name,) };
}

/// Constructs a new `Event`.
#[macro_export]
macro_rules! event {
     // Name / target / parent.
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } )=> ({
        $crate::__event_with_level!(level: $lvl, name: $name, target: $target, parent: $parent, $lvl, { $($fields)* })
    });
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
        $crate::event!(
            name: $name,
            target: $target,
            parent: $parent,
            $lvl,
            { message = $crate::__macro_support::format_args!($($arg)+), $($fields)* }
        )
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $lvl, { $($k).+ = $($fields)* })
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, $($arg:tt)+) => (
        $crate::event!(name: $name, target: $target, parent: $parent, $lvl, { $($arg)+ })
    );

    // Name / target.
    (name: $name:expr, target: $target:expr, $lvl:expr, { $($fields:tt)* } )=> ({
        $crate::__event_with_level!(level: $lvl, name: $name, target: $target, $lvl, { $($fields)* })
    });
    (name: $name:expr, target: $target:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
        $crate::event!(
            name: $name,
            target: $target,
            $lvl,
            { message = $crate::__macro_support::format_args!($($arg)+), $($fields)* }
        )
    );
    (name: $name:expr, target: $target:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => (
        $crate::event!(name: $name, target: $target, $lvl, { $($k).+ = $($fields)* })
    );
    (name: $name:expr, target: $target:expr, $lvl:expr, $($arg:tt)+) => (
        $crate::event!(name: $name, target: $target, $lvl, { $($arg)+ })
    );

    // Target / parent.
    (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } )=> ({
        $crate::__event_with_level!(level: $lvl, target: $target, parent: $parent, $lvl, { $($fields)* })
    });
    (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
        $crate::event!(
            target: $target,
            parent: $parent,
            $lvl,
            { message = $crate::__macro_support::format_args!($($arg)+), $($fields)* }
        )
    );
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $lvl, { $($k).+ = $($fields)* })
    );
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $($arg:tt)+) => (
        $crate::event!(target: $target, parent: $parent, $lvl, { $($arg)+ })
    );

    // Name / parent.
    (name: $name:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } )=> ({
        $crate::__event_with_level!(level: $lvl, name: $name, parent: $parent, $lvl, { $($fields)* })
    });
    (name: $name:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
        $crate::event!(
            name: $name,
            parent: $parent,
            $lvl,
            { message = $crate::__macro_support::format_args!($($arg)+), $($fields)* }
        )
    );
    (name: $name:expr, parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => (
        $crate::event!(name: $name, parent: $parent, $lvl, { $($k).+ = $($fields)* })
    );
    (name: $name:expr, parent: $parent:expr, $lvl:expr, $($arg:tt)+) => (
        $crate::event!(name: $name, parent: $parent, $lvl, { $($arg)+ })
    );

    // Name.
    (name: $name:expr, $lvl:expr, { $($fields:tt)* } )=> ({
        $crate::__event_with_level!(level: $lvl, name: $name, $lvl, { $($fields)* })
    });
    (name: $name:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
        $crate::event!(
            name: $name,
            $lvl,
            { message = $crate::__macro_support::format_args!($($arg)+), $($fields)* }
        )
    );
    (name: $name:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => (
        $crate::event!(name: $name, $lvl, { $($k).+ = $($fields)* })
    );
    (name: $name:expr, $lvl:expr, $($arg:tt)+ ) => (
        $crate::event!(name: $name, $lvl, { $($arg)+ })
    );

    // Target.
    (target: $target:expr, $lvl:expr, { $($fields:tt)* } )=> ({
        $crate::__event_with_level!(level: $lvl, target: $target, $lvl, { $($fields)* })
    });
    (target: $target:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
        $crate::event!(
            target: $target,
            $lvl,
            { message = $crate::__macro_support::format_args!($($arg)+), $($fields)* }
        )
    );
    (target: $target:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => (
        $crate::event!(target: $target, $lvl, { $($k).+ = $($fields)* })
    );
    (target: $target:expr, $lvl:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $lvl, { $($arg)+ })
    );

    // Parent.
    (parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $lvl,
            { message = $crate::__macro_support::format_args!($($arg)+), $($fields)* }
        )
    );
    (parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $lvl,
            { $($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $lvl:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $lvl,
            { ?$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $lvl:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $lvl,
            { %$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $lvl:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $lvl,
            { $($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $lvl:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $lvl,
            { %$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $lvl:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $lvl,
            { ?$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $lvl:expr, $($arg:tt)+ ) => (
        $crate::event!(target: module_path!(), parent: $parent, $lvl, { $($arg)+ })
    );

    // ...
    ( $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { message = $crate::__macro_support::format_args!($($arg)+), $($fields)* }
        )
    );
    ( $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { message = format_args!($($arg)+), $($fields)* }
        )
    );
    ($lvl:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { $($k).+ = $($field)*}
        )
    );
    ($lvl:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { $($k).+, $($field)*}
        )
    );
    ($lvl:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { ?$($k).+, $($field)*}
        )
    );
    ($lvl:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { %$($k).+, $($field)*}
        )
    );
    ($lvl:expr, ?$($k:ident).+) => (
        $crate::event!($lvl, ?$($k).+,)
    );
    ($lvl:expr, %$($k:ident).+) => (
        $crate::event!($lvl, %$($k).+,)
    );
    ($lvl:expr, $($k:ident).+) => (
        $crate::event!($lvl, $($k).+,)
    );
    ( $lvl:expr, $($arg:tt)+ ) => (
        $crate::event!(target: module_path!(), $lvl, { $($arg)+ })
    );
}

/// Constructs a new span.
#[macro_export]
macro_rules! span {
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr) => {
        $crate::span!(target: $target, parent: $parent, $lvl, $name,)
    };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr, $($fields:tt)*) => {
        $crate::__span_with_level!(level: $lvl, target: $target, parent: $parent, $lvl, $name, $($fields)*)
    };
    (target: $target:expr, $lvl:expr, $name:expr, $($fields:tt)*) => {
        $crate::__span_with_level!(level: $lvl, target: $target, $lvl, $name, $($fields)*)
    };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr) => {
        $crate::span!(target: $target, parent: $parent, $lvl, $name,)
    };
    (parent: $parent:expr, $lvl:expr, $name:expr, $($fields:tt)*) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $lvl,
            $name,
            $($fields)*
        )
    };
    (parent: $parent:expr, $lvl:expr, $name:expr) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $lvl,
            $name,
        )
    };
    (target: $target:expr, $lvl:expr, $name:expr, $($fields:tt)*) => {
        $crate::span!(
            target: $target,
            $lvl,
            $name,
            $($fields)*
        )
    };
    (target: $target:expr, $lvl:expr, $name:expr) => {
        $crate::span!(target: $target, $lvl, $name,)
    };
    ($lvl:expr, $name:expr, $($fields:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $lvl,
            $name,
            $($fields)*
        )
    };
    ($lvl:expr, $name:expr) => {
        $crate::span!(
            target: module_path!(),
            $lvl,
            $name,
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __event_with_level {
    (level: $lvl:expr, $($arg:tt)*) => {{
        let emit = {
            #[cold] #[inline(never)] || { $crate::__macro_support::tracing::event!($($arg)*) }
        };

        if $crate::__macro_support::tracing::enabled!($lvl) {
            emit();
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __span_with_level {
    (level: $lvl:expr, $($arg:tt)*) => {{
        let emit = {
            #[cold] #[inline(never)] || { $crate::__macro_support::tracing::span!($($arg)*) }
        };

        if $crate::__macro_support::tracing::enabled!($lvl) {
            emit()
        } else {
            $crate::__macro_support::tracing::Span::none()
        }
    }};
}

#[doc(hidden)]
pub mod __macro_support {
    pub use core::format_args;
    pub use tracing;
}
