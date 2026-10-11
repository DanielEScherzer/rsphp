--TEST--
Check if rustdemo is loaded
--EXTENSIONS--
rustdemo
--FILE--
<?php
echo 'The extension "rustdemo" is available';
?>
--EXPECT--
The extension "rustdemo" is available
