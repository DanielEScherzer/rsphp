--TEST--
test1() with invalid args
--EXTENSIONS--
rustdemo
--FILE--
<?php
rd_test1(true);
?>
--EXPECTF--
Fatal error: Uncaught ArgumentCountError: rd_test1() expects exactly 0 arguments, 1 given in %s:%d
Stack trace:
#0 %s(%d): rd_test1(true)
#1 {main}
  thrown in %s on line %d
