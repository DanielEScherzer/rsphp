--TEST--
Display values
--EXTENSIONS--
rustdemo
--FILE--
<?php

class Holder {
	public function __construct(
		public mixed $v
	) {}
}

$val = new Holder(123);

$basic = [
	null,
	false,
	true,
	123,
	PHP_INT_MAX,
	PHP_INT_MIN,
	0.0,
	-0.0,
	1.0,
	PHP_FLOAT_MAX,
	PHP_FLOAT_MIN,
	INF,
	-INF,
	sqrt(-1),
	"",
	"foo",
	"foo\0bar",
	"fizz\0b\xffuz\xC0z",
	[],
	[true],
	[ 2 => 2 ],
	[ "key" => true ],
	(object)[],
	$val,
];

foreach ( $basic as $case ) {
	var_dump( $case );
	display_value( $case );
	echo "\n";
}

?>
--EXPECTF--
NULL
null

bool(false)
false

bool(true)
true

int(123)
123

int(9223372036854775807)
9223372036854775807

int(-9223372036854775808)
-9223372036854775808

float(0)
0e0

float(-0)
-0e0

float(1)
1e0

float(1.7976931348623157E+308)
1.7976931348623157e308

float(2.2250738585072014E-308)
2.2250738585072014e-308

float(INF)
inf

float(-INF)
-inf

float(NAN)
NaN

string(0) ""
string: ""

string(3) "foo"
string: "foo"

string(7) "foo%0bar"
string: "foo%0bar"

string(11) "fizz%0bÿuzÀz"
string: "fizz%0bï¿½uzï¿½z"

array(0) {
}
non-packed array, num_used = 0, num_elements = 0

array(1) {
  [0]=>
  bool(true)
}
packed array, num_used = 1, num_elements = 1

array(1) {
  [2]=>
  int(2)
}
packed array, num_used = 3, num_elements = 1

array(1) {
  ["key"]=>
  bool(true)
}
non-packed array, num_used = 1, num_elements = 1

object(stdClass)#%d (0) {
}
Instance of stdClass

object(Holder)#%d (1) {
  ["v"]=>
  int(123)
}
Instance of Holder
