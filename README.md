# Pronounce

A tool for pronouncing English words using the database of [Oxford Learner’s
Dictionaries][oxford].

## Installation

Clone the repository and then run:

```bash
cargo run cat
```

For convenience of invocation, add an alias to your `.bash_profile`:

```bash
echo "alias pronounce='cargo run --quiet --manifest-path="`pwd`/Cargo.toml"'" >> ~/.bash_profile
```

On macOS, one can also assign a shortcut to the tool so that, regardless of the
active application, a selected word can be easily looked up using a single
keystroke. To this end,

* open Automator,
* select “Service” as the document type,
* add “Run AppleScript” to the workflow,
* copy, paste, and adjust the following code:

```applescript
on run {input, parameters}
	set rustup to "${HOME}/.rustup/toolchains/nightly-x86_64-apple-darwin"
	set pronounce to "<PATH TO THIS REPOSITORY>"
	set command to "PATH=${PATH}:" & rustup & "/bin cargo run --manifest-path=" & pronounce & "/Cargo.toml " & input
	try
		do shell script command
	end try
	return input
end run
```

* save the service,
* open System Preferences,
* go to Keyboard → Shortcuts → Services, and
* look for your service and add a shortcut, for instance, ⌃⌘P.

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[oxford]: http://www.oxfordlearnersdictionaries.com/
