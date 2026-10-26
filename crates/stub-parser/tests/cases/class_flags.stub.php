<?php

/**
 * @generate-class-entries
 */

/** @strict-properties */
final class Demo {}

/** @not-serializable */
final class Demo2 {}

/**
 * @strict-properties
 * @not-serializable
 */
final class Demo3 {}

abstract class IsAbstract {}

final readonly class IsReadOnly {}

#[\AllowDynamicProperties]
class AllowsDynamicProperties {}
