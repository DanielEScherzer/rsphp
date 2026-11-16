--TEST--
Color enum tests
--EXTENSIONS--
rustdemo
--FILE--
<?php

$r = new ReflectionEnum(Color::class);
echo $r;

foreach (Color::cases() as $case) {
	var_dump($case, $case->isGreyScale());
}

?>
--EXPECT--
Enum [ <internal:rustdemo> enum Color implements UnitEnum ] {

  - Enum cases [5] {
    Case White
    Case Black
    Case Red
    Case Green
    Case Blue
  }

  - Constants [0] {
  }

  - Static properties [0] {
  }

  - Static methods [1] {
    Method [ <internal:rustdemo, prototype UnitEnum> static public method cases ] {

      - Parameters [0] {
      }
      - Return [ array ]
    }
  }

  - Properties [1] {
    Property [ public protected(set) readonly string $name ]
  }

  - Methods [1] {
    Method [ <internal:rustdemo> public method isGreyScale ] {

      - Parameters [0] {
      }
      - Return [ bool ]
    }
  }
}
enum(Color::White)
bool(true)
enum(Color::Black)
bool(true)
enum(Color::Red)
bool(false)
enum(Color::Green)
bool(false)
enum(Color::Blue)
bool(false)
