--TEST--
Demo object - to string
--EXTENSIONS--
rustdemo
--FILE--
<?php

$d = new RSDemo(PHP_INT_MAX - 2);
var_dump($d);
var_dump($d->asString());
$d->increment();
var_dump($d);
var_dump($d->asString());
?>
--EXPECTF--
object(RSDemo)#%d (1) {
  ["inner":"RSDemo":private]=>
  int(9223372036854775805)
}
string(19) "9223372036854775805"
object(RSDemo)#%d (1) {
  ["inner":"RSDemo":private]=>
  int(9223372036854775806)
}
string(19) "9223372036854775806"
