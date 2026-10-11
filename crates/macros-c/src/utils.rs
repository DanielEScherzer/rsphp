/// Create a markdown string with the C version of a macro, for use in doc
/// attributes.
#[cfg_attr(
    all(not(doc), php_zend_zts),
    expect(
        unused_macro_rules,
        reason = "ZTS/NTS variants documented on the NTS version"
    )
)]
macro_rules! c_version {
    ($code:expr) => {
        c_version!(@overall c_version!(@code $code))
    };
    ($line1:literal, $line2:literal) => {
        c_version!(@overall c_version!(@code $line1, $line2))
    };
    (NTS => $nts:literal, ZTS => $zts:literal,) => {
        c_version!(
            @overall
            concat!(
                c_version!(@subsection "NTS" c_version!(@code $nts)),
                "\n\n",
                c_version!(@subsection "ZTS" c_version!(@code $zts)),
            )
        )
    };
    (NTS => [$nts1:literal, $nts2:literal,], ZTS => [$zts1:literal, $zts2:literal,],) => {
        c_version!(
            @overall
            concat!(
                c_version!(@subsection "NTS" c_version!(@code $nts1, $nts2)),
                "\n\n",
                c_version!(@subsection "ZTS" c_version!(@code $zts1, $zts2)),
            )
        )
    };

    (@code $code:expr) => {
        concat!("```c\n", $code, "\n", "```")
    };
    (@code $line1:literal, $line2:literal) => {
        c_version!(@code concat!($line1, " \\", "\n", "    ", $line2))
    };
    (@overall $contents:expr) => {
        concat!("\n", "# C version\n", "\n", $contents)
    };
    (@subsection $label:literal $contents:expr) => {
        concat!("## ", $label, "\n", "\n", $contents)
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
