<?php

namespace RSGit;

/**
 * @strict-properties
 * @not-serializable
 */
final class Commit {

	public readonly Oid $id;
	public readonly Signature $author;
	public readonly Signature $committer;

}

/**
 * @strict-properties
 * @not-serializable
 */
final class Oid {

	public readonly string $asHex;

	public function __toString(): string {}
}

/**
 * @strict-properties
 * @not-serializable
 */
final class Signature {

	public readonly string $name;
	public readonly string $email;
	public readonly \DateTimeImmutable $time;

}
