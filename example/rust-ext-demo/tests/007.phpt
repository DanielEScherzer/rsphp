--TEST--
Demo object - increment overflow
--EXTENSIONS--
rustdemo
--FILE--
<?php

$d = new RSDemo(PHP_INT_MAX - 2);
var_dump($d);
$d->increment();
var_dump($d);
$d->increment();
var_dump($d);
$d->increment();
var_dump($d);
?>
--EXPECTF--
object(RSDemo)#%d (1) {
  ["inner":"RSDemo":private]=>
  int(9223372036854775805)
}
object(RSDemo)#%d (1) {
  ["inner":"RSDemo":private]=>
  int(9223372036854775806)
}
object(RSDemo)#%d (1) {
  ["inner":"RSDemo":private]=>
  int(9223372036854775807)
}

Fatal error: Uncaught ArithmeticError: Attempting to increment beyond maximum value in %s:%d
Stack trace:
#0 %s(%d): RSDemo->increment()
#1 {main}
  thrown in %s on line %d
