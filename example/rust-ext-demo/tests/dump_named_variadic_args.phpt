--TEST--
dump_named_variadic_args()
--EXTENSIONS--
rustdemo
--FILE--
<?php

echo "No named parameters:\n";
dump_named_variadic_args(1, 5, true, "test");

echo "\nWith named parameters:\n";
dump_named_variadic_args(2, key: "value");

echo "\nFailing ZPP due to missing leading int:\n";
try {
	dump_named_variadic_args(key: "value");
} catch (Throwable $e) {
	echo $e;
}

?>
--EXPECTF--
No named parameters:
Initial value: 1
int(5)
bool(true)
string(4) "test"

With named parameters:
Initial value: 2
key:key
string(5) "value"

Failing ZPP due to missing leading int:
ArgumentCountError: dump_named_variadic_args() expects at least 1 argument, 0 given in %s:%d
Stack trace:
#0 %s(%d): dump_named_variadic_args()
#1 {main}
