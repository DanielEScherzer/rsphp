--TEST--
zpp_testing() basic testing
--EXTENSIONS--
rustdemo
--FILE--
<?php

class OptionYielder {
	public function __construct(private array $values, private ?OptionYielder $inner = null) {}

	public function yieldOptions(): Generator {
		if ($this->inner === null) {
			foreach ( $this->values as $val ) {
				yield [ $val ];
			}
			return;
		}
		foreach ($this->inner->yieldOptions() as $innerOpt) {
			foreach ( $this->values as $val ) {
				yield [...$innerOpt, $val];
			}
		}
	}
}

$bool = new OptionYielder([true]);
$nullbull = new OptionYielder([false, null], $bool);
$int = new OptionYielder([1], $nullbull);
$nullInt = new OptionYielder([9, null], $int);
$float = new OptionYielder([3.14], $nullInt);
$nullFloat = new OptionYielder([12.34, null], $float);
$string = new OptionYielder(["foo", "baz\0qux"], $nullFloat);
$nullString = new OptionYielder(["fizz\0b\xffuz\xC0z", null], $string);

$bad = [];
foreach ($nullString->yieldOptions() as $options) {
	$result1 = zpp_testing(...$options);
	var_dump( $result1 );

	$result2 = zpp_testing_macro(...$options);
	var_dump( $result2 );

	if ($result1 !== $result2) {
		$bad[] = [$options, $result1, $result2];
	}
}

