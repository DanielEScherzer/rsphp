/// Create a markdown string with the C version of a macro, for use in doc
/// attributes.
macro_rules! c_version {
    ($code:expr) => {
        c_version!(@overall c_version!(@code $code))
    };

    (@code $code:expr) => {
        concat!("```c\n", $code, "\n", "```")
    };
    (@overall $contents:expr) => {
        concat!("\n", "# C version\n", "\n", $contents)
    };
}

/// Creates a markdown string with differences from the C version.
macro_rules! c_differences {
    ( $( $bullet:literal )+ ) => {
        concat!(
            "\n",
            "# Differences from C version\n",
            "\n",
            $(
                "* ",
                $bullet,
                "\n",
            )+
        )
    }
}

pub(crate) use c_differences;
pub(crate) use c_version;
