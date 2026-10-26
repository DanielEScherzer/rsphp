--TEST--
Reflection basic test
--EXTENSIONS--
rustdemo
--FILE--
<?php
$ref = new ReflectionExtension( 'rustdemo' );
echo $ref;
?>
--EXPECTF--
Extension [ <persistent> extension #%d rustdemo version 0.1.0 ] {

  - INI {
    Entry [ rustdemo.ini_setting <ALL> ]
      Current = '1234'
    }
  }

  - Functions {
    Function [ <internal:rustdemo> function rd_test1 ] {

      - Parameters [0] {
      }
      - Return [ void ]
    }
    Function [ <internal:rustdemo> function rd_test2 ] {

      - Parameters [1] {
        Parameter #0 [ <optional> string $str = "" ]
      }
      - Return [ string ]
    }
    Function [ <internal:rustdemo> function rd_test3 ] {

      - Parameters [1] {
        Parameter #0 [ <required> string $str ]
      }
      - Return [ string ]
    }
    Function [ <internal:rustdemo> function display_value ] {

      - Parameters [1] {
        Parameter #0 [ <required> $v ]
      }
      - Return [ void ]
    }
    Function [ <internal:rustdemo> function zpp_testing ] {

      - Parameters [8] {
        Parameter #0 [ <required> bool $b ]
        Parameter #1 [ <required> ?bool $bn ]
        Parameter #2 [ <required> int $l ]
        Parameter #3 [ <required> ?int $ln ]
        Parameter #4 [ <required> float $f ]
        Parameter #5 [ <required> ?float $fn ]
        Parameter #6 [ <required> string $s1 ]
        Parameter #7 [ <required> ?string $s1n ]
      }
      - Return [ string ]
    }
    Function [ <internal:rustdemo> function zpp_testing_macro ] {

      - Parameters [8] {
        Parameter #0 [ <required> bool $b ]
        Parameter #1 [ <required> ?bool $bn ]
        Parameter #2 [ <required> int $l ]
        Parameter #3 [ <required> ?int $ln ]
        Parameter #4 [ <required> float $f ]
        Parameter #5 [ <required> ?float $fn ]
        Parameter #6 [ <required> string $s1 ]
        Parameter #7 [ <required> ?string $s1n ]
      }
      - Return [ string ]
    }
    Function [ <internal:rustdemo> function dump_named_variadic_args ] {

      - Parameters [2] {
        Parameter #0 [ <required> int $initial ]
        Parameter #1 [ <optional> mixed ...$args ]
      }
      - Return [ void ]
    }
  }

  - Classes [4] {
    Class [ <internal:rustdemo> class RSDemo ] {

      - Constants [0] {
      }

      - Static properties [0] {
      }

      - Static methods [0] {
      }

      - Properties [1] {
        Property [ private int $inner ]
      }

      - Methods [3] {
        Method [ <internal:rustdemo, ctor> public method __construct ] {

          - Parameters [1] {
            Parameter #0 [ <required> int $initial ]
          }
        }

        Method [ <internal:rustdemo> public method increment ] {

          - Parameters [0] {
          }
          - Return [ void ]
        }

        Method [ <internal:rustdemo> public method asString ] {

          - Parameters [0] {
          }
          - Return [ string ]
        }
      }
    }

    Class [ <internal:rustdemo> class RustDemo\OpaqueData ] {

      - Constants [0] {
      }

      - Static properties [0] {
      }

      - Static methods [0] {
      }

      - Properties [0] {
      }

      - Methods [4] {
        Method [ <internal:rustdemo, ctor> public method __construct ] {

          - Parameters [0] {
          }
        }

        Method [ <internal:rustdemo> public method __destruct ] {

          - Parameters [0] {
          }
        }

        Method [ <internal:rustdemo> public method getBool ] {

          - Parameters [0] {
          }
          - Return [ bool ]
        }

        Method [ <internal:rustdemo> public method setBool ] {

          - Parameters [1] {
            Parameter #0 [ <required> bool $b ]
          }
          - Return [ void ]
        }
      }
    }

    Class [ <internal:rustdemo> class IniDemo ] {

      - Constants [0] {
      }

      - Static properties [0] {
      }

      - Static methods [5] {
        Method [ <internal:rustdemo> static public method get_int ] {

          - Parameters [0] {
          }
          - Return [ int ]
        }

        Method [ <internal:rustdemo> static public method get_float ] {

          - Parameters [0] {
          }
          - Return [ float ]
        }

        Method [ <internal:rustdemo> static public method get_string ] {

          - Parameters [0] {
          }
          - Return [ string ]
        }

        Method [ <internal:rustdemo> static public method get_string_untruncated ] {

          - Parameters [0] {
          }
          - Return [ string ]
        }

        Method [ <internal:rustdemo> static public method get_bool ] {

          - Parameters [0] {
          }
          - Return [ bool ]
        }
      }

      - Properties [0] {
      }

      - Methods [0] {
      }
    }

    Class [ <internal:rustdemo> class PropertiesDemo ] {

      - Constants [0] {
      }

      - Static properties [1] {
        Property [ public static mixed $is_static ]
      }

      - Static methods [0] {
      }

      - Properties [13] {
        Property [ public mixed $is_public ]
        Property [ protected mixed $is_protected ]
        Property [ private mixed $is_private ]
        Property [ public protected(set) readonly mixed $pub_readonly ]
        Property [ protected readonly mixed $prot_readonly ]
        Property [ private readonly mixed $priv_readonly ]
        Property [ public readonly mixed $pub_pub_readonly ]
        Property [ public protected(set) readonly mixed $pub_prot_readonly ]
        Property [ final public private(set) readonly mixed $pub_priv_readonly ]
        Property [ protected readonly mixed $prot_prot_readonly ]
        Property [ final protected private(set) readonly mixed $prot_priv_readonly ]
        Property [ private readonly mixed $priv_priv_readonly ]
        Property [ final public mixed $is_final ]
      }

      - Methods [0] {
      }
    }
  }
}
