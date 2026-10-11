--TEST--
OpaqueData
--EXTENSIONS--
rustdemo
--FILE--
<?php

use RustDemo\OpaqueData;

$od = new OpaqueData();
var_dump($od);

var_dump($od->getBool());
$od->setBool(true);
var_dump($od->getBool());
$od->setBool(false);
var_dump($od->getBool());

?>
--EXPECTF--
object(RustDemo\OpaqueData)#%d (0) {
}
bool(false)
bool(true)
bool(false)
