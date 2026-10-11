--TEST--
test2() Basic test
--EXTENSIONS--
rustdemo
--FILE--
<?php
var_dump(rd_test2());
var_dump(rd_test2('PHP'));
?>
--EXPECT--
string(11) "Hello World"
string(9) "Hello PHP"
