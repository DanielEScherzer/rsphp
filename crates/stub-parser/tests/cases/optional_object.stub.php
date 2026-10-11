<?php

/**
 * @strict-properties
 * @not-serializable
 */
final class Repository {

	public function commit(
		Oid $treeId,
		string $message,
		array $parents,
		Signature $author,
		?Signature $committer = null,
	): Oid {}
}

/**
 * @strict-properties
 * @not-serializable
 */
final class Oid {}

/**
 * @strict-properties
 * @not-serializable
 */
final class Signature {}
