--TEST--
test2() with invalid args
--EXTENSIONS--
rustdemo
--FILE--
<?php
rd_test2([]);
?>
--EXPECTF--
Fatal error: Uncaught TypeError: rd_test2(): Argument #1 ($str) must be of type string, array given in %s:%d
Stack trace:
#0 %s(%d): rd_test2(Array)
#1 {main}
  thrown in %s on line %d
