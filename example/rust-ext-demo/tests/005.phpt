--TEST--
Demo object
--EXTENSIONS--
rustdemo
--FILE--
<?php

$d = new RSDemo(5);
var_dump($d);
?>
--EXPECTF--
object(RSDemo)#%d (1) {
  ["inner":"RSDemo":private]=>
  int(5)
}
