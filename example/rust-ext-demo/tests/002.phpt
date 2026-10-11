--TEST--
test1() Basic test
--EXTENSIONS--
rustdemo
--FILE--
<?php
$ret = rd_test1();

var_dump($ret);
?>
--EXPECT--
The extension rustdemo is loaded and working!
NULL
