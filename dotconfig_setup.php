<?php

function dotLink(string $from, string $to): void
{
	$fromPath = createPath(__DIR__ . "/{$from}");
	$toPath = createPath($to);

	if (!is_link($toPath)) {
		exec("rm -rf \"{$toPath}\"");
		exec("ln -s \"{$fromPath}\" \"{$toPath}\"");
		info("{$fromPath} linked.");
	} else {
		info("Skipping {$fromPath}");
	}
}

function createPath(string|array $paths): string
{
	if (!is_array($paths)) {
		$paths = explode('/', $paths);
	}
	return implode(DIRECTORY_SEPARATOR, $paths);
}

function info(string $message): void
{
	echo "INFO: $message" . PHP_EOL;
}
