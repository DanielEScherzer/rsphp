/// Create a markdown string with the C version of a macro, for use in doc
/// attributes.
#[cfg_attr(
    all(not(doc), any(php_zend_zts, not(php_is_86))),
    expect(
        unused_macro_rules,
        reason = "ZTS/NTS variants documented on the NTS version and on the PHP 8.5 version"
    )
)]
macro_rules! c_version {
    ($code:expr) => {
        c_version!(@overall c_version!(@code $code))
    };
    ($line1:literal, $line2:literal) => {
        c_version!(@overall c_version!(@code $line1, $line2))
    };
    (85 => $code:literal, 86 => None,) => {
        c_version!(
            @overall
            concat!(
                c_version!(@subsection "PHP 8.5" c_version!(@code $code)),
                "\n\n",
                c_version!(@subsection "PHP 8.6" "The corresponding C macro was removed.")
            )
        )
    };
    (85 => [$( $line85:literal, )+], 86 => None,) => {
        c_version!(
            @overall
            concat!(
                c_version!(@subsection "PHP 8.5" c_version!(@code $( $line85, )+ )),
                "\n\n",
                c_version!(@subsection "PHP 8.6" "The corresponding C macro was removed.")
            )
        )
    };
    (85 => [$( $line85:literal, )+], 86 => [$( $line86:literal, )+],) => {
        c_version!(
            @overall
            concat!(
                c_version!(@subsection "PHP 8.5" c_version!(@code $( $line85, )+ )),
                "\n\n",
                c_version!(@subsection "PHP 8.6" c_version!(@code $( $line86, )+ ))
            )
        )
    };
    (
        85 => [
            NTS => [$( $line85NTS:literal, )+],
            ZTS => [$( $line85ZTS:literal, )+],
        ],
        86 => [
            NTS => [$( $line86NTS:literal, )+],
            ZTS => [$( $line86ZTS:literal, )+],
        ],
    ) => {
        c_version!(
            @overall
            concat!(
                c_version!(
                    @subsection
                    "PHP 8.5"
                    concat!(
                        c_version!(@subsubsection "NTS" c_version!(@code $( $line85NTS, )+ ) ),
                        "\n\n",
                        c_version!(@subsubsection "ZTS" c_version!(@code $( $line85ZTS, )+ ) ),
                    )
                ),
                "\n\n",
                c_version!(
                    @subsection
                    "PHP 8.6"
                    concat!(
                        c_version!(@subsubsection "NTS" c_version!(@code $( $line86NTS, )+ ) ),
                        "\n\n",
                        c_version!(@subsubsection "ZTS" c_version!(@code $( $line86ZTS, )+ ) ),
                    )
                ),
            )
        )
    };

    (@code $code:expr $(,)?) => {
        concat!("```c\n", $code, "\n", "```")
    };
    (@code $line1:literal, $line2:literal $(,)?) => {
        c_version!(@code concat!($line1, " \\", "\n", "    ", $line2))
    };
    (@overall $contents:expr) => {
        concat!("\n", "# C version\n", "\n", $contents)
    };
    (@subsection $label:literal $contents:expr) => {
        concat!("## ", $label, "\n", "\n", $contents)
    };
    (@subsubsection $label:literal $contents:expr) => {
        concat!("### ", $label, "\n", "\n", $contents)
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
