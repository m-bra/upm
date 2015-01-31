Every time upm has to know about a package you provide it an **identifier**. The problem is, that, given one identifier, upm has to know which exact package you meant, which can be difficult if there are many packages.

## Identifiers for local packages
Local identifiers are easy. They are just a path to your package, either absolute or relative.  
If you have a relative path, begin your path with `./`.
If you have an absolute path, begin your path with `/` on Unix or `X:` on Windows (with X being the disk).

## Identifiers for global packages
Each package has a **server**, **group**, a **name**, and a **version**.  
An identifier consists of the name and optionally the server, group and/or version of the package.  
There are full identifiers which give all the information, which can only match exactly one or no package *in the entire freaking world* (or the entire server if server is omitted).  
If you leave out some information, multiple package can fit to the identifier. Some commands like `upm copy` will infer missing information.  
The syntax of an identifier is: `[<servers>:][<group>/]<name>[<op><version>]`.  
`<servers>` is a comma separated list of the possible servers the matching package is in, but is usually just one server (if not omitted).

## Versions
`<op>` can be:  
 * `@` - matches the exact version
 * `>` - matches versions above, excluding denoted version
 * `>=` - matches version above, including denoted version
 * `<` - matches versions below, excluding denoted version
 * `<=` - matches version below, including denoted version  

Special case: `<version-from><op><version-to>` for ranges.
For `<op>`:
 * `-` - matches all versions between the two, inclusive
 * `-<` - matches all versions in between, excluding `<version-to>` (because of `<`, see above)
 * `>-` - matches all versions in between, excluding `<version-from>`
 * `>-<` - matches all versions in between, excluding both.

The version has to be a list separated by dots, where each element is a sequence of numbers and letters, and optionally followed by a underscore with a text (the **flag**) following.
Right: `12.5.a`, `12`, `A.1.0`, `1.2.b_debug`  
 
If no flag is specified but there is a underscore (as in `3.0_`), then only packages without flags will pass.
If no flag and underscore is specified, any flag will pass through the identifier.  
For example: `cmake@3.1.1_debug` will not pass through `cmake@3.1.1_` but `cmake@3.1.1`


### Wildcards

`*` acts as a wildcard for an element in the number list of the version.  
So `upm@0.1.1` and `upm@0.1.12` will match for `upm@0.1.*`, even if `12` is two characters.  
You can also use limits as wildcards, for example `upm@0.1.13` will pass to `upm@0.1.>2`, whereas `upm@0.1.1` will not, because the last number is smaller. And `upm@0.2.1` will not pass either because the second number is different.  
Possible operations are: `>`, `>=`, `<`, `<=`  
You can also use range wildcards (with same range-sytax as above), e.g. `upm@0.2->4.?`.

