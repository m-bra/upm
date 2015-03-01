upm will let you decide how you want to structure your package, it makes as few restrictions as possible.  
However, it somehow *has* to read your package, so you have to do at least one of these:  
  1. Create a script called `.upm-package-load.*` in the root of your package.  
  2. Create a config file called `.upm-package-meta`  

You can leave out the dot at the beginning if you want, it's just there to hide the file on unix if you want.

## The meta file
The meta file (`.upm-package-meta`) contains information about the package its located in and its structure

It has a very basic syntax:  
For every line there is one key-value-binding of the form:  
`key: value`  
Whitespace in both key and value will be trimmed.

## The main script
The `.upm-package-load.*` is the main script. It is used to dynamically generate the meta file .

Before calling the main script, upm will read everything you have already put there. upm will then put some predefined values into the file which the main script can use. Then the main script is called which can put even more bindings in the meta file.  

The main script gets called whenever the package moves its location. However, it may not be called if the package has been moved manually (if it is loose).

## No main script
If there is no main script, upm just uses the meta file as it is.

## Keys written by upm
These keys will be put into the meta file before calling the main script:
 * `platform`: either `Windows`, `Unix` or `Mac` according to the platform the package is loaded in.
 * `package-path`: the path to the package
 * `is-maintained`

## Keys read by upm
 * `name` *(default: name of package directory)*  
The name of the package, should be unique.  

***

 * `version` *(default: `v0.0.0`)*  
The version of the package  

***

 * `description` *(default: `no description`)*  
Short description of the package  

***

 * `contact` *(default: ` `)*
List of contact addresses (e.g. email, website...)

***

 * `kind` *(default: TODO)*
The kind of the package (e.g. `python-library`).
Should fit to form: "Package is a(n) `<kind>`"
Currently used for human reading.

***

 * `install-script` _(default: `scripts/install.*`)_
See Command `upm install`.  
Path relative to package root.

***

 * `test-log` *(default: `scripts/test.log`)*
 * `test-script` _(default: `scripts/test.*`)_  
Script relative to package directory.
Invoked on `upm test`. Should test if this package is able to run flawlessly and has been build properly. Return `0` if this package can be run, anything else on failure. Write into file given by the key `test-log` to return a message.

***

 * `run-script` _(default: `scripts/run.*`)_  
Will be invoked on `upm run`.
Path relative to package root.

***

 * `main-script-dep`  
The dependency of the main script. Must be `upm run`nable.

***

## Script interpreters
You can use any type of script you want (let it be perl, python, bash, windows batch (bad decision))...
Thats why scripts have the ending `.*` in the docs.  
To enable upm to call the scripts, add a shebang (or hashbang) line to your script on unix or associate the file type with the script interpreter if you're on windows.  
And don't forget to add your script interpreters to your [package dependencies](https://github.com/m-bra/upm/wiki/Structure-of-a-package#keys-read-by-upm)!

## Subpackages
Packages inside a package will only be used by upm when referenced to directly.