var_dump($bad);
?>
--EXPECTF--
string(104) "bool=true, ?bool=false, long=1, ?long=9, double=3.14, ?double=12.34, string=foo, ?string=fizz%0b�uz�z"
string(104) "bool=true, ?bool=false, long=1, ?long=9, double=3.14, ?double=12.34, string=foo, ?string=fizz%0b�uz�z"
string(93) "bool=true, ?bool=false, long=1, ?long=9, double=3.14, ?double=12.34, string=foo, ?string=null"
string(93) "bool=true, ?bool=false, long=1, ?long=9, double=3.14, ?double=12.34, string=foo, ?string=null"
string(108) "bool=true, ?bool=false, long=1, ?long=9, double=3.14, ?double=12.34, string=baz%0qux, ?string=fizz%0b�uz�z"
string(108) "bool=true, ?bool=false, long=1, ?long=9, double=3.14, ?double=12.34, string=baz%0qux, ?string=fizz%0b�uz�z"
string(97) "bool=true, ?bool=false, long=1, ?long=9, double=3.14, ?double=12.34, string=baz%0qux, ?string=null"
string(97) "bool=true, ?bool=false, long=1, ?long=9, double=3.14, ?double=12.34, string=baz%0qux, ?string=null"
string(103) "bool=true, ?bool=false, long=1, ?long=9, double=3.14, ?double=null, string=foo, ?string=fizz%0b�uz�z"
string(103) "bool=true, ?bool=false, long=1, ?long=9, double=3.14, ?double=null, string=foo, ?string=fizz%0b�uz�z"
string(92) "bool=true, ?bool=false, long=1, ?long=9, double=3.14, ?double=null, string=foo, ?string=null"
string(92) "bool=true, ?bool=false, long=1, ?long=9, double=3.14, ?double=null, string=foo, ?string=null"
string(107) "bool=true, ?bool=false, long=1, ?long=9, double=3.14, ?double=null, string=baz%0qux, ?string=fizz%0b�uz�z"
string(107) "bool=true, ?bool=false, long=1, ?long=9, double=3.14, ?double=null, string=baz%0qux, ?string=fizz%0b�uz�z"
string(96) "bool=true, ?bool=false, long=1, ?long=9, double=3.14, ?double=null, string=baz%0qux, ?string=null"
string(96) "bool=true, ?bool=false, long=1, ?long=9, double=3.14, ?double=null, string=baz%0qux, ?string=null"
string(104) "bool=true, ?bool=false, long=1, ?long=0, double=3.14, ?double=12.34, string=foo, ?string=fizz%0b�uz�z"
string(104) "bool=true, ?bool=false, long=1, ?long=0, double=3.14, ?double=12.34, string=foo, ?string=fizz%0b�uz�z"
string(93) "bool=true, ?bool=false, long=1, ?long=0, double=3.14, ?double=12.34, string=foo, ?string=null"
string(93) "bool=true, ?bool=false, long=1, ?long=0, double=3.14, ?double=12.34, string=foo, ?string=null"
string(108) "bool=true, ?bool=false, long=1, ?long=0, double=3.14, ?double=12.34, string=baz%0qux, ?string=fizz%0b�uz�z"
string(108) "bool=true, ?bool=false, long=1, ?long=0, double=3.14, ?double=12.34, string=baz%0qux, ?string=fizz%0b�uz�z"
string(97) "bool=true, ?bool=false, long=1, ?long=0, double=3.14, ?double=12.34, string=baz%0qux, ?string=null"
string(97) "bool=true, ?bool=false, long=1, ?long=0, double=3.14, ?double=12.34, string=baz%0qux, ?string=null"
string(103) "bool=true, ?bool=false, long=1, ?long=0, double=3.14, ?double=null, string=foo, ?string=fizz%0b�uz�z"
string(103) "bool=true, ?bool=false, long=1, ?long=0, double=3.14, ?double=null, string=foo, ?string=fizz%0b�uz�z"
string(92) "bool=true, ?bool=false, long=1, ?long=0, double=3.14, ?double=null, string=foo, ?string=null"
string(92) "bool=true, ?bool=false, long=1, ?long=0, double=3.14, ?double=null, string=foo, ?string=null"
string(107) "bool=true, ?bool=false, long=1, ?long=0, double=3.14, ?double=null, string=baz%0qux, ?string=fizz%0b�uz�z"
string(107) "bool=true, ?bool=false, long=1, ?long=0, double=3.14, ?double=null, string=baz%0qux, ?string=fizz%0b�uz�z"
string(96) "bool=true, ?bool=false, long=1, ?long=0, double=3.14, ?double=null, string=baz%0qux, ?string=null"
string(96) "bool=true, ?bool=false, long=1, ?long=0, double=3.14, ?double=null, string=baz%0qux, ?string=null"
string(103) "bool=true, ?bool=null, long=1, ?long=9, double=3.14, ?double=12.34, string=foo, ?string=fizz%0b�uz�z"
string(103) "bool=true, ?bool=null, long=1, ?long=9, double=3.14, ?double=12.34, string=foo, ?string=fizz%0b�uz�z"
string(92) "bool=true, ?bool=null, long=1, ?long=9, double=3.14, ?double=12.34, string=foo, ?string=null"
string(92) "bool=true, ?bool=null, long=1, ?long=9, double=3.14, ?double=12.34, string=foo, ?string=null"
string(107) "bool=true, ?bool=null, long=1, ?long=9, double=3.14, ?double=12.34, string=baz%0qux, ?string=fizz%0b�uz�z"
string(107) "bool=true, ?bool=null, long=1, ?long=9, double=3.14, ?double=12.34, string=baz%0qux, ?string=fizz%0b�uz�z"
string(96) "bool=true, ?bool=null, long=1, ?long=9, double=3.14, ?double=12.34, string=baz%0qux, ?string=null"
string(96) "bool=true, ?bool=null, long=1, ?long=9, double=3.14, ?double=12.34, string=baz%0qux, ?string=null"
string(102) "bool=true, ?bool=null, long=1, ?long=9, double=3.14, ?double=null, string=foo, ?string=fizz%0b�uz�z"
string(102) "bool=true, ?bool=null, long=1, ?long=9, double=3.14, ?double=null, string=foo, ?string=fizz%0b�uz�z"
string(91) "bool=true, ?bool=null, long=1, ?long=9, double=3.14, ?double=null, string=foo, ?string=null"
string(91) "bool=true, ?bool=null, long=1, ?long=9, double=3.14, ?double=null, string=foo, ?string=null"
string(106) "bool=true, ?bool=null, long=1, ?long=9, double=3.14, ?double=null, string=baz%0qux, ?string=fizz%0b�uz�z"
string(106) "bool=true, ?bool=null, long=1, ?long=9, double=3.14, ?double=null, string=baz%0qux, ?string=fizz%0b�uz�z"
string(95) "bool=true, ?bool=null, long=1, ?long=9, double=3.14, ?double=null, string=baz%0qux, ?string=null"
string(95) "bool=true, ?bool=null, long=1, ?long=9, double=3.14, ?double=null, string=baz%0qux, ?string=null"
string(103) "bool=true, ?bool=null, long=1, ?long=0, double=3.14, ?double=12.34, string=foo, ?string=fizz%0b�uz�z"
string(103) "bool=true, ?bool=null, long=1, ?long=0, double=3.14, ?double=12.34, string=foo, ?string=fizz%0b�uz�z"
string(92) "bool=true, ?bool=null, long=1, ?long=0, double=3.14, ?double=12.34, string=foo, ?string=null"
string(92) "bool=true, ?bool=null, long=1, ?long=0, double=3.14, ?double=12.34, string=foo, ?string=null"
string(107) "bool=true, ?bool=null, long=1, ?long=0, double=3.14, ?double=12.34, string=baz%0qux, ?string=fizz%0b�uz�z"
string(107) "bool=true, ?bool=null, long=1, ?long=0, double=3.14, ?double=12.34, string=baz%0qux, ?string=fizz%0b�uz�z"
string(96) "bool=true, ?bool=null, long=1, ?long=0, double=3.14, ?double=12.34, string=baz%0qux, ?string=null"
string(96) "bool=true, ?bool=null, long=1, ?long=0, double=3.14, ?double=12.34, string=baz%0qux, ?string=null"
string(102) "bool=true, ?bool=null, long=1, ?long=0, double=3.14, ?double=null, string=foo, ?string=fizz%0b�uz�z"
string(102) "bool=true, ?bool=null, long=1, ?long=0, double=3.14, ?double=null, string=foo, ?string=fizz%0b�uz�z"
string(91) "bool=true, ?bool=null, long=1, ?long=0, double=3.14, ?double=null, string=foo, ?string=null"
string(91) "bool=true, ?bool=null, long=1, ?long=0, double=3.14, ?double=null, string=foo, ?string=null"
string(106) "bool=true, ?bool=null, long=1, ?long=0, double=3.14, ?double=null, string=baz%0qux, ?string=fizz%0b�uz�z"
string(106) "bool=true, ?bool=null, long=1, ?long=0, double=3.14, ?double=null, string=baz%0qux, ?string=fizz%0b�uz�z"
string(95) "bool=true, ?bool=null, long=1, ?long=0, double=3.14, ?double=null, string=baz%0qux, ?string=null"
string(95) "bool=true, ?bool=null, long=1, ?long=0, double=3.14, ?double=null, string=baz%0qux, ?string=null"
array(0) {
}
