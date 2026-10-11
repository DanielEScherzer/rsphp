--TEST--
test3() Basic test
--EXTENSIONS--
rustdemo
--FILE--
<?php
var_dump(rd_test3("World"));
var_dump(rd_test3(""));
var_dump(rd_test3("PHP\0after"));
var_dump(rd_test3("fizz\0b\xffuz\xC0z"));
try {
	var_dump(rd_test3());
} catch (\Throwable $e) {
	echo $e . "\n\n";
}
try {
	var_dump(rd_test3([]));
} catch (\Throwable $e) {
	echo $e . "\n\n";
}
?>
--EXPECTF--
string(12) "Hello3 World"
string(7) "Hello3 "
string(16) "Hello3 PHP%0after"
string(22) "Hello3 fizz%0b�uz�z"
ArgumentCountError: rd_test3() expects exactly 1 argument, 0 given in %s:%d
Stack trace:
#0 %s(%d): rd_test3()
#1 {main}

TypeError: rd_test3(): Argument #1 ($str) must be of type string, array given in %s:%d
Stack trace:
#0 %s(%d): rd_test3(Array)
#1 {main}
