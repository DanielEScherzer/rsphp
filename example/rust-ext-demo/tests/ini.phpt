--TEST--
Ini usage
--EXTENSIONS--
rustdemo
--FILE--
<?php

function dump() {
	echo "As int: ";
	var_dump(IniDemo::get_int());
	echo "As float: ";
	var_dump(IniDemo::get_float());
	echo "As string: ";
	var_dump(IniDemo::get_string());
	echo "As untruncated string: ";
	var_dump(IniDemo::get_string_untruncated());
	echo "As bool: ";
	var_dump(IniDemo::get_bool());
	echo "\n";
}


dump();
ini_set("rustdemo.ini_setting", 5);
dump();
ini_set("rustdemo.ini_setting", 12.34);
dump();
ini_set("rustdemo.ini_setting", "abc\x00123");
dump();

?>
--EXPECTF--
As int: int(1234)
As float: float(1234)
As string: string(4) "1234"
As untruncated string: string(4) "1234"
As bool: bool(true)

As int: int(5)
As float: float(5)
As string: string(1) "5"
As untruncated string: string(1) "5"
As bool: bool(true)

As int: int(12)
As float: float(12.34)
As string: string(5) "12.34"
As untruncated string: string(5) "12.34"
As bool: bool(true)

As int: int(0)
As float: float(0)
As string: string(3) "abc"
As untruncated string: string(7) "abc%0123"
As bool: bool(false)
