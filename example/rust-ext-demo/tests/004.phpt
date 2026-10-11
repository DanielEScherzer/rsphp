--TEST--
test2() Basic test with null bytes and invalid utf8
--EXTENSIONS--
rustdemo
--FILE--
<?php
var_dump(rd_test2("PHP\0after"));
var_dump(rd_test2("fizz\0b\xffuz\xC0z"));
?>
--EXPECTF--
string(15) "Hello PHP%0after"
string(21) "Hello fizz%0b�uz�z"
